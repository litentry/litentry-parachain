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

use itp_rpc::{Id, RpcRequest, RpcReturnValue};

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
	type Client: RpcClient + Send + Clone;
	fn create(&self, url: &str) -> Result<Self::Client, Box<dyn Error>>;
}

pub struct DirectRpcClientFactory {}

impl RpcClientFactory for DirectRpcClientFactory {
	type Client = DirectRpcClient;

	fn create(&self, url: &str) -> Result<Self::Client, Box<dyn Error>> {
		DirectRpcClient::new(url)
	}
}

pub trait RpcClient {
	fn send(&mut self, request: &RpcRequest) -> Result<(), Box<dyn Error>>;
}

#[derive(Clone)]
pub struct DirectRpcClient {
	request_sink: Sender<(String, Sender<bool>)>,
}

impl DirectRpcClient {
	pub fn new(url: &str) -> Result<Self, Box<dyn Error>> {
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

		let (request_sender, request_receiver) = channel::<(String, Sender<bool>)>();

		//it fails to perform handshake in non_blocking mode so we are setting it up after the handshake is performed
		Self::switch_to_non_blocking(&mut socket);

		std::thread::spawn(move || {
			while let Ok((request, result_sender)) = request_receiver.recv() {
				let mut result = true;
				if let Err(e) = socket.write_message(Message::Text(request)) {
					error!("Could not write message to socket, reason: {:?}", e);
					result = false;
				}
				if let Err(e) = result_sender.send(result) {
					log::error!("Could not send rpc result back, reason: {:?}", e);
				}
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
		let (sender, receiver) = channel();
		self.request_sink
			.send((request, sender))
			.map_err(|e| format!("Could not parse RpcRequest {:?}", e))?;

		if receiver.recv()? {
			Ok(())
		} else {
			Err("Could not send request".into())
		}
	}
}
