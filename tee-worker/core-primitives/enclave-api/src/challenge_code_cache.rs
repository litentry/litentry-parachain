// Copyright 2020-2023 Litentry Technologies GmbH.
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
use frame_support::{ensure, traits::Len};
use itp_enclave_api_ffi as ffi;
use litentry_primitives::{ChallengeCode, Identity, CHALLENGE_CODE_SIZE};
use sgx_types::*;

/// Trait to run a stf task handling thread inside the enclave.
pub trait ChallengeCodeCache {
	fn enable_challenge_code_cache(&self) -> EnclaveResult<()>;

	fn get_challenge_code(&self, identity: Identity) -> EnclaveResult<ChallengeCode>;
}

impl ChallengeCodeCache for Enclave {
	fn enable_challenge_code_cache(&self) -> EnclaveResult<()> {
		let mut retval = sgx_status_t::SGX_SUCCESS;

		let result = unsafe { ffi::enable_challenge_code_cache(self.eid, &mut retval) };

		ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
		ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));

		Ok(())
	}

	fn get_challenge_code(&self, identity: Identity) -> EnclaveResult<ChallengeCode> {
		let mut retval = sgx_status_t::SGX_SUCCESS;
		let identity = identity.flat();
		let mut code = [0u8; CHALLENGE_CODE_SIZE];

		let result = unsafe {
			ffi::get_challenge_code(
				self.eid,
				&mut retval,
				identity.as_ptr(),
				identity.len() as u32,
				code.as_mut_ptr(),
				code.len() as u32,
			)
		};

		ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
		ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));

		Ok(code)
	}
}
