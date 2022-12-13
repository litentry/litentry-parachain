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

use sgx_types::{
	sgx_quote_nonce_t, sgx_quote_sign_type_t, sgx_report_data_t, sgx_spid_t, sgx_target_info_t,
};
use std::{string::String, vec::Vec};

use crate::Result as EnclaveResult;

#[cfg(feature = "sgx")]
use std::sync::{Arc, SgxRwLock as RwLock};

#[cfg(feature = "std")]
use std::sync::{Arc, RwLock};

#[derive(Default, Clone)]
pub struct ReportInputs {
	pub target_info: sgx_target_info_t,
	pub report_data: sgx_report_data_t,
}

#[derive(Default, Clone)]
pub struct QuoteInputs {
	pub quote_type: sgx_quote_sign_type_t,
	pub spid: sgx_spid_t,
	pub nonce: sgx_quote_nonce_t,
	pub sig_rl: Vec<u8>,
}

#[derive(Default, Clone)]
pub struct Inputs {
	pub report: ReportInputs,
	pub quote: QuoteInputs,
}

#[derive(Default, Clone)]
pub struct Outputs {
	pub quote: Vec<u8>,
	pub isv_enclave_quote: String,
}

// finally ≈ 2K
#[derive(Default, Clone)]
pub struct VCQuote {
	pub inputs: Inputs,
	pub outputs: Outputs,
}

impl VCQuote {
	pub fn add_report_inputs(
		&mut self,
		target_info: sgx_target_info_t,
		report_data: sgx_report_data_t,
	) {
		self.inputs.report.target_info = target_info;
		self.inputs.report.report_data = report_data;
	}

	pub fn add_quote_inputs(
		&mut self,
		quote_type: sgx_quote_sign_type_t,
		spid: sgx_spid_t,
		nonce: sgx_quote_nonce_t,
		sig_rl: Vec<u8>,
	) {
		self.inputs.quote.quote_type = quote_type;
		self.inputs.quote.spid = spid;
		self.inputs.quote.nonce = nonce;
		self.inputs.quote.sig_rl = sig_rl;
	}

	pub fn add_outputs(&mut self, quote: Vec<u8>, isv_enclave_quote: String) {
		self.outputs.quote = quote;
		self.outputs.isv_enclave_quote = isv_enclave_quote;
	}
}

/// Facade for handling STF state loading and storing (e.g. from file).
pub trait QuoteState {
	fn load(&self) -> EnclaveResult<Arc<RwLock<VCQuote>>>;
}
