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

use crate::utils::DecodeRaw;
use itp_component_container::ComponentGetter;
use itp_sgx_crypto::Rsa3072Seal;
use itp_sgx_io::StaticSealedIO;
use lc_data_providers::{DataProviderConfig, GLOBAL_DATA_PROVIDER_CONFIG};
use lc_stf_task_receiver::{run_stf_task_receiver, StfTaskContext};
use log::*;
use sgx_types::sgx_status_t;
use std::sync::Arc;

use crate::{
	error::{Error, Result},
	initialization::global_components::{
		EnclaveStfEnclaveSigner, GLOBAL_OCALL_API_COMPONENT,
		GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT, GLOBAL_STATE_OBSERVER_COMPONENT,
		GLOBAL_TOP_POOL_AUTHOR_COMPONENT,
	},
	GLOBAL_STATE_HANDLER_COMPONENT,
};

#[no_mangle]
pub unsafe extern "C" fn run_stf_task_handler(dpc: *const u8, dpc_size: usize) -> sgx_status_t {
	let data_provider_config = match DataProviderConfig::decode_raw(dpc, dpc_size) {
		Ok(data_provider_config) => data_provider_config,
		Err(e) => return Error::Codec(e).into(),
	};

	match GLOBAL_DATA_PROVIDER_CONFIG.write() {
		Ok(mut dpc) => {
			dpc.set_twitter_official_url(data_provider_config.twitter_official_url);
			dpc.set_twitter_litentry_url(data_provider_config.twitter_litentry_url);
			dpc.set_twitter_auth_token_v1_1(data_provider_config.twitter_auth_token_v1_1);
			dpc.set_twitter_auth_token_v2(data_provider_config.twitter_auth_token_v2);
			dpc.set_discord_official_url(data_provider_config.discord_official_url);
			dpc.set_discord_litentry_url(data_provider_config.discord_litentry_url);
			dpc.set_discord_auth_token(data_provider_config.discord_auth_token);
			dpc.set_achainable_url(data_provider_config.achainable_url);
			dpc.set_achainable_auth_key(data_provider_config.achainable_auth_key);
			dpc.set_credential_endpoint(data_provider_config.credential_endpoint);
			dpc.set_achainable_rest_key(data_provider_config.achainable_rest_key);
		},
		Err(e) => {
			error!("Error while setting data provider config: {:?}", e);
			return Error::MutexAccess.into()
		},
	}

	if let Err(e) = run_stf_task_handler_internal() {
		error!("Error while running stf task handler thread: {:?}", e);
		return e.into()
	}

	sgx_status_t::SGX_SUCCESS
}

/// Internal [`run_stf_task_handler`] function to be able to use the `?` operator.
///
/// Runs an extrinsic request inside the enclave, opening a channel and waiting for
/// senders to send requests.
fn run_stf_task_handler_internal() -> Result<()> {
	let author_api = GLOBAL_TOP_POOL_AUTHOR_COMPONENT.get()?;
	let state_handler = GLOBAL_STATE_HANDLER_COMPONENT.get()?;
	let state_observer = GLOBAL_STATE_OBSERVER_COMPONENT.get()?;

	let shielding_key_repository = GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT.get()?;
	let shielding_key = Rsa3072Seal::unseal_from_static_file().unwrap();

	let ocall_api = GLOBAL_OCALL_API_COMPONENT.get()?;
	let stf_enclave_signer = Arc::new(EnclaveStfEnclaveSigner::new(
		state_observer,
		ocall_api,
		shielding_key_repository,
		author_api.clone(),
	));

	let stf_task_context =
		StfTaskContext::new(shielding_key, author_api, stf_enclave_signer, state_handler);

	run_stf_task_receiver(Arc::new(stf_task_context)).map_err(Error::StfTaskReceiver)
}
