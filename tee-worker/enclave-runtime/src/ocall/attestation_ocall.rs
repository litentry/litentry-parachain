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

use crate::ocall::{ffi, OcallApi};
use frame_support::ensure;
use itp_ocall_api::EnclaveAttestationOCallApi;
use lazy_static::lazy_static;
use log::*;
use sgx_tse::EnclaveReport;
use sgx_types::{error::*, types::*};
use std::{
	ptr,
	sync::{Arc, RwLock},
	vec::Vec,
};

const RET_QUOTE_BUF_LEN: usize = 2048;

lazy_static! {
	/// Global cache of MRENCLAVE
	/// will never change at runtime but must be initialized at runtime
	static ref MY_MRENCLAVE: RwLock<Arc<MrEnclave>> = RwLock::new(Default::default());
}

#[derive(Default, Copy, Clone, Debug)]
pub struct MrEnclave {
	pub maybe_mrenclave: Option<Measurement>,
}

impl MrEnclave {
	pub fn current() -> SgxResult<Arc<MrEnclave>> {
		Ok(MY_MRENCLAVE
			.read()
			.map_err(|e| {
				error!("fetching current value of MR_ENCLAVE lazy static failed: {:?}", e);
				SgxStatus::Unexpected
			})?
			.clone())
	}
	pub fn make_current(self) -> SgxResult<()> {
		*MY_MRENCLAVE.write().map_err(|e| {
			error!("writing current value of MR_ENCLAVE lazy static failed: {:?}", e);
			SgxStatus::Unexpected
		})? = Arc::new(self);
		Ok(())
	}
}

impl EnclaveAttestationOCallApi for OcallApi {
	fn sgx_init_quote(&self) -> SgxResult<(TargetInfo, EpidGroupId)> {
		let mut ti: TargetInfo = TargetInfo::default();
		let mut eg: EpidGroupId = EpidGroupId::default();
		let mut rt: SgxStatus = SgxStatus::Unexpected;

		let res = unsafe {
			ffi::ocall_sgx_init_quote(
				&mut rt as *mut SgxStatus,
				&mut ti as *mut TargetInfo,
				&mut eg as *mut EpidGroupId,
			)
		};

		ensure!(res == SgxStatus::Success, res);
		ensure!(rt == SgxStatus::Success, rt);

		Ok((ti, eg))
	}

	fn get_ias_socket(&self) -> SgxResult<i32> {
		let mut rt: SgxStatus = SgxStatus::Unexpected;
		let mut ias_sock: i32 = 0;

		let res = unsafe {
			ffi::ocall_get_ias_socket(&mut rt as *mut SgxStatus, &mut ias_sock as *mut i32)
		};

		ensure!(res == SgxStatus::Success, res);
		ensure!(rt == SgxStatus::Success, rt);

		Ok(ias_sock)
	}

	fn get_quote(
		&self,
		sig_rl: Vec<u8>,
		report: Report,
		sign_type: QuoteSignType,
		spid: Spid,
		quote_nonce: QuoteNonce,
	) -> SgxResult<(Report, Vec<u8>)> {
		let mut qe_report = Report::default();
		let mut return_quote_buf = [0u8; RET_QUOTE_BUF_LEN];
		let mut quote_len: u32 = 0;

		let (p_sigrl, sigrl_len) = if sig_rl.is_empty() {
			(ptr::null(), 0)
		} else {
			(sig_rl.as_ptr(), sig_rl.len() as u32)
		};
		let p_report = &report as *const Report;
		let quote_type = sign_type;

		let mut rt: SgxStatus = SgxStatus::Unexpected;
		let p_spid = &spid as *const Spid;
		let p_nonce = &quote_nonce as *const QuoteNonce;
		let p_qe_report = &mut qe_report as *mut Report;
		let p_quote = return_quote_buf.as_mut_ptr();
		let maxlen = RET_QUOTE_BUF_LEN as u32;
		let p_quote_len = &mut quote_len as *mut u32;

		let result = unsafe {
			ffi::ocall_get_quote(
				&mut rt as *mut SgxStatus,
				p_sigrl,
				sigrl_len,
				p_report,
				quote_type,
				p_spid,
				p_nonce,
				p_qe_report,
				p_quote,
				maxlen,
				p_quote_len,
			)
		};

		ensure!(result == SgxStatus::Success, result);
		ensure!(rt == SgxStatus::Success, rt);

		#[allow(clippy::indexing_slicing)]
		let quote_vec: Vec<u8> = Vec::from(&return_quote_buf[..quote_len as usize]);

		Ok((qe_report, quote_vec))
	}

	fn get_dcap_quote(&self, report: Report, quote_size: u32) -> SgxResult<Vec<u8>> {
		let mut return_quote_buf = vec![0u8; quote_size as usize];
		let p_quote = return_quote_buf.as_mut_ptr();
		let p_report = &report as *const Report;
		let mut rt: SgxStatus = SgxStatus::Unexpected;

		let result = unsafe {
			ffi::ocall_get_dcap_quote(&mut rt as *mut SgxStatus, p_report, p_quote, quote_size)
		};
		ensure!(result == SgxStatus::Success, result);
		ensure!(rt == SgxStatus::Success, rt);
		#[allow(clippy::indexing_slicing)]
		let quote_vec: Vec<u8> = Vec::from(&return_quote_buf[..quote_size as usize]);
		Ok(quote_vec)
	}

	fn get_qve_report_on_quote(
		&self,
		quote: Vec<u8>,
		current_time: i64,
		quote_collateral: CQlQveCollateral,
		qve_report_info: QlQeReportInfo,
		supplemental_data_size: u32,
	) -> SgxResult<(u32, QlQvResult, QlQeReportInfo, Vec<u8>)> {
		let mut supplemental_data = vec![0u8; supplemental_data_size as usize];
		let mut qve_report_info_return_value: QlQeReportInfo = qve_report_info;
		let mut quote_verification_result = QlQvResult::Unspecified;
		let mut collateral_expiration_status = 1u32;
		let mut rt: SgxStatus = SgxStatus::Unexpected;

		let result = unsafe {
			ffi::ocall_get_qve_report_on_quote(
				&mut rt as *mut SgxStatus,
				quote.as_ptr(),
				quote.len() as u32,
				current_time,
				&quote_collateral as *const CQlQveCollateral,
				&mut collateral_expiration_status as *mut u32,
				&mut quote_verification_result as *mut QlQvResult,
				&mut qve_report_info_return_value as *mut QlQeReportInfo,
				supplemental_data.as_mut_ptr(),
				supplemental_data_size,
			)
		};
		ensure!(result == SgxStatus::Success, result);
		ensure!(rt == SgxStatus::Success, rt);

		Ok((
			collateral_expiration_status,
			quote_verification_result,
			qve_report_info_return_value,
			supplemental_data.to_vec(),
		))
	}

	fn get_update_info(
		&self,
		platform_info: PlatformInfo,
		enclave_trusted: i32,
	) -> SgxResult<UpdateInfoBit> {
		let mut rt: SgxStatus = SgxStatus::Unexpected;
		let mut update_info = UpdateInfoBit::default();

		let result = unsafe {
			ffi::ocall_get_update_info(
				&mut rt as *mut SgxStatus,
				&platform_info as *const PlatformInfo,
				enclave_trusted,
				&mut update_info as *mut UpdateInfoBit,
			)
		};

		// debug logging
		if rt != SgxStatus::Success {
			warn!("ocall_get_update_info unsuccessful. rt={:?}", rt);
			// Curly braces to copy `unaligned_references` of packed fields into properly aligned temporary:
			// https://github.com/rust-lang/rust/issues/82523
			debug!("update_info.pswUpdate: {}", { update_info.pswUpdate });
			debug!("update_info.csmeFwUpdate: {}", { update_info.csmeFwUpdate });
			debug!("update_info.ucodeUpdate: {}", { update_info.ucodeUpdate });
		}

		ensure!(result == SgxStatus::Success, result);
		ensure!(rt == SgxStatus::Success, rt);

		Ok(update_info)
	}

	fn get_mrenclave_of_self(&self) -> SgxResult<Measurement> {
		if let Some(mrenclave) = MrEnclave::current()?.maybe_mrenclave {
			trace!("found cached MRENCLAVE");
			return Ok(mrenclave)
		};
		debug!("initializing MY_MRENCLAVE cache");
		let mrenclave_value = self.get_report_of_self()?.mr_enclave;
		MrEnclave { maybe_mrenclave: Some(mrenclave_value) }.make_current()?;
		Ok(mrenclave_value)
	}
}

trait GetSgxReport {
	fn get_report_of_self(&self) -> SgxResult<ReportBody>;
}

impl<T: EnclaveAttestationOCallApi> GetSgxReport for T {
	fn get_report_of_self(&self) -> SgxResult<ReportBody> {
		// (1) get ti + eg
		let target_info = self.sgx_init_quote()?.0;
		let report_data: ReportData = ReportData::default();

		Report::for_target(&target_info, &report_data).map(|r| r.body)
	}
}
