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

#[cfg(feature = "teeracle")]
use crate::teeracle::teeracle_metrics::update_teeracle_metrics;

use crate::{
	account_funding::EnclaveAccountInfo,
	error::{Error, ServiceResult},
};
use async_trait::async_trait;
use codec::{Decode, Encode};
#[cfg(feature = "dcap")]
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
use lc_stf_task_sender::RequestType;
use litentry_primitives::{
	Assertion, Identity, IdentityNetworkTuple, Web2ValidationData, Web3Network,
};
use log::*;
use prometheus::{
	proto::MetricFamily, register_histogram_vec, register_int_gauge, register_int_gauge_vec,
	HistogramVec, IntGauge, IntGaugeVec, Opts,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use warp::{Filter, Rejection, Reply};

lazy_static! {
	/// Register all the prometheus metrics we want to monitor (aside from the default process ones).

	static ref ENCLAVE_ACCOUNT_FREE_BALANCE: IntGauge =
		register_int_gauge!("litentry_worker_enclave_account_free_balance", "Free balance of the enclave account")
			.unwrap();
	static ref ENCLAVE_SIDECHAIN_BLOCK_HEIGHT: IntGauge =
		register_int_gauge!("litentry_worker_enclave_sidechain_block_height", "Enclave sidechain block height")
			.unwrap();
	static ref ENCLAVE_SIDECHAIN_TOP_POOL_SIZE: IntGauge =
		register_int_gauge!("litentry_worker_enclave_sidechain_top_pool_size", "Enclave sidechain top pool size")
			.unwrap();
	static ref ENCLAVE_STF_CALLS: IntGaugeVec =
		register_int_gauge_vec!("litentry_worker_enclave_stf_total_calls", "Litentry Stf Calls", &["a", "b"])
			.unwrap();
	static ref ENCLAVE_STF_CALLS_EXECUTION: HistogramVec =
		register_histogram_vec!("litentry_worker_enclave_stf_exeuction_times", "Litentry Stf Call Exeuction Time", &["a", "b"])
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
pub struct MetricsHandler<Wallet> {
	enclave_wallet: Arc<Wallet>,
}

#[async_trait]
impl<Wallet> HandleMetrics for MetricsHandler<Wallet>
where
	Wallet: EnclaveAccountInfo + Send + Sync,
{
	type ReplyType = String;

	async fn handle_metrics(&self) -> Result<Self::ReplyType, Rejection> {
		self.update_metrics().await;

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

impl<Wallet> MetricsHandler<Wallet>
where
	Wallet: EnclaveAccountInfo + Send + Sync,
{
	pub fn new(enclave_wallet: Arc<Wallet>) -> Self {
		MetricsHandler { enclave_wallet }
	}

	async fn update_metrics(&self) {
		match self.enclave_wallet.free_balance() {
			Ok(b) => {
				ENCLAVE_ACCOUNT_FREE_BALANCE.set(b as i64);
			},
			Err(e) => {
				error!("Failed to fetch free balance metric, value will not be updated: {:?}", e);
			},
		}
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
			EnclaveMetric::SetSidechainBlockHeight(h) => {
				ENCLAVE_SIDECHAIN_BLOCK_HEIGHT.set(h as i64);
			},
			EnclaveMetric::TopPoolSizeSet(pool_size) => {
				ENCLAVE_SIDECHAIN_TOP_POOL_SIZE.set(pool_size as i64);
			},
			EnclaveMetric::TopPoolSizeIncrement => {
				ENCLAVE_SIDECHAIN_TOP_POOL_SIZE.inc();
			},
			EnclaveMetric::TopPoolSizeDecrement => {
				ENCLAVE_SIDECHAIN_TOP_POOL_SIZE.dec();
			},
			EnclaveMetric::StfCallIncrement(request) => {
				handle_stf_call_request(request)
				// ENCLAVE_STF_CALLS.with_label_values(&["link_identity", "web3"]).inc();
			},
			EnclaveMetric::StfCallObserveExecutionTime(time, req) => {
				log::debug!("Observing Execution time for Histogram onto registry");
				// ENCLAVE_STF_CALLS_EXECUTION.with_label_values(&["link_identity", "Twitter"]).observe(time)
				observe_stf_call_execution_time(time, req);
			},
			#[cfg(feature = "teeracle")]
			EnclaveMetric::ExchangeRateOracle(m) => update_teeracle_metrics(m)?,
			#[cfg(not(feature = "teeracle"))]
			EnclaveMetric::ExchangeRateOracle(_) => {
				error!("Received Teeracle metric, but Teeracle feature is not enabled, ignoring metric item.")
			},
		}
		Ok(())
	}
}

fn handle_stf_call_request(req: RequestType) {
	match req {
		RequestType::IdentityVerification(request) => match request.identity {
			Identity::Twitter(_) =>
				ENCLAVE_STF_CALLS.with_label_values(&["link_identity", "Twitter"]).inc(),
			Identity::Discord(_) =>
				ENCLAVE_STF_CALLS.with_label_values(&["link_identity", "Discord"]).inc(),
			Identity::Github(_) =>
				ENCLAVE_STF_CALLS.with_label_values(&["link_identity", "Github"]).inc(),
			Identity::Substrate(_) =>
				ENCLAVE_STF_CALLS.with_label_values(&["link_identity", "Substrate"]).inc(),
			Identity::Evm(_) =>
				ENCLAVE_STF_CALLS.with_label_values(&["link_identity", "Evm"]).inc(),
		},
		RequestType::AssertionVerification(request) => match request.assertion {
			Assertion::A1 => ENCLAVE_STF_CALLS.with_label_values(&["request_vc", "A1"]).inc(),
			Assertion::A2(_) => ENCLAVE_STF_CALLS.with_label_values(&["request_vc", "A2"]).inc(),
			Assertion::A3(..) => ENCLAVE_STF_CALLS.with_label_values(&["request_vc", "A3"]).inc(),
			Assertion::A4(_) => ENCLAVE_STF_CALLS.with_label_values(&["request_vc", "A4"]).inc(),
			Assertion::A6 => ENCLAVE_STF_CALLS.with_label_values(&["request_vc", "A6"]).inc(),
			Assertion::A7(_) => ENCLAVE_STF_CALLS.with_label_values(&["request_vc", "A7"]).inc(),
			Assertion::A8(_) => ENCLAVE_STF_CALLS.with_label_values(&["request_vc", "A8"]).inc(),
			Assertion::A9 => ENCLAVE_STF_CALLS.with_label_values(&["request_vc", "A9"]).inc(),
			Assertion::A10(_) => ENCLAVE_STF_CALLS.with_label_values(&["request_vc", "A10"]).inc(),
			Assertion::A11(_) => ENCLAVE_STF_CALLS.with_label_values(&["request_vc", "A11"]).inc(),
			Assertion::A13(_) => ENCLAVE_STF_CALLS.with_label_values(&["request_vc", "A13"]).inc(),
			Assertion::A14 => ENCLAVE_STF_CALLS.with_label_values(&["request_vc", "A14"]).inc(),
			Assertion::Achainable(..) =>
				ENCLAVE_STF_CALLS.with_label_values(&["request_vc", "Achainable"]).inc(),
		},
	}
}

fn observe_stf_call_execution_time(time: f64, req: RequestType) {
	match req {
		RequestType::IdentityVerification(request) => match request.identity {
			Identity::Twitter(_) => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["link_identity", "Twitter"])
				.observe(time),
			Identity::Discord(_) => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["link_identity", "Discord"])
				.observe(time),
			Identity::Github(_) => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["link_identity", "Github"])
				.observe(time),
			Identity::Substrate(_) => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["link_identity", "Substrate"])
				.observe(time),
			Identity::Evm(_) => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["link_identity", "Evm"])
				.observe(time),
		},
		RequestType::AssertionVerification(request) => match request.assertion {
			Assertion::A1 => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["request_vc", "A1"])
				.observe(time),
			Assertion::A2(_) => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["request_vc", "A2"])
				.observe(time),
			Assertion::A3(..) => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["request_vc", "A3"])
				.observe(time),
			Assertion::A4(_) => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["request_vc", "A4"])
				.observe(time),
			Assertion::A6 => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["request_vc", "A6"])
				.observe(time),
			Assertion::A7(_) => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["request_vc", "A7"])
				.observe(time),
			Assertion::A8(_) => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["request_vc", "A8"])
				.observe(time),
			Assertion::A9 => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["request_vc", "A9"])
				.observe(time),
			Assertion::A10(_) => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["request_vc", "A10"])
				.observe(time),
			Assertion::A11(_) => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["request_vc", "A11"])
				.observe(time),
			Assertion::A13(_) => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["request_vc", "A13"])
				.observe(time),
			Assertion::A14 => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["request_vc", "A14"])
				.observe(time),
			Assertion::Achainable(..) => ENCLAVE_STF_CALLS_EXECUTION
				.with_label_values(&["request_vc", "Achainable"])
				.observe(time),
		},
	}
}

#[derive(Serialize, Deserialize, Debug)]
struct PrometheusMarblerunEvents(pub Vec<PrometheusMarblerunEvent>);

#[cfg(feature = "dcap")]
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
