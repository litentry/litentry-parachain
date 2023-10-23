// Copyright 2020-2023 Trust Computing GmbH.
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
	pub use url_sgx as url;
	pub use webpki_sgx as webpki;
}

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

extern crate alloc;

use alloc::format;

use core::str::FromStr;
use log::debug;

use serde_json::from_str;

use itp_rpc::{Id, RpcRequest, RpcResponse, RpcReturnValue};

use itp_types::Request;
use itp_utils::{FromHexPrefixed, ToHexPrefixed};

use std::{
	boxed::Box,
	error::Error,
	net::TcpStream,
	string::{String, ToString},
	sync::Arc,
	time::Duration,
	vec,
	vec::Vec,
};
use tungstenite::{client_tls_with_config, stream::MaybeTlsStream, Connector, Message, WebSocket};
use url::Url;
use webpki::{DNSName, DNSNameRef};

pub type MaybeResponse = Option<(Id, RpcReturnValue)>;

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
	type RpcClient: RpcClient;
	fn create(&self, url: &str) -> Result<Self::RpcClient, Box<dyn Error>>;
}

pub struct DirectRpcClientFactory {}

impl RpcClientFactory for DirectRpcClientFactory {
	type RpcClient = DirectRpcClient;

	fn create(&self, url: &str) -> Result<Self::RpcClient, Box<dyn Error>> {
		DirectRpcClient::new(url)
	}
}

pub trait RpcClient {
	fn send(&mut self, request_id: String, params: Vec<String>) -> Result<(), Box<dyn Error>>;
	fn read_response(&mut self) -> Result<MaybeResponse, Box<dyn Error>>;
}

pub struct DirectRpcClient {
	ws: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl DirectRpcClient {
	pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
		let ws_server_url =
			Url::from_str(url).map_err(|e| format!("Could not connect, reason: {:?}", e))?;
		let mut config = rustls::ClientConfig::new();
		config.dangerous().set_certificate_verifier(Arc::new(IgnoreCertVerifier {}));
		let connector = Connector::Rustls(Arc::new(config));
		let addrs = ws_server_url.socket_addrs(|| None).unwrap();
		let stream = TcpStream::connect(&*addrs)
			.map_err(|e| format!("Could not connect to {:?}, reason: {:?}", &addrs, e))?;

		let (mut socket, _response) =
			client_tls_with_config(ws_server_url, stream, None, Some(connector))
				.map_err(|e| format!("Could not open websocket connection: {:?}", e))?;

		//it fails to perform handshake in non_blocking mode so we are setting it up after the handshake is performed
		Self::switch_to_non_blocking(&mut socket);

		debug!("Connected to peer: {}", url);
		Ok(Self { ws: socket })
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

	fn prepare_request(
		&mut self,
		request_id: String,
		parsed_params: Vec<String>,
	) -> Result<String, Box<dyn Error>> {
		let req = Request::from_hex(&parsed_params[0].clone())
			.map_err(|e| format!("Could not create request from hex, reason: {:?}", e))?;
		let request = Request { shard: req.shard, cyphertext: req.cyphertext };
		// if it's broadcasted it's not going to be broadcasted again
		let request = RpcRequest::compose_jsonrpc_call(
			Id::Text(request_id),
			"author_submitAndWatchBroadcastedExtrinsic".to_string(),
			vec![request.to_hex()],
		)
		.map_err(|e| format!("Could not compose RpcRequest, reason: {:?}", e))?;
		Ok(request)
	}

	fn handle_ws_message(message: Message) -> Result<MaybeResponse, Box<dyn Error>> {
		match message {
			Message::Text(text) => {
				let rpc_response: RpcResponse = from_str(&text)
					.map_err(|e| format!("Could not deserialize RpcResponse, reason: {:?}", e))?;
				let id = match rpc_response.id {
					Id::Text(id) =>
						Id::from_hex(&id).map_err(|e| format!("Could parse Id, reason: {:?}", e))?,
					Id::Number(_id) => panic!("Id in number format are not supported"),
				};
				let return_value: RpcReturnValue =
					RpcReturnValue::from_hex(&rpc_response.result)
						.map_err(|e| format!("Could not deserialize value , reason: {:?}", e))?;
				Ok(Some((id, return_value)))
			},
			_ => {
				log::warn!("Only text messages are supported");
				Ok(None)
			},
		}
	}
}

impl RpcClient for DirectRpcClient {
	fn send(&mut self, request_id: String, params: Vec<String>) -> Result<(), Box<dyn Error>> {
		let request = self.prepare_request(request_id, params)?;
		self.ws
			.write_message(Message::Text(request))
			.map_err(|e| format!("Could not write message, reason: {:?}", e).into())
	}

	#[allow(clippy::type_complexity)]
	fn read_response(&mut self) -> Result<MaybeResponse, Box<dyn Error>> {
		if let Ok(message) = self.ws.read_message() {
			Self::handle_ws_message(message)
		} else {
			Ok(None)
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::DirectRpcClient;
	use codec::Encode;
	use itp_rpc::{Id, RpcResponse, RpcReturnValue};
	use itp_types::{DirectRequestStatus, TrustedOperationStatus, H256};
	use itp_utils::ToHexPrefixed;
	use tungstenite::Message;

	#[test]
	fn test_response_handling() {
		let id = "0x0000000000000000000000000000000000000000000000000000000000000000".to_owned();
		let return_value: RpcReturnValue = RpcReturnValue::new(
			vec![],
			false,
			DirectRequestStatus::TrustedOperationStatus(
				TrustedOperationStatus::TopExecuted(vec![]),
				H256::random(),
			),
		);
		let rpc_response: RpcResponse = RpcResponse {
			jsonrpc: "2.0".to_owned(),
			result: return_value.to_hex(),
			id: Id::Text(id.clone()),
		};
		let serialized_rpc_response = serde_json::to_string(&rpc_response).unwrap();
		let message = Message::text(serialized_rpc_response);

		let (result_id, result) = DirectRpcClient::handle_ws_message(message).unwrap().unwrap();

		assert_eq!("0", serde_json::to_string(&result_id).unwrap());
		assert_eq!(return_value.encode(), result.encode());
	}
}
