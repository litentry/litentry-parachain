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
	fn run_stf_task_handler(
		&self,
		twitter_official_url: Vec<u8>,
		twitter_litentry_url: Vec<u8>,
		twitter_auth_token: Vec<u8>,
		discord_official_url: Vec<u8>,
		discord_litentry_url: Vec<u8>,
		discord_auth_token: Vec<u8>,
		graphql_url: Vec<u8>,
		graphql_auth_key: Vec<u8>,
	) -> EnclaveResult<()>;
}

impl StfTaskHandler for Enclave {
	fn run_stf_task_handler(
		&self,
		twitter_official_url: Vec<u8>,
		twitter_litentry_url: Vec<u8>,
		twitter_auth_token: Vec<u8>,
		discord_official_url: Vec<u8>,
		discord_litentry_url: Vec<u8>,
		discord_auth_token: Vec<u8>,
		graphql_url: Vec<u8>,
		graphql_auth_key: Vec<u8>,
	) -> EnclaveResult<()> {
		let mut retval = sgx_status_t::SGX_SUCCESS;

		let result = unsafe {
			ffi::run_stf_task_handler(
				self.eid,
				&mut retval,
				twitter_official_url.as_ptr(),
				twitter_official_url.len() as u32,
				twitter_litentry_url.as_ptr(),
				twitter_litentry_url.len() as u32,
				twitter_auth_token.as_ptr(),
				twitter_auth_token.len() as u32,
				discord_official_url.as_ptr(),
				discord_official_url.len() as u32,
				discord_litentry_url.as_ptr(),
				discord_litentry_url.len() as u32,
				discord_auth_token.as_ptr(),
				discord_auth_token.len() as u32,
				graphql_url.as_ptr(),
				graphql_url.len() as u32,
				graphql_auth_key.as_ptr(),
				graphql_auth_key.len() as u32,
			)
		};

		ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
		ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));

		Ok(())
	}
}
