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

use itp_rpc::{Id, RpcRequest, RpcReturnValue};

use std::{
	boxed::Box,
	error::Error,
	net::TcpStream,
	string::String,
	sync::{mpsc::SyncSender, Arc},
	vec::Vec,
};
use tungstenite::{client_tls_with_config, Connector, Message};
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
		response_sink: SyncSender<Response>,
	) -> Result<Self::Client, Box<dyn Error>>;
}

pub struct DirectRpcClientFactory {}

impl RpcClientFactory for DirectRpcClientFactory {
	type Client = DirectRpcClient;

	fn create(
		&self,
		url: &str,
		response_sink: SyncSender<Response>,
	) -> Result<Self::Client, Box<dyn Error>> {
		DirectRpcClient::new(url, response_sink)
	}
}

pub trait RpcClient {
	fn send(&mut self, url: &str, request: &RpcRequest) -> Result<(), Box<dyn Error>>;
}

pub struct DirectRpcClient {}

impl DirectRpcClient {
	pub fn new(_url: &str, _response_sink: SyncSender<Response>) -> Result<Self, Box<dyn Error>> {
		Ok(Self {})
	}
}

#[derive(Clone)]
pub enum RequestParams {
	Rsa(Vec<String>),
	Aes(Vec<String>),
}

impl RpcClient for DirectRpcClient {
	fn send(&mut self, url: &str, request: &RpcRequest) -> Result<(), Box<dyn Error>> {
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

		let request = serde_json::to_string(request)
			.map_err(|e| format!("Could not parse RpcRequest {:?}", e))?;

		log::trace!("Sending request: {:?}", request);
		socket.write_message(Message::Text(request))?;
		Ok(())
	}
}
