// Copyright 2020-2022 Litentry Technologies GmbH.
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

use crate::{error::Error, Enclave, EnclaveResult};
use frame_support::ensure;
use itp_enclave_api_ffi as ffi;
use sgx_types::*;

/// Trait to run a stf task handling thread inside the enclave.
pub trait StfTaskHandler {
	fn run_stf_task_handler(&self) -> EnclaveResult<()>;
}

impl StfTaskHandler for Enclave {
	fn run_stf_task_handler(&self) -> EnclaveResult<()> {
		let mut retval = sgx_status_t::SGX_SUCCESS;

		let result = unsafe { ffi::run_stf_task_handler(self.eid, &mut retval) };

		ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
		ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));

		Ok(())
	}
}
