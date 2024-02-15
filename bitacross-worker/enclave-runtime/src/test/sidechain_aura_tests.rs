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

use crate::{
	test::{
		fixtures::{
			components::{
				create_ocall_api, create_top_pool, encrypt_trusted_operation, sign_trusted_call,
			},
			initialize_test_state::init_state,
			test_setup::{enclave_call_signer, TestStf},
		},
		mocks::{
			peer_updater_mock::PeerUpdaterMock,
			propose_to_import_call_mock::ProposeToImportOCallApi, types::*,
		},
	},
	top_pool_execution::{exec_aura_on_slot, send_blocks_and_extrinsics},
};
use codec::Decode;
use ita_stf::{
	test_genesis::{endowed_account, second_endowed_account, unendowed_account},
	Balance, Getter, TrustedCall, TrustedCallSigned,
};
use itc_parentchain_test::ParentchainHeaderBuilder;
use itp_node_api::metadata::{metadata_mocks::NodeMetadataMock, provider::NodeMetadataRepository};
use itp_ocall_api::EnclaveAttestationOCallApi;
use itp_settings::sidechain::SLOT_DURATION;
use itp_sgx_crypto::{Aes, ShieldingCryptoEncrypt, StateCrypto};
use itp_sgx_externalities::SgxExternalitiesDiffType;
use itp_stf_interface::system_pallet::{SystemPalletAccountInterface, SystemPalletEventInterface};
use itp_stf_primitives::types::{StatePayload, TrustedOperation};
use itp_stf_state_handler::handle_state::HandleState;
use itp_test::mock::{handle_state_mock::HandleStateMock, metrics_ocall_mock::MetricsOCallMock};
use itp_time_utils::duration_now;
use itp_top_pool_author::{
	top_filter::{AllowAllTopsFilter, DirectCallsOnlyFilter},
	traits::AuthorApi,
};
use itp_types::{AccountId, Block as ParentchainBlock, RsaRequest, ShardIdentifier};
use its_block_verification::slot::slot_from_timestamp_and_duration;
use its_primitives::{traits::Block, types::SignedBlock as SignedSidechainBlock};
use its_sidechain::{aura::proposer_factory::ProposerFactory, slots::SlotInfo};
use jsonrpc_core::futures::executor;
use lc_scheduled_enclave::ScheduledEnclaveMock;
use litentry_primitives::Identity;
use log::*;
use primitive_types::H256;
use sgx_crypto_helper::RsaKeyPair;
use sp_core::{ed25519, Pair};
use std::{sync::Arc, vec, vec::Vec};

/// Integration test for sidechain block production and block import.
/// (requires Sidechain mode)
///
/// - Create trusted calls and add them to the TOP pool.
/// - Run AURA on a valid and claimed slot, which executes the trusted operations and produces a new block.
/// - Import the new sidechain block, which updates the state.
pub fn produce_sidechain_block_and_import_it() {
	info!("Ignoring sidechain block production test: Not in sidechain mode");
	return
}

fn encrypted_trusted_operation_transfer_balance<
	AttestationApi: EnclaveAttestationOCallApi,
	ShieldingKey: ShieldingCryptoEncrypt,
>(
	attestation_api: &AttestationApi,
	shard_id: &ShardIdentifier,
	shielding_key: &ShieldingKey,
	from: ed25519::Pair,
	to: AccountId,
	amount: Balance,
) -> Vec<u8> {
	let call = TrustedCall::balance_transfer(Identity::Substrate(from.public().into()), to, amount);
	let call_signed = sign_trusted_call(&call, attestation_api, shard_id, from);
	let trusted_operation = TrustedOperation::<TrustedCallSigned, Getter>::direct_call(call_signed);
	encrypt_trusted_operation(shielding_key, &trusted_operation)
}

fn get_state_hashes_from_block(
	signed_block: &SignedSidechainBlock,
	state_key: &Aes,
) -> (H256, H256) {
	let mut encrypted_state_diff = signed_block.block.block_data().encrypted_state_diff.clone();
	state_key.decrypt(&mut encrypted_state_diff).unwrap();
	let decoded_state =
		StatePayload::<SgxExternalitiesDiffType>::decode(&mut encrypted_state_diff.as_slice())
			.unwrap();
	(decoded_state.state_hash_apriori(), decoded_state.state_hash_aposteriori())
}

fn get_state_hash(state_handler: &HandleStateMock, shard_id: &ShardIdentifier) -> H256 {
	let (_, state_hash) = state_handler.load_cloned(shard_id).unwrap();
	state_hash
}
