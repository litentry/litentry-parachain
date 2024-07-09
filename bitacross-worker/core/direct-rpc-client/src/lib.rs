// Copyright 2020-2024 Trust Computing GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use rustls_sgx as rustls;
	pub use tungstenite_sgx as tungstenite;
	pub use webpki_sgx as webpki;
}

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

extern crate alloc;

use alloc::format;

use core::str::FromStr;

use log::{debug, error};

use serde_json::from_str;

use itp_rpc::{Id, RpcRequest, RpcResponse, RpcReturnValue};

use itp_utils::FromHexPrefixed;

use std::{
	boxed::Box,
	error::Error,
	net::TcpStream,
	string::String,
	sync::{
		mpsc::{channel, Sender},
		Arc,
	},
	time::Duration,
	vec::Vec,
};
use tungstenite::{client_tls_with_config, stream::MaybeTlsStream, Connector, Message, WebSocket};
use url::Url;
use webpki::{DNSName, DNSNameRef};

pub type Response = (Id, RpcReturnValue);

pub struct IgnoreCertVerifier {}

impl rustls::ServerCertVerifier for IgnoreCertVerifier {
	fn verify_server_cert(
		&self,
		_: &rustls::RootCertStore,
		_: &[rustls::Certificate],
		_: DNSNameRef<'_>,
		_: &[u8],
	) -> Result<rustls::ServerCertVerified, rustls::TLSError> {
		log::warn!("Using NoCertVerifier");
		Ok(rustls::ServerCertVerified::assertion())
	}
}

impl rustls::ClientCertVerifier for IgnoreCertVerifier {
	fn client_auth_root_subjects(
		&self,
		_sni: Option<&DNSName>,
	) -> Option<rustls::DistinguishedNames> {
		None
	}

	fn verify_client_cert(
		&self,
		_presented_certs: &[rustls::Certificate],
		_sni: Option<&DNSName>,
	) -> Result<rustls::ClientCertVerified, rustls::TLSError> {
		Ok(rustls::ClientCertVerified::assertion())
	}
}

pub trait RpcClientFactory {
	type Client: RpcClient;
	fn create(
		&self,
		url: &str,
		response_sink: Sender<Response>,
	) -> Result<Self::Client, Box<dyn Error>>;
}

pub struct DirectRpcClientFactory {}

impl RpcClientFactory for DirectRpcClientFactory {
	type Client = DirectRpcClient;

	fn create(
		&self,
		url: &str,
		response_sink: Sender<Response>,
	) -> Result<Self::Client, Box<dyn Error>> {
		DirectRpcClient::new(url, response_sink)
	}
}

pub trait RpcClient {
	fn send(&mut self, request: &RpcRequest) -> Result<(), Box<dyn Error>>;
}

pub struct DirectRpcClient {
	request_sink: Sender<String>,
}

impl DirectRpcClient {
	pub fn new(url: &str, response_sink: Sender<Response>) -> Result<Self, Box<dyn Error>> {
		let server_url =
			Url::from_str(url).map_err(|e| format!("Could not connect, reason: {:?}", e))?;
		let mut config = rustls::ClientConfig::new();
		// we need to set this cert verifier or client will fail to connect with following error
		// HandshakeError::Failure(Io(Custom { kind: InvalidData, error: WebPKIError(UnknownIssuer) }))
		config.dangerous().set_certificate_verifier(Arc::new(IgnoreCertVerifier {}));
		let connector = Connector::Rustls(Arc::new(config));
		let stream = TcpStream::connect(server_url.authority())
			.map_err(|e| format!("Could not connect to {:?}, reason: {:?}", url, e))?;

		let (mut socket, _response) =
			client_tls_with_config(server_url.as_str(), stream, None, Some(connector))
				.map_err(|e| format!("Could not open websocket connection: {:?}", e))?;

		let (request_sender, request_receiver) = channel();

		//it fails to perform handshake in non_blocking mode so we are setting it up after the handshake is performed
		Self::switch_to_non_blocking(&mut socket);

		std::thread::spawn(move || {
			loop {
				// let's flush all pending requests first
				while let Ok(request) = request_receiver.try_recv() {
					if let Err(e) = socket.write_message(Message::Text(request)) {
						error!("Could not write message to socket, reason: {:?}", e)
					}
				}

				if let Ok(message) = socket.read_message() {
					if let Ok(Some(response)) = Self::handle_ws_message(message) {
						if let Err(e) = response_sink.send(response) {
							log::error!("Could not forward response, reason: {:?}", e)
						};
					}
				}
				std::thread::sleep(Duration::from_millis(1))
			}
		});
		debug!("Connected to peer: {}", url);
		Ok(Self { request_sink: request_sender })
	}

	fn switch_to_non_blocking(socket: &mut WebSocket<MaybeTlsStream<TcpStream>>) {
		match socket.get_ref() {
			MaybeTlsStream::Plain(stream) => {
				stream.set_nonblocking(true).expect("set_nonblocking call failed");
				stream
					.set_read_timeout(Some(Duration::from_millis(5)))
					.expect("set_read_timeout call failed");
			},
			MaybeTlsStream::Rustls(stream) => {
				stream.get_ref().set_nonblocking(true).expect("set_nonblocking call failed");
				stream
					.get_ref()
					.set_read_timeout(Some(Duration::from_millis(1)))
					.expect("set_read_timeout call failed");
			},
			_ => {},
		}
	}

	fn handle_ws_message(message: Message) -> Result<Option<Response>, Box<dyn Error>> {
		match message {
			Message::Text(text) => {
				let rpc_response: RpcResponse = from_str(&text)
					.map_err(|e| format!("Could not deserialize RpcResponse, reason: {:?}", e))?;
				let return_value: RpcReturnValue =
					RpcReturnValue::from_hex(&rpc_response.result)
						.map_err(|e| format!("Could not deserialize value , reason: {:?}", e))?;
				Ok(Some((rpc_response.id, return_value)))
			},
			_ => {
				log::warn!("Only text messages are supported");
				Ok(None)
			},
		}
	}
}

#[derive(Clone)]
pub enum RequestParams {
	Rsa(Vec<String>),
	Aes(Vec<String>),
}

impl RpcClient for DirectRpcClient {
	fn send(&mut self, request: &RpcRequest) -> Result<(), Box<dyn Error>> {
		let request = serde_json::to_string(request)
			.map_err(|e| format!("Could not parse RpcRequest {:?}", e))?;
		self.request_sink
			.send(request)
			.map_err(|e| format!("Could not write message, reason: {:?}", e).into())
	}
}

#[cfg(test)]
mod tests {
	use crate::DirectRpcClient;
	use itp_rpc::{Id, RpcResponse, RpcReturnValue};
	use itp_types::{DirectRequestStatus, TrustedOperationStatus, H256};
	use itp_utils::ToHexPrefixed;
	use tungstenite::Message;

	#[test]
	fn test_response_handling() {
		let id = Id::Text(
			"0x0000000000000000000000000000000000000000000000000000000000000000".to_owned(),
		);
		let return_value: RpcReturnValue = RpcReturnValue::new(
			vec![],
			false,
			DirectRequestStatus::TrustedOperationStatus(
				TrustedOperationStatus::TopExecuted(vec![], true),
				H256::random(),
			),
		);
		let rpc_response: RpcResponse = RpcResponse {
			jsonrpc: "2.0".to_owned(),
			result: return_value.to_hex(),
			id: id.clone(),
		};
		let serialized_rpc_response = serde_json::to_string(&rpc_response).unwrap();
		let message = Message::text(serialized_rpc_response);

		let (result_id, result) = DirectRpcClient::handle_ws_message(message).unwrap().unwrap();

		assert_eq!(id, result_id);
		assert_eq!(return_value, result);
	}
}
