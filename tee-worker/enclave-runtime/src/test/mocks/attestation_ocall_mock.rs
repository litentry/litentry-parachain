/*
	CCopyright 2021 Integritee AG and Supercomputing Systems AG
	Copyright (C) 2017-2019 Baidu, Inc. All Rights Reserved.

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

use itp_ocall_api::EnclaveAttestationOCallApi;
use sgx_types::{error::*, types::*};
use std::{
	fmt::{Debug, Formatter, Result as FormatResult},
	vec::Vec,
};

#[derive(Clone)]
pub struct AttestationOCallMock {
	mr_enclave: Measurement,
}

impl AttestationOCallMock {
	pub fn new() -> Self {
		Default::default()
	}

	pub fn create_with_mr_enclave(mr_enclave: Measurement) -> Self {
		AttestationOCallMock { mr_enclave }
	}
}

impl EnclaveAttestationOCallApi for AttestationOCallMock {
	fn sgx_init_quote(&self) -> SgxResult<(TargetInfo, EpidGroupId)> {
		unreachable!()
	}

	fn get_ias_socket(&self) -> SgxResult<i32> {
		unreachable!()
	}

	fn get_quote(
		&self,
		_sig_rl: Vec<u8>,
		_report: Report,
		_sign_type: QuoteSignType,
		_spid: Spid,
		_quote_nonce: QuoteNonce,
	) -> SgxResult<(Report, Vec<u8>)> {
		unreachable!()
	}

	fn get_dcap_quote(&self, _report: Report, _quote_size: u32) -> SgxResult<Vec<u8>> {
		unreachable!()
	}

	fn get_qve_report_on_quote(
		&self,
		_quote: Vec<u8>,
		_current_time: i64,
		_quote_collateral: CQlQveCollateral,
		_qve_report_info: QlQeReportInfo,
		_supplemental_data_size: u32,
	) -> SgxResult<(u32, QlQvResult, QlQeReportInfo, Vec<u8>)> {
		unreachable!()
	}

	fn get_update_info(
		&self,
		_platform_info: PlatformInfo,
		_enclave_trusted: i32,
	) -> SgxResult<UpdateInfoBit> {
		Ok(UpdateInfoBit { csmeFwUpdate: 0, pswUpdate: 0, ucodeUpdate: 0 })
	}

	fn get_mrenclave_of_self(&self) -> SgxResult<Measurement> {
		Ok(self.mr_enclave)
	}
}

impl Default for AttestationOCallMock {
	fn default() -> Self {
		AttestationOCallMock { mr_enclave: Measurement { m: [1; 32] } }
	}
}

impl Debug for AttestationOCallMock {
	fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
		f.debug_struct("AttestationOCallMock")
			.field("mr_enclave", &self.mr_enclave.m)
			.finish()
	}
}
