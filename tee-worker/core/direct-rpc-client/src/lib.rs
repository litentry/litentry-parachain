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
use codec::Decode;
use core::str::FromStr;

use serde_json::from_str;

use itc_crypto_helper::from_str_json;
use itp_rpc::{Id, RpcRequest, RpcResponse, RpcReturnValue};
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_types::{DirectRequestStatus, Request};
use itp_utils::{FromHexPrefixed, ToHexPrefixed};

use sgx_crypto_helper::rsa3072::Rsa3072PubKey;
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

pub struct NoCertVerifier {}

impl rustls::ServerCertVerifier for NoCertVerifier {
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

impl rustls::ClientCertVerifier for NoCertVerifier {
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

pub struct DirectRpcClient<ShieldingKeyRepository>
where
	ShieldingKeyRepository: AccessKey,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoDecrypt,
{
	ws: WebSocket<MaybeTlsStream<TcpStream>>,
	shielding_key_repo: Arc<ShieldingKeyRepository>,
	peer_shielding_key: Rsa3072PubKey,
}

impl<ShieldingKeyRepository> DirectRpcClient<ShieldingKeyRepository>
where
	ShieldingKeyRepository: AccessKey,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoDecrypt,
{
	pub fn new(
		url: &str,
		encryption_key: Arc<ShieldingKeyRepository>,
	) -> Result<Self, Box<dyn Error>> {
		let ws_server_url =
			Url::from_str(url).map_err(|e| format!("Could not connect, reason: {:?}", e))?;
		let mut config = rustls::ClientConfig::new();
		config.dangerous().set_certificate_verifier(Arc::new(NoCertVerifier {}));
		let connector = Connector::Rustls(Arc::new(config));
		let addrs = ws_server_url.socket_addrs(|| None).unwrap();
		let stream = TcpStream::connect(&*addrs)
			.map_err(|e| format!("Could not connect to {:?}, reason: {:?}", &addrs, e))?;

		let (mut socket, _response) =
			client_tls_with_config(ws_server_url, stream, None, Some(connector))
				.map_err(|e| format!("Could not open websocket connection: {:?}", e))?;

		let shielding_pubkey = Self::read_peer_shielding_key(&mut socket)?;
		//it fails to perform handshake in non_blocking mode so we are setting it up after the handshake is performed
		Self::switch_to_non_blocking(&mut socket);
		Ok(Self {
			ws: socket,
			shielding_key_repo: encryption_key,
			peer_shielding_key: shielding_pubkey,
		})
	}

	fn read_peer_shielding_key(
		socket: &mut WebSocket<MaybeTlsStream<TcpStream>>,
	) -> Result<Rsa3072PubKey, Box<dyn Error>> {
		let get_sheilding_key_req: String = RpcRequest::compose_jsonrpc_call(
			Id::Text("1".to_string()),
			"author_getShieldingKey".to_string(),
			Default::default(),
		)
		.map_err(|e| format!("Could not compose rpc call, reason: {:?}", e))?;

		socket
			.write_message(Message::Text(get_sheilding_key_req))
			.map_err(|e| format!("Could not send get shielding key request, reason: {:?}", e))?;
		let response_str = socket
			.read_message()
			.map_err(|e| format!("Could not read get shielding key response, reason: {:?}", e))?
			.to_string();
		let rpc_response: RpcResponse = from_str(&response_str)
			.map_err(|e| format!("Failed to deserialize response, reason: {:?}", e))?;
		let rpc_return_value = RpcReturnValue::from_hex(&rpc_response.result)
			.map_err(|e| format!("Failed to deserialize RpcReturnValue, reason: {:?}", e))?;
		match rpc_return_value.status {
			DirectRequestStatus::Ok => {
				let value =
					String::decode(&mut rpc_return_value.value.as_slice()).map_err(|e| {
						format!("Failed to deserialize RpcReturnValue.value, reason: {:?}", e)
					})?;
				let shielding_pubkey: Rsa3072PubKey = from_str_json(&value)
					.map_err(|e| format!("Could not deserialize shielding key, reason: {:?}", e))?;
				Ok(shielding_pubkey)
			},
			_ => Err("Could not get shielding key".into()),
		}
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

	pub fn send(&mut self, request_id: String, params: Vec<String>) -> Result<(), Box<dyn Error>> {
		let request = self.prepare_request(request_id, params)?;
		self.ws
			.write_message(Message::Text(request))
			.map_err(|e| format!("Could not write message, reason: {:?}", e).into())
	}

	fn prepare_request(
		&mut self,
		request_id: String,
		parsed_params: Vec<String>,
	) -> Result<String, Box<dyn Error>> {
		let req = Request::from_hex(&parsed_params[0].clone())
			.map_err(|e| format!("Could not create request, reason: {:?}", e))?;
		// decrypt call
		let shielding_key = self
			.shielding_key_repo
			.retrieve_key()
			.map_err(|e| format!("Could not get shielding key, reason: {:?}", e))?;
		let request_vec = shielding_key
			.decrypt(req.cyphertext.as_slice())
			.map_err(|e| format!("Could not decrypt request, reason: {:?}", e))?;
		// encrypt with peer shielding key
		let encrypted_request = self
			.peer_shielding_key
			.encrypt(&request_vec)
			.map_err(|e| format!("Could not encrypt request, reason: {:?}", e))?;
		// send request
		let request = Request { shard: req.shard, cyphertext: encrypted_request };
		let request = RpcRequest::compose_jsonrpc_call(
			Id::Text(request_id),
			"author_submitAndWatchExtrinsic".to_string(),
			vec![request.to_hex()],
		)
		.map_err(|e| format!("Could not compose RpcRequest, reason: {:?}", e))?;
		Ok(request)
	}

	#[allow(clippy::type_complexity)]
	pub fn read_response<T: FromHexPrefixed>(
		&mut self,
	) -> Result<Option<(T::Output, RpcReturnValue)>, Box<dyn Error>> {
		if let Ok(message) = self.ws.read_message() {
			match message {
				Message::Text(text) => {
					let rpc_response: RpcResponse = from_str(&text).map_err(|e| {
						format!("Could not deserialize RpcResponse, reason: {:?}", e)
					})?;
					let id = match rpc_response.id {
						Id::Text(id) => T::from_hex(&id)
							.map_err(|e| format!("Could parse Id, reason: {:?}", e))?,
						Id::Number(_id) => panic!("cannot get number"),
					};
					let return_value: RpcReturnValue =
						RpcReturnValue::from_hex(&rpc_response.result).map_err(|e| {
							format!("Could not deserialize value , reason: {:?}", e)
						})?;
					Ok(Some((id, return_value)))
				},
				_ => Ok(None),
			}
		} else {
			Ok(None)
		}
	}
}
