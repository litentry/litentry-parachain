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

use crate::config::Config;
use itp_enclave_api::{enclave_base::EnclaveBase, error::Error as EnclaveApiError, EnclaveResult};
use itp_settings::files::{ENCLAVE_FILE, ENCLAVE_TOKEN};
use log::*;
use sgx_types::types::*;
use std::{
	fs::File,
	io::{Read, Write},
	path::PathBuf,
};

use itp_enclave_api::{Enclave, SgxEnclave};

pub fn enclave_init(config: &Config) -> EnclaveResult<Enclave> {
	// call sgx_create_enclave to initialize an enclave instance
	// Debug Support: 1 = debug mode, 0 = not debug mode
	#[cfg(feature = "development")]
	let debug = true;
	#[cfg(not(feature = "development"))]
	let debug = false;

	let enclave = (SgxEnclave::create(ENCLAVE_FILE, debug)).map_err(EnclaveApiError::Sgx)?;

	// create an enclave API and initialize it
	let enclave_api = Enclave::new(enclave);
	enclave_api.init(
		&config.mu_ra_url_external(),
		&config.untrusted_worker_url_external(),
		&config.data_dir().display().to_string(),
	)?;

	Ok(enclave_api)
}
