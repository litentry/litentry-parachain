// Copyright 2022 Integritee AG and Supercomputing Systems AG
// Copyright (C) 2017-2019 Baidu, Inc. All Rights Reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  * Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
//  * Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in
//    the documentation and/or other materials provided with the
//    distribution.
//  * Neither the name of Baidu, Inc., nor the names of its
//    contributors may be used to endorse or promote products derived
//    from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT
// OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL,
// SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT
// LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE,
// DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY
// THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
// OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::{serde_json, http_req, base64};

use crate::Result as EnclaveResult;
use http_req::{
	request::{Method, RequestBuilder},
	tls,
	uri::Uri,
};
use log::debug;
use std::{
	net::TcpStream,
	string::{String, ToString},
	vec::Vec,
};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MAAResponse {
	pub token: String,
}

/// Trait to do Microsoft Azure Attestation
pub trait MAAHandler {
	/// Verify DCAP quote from MAA
	fn azure_attest(&self, quote: &[u8]) -> EnclaveResult<Vec<u8>>;
}

pub struct MAAService;
impl MAAService {
	pub fn parse_maa_policy(writer: &[u8]) -> EnclaveResult<Vec<u8>> {
		let maa_res: MAAResponse = serde_json::from_slice(&writer).unwrap();
		let decompose_token: Vec<&str> = maa_res.token.split(".").collect();
		if decompose_token.len() != 3 {
			log::error!("JSON Web Tokens must have 3 components delimited by '.' characters.");
		}

		let policy = base64::decode(decompose_token[1]).unwrap();
		Ok(policy)
	}
}

impl MAAHandler for MAAService {
	fn azure_attest(&self, quote: &[u8]) -> EnclaveResult<Vec<u8>> {
		debug!("    [Enclave] Entering azure_attest.");

		let req_body = serde_json::json!({
			"quote": base64::encode(quote.to_vec())
		})
		.to_string();

		let endpoint = "";
		let token = "";
		let url = endpoint.to_string() + "/attest/SgxEnclave?api-version=2020-10-01";
		let addr = Uri::try_from(&url[..]).unwrap();

		let sock = TcpStream::connect((addr.host().unwrap(), addr.corr_port())).unwrap();
		let mut stream = tls::Config::default().connect(addr.host().unwrap_or(""), sock).unwrap();

		let mut writer = Vec::new();
		let response = RequestBuilder::new(&addr)
			.method(Method::POST)
			.body(req_body.as_bytes())
			.header("Content-Length", &req_body.len())
			.header("Connection", "Close")
			.header("Content-Type", "application/json")
			.header("Authorization", &format!("Bearer {}", token))
			.send(&mut stream, &mut writer)
			.unwrap();

		Self::parse_maa_policy(&writer)
	}
}

#[cfg(feature = "test")]
pub mod tests {
	use super::*;

	// Policy exmaple
	// MAAPolicy {
	//     is_debuggable: false,
	//     product_id: 1,
	//     sgx_mrenclave: "d37d983a85d63fb49649610e2eba0930ecdbff6d113aca3ff3fc7261696c0134",
	//     sgx_mrsigner: "feb995eb86c349ac98e5afbbb5732ca7376ec9979002702ea17ad476e0853a04",
	//     svn: 8888,
	//     tee: "sgx",
	// },

	pub fn azure_attest_works() {
		pub const sample: &[u8] = include_bytes!("./maa_response_sample.json");
		let ret = MAAService::parse_maa_policy(sample);
		assert!(ret.is_ok());
	}
}
