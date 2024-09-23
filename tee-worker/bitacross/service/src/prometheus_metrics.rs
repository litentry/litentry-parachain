/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

//! Service for prometheus metrics, hosted on a http server.

use crate::{
	account_funding::EnclaveAccountInfo,
	error::{Error, ServiceResult},
};
use async_trait::async_trait;
use codec::{Decode, Encode};
#[cfg(feature = "attesteer")]
use core::time::Duration;
use frame_support::scale_info::TypeInfo;
#[cfg(feature = "dcap")]
use itc_rest_client::{
	http_client::{DefaultSend, HttpClient},
	rest_client::{RestClient, Url as URL},
	RestGet, RestPath,
};
use itp_enclave_metrics::EnclaveMetric;
use lazy_static::lazy_static;
use log::*;
use prometheus::{
	proto::MetricFamily, register_counter, register_histogram, register_histogram_vec,
	register_int_gauge, register_int_gauge_vec, Counter, Histogram, HistogramVec, IntGauge,
	IntGaugeVec,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use warp::{Filter, Rejection, Reply};

lazy_static! {
	/// Register all the prometheus metrics we want to monitor (aside from the default process ones).
	static ref ENCLAVE_PARENTCHAIN_BLOCK_IMPORT_TIME: Histogram =
		register_histogram!("bitacross_worker_enclave_parentchain_block_import_time", "Time taken to import parentchain block")
			.unwrap();
	static ref MUSIG2_CEREMONIES_STARTED: Counter =
		register_counter!("bitacross_worker_ceremonies_started", "Musig2 ceremonies started")
			.unwrap();
	static ref MUSIG2_CEREMONIES_FAILED: Counter =
		register_counter!("bitacross_worker_ceremonies_failed", "Musig2 ceremonies failed")
			.unwrap();
	static ref MUSIG2_CEREMONIES_TIMED_OUT: Counter =
		register_counter!("bitacross_worker_ceremonies_timed_out", "Musig2 ceremonies timed out")
			.unwrap();
	static ref MUSIG2_CEREMONY_DURATION: Histogram =
		register_histogram!("bitacross_worker_ceremony_duration", "Time taken to perform musig2 ceremony", vec![0.0005, 0.005, 0.01, 0.025, 0.05, 0.1])
			.unwrap();


}

pub async fn start_metrics_server<MetricsHandler>(
	metrics_handler: Arc<MetricsHandler>,
	port: u16,
) -> ServiceResult<()>
where
	MetricsHandler: HandleMetrics + Send + Sync + 'static,
{
	let metrics_route = warp::path!("metrics").and_then(move || {
		let handler_clone = metrics_handler.clone();
		async move { handler_clone.handle_metrics().await }
	});
	let socket_addr: SocketAddr = ([0, 0, 0, 0], port).into();

	info!("Running prometheus metrics server on: {:?}", socket_addr);
	warp::serve(metrics_route).run(socket_addr).await;

	info!("Prometheus metrics server shut down");
	Ok(())
}

#[async_trait]
pub trait HandleMetrics {
	type ReplyType: Reply;

	async fn handle_metrics(&self) -> Result<Self::ReplyType, Rejection>;
}

/// Metrics handler implementation.
pub struct MetricsHandler {}

#[async_trait]
impl HandleMetrics for MetricsHandler {
	type ReplyType = String;

	async fn handle_metrics(&self) -> Result<Self::ReplyType, Rejection> {
		let default_metrics = match gather_metrics_into_reply(&prometheus::gather()) {
			Ok(r) => r,
			Err(e) => {
				error!("Failed to gather prometheus metrics: {:?}", e);
				String::default()
			},
		};

		Ok(default_metrics)
	}
}

fn gather_metrics_into_reply(metrics: &[MetricFamily]) -> ServiceResult<String> {
	use prometheus::Encoder;
	let encoder = prometheus::TextEncoder::new();

	let mut buffer = Vec::new();
	encoder.encode(metrics, &mut buffer).map_err(|e| {
		Error::Custom(format!("Failed to encode prometheus metrics: {:?}", e).into())
	})?;

	let result_string = String::from_utf8(buffer).map_err(|e| {
		Error::Custom(
			format!("Failed to convert Prometheus encoded metrics to UTF8: {:?}", e).into(),
		)
	})?;

	Ok(result_string)
}

/// Trait to receive metric updates from inside the enclave.
pub trait ReceiveEnclaveMetrics {
	fn receive_enclave_metric(&self, metric: EnclaveMetric) -> ServiceResult<()>;
}

pub struct EnclaveMetricsReceiver;

impl ReceiveEnclaveMetrics for EnclaveMetricsReceiver {
	fn receive_enclave_metric(&self, metric: EnclaveMetric) -> ServiceResult<()> {
		match metric {
			EnclaveMetric::ParentchainBlockImportTime(time) =>
				ENCLAVE_PARENTCHAIN_BLOCK_IMPORT_TIME.observe(time.as_secs_f64()),
			EnclaveMetric::Musig2CeremonyStarted => MUSIG2_CEREMONIES_STARTED.inc(),
			EnclaveMetric::Musig2CeremonyFailed => MUSIG2_CEREMONIES_FAILED.inc(),
			EnclaveMetric::Musig2CeremonyTimedout(count) =>
				for i in 0..count {
					MUSIG2_CEREMONIES_TIMED_OUT.inc()
				},
			EnclaveMetric::Musig2CeremonyDuration(time) =>
				MUSIG2_CEREMONY_DURATION.observe(time.as_secs_f64()),
		}
		Ok(())
	}
}

#[derive(Serialize, Deserialize, Debug)]
struct PrometheusMarblerunEvents(pub Vec<PrometheusMarblerunEvent>);

#[cfg(feature = "attesteer")]
impl RestPath<&str> for PrometheusMarblerunEvents {
	fn get_path(path: &str) -> Result<String, itc_rest_client::error::Error> {
		Ok(format!("{}", path))
	}
}

#[cfg(feature = "attesteer")]
pub fn fetch_marblerun_events(base_url: &str) -> Result<Vec<PrometheusMarblerunEvent>, Error> {
	let base_url = URL::parse(&base_url).map_err(|e| {
		Error::Custom(
			format!("Failed to parse marblerun prometheus endpoint base URL: {:?}", e).into(),
		)
	})?;
	let timeout = 3u64;
	let http_client =
		HttpClient::new(DefaultSend {}, true, Some(Duration::from_secs(timeout)), None, None);

	let mut rest_client = RestClient::new(http_client, base_url.clone());
	let events: PrometheusMarblerunEvents = rest_client.get("events").map_err(|e| {
		Error::Custom(
			format!("Failed to fetch marblerun prometheus events from: {}, error: {}", base_url, e)
				.into(),
		)
	})?;

	Ok(events.0)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
pub struct PrometheusMarblerunEvent {
	pub time: String,
	pub activation: PrometheusMarblerunEventActivation,
}

#[cfg(feature = "attesteer")]
impl PrometheusMarblerunEvent {
	pub fn get_quote_without_prepended_bytes(&self) -> &[u8] {
		let marblerun_magic_prepended_header_size = 16usize;
		&self.activation.quote.as_bytes()[marblerun_magic_prepended_header_size..]
	}
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct PrometheusMarblerunEventActivation {
	pub marble_type: String,
	pub uuid: String,
	pub quote: String,
}
