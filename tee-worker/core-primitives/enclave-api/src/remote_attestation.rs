/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG
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

use crate::EnclaveResult;
use itp_types::ShardIdentifier;
use pallet_teebag::Fmspc;
use sgx_types::types::*;

/// Struct that unites all relevant data reported by the QVE
pub struct QveReport {
	pub supplemental_data: Vec<u8>,
	pub qve_report_info_return_value: QlQeReportInfo,
	pub quote_verification_result: QlQvResult,
	pub collateral_expiration_status: u32,
}

/// general remote attestation methods
pub trait RemoteAttestation {
	fn generate_ias_ra_extrinsic(&self, w_url: &str, skip_ra: bool) -> EnclaveResult<Vec<u8>>;

	fn generate_dcap_ra_extrinsic(&self, w_url: &str, skip_ra: bool) -> EnclaveResult<Vec<u8>>;
	fn generate_dcap_ra_extrinsic_from_quote(
		&self,
		url: String,
		quote: &[u8],
	) -> EnclaveResult<Vec<u8>>;
	fn generate_dcap_ra_quote(&self, skip_ra: bool) -> EnclaveResult<Vec<u8>>;

	fn generate_register_quoting_enclave_extrinsic(&self, fmspc: Fmspc) -> EnclaveResult<Vec<u8>>;

	fn generate_register_tcb_info_extrinsic(&self, fmspc: Fmspc) -> EnclaveResult<Vec<u8>>;

	fn dump_ias_ra_cert_to_disk(&self) -> EnclaveResult<()>;

	fn dump_dcap_ra_cert_to_disk(&self) -> EnclaveResult<()>;

	fn dump_dcap_collateral_to_disk(&self, fmspc: Fmspc) -> EnclaveResult<()>;

	fn set_ql_qe_enclave_paths(&self) -> EnclaveResult<()>;

	fn set_sgx_qpl_logging(&self) -> EnclaveResult<()>;

	fn qe_get_target_info(&self) -> EnclaveResult<TargetInfo>;

	fn qe_get_quote_size(&self) -> EnclaveResult<u32>;

	fn get_dcap_collateral(&self, fmspc: Fmspc) -> EnclaveResult<*const CQlQveCollateral>;
}

/// call-backs that are made from inside the enclave (using o-call), to e-calls again inside the enclave
pub trait RemoteAttestationCallBacks {
	fn init_quote(&self) -> EnclaveResult<(TargetInfo, EpidGroupId)>;

	fn calc_quote_size(&self, revocation_list: Vec<u8>) -> EnclaveResult<u32>;

	fn get_quote(
		&self,
		revocation_list: Vec<u8>,
		report: Report,
		quote_type: QuoteSignType,
		spid: Spid,
		quote_nonce: QuoteNonce,
		quote_length: u32,
	) -> EnclaveResult<(Report, Vec<u8>)>;

	fn get_dcap_quote(&self, report: Report, quote_size: u32) -> EnclaveResult<Vec<u8>>;

	fn get_qve_report_on_quote(
		&self,
		quote: Vec<u8>,
		current_time: i64,
		quote_collateral: &CQlQveCollateral,
		qve_report_info: QlQeReportInfo,
		supplemental_data_size: u32,
	) -> EnclaveResult<QveReport>;

	fn get_update_info(
		&self,
		platform_blob: PlatformInfo,
		enclave_trusted: i32,
	) -> EnclaveResult<UpdateInfoBit>;
}

/// TLS remote attestations methods
pub trait TlsRemoteAttestation {
	fn run_state_provisioning_server(
		&self,
		socket_fd: c_int,
		sign_type: QuoteSignType,
		quoting_enclave_target_info: Option<&TargetInfo>,
		quote_size: Option<&u32>,
		skip_ra: bool,
	) -> EnclaveResult<()>;

	fn request_state_provisioning(
		&self,
		socket_fd: c_int,
		sign_type: QuoteSignType,
		quoting_enclave_target_info: Option<&TargetInfo>,
		quote_size: Option<&u32>,
		shard: &ShardIdentifier,
		skip_ra: bool,
	) -> EnclaveResult<()>;
}

#[cfg(feature = "implement-ffi")]
mod impl_ffi {
	use super::{QveReport, RemoteAttestation, RemoteAttestationCallBacks, TlsRemoteAttestation};
	use crate::{error::Error, utils, Enclave, EnclaveResult};
	use codec::Encode;
	use frame_support::ensure;
	use itp_enclave_api_ffi as ffi;
	use itp_settings::worker::EXTRINSIC_MAX_SIZE;
	use itp_types::ShardIdentifier;
	use log::*;
	use pallet_teebag::Fmspc;
	use sgx_types::{error::*, function::*, types::*};

	const OS_SYSTEM_PATH: &str = "/usr/lib/x86_64-linux-gnu/";
	const C_STRING_ENDING: &str = "\0";
	const PCE_ENCLAVE: &str = "libsgx_pce.signed.so.1";
	const QE3_ENCLAVE: &str = "libsgx_qe3.signed.so.1";
	const ID_ENCLAVE: &str = "libsgx_id_enclave.signed.so.1";
	const LIBDCAP_QUOTEPROV: &str = "libdcap_quoteprov.so.1";
	const QVE_ENCLAVE: &str = "libsgx_qve.signed.so.1";

	impl RemoteAttestation for Enclave {
		fn generate_ias_ra_extrinsic(&self, w_url: &str, skip_ra: bool) -> EnclaveResult<Vec<u8>> {
			let mut retval = SgxStatus::Success;

			let mut unchecked_extrinsic: Vec<u8> = vec![0u8; EXTRINSIC_MAX_SIZE];
			let mut unchecked_extrinsic_size: u32 = 0;

			trace!("Generating ias_ra_extrinsic with URL: {}", w_url);

			let url = w_url.encode();

			let result = unsafe {
				ffi::generate_ias_ra_extrinsic(
					self.eid,
					&mut retval,
					url.as_ptr(),
					url.len() as u32,
					unchecked_extrinsic.as_mut_ptr(),
					unchecked_extrinsic.len() as u32,
					&mut unchecked_extrinsic_size as *mut u32,
					skip_ra.into(),
				)
			};

			ensure!(result == SgxStatus::Success, Error::Sgx(result));
			ensure!(retval == SgxStatus::Success, Error::Sgx(retval));
			ensure!(
				(unchecked_extrinsic_size as usize) < unchecked_extrinsic.len(),
				Error::Sgx(SgxStatus::InvalidParameter)
			);
			Ok(Vec::from(&unchecked_extrinsic[..unchecked_extrinsic_size as usize]))
		}
		fn generate_dcap_ra_extrinsic_from_quote(
			&self,
			url: String,
			quote: &[u8],
		) -> EnclaveResult<Vec<u8>> {
			let mut retval = SgxStatus::Success;
			let mut unchecked_extrinsic: Vec<u8> = vec![0u8; EXTRINSIC_MAX_SIZE];
			let mut unchecked_extrinsic_size: u32 = 0;
			let url = url.encode();

			let result = unsafe {
				ffi::generate_dcap_ra_extrinsic_from_quote(
					self.eid,
					&mut retval,
					url.as_ptr(),
					url.len() as u32,
					quote.as_ptr(),
					quote.len() as u32,
					unchecked_extrinsic.as_mut_ptr(),
					unchecked_extrinsic.len() as u32,
					&mut unchecked_extrinsic_size as *mut u32,
				)
			};

			ensure!(result == SgxStatus::Success, Error::Sgx(result));
			ensure!(retval == SgxStatus::Success, Error::Sgx(retval));
			ensure!(
				(unchecked_extrinsic_size as usize) < unchecked_extrinsic.len(),
				Error::Sgx(SgxStatus::InvalidParameter)
			);
			Ok(Vec::from(&unchecked_extrinsic[..unchecked_extrinsic_size as usize]))
		}

		fn generate_dcap_ra_quote(&self, skip_ra: bool) -> EnclaveResult<Vec<u8>> {
			let mut retval = SgxStatus::Success;
			let quoting_enclave_target_info = self.qe_get_target_info()?;
			let quote_size = self.qe_get_quote_size()?;

			let mut dcap_quote_vec: Vec<u8> = vec![0; quote_size as usize];
			let (dcap_quote_p, dcap_quote_size) =
				(dcap_quote_vec.as_mut_ptr(), dcap_quote_vec.len() as u32);

			let result = unsafe {
				ffi::generate_dcap_ra_quote(
					self.eid,
					&mut retval,
					skip_ra.into(),
					&quoting_enclave_target_info,
					quote_size,
					dcap_quote_p,
					dcap_quote_size,
				)
			};

			ensure!(result == SgxStatus::Success, Error::Sgx(result));
			ensure!(retval == SgxStatus::Success, Error::Sgx(retval));

			unsafe {
				trace!("Generating DCAP RA Quote: {}", *dcap_quote_p);
			}

			Ok(dcap_quote_vec)
		}

		fn generate_dcap_ra_extrinsic(&self, w_url: &str, skip_ra: bool) -> EnclaveResult<Vec<u8>> {
			let mut retval = SgxStatus::Success;

			self.set_ql_qe_enclave_paths()?;
			let quoting_enclave_target_info = if !skip_ra {
				match self.qe_get_target_info() {
					Ok(target_info) => Some(target_info),
					Err(e) => return Err(e),
				}
			} else {
				None
			};
			let quote_size = if !skip_ra {
				match self.qe_get_quote_size() {
					Ok(quote_size) => Some(quote_size),
					Err(e) => return Err(e),
				}
			} else {
				None
			};
			info!("Retrieved quote size of {:?}", quote_size);

			trace!("Generating dcap_ra_extrinsic with URL: {}", w_url);

			let mut unchecked_extrinsic: Vec<u8> = vec![0u8; EXTRINSIC_MAX_SIZE];
			let mut unchecked_extrinsic_size: u32 = 0;
			let url = w_url.encode();

			let result = unsafe {
				ffi::generate_dcap_ra_extrinsic(
					self.eid,
					&mut retval,
					url.as_ptr(),
					url.len() as u32,
					unchecked_extrinsic.as_mut_ptr(),
					unchecked_extrinsic.len() as u32,
					&mut unchecked_extrinsic_size as *mut u32,
					skip_ra.into(),
					quoting_enclave_target_info.as_ref(),
					quote_size.as_ref(),
				)
			};

			ensure!(result == SgxStatus::Success, Error::Sgx(result));
			ensure!(retval == SgxStatus::Success, Error::Sgx(retval));
			ensure!(
				(unchecked_extrinsic_size as usize) < unchecked_extrinsic.len(),
				Error::Sgx(SgxStatus::InvalidParameter)
			);
			Ok(Vec::from(&unchecked_extrinsic[..unchecked_extrinsic_size as usize]))
		}

		fn generate_register_quoting_enclave_extrinsic(
			&self,
			fmspc: Fmspc,
		) -> EnclaveResult<Vec<u8>> {
			let mut retval = SgxStatus::Success;
			let mut unchecked_extrinsic: Vec<u8> = vec![0u8; EXTRINSIC_MAX_SIZE];
			let mut unchecked_extrinsic_size: u32 = 0;

			trace!("Generating register quoting enclave");

			let collateral_ptr = self.get_dcap_collateral(fmspc)?;

			let result = unsafe {
				ffi::generate_register_quoting_enclave_extrinsic(
					self.eid,
					&mut retval,
					collateral_ptr,
					unchecked_extrinsic.as_mut_ptr(),
					unchecked_extrinsic.len() as u32,
					&mut unchecked_extrinsic_size as *mut u32,
				)
			};
			let free_status = unsafe { sgx_ql_free_quote_verification_collateral(collateral_ptr) };
			ensure!(result == SgxStatus::Success, Error::Sgx(result));
			ensure!(retval == SgxStatus::Success, Error::Sgx(retval));
			ensure!(free_status == Quote3Error::Success, Error::SgxQuote(free_status));
			ensure!(
				(unchecked_extrinsic_size as usize) < unchecked_extrinsic.len(),
				Error::Sgx(SgxStatus::InvalidParameter)
			);
			Ok(Vec::from(&unchecked_extrinsic[..unchecked_extrinsic_size as usize]))
		}

		fn generate_register_tcb_info_extrinsic(&self, fmspc: Fmspc) -> EnclaveResult<Vec<u8>> {
			let mut retval = SgxStatus::Success;
			let mut unchecked_extrinsic: Vec<u8> = vec![0u8; EXTRINSIC_MAX_SIZE];
			let mut unchecked_extrinsic_size: u32 = 0;

			trace!("Generating tcb_info registration");

			let collateral_ptr = self.get_dcap_collateral(fmspc)?;

			let result = unsafe {
				ffi::generate_register_tcb_info_extrinsic(
					self.eid,
					&mut retval,
					collateral_ptr,
					unchecked_extrinsic.as_mut_ptr(),
					unchecked_extrinsic.len() as u32,
					&mut unchecked_extrinsic_size as *mut u32,
				)
			};
			let free_status = unsafe { sgx_ql_free_quote_verification_collateral(collateral_ptr) };
			ensure!(result == SgxStatus::Success, Error::Sgx(result));
			ensure!(retval == SgxStatus::Success, Error::Sgx(retval));
			ensure!(free_status == Quote3Error::Success, Error::SgxQuote(free_status));
			ensure!(
				(unchecked_extrinsic_size as usize) < unchecked_extrinsic.len(),
				Error::Sgx(SgxStatus::InvalidParameter)
			);
			Ok(Vec::from(&unchecked_extrinsic[..unchecked_extrinsic_size as usize]))
		}

		fn dump_ias_ra_cert_to_disk(&self) -> EnclaveResult<()> {
			let mut retval = SgxStatus::Success;

			let result = unsafe { ffi::dump_ias_ra_cert_to_disk(self.eid, &mut retval) };

			ensure!(result == SgxStatus::Success, Error::Sgx(result));
			ensure!(retval == SgxStatus::Success, Error::Sgx(retval));

			Ok(())
		}

		fn dump_dcap_ra_cert_to_disk(&self) -> EnclaveResult<()> {
			let mut retval = SgxStatus::Success;

			self.set_ql_qe_enclave_paths()?;
			let quoting_enclave_target_info = self.qe_get_target_info()?;
			let quote_size = self.qe_get_quote_size()?;

			let result = unsafe {
				ffi::dump_dcap_ra_cert_to_disk(
					self.eid,
					&mut retval,
					&quoting_enclave_target_info,
					quote_size,
				)
			};

			ensure!(result == SgxStatus::Success, Error::Sgx(result));
			ensure!(retval == SgxStatus::Success, Error::Sgx(retval));

			Ok(())
		}

		fn set_ql_qe_enclave_paths(&self) -> EnclaveResult<()> {
			set_ql_path(QlPathType::PcePath, PCE_ENCLAVE)?;
			set_ql_path(QlPathType::Qe3Path, QE3_ENCLAVE)?;
			set_ql_path(QlPathType::IdePath, ID_ENCLAVE)?;
			if set_ql_path(QlPathType::QplPath, LIBDCAP_QUOTEPROV).is_err() {
				// Ignore the error, because user may want to get cert type=3 quote.
				warn!("Cannot set QPL directory, you may get ECDSA quote with `Encrypted PPID` cert type.\n");
			};
			set_qv_path(QvPathType::QvePath, QVE_ENCLAVE)?;

			Ok(())
		}

		fn set_sgx_qpl_logging(&self) -> EnclaveResult<()> {
			let res = unsafe { sgx_ql_set_logging_callback(forward_qpl_log, QlLogLevel::LogInfo) };
			if res == Quote3Error::Success {
				Ok(())
			} else {
				error!("Setting logging function failed with: {:?}", res);
				Err(Error::SgxQuote(res))
			}
		}

		fn qe_get_target_info(&self) -> EnclaveResult<TargetInfo> {
			let mut quoting_enclave_target_info: TargetInfo = TargetInfo::default();
			let qe3_ret =
				unsafe { sgx_qe_get_target_info(&mut quoting_enclave_target_info as *mut _) };
			ensure!(qe3_ret == Quote3Error::Success, Error::SgxQuote(qe3_ret));

			Ok(quoting_enclave_target_info)
		}

		fn qe_get_quote_size(&self) -> EnclaveResult<u32> {
			let mut quote_size: u32 = 0;
			let qe3_ret = unsafe { sgx_qe_get_quote_size(&mut quote_size as *mut _) };
			ensure!(qe3_ret == Quote3Error::Success, Error::SgxQuote(qe3_ret));

			Ok(quote_size)
		}

		fn dump_dcap_collateral_to_disk(&self, fmspc: Fmspc) -> EnclaveResult<()> {
			let mut retval = SgxStatus::Success;
			let collateral_ptr = self.get_dcap_collateral(fmspc)?;
			let result =
				unsafe { ffi::dump_dcap_collateral_to_disk(self.eid, &mut retval, collateral_ptr) };
			let free_status = unsafe { sgx_ql_free_quote_verification_collateral(collateral_ptr) };
			ensure!(result == SgxStatus::Success, Error::Sgx(result));
			ensure!(retval == SgxStatus::Success, Error::Sgx(result));
			ensure!(free_status == Quote3Error::Success, Error::SgxQuote(free_status));
			Ok(())
		}

		fn get_dcap_collateral(&self, fmspc: Fmspc) -> EnclaveResult<*const CQlQveCollateral> {
			let pck_ra = b"processor\x00";

			// SAFETY: Just get a nullptr for the FFI to overwrite later
			let mut collateral_ptr: *mut CQlQveCollateral = unsafe { std::mem::zeroed() };

			let collateral_ptr_ptr: *mut *mut CQlQveCollateral = &mut collateral_ptr;
			// SAFETY: All parameters are properly initialized so the FFI call should be fine
			let sgx_status = unsafe {
				sgx_ql_get_quote_verification_collateral(
					fmspc.as_ptr(),
					fmspc.len() as uint16_t, //fmspc len is fixed in the function signature
					pck_ra.as_ptr() as _,
					collateral_ptr_ptr,
				)
			};

			trace!("FMSPC: {:?}", hex::encode(fmspc));

			if collateral_ptr.is_null() {
				error!("PCK quote collateral data is null, sgx_status is: {}", sgx_status);
				return Err(Error::SgxQuote(sgx_status))
			}

			trace!("collateral:");
			// SAFETY: the previous block checks for `collateral_ptr` being null.
			// SAFETY: the fields should be nul terminated C strings.
			unsafe {
				let collateral = &*collateral_ptr;
				trace!(
					"version: {}\n, \
				 tee_type: {}\n, \
				 pck_crl_issuer_chain: {:?}\n, \
				 pck_crl_issuer_chain_size: {}\n, \
				 root_ca_crl: {:?}\n, \
				 root_ca_crl_size: {}\n, \
				 pck_crl: {:?}\n, \
				 pck_crl_size: {}\n, \
				 tcb_info_issuer_chain: {:?}\n, \
				 tcb_info_issuer_chain_size: {}\n, \
				 tcb_info: {}\n, \
				 tcb_info_size: {}\n, \
				 qe_identity_issuer_chain: {:?}\n, \
				 qe_identity_issuer_chain_size: {}\n, \
				 qe_identity: {}\n, \
				 qe_identity_size: {}\n",
					collateral.version,
					collateral.tee_type,
					std::ffi::CStr::from_ptr(collateral.pck_crl_issuer_chain).to_string_lossy(),
					collateral.pck_crl_issuer_chain_size,
					std::ffi::CStr::from_ptr(collateral.root_ca_crl).to_string_lossy(),
					collateral.root_ca_crl_size,
					std::ffi::CStr::from_ptr(collateral.pck_crl).to_string_lossy(),
					collateral.pck_crl_size,
					std::ffi::CStr::from_ptr(collateral.tcb_info_issuer_chain).to_string_lossy(),
					collateral.tcb_info_issuer_chain_size,
					std::ffi::CStr::from_ptr(collateral.tcb_info).to_string_lossy(),
					collateral.tcb_info_size,
					std::ffi::CStr::from_ptr(collateral.qe_identity_issuer_chain).to_string_lossy(),
					collateral.qe_identity_issuer_chain_size,
					std::ffi::CStr::from_ptr(collateral.qe_identity).to_string_lossy(),
					collateral.qe_identity_size,
				);
			};

			ensure!(sgx_status == Quote3Error::Success, Error::SgxQuote(sgx_status));
			Ok(collateral_ptr)
		}
	}

	#[cfg(feature = "implement-ffi")]
	impl RemoteAttestationCallBacks for Enclave {
		fn init_quote(&self) -> EnclaveResult<(TargetInfo, EpidGroupId)> {
			let mut ti: TargetInfo = TargetInfo::default();
			let mut eg: EpidGroupId = EpidGroupId::default();

			let result =
				unsafe { sgx_init_quote(&mut ti as *mut TargetInfo, &mut eg as *mut EpidGroupId) };

			ensure!(result == SgxStatus::Success, Error::Sgx(result));

			Ok((ti, eg))
		}

		fn calc_quote_size(&self, revocation_list: Vec<u8>) -> EnclaveResult<u32> {
			let mut real_quote_len: u32 = 0;

			let (p_sig_rl, sig_rl_size) = utils::vec_to_c_pointer_with_len(revocation_list);

			let result = unsafe {
				sgx_calc_quote_size(p_sig_rl, sig_rl_size, &mut real_quote_len as *mut u32)
			};

			ensure!(result == SgxStatus::Success, Error::Sgx(result));

			Ok(real_quote_len)
		}

		fn get_quote(
			&self,
			revocation_list: Vec<u8>,
			report: Report,
			quote_type: QuoteSignType,
			spid: Spid,
			quote_nonce: QuoteNonce,
			quote_length: u32,
		) -> EnclaveResult<(Report, Vec<u8>)> {
			let (p_sig_rl, sig_rl_size) = utils::vec_to_c_pointer_with_len(revocation_list);
			let p_report = &report as *const Report;
			let p_spid = &spid as *const Spid;
			let p_nonce = &quote_nonce as *const QuoteNonce;

			let mut qe_report = Report::default();
			let p_qe_report = &mut qe_report as *mut Report;

			let mut return_quote_buf = vec![0u8; quote_length as usize];
			let p_quote = return_quote_buf.as_mut_ptr();

			let ret = unsafe {
				sgx_get_quote(
					p_report,
					quote_type,
					p_spid,
					p_nonce,
					p_sig_rl,
					sig_rl_size,
					p_qe_report,
					p_quote as *mut Quote,
					quote_length,
				)
			};

			ensure!(ret == SgxStatus::Success, Error::Sgx(ret));

			Ok((qe_report, return_quote_buf))
		}

		fn get_dcap_quote(&self, report: Report, quote_size: u32) -> EnclaveResult<Vec<u8>> {
			let mut quote_vec: Vec<u8> = vec![0; quote_size as usize];
			let qe3_ret =
				unsafe { sgx_qe_get_quote(&report, quote_size, quote_vec.as_mut_ptr() as _) };

			ensure!(qe3_ret == Quote3Error::Success, Error::SgxQuote(qe3_ret));

			Ok(quote_vec)
		}

		fn get_qve_report_on_quote(
			&self,
			quote: Vec<u8>,
			current_time: i64,
			quote_collateral: &CQlQveCollateral,
			qve_report_info: QlQeReportInfo,
			supplemental_data_size: u32,
		) -> EnclaveResult<QveReport> {
			let mut collateral_expiration_status = 1u32;
			let mut quote_verification_result = QlQvResult::Ok;
			let mut supplemental_data: Vec<u8> = vec![0; supplemental_data_size as usize];
			let mut qve_report_info_return_value: QlQeReportInfo = qve_report_info;

			// Set QvE (Quote verification Enclave) loading policy.
			let dcap_ret = unsafe { sgx_qv_set_enclave_load_policy(QlRequestPolicy::Ephemeral) };

			if dcap_ret != Quote3Error::Success {
				error!("sgx_qv_set_enclave_load_policy failed: {:#04x}", dcap_ret as u32);
				return Err(Error::SgxQuote(dcap_ret))
			}

			// Retrieve supplemental data size from QvE.
			let mut qve_supplemental_data_size = 0u32;
			let dcap_ret =
				unsafe { sgx_qv_get_quote_supplemental_data_size(&mut qve_supplemental_data_size) };

			if dcap_ret != Quote3Error::Success {
				error!("sgx_qv_get_quote_supplemental_data_size failed: {:?}", dcap_ret);
				return Err(Error::SgxQuote(dcap_ret))
			}
			if qve_supplemental_data_size != supplemental_data_size {
				warn!("Quote supplemental data size is different between DCAP QVL and QvE, please make sure you installed DCAP QVL and QvE from same release.");
				return Err(Error::Sgx(SgxStatus::InvalidParameter))
			}

			// Check if a collateral has been given, or if it's a simple zero assignment.
			// If it's zero, let the pointer point to null. The collateral will then be retrieved
			// directly by the QvE in `sgx_qv_verify_quote`.
			let p_quote_collateral: *const CQlQveCollateral = if quote_collateral.version == 0 {
				std::ptr::null()
			} else {
				quote_collateral as *const CQlQveCollateral
			};

			// Call the QvE for quote verification
			// here you can choose 'trusted' or 'untrusted' quote verification by specifying parameter '&qve_report_info'
			// if '&qve_report_info' is NOT NULL, this API will call Intel QvE to verify quote
			// if '&qve_report_info' is NULL, this API will call 'untrusted quote verify lib' to verify quote,
			// this mode doesn't rely on SGX capable system, but the results can not be cryptographically authenticated
			let dcap_ret = unsafe {
				sgx_qv_verify_quote(
					quote.as_ptr(),
					quote.len() as u32,
					p_quote_collateral,
					current_time,
					&mut collateral_expiration_status as *mut u32,
					&mut quote_verification_result as *mut QlQvResult,
					&mut qve_report_info_return_value as *mut QlQeReportInfo,
					supplemental_data_size,
					supplemental_data.as_mut_ptr(),
				)
			};

			if Quote3Error::Success != dcap_ret {
				error!("sgx_qv_verify_quote failed: {:?}", dcap_ret);
				error!("quote_verification_result: {:?}", quote_verification_result);
				return Err(Error::SgxQuote(dcap_ret))
			}

			// Check and print verification result.
			match quote_verification_result {
				QlQvResult::Ok => {
					// Check verification collateral expiration status.
					// This value should be considered in your own attestation/verification policy.
					if 0u32 == collateral_expiration_status {
						info!("QvE verification completed successfully.");
					} else {
						warn!("QvE verification completed, but collateral is out of date based on 'expiration_check_date' you provided.");
					}
				},
				QlQvResult::ConfigNeeded
				| QlQvResult::OutOfDate
				| QlQvResult::OutOfDateConfigNeeded
				| QlQvResult::SWHardeningNeeded
				| QlQvResult::ConfigAndSWHardeningNeeded => {
					warn!(
						"QvE verification completed with Non-terminal result: {:?}",
						quote_verification_result
					);
				},
				_ => {
					error!(
						"QvE verification completed with Terminal result: {:?}",
						quote_verification_result
					);
				},
			}

			// Check supplemental data.
			if supplemental_data_size > 0 {
				// For now we simply print it, no checks done.
				let p_supplemental_data: *const QlQvSupplemental =
					supplemental_data.as_ptr() as *const QlQvSupplemental;
				let qv_supplemental_data: QlQvSupplemental = unsafe { *p_supplemental_data };
				info!(
					"QvE verification: Supplemental data version: {}",
					qv_supplemental_data.version
				);
			}

			Ok(QveReport {
				collateral_expiration_status,
				quote_verification_result,
				qve_report_info_return_value,
				supplemental_data,
			})
		}

		fn get_update_info(
			&self,
			platform_blob: PlatformInfo,
			enclave_trusted: i32,
		) -> EnclaveResult<UpdateInfoBit> {
			let mut update_info: UpdateInfoBit = UpdateInfoBit::default();

			let result = unsafe {
				sgx_report_attestation_status(
					&platform_blob as *const PlatformInfo,
					enclave_trusted,
					&mut update_info as *mut UpdateInfoBit,
				)
			};

			ensure!(result == SgxStatus::Success, Error::Sgx(result));

			Ok(update_info)
		}
	}

	#[cfg(feature = "implement-ffi")]
	impl TlsRemoteAttestation for Enclave {
		fn run_state_provisioning_server(
			&self,
			socket_fd: c_int,
			sign_type: QuoteSignType,
			quoting_enclave_target_info: Option<&TargetInfo>,
			quote_size: Option<&u32>,
			skip_ra: bool,
		) -> EnclaveResult<()> {
			let mut retval = SgxStatus::Success;

			let result = unsafe {
				ffi::run_state_provisioning_server(
					self.eid,
					&mut retval,
					socket_fd,
					sign_type,
					quoting_enclave_target_info,
					quote_size,
					skip_ra.into(),
				)
			};

			ensure!(result == SgxStatus::Success, Error::Sgx(result));
			ensure!(retval == SgxStatus::Success, Error::Sgx(retval));

			Ok(())
		}

		fn request_state_provisioning(
			&self,
			socket_fd: c_int,
			sign_type: QuoteSignType,
			quoting_enclave_target_info: Option<&TargetInfo>,
			quote_size: Option<&u32>,
			shard: &ShardIdentifier,
			skip_ra: bool,
		) -> EnclaveResult<()> {
			let mut retval = SgxStatus::Success;

			let encoded_shard = shard.encode();

			let result = unsafe {
				ffi::request_state_provisioning(
					self.eid,
					&mut retval,
					socket_fd,
					sign_type,
					quoting_enclave_target_info,
					quote_size,
					encoded_shard.as_ptr(),
					encoded_shard.len() as u32,
					skip_ra.into(),
				)
			};

			ensure!(result == SgxStatus::Success, Error::Sgx(result));
			ensure!(retval == SgxStatus::Success, Error::Sgx(retval));

			Ok(())
		}
	}

	fn create_system_path(file_name: &str) -> String {
		trace!("create_system_path:: file_name={}", &file_name);
		let default_path = format!("{}{}", OS_SYSTEM_PATH, file_name);

		let full_path = find_library_by_name(file_name).unwrap_or(default_path);

		let c_terminated_path = format!("{}{}", full_path, C_STRING_ENDING);
		trace!("create_system_path:: created path={}", &c_terminated_path);
		c_terminated_path
	}

	fn find_library_by_name(lib_name: &str) -> Option<String> {
		use std::process::Command;
		// ldconfig -p | grep libsgx_pce_logic.so.1

		let ldconfig_output = Command::new("ldconfig").args(["-p"]).output().ok()?;
		let possible_path = String::from_utf8(ldconfig_output.stdout)
			.ok()?
			.lines()
			.filter(|line| line.contains(lib_name))
			.map(|lib_name_and_path| {
				lib_name_and_path
					.rsplit_once("=>")
					.map(|(_, lib_path)| lib_path.trim().to_owned())
			})
			.next()?;

		possible_path
	}

	fn set_ql_path(path_type: QlPathType, path: &str) -> EnclaveResult<()> {
		let ret_val = unsafe { sgx_ql_set_path(path_type, create_system_path(path).as_ptr() as _) };
		if ret_val != Quote3Error::Success {
			error!("Could not set {:?}", path_type);
			return Err(Error::SgxQuote(ret_val))
		}
		Ok(())
	}

	fn set_qv_path(path_type: QvPathType, path: &str) -> EnclaveResult<()> {
		let ret_val = unsafe { sgx_qv_set_path(path_type, create_system_path(path).as_ptr() as _) };
		if ret_val != Quote3Error::Success {
			error!("Could not set {:?}", path_type);
			return Err(Error::SgxQuote(ret_val))
		}
		Ok(())
	}

	#[allow(clippy::not_unsafe_ptr_arg_deref)]
	/// Make sure that the `log_slice_ptr` points to a null terminated string.
	// This function must not be marked as `unsafe`, because `sgx_ql_set_logging_callback` expects a safe (i.e. not `unsafe`) function.
	pub extern "C" fn forward_qpl_log(log_level: QlLogLevel, log_slice_ptr: *const c_char) {
		if log_slice_ptr.is_null() {
			error!("[QPL - ERROR], slice to print was NULL");
			return
		}
		// This is safe, as the previous block checks for `NULL` pointer.
		let slice = unsafe { core::ffi::CStr::from_ptr(log_slice_ptr) };
		match log_level {
			QlLogLevel::LogInfo => info!("[QPL - INFO], {:?}", slice),
			QlLogLevel::LogError => error!("[QPL - ERROR], {:?}", slice),
		}
	}
}
