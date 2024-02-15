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
			components::{create_ocall_api, create_top_pool},
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
use ita_sgx_runtime::Runtime;
use ita_stf::{helpers::set_block_number, Getter, TrustedCallSigned};
use itc_parentchain_test::ParentchainHeaderBuilder;
use itp_node_api::metadata::{metadata_mocks::NodeMetadataMock, provider::NodeMetadataRepository};
use itp_settings::sidechain::SLOT_DURATION;
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_interface::system_pallet::SystemPalletEventInterface;
use itp_stf_state_handler::handle_state::HandleState;
use itp_test::mock::metrics_ocall_mock::MetricsOCallMock;
use itp_time_utils::duration_now;
use itp_top_pool_author::top_filter::{AllowAllTopsFilter, DirectCallsOnlyFilter};
use itp_types::Block as ParentchainBlock;
use its_block_verification::slot::slot_from_timestamp_and_duration;
use its_primitives::types::SignedBlock as SignedSidechainBlock;
use its_sidechain::{aura::proposer_factory::ProposerFactory, slots::SlotInfo};
use lc_scheduled_enclave::ScheduledEnclaveMock;
use log::*;
use primitive_types::H256;
use sgx_crypto_helper::RsaKeyPair;
use sp_core::Pair;
use std::{sync::Arc, vec};

/// Integration test to ensure the events are reset upon block import.
/// Otherwise we will have an ever growing state.
/// (requires Sidechain mode)
pub fn ensure_events_get_reset_upon_block_proposal() {
	info!("Ignoring sidechain block production test: Not in sidechain mode");
	return
}
