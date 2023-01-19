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

use codec::{Decode, Input};
use itp_component_container::ComponentGetter;
use itp_sgx_crypto::Rsa3072Seal;
use itp_sgx_io::StaticSealedIO;
use lc_data_providers::G_DATA_PROVIDERS;
use lc_stf_task_receiver::{run_stf_task_receiver, StfTaskContext};
use log::*;
use sgx_types::sgx_status_t;
use std::{slice, string::String, sync::Arc};

use crate::{
	error::{Error, Result},
	initialization::global_components::{
		EnclaveStfEnclaveSigner, GLOBAL_OCALL_API_COMPONENT,
		GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT, GLOBAL_STATE_OBSERVER_COMPONENT,
		GLOBAL_TOP_POOL_AUTHOR_COMPONENT,
	},
	utils::{
		get_extrinsic_factory_from_solo_or_parachain,
		get_node_metadata_repository_from_solo_or_parachain,
	},
	GLOBAL_STATE_HANDLER_COMPONENT,
};

#[no_mangle]
pub unsafe extern "C" fn run_stf_task_handler(
	twitter_official_url: *const u8,
	twitter_official_url_size: u32,
	twitter_litentry_url: *const u8,
	twitter_litentry_url_size: u32,
	twitter_auth_token: *const u8,
	twitter_auth_token_size: u32,
	discord_official_url: *const u8,
	discord_official_url_size: u32,
	discord_litentry_url: *const u8,
	discord_litentry_url_size: u32,
	discord_auth_token: *const u8,
	discord_auth_token_size: u32,
	graphql_url: *const u8,
	graphql_url_size: u32,
	graphql_auth_key: *const u8,
	graphql_auth_key_size: u32,
) -> sgx_status_t {
	let mut twitter_official_url_slice =
		slice::from_raw_parts(twitter_official_url, twitter_official_url_size as usize);
	let tw_o_url = match String::decode(&mut twitter_official_url_slice) {
		Ok(val) => val,
		Err(e) => {
			error!("Could not decode longitude: {:?}", e);
			return sgx_status_t::SGX_ERROR_UNEXPECTED
		},
	};

	let mut twitter_litentry_url_slice =
		slice::from_raw_parts(twitter_litentry_url, twitter_litentry_url_size as usize);
	let tw_l_url = match String::decode(&mut twitter_litentry_url_slice) {
		Ok(val) => val,
		Err(e) => {
			error!("Could not decode longitude: {:?}", e);
			return sgx_status_t::SGX_ERROR_UNEXPECTED
		},
	};

	let mut twitter_auth_token_slice =
		slice::from_raw_parts(twitter_auth_token, twitter_auth_token_size as usize);
	let tw_auth_t = match String::decode(&mut twitter_auth_token_slice) {
		Ok(val) => val,
		Err(e) => {
			error!("Could not decode longitude: {:?}", e);
			return sgx_status_t::SGX_ERROR_UNEXPECTED
		},
	};

	let mut discord_official_url_slice =
		slice::from_raw_parts(discord_official_url, discord_official_url_size as usize);
	let dis_o_url = match String::decode(&mut discord_official_url_slice) {
		Ok(val) => val,
		Err(e) => {
			error!("Could not decode longitude: {:?}", e);
			return sgx_status_t::SGX_ERROR_UNEXPECTED
		},
	};

	let mut discord_litentry_url_slice =
		slice::from_raw_parts(discord_litentry_url, discord_litentry_url_size as usize);
	let dis_l_url = match String::decode(&mut discord_litentry_url_slice) {
		Ok(val) => val,
		Err(e) => {
			error!("Could not decode longitude: {:?}", e);
			return sgx_status_t::SGX_ERROR_UNEXPECTED
		},
	};

	let mut discord_auth_token_slice =
		slice::from_raw_parts(discord_auth_token, discord_auth_token_size as usize);
	let dis_auth_t = match String::decode(&mut discord_auth_token_slice) {
		Ok(val) => val,
		Err(e) => {
			error!("Could not decode longitude: {:?}", e);
			return sgx_status_t::SGX_ERROR_UNEXPECTED
		},
	};

	let mut graphql_url_slice = slice::from_raw_parts(graphql_url, graphql_url_size as usize);
	let gql_url = match String::decode(&mut graphql_url_slice) {
		Ok(val) => val,
		Err(e) => {
			error!("Could not decode longitude: {:?}", e);
			return sgx_status_t::SGX_ERROR_UNEXPECTED
		},
	};

	let mut graphql_auth_key_slice =
		slice::from_raw_parts(graphql_auth_key, graphql_auth_key_size as usize);
	let gql_key = match String::decode(&mut graphql_auth_key_slice) {
		Ok(val) => val,
		Err(e) => {
			error!("Could not decode longitude: {:?}", e);
			return sgx_status_t::SGX_ERROR_UNEXPECTED
		},
	};

	let mut mut_handle = G_DATA_PROVIDERS.write().unwrap();
	mut_handle.set_twitter_official_url(tw_o_url);
	mut_handle.set_twitter_litentry_url(tw_l_url);
	mut_handle.set_twitter_auth_token(tw_auth_t);
	mut_handle.set_discord_official_url(dis_o_url);
	mut_handle.set_discord_litentry_url(dis_l_url);
	mut_handle.set_discord_auth_token(dis_auth_t);
	mut_handle.set_graphql_url(gql_url);
	mut_handle.set_graphql_auth_key(gql_key);

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

	let node_metadata = get_node_metadata_repository_from_solo_or_parachain()?;
	let extrinsic_factory = get_extrinsic_factory_from_solo_or_parachain()?;

	let stf_enclave_signer = Arc::new(EnclaveStfEnclaveSigner::new(
		state_observer,
		ocall_api.clone(),
		shielding_key_repository,
		author_api.clone(),
	));

	let stf_task_context = StfTaskContext::new(
		shielding_key,
		ocall_api,
		extrinsic_factory,
		node_metadata,
		author_api,
		stf_enclave_signer,
		state_handler,
	);

	run_stf_task_receiver(Arc::new(stf_task_context)).map_err(Error::StfTaskReceiver)
}
