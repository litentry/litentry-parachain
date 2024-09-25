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

#[cfg(feature = "test")]
use crate::test_genesis::test_genesis_setup;
use crate::{
	format,
	helpers::{enclave_signer_account, shard_creation_info},
	vec, Arc, Box, Debug, From, Stf, Vec, ENCLAVE_ACCOUNT_KEY,
};
use codec::{Decode, Encode};
use frame_support::traits::{OriginTrait, UnfilteredDispatchable};
use ita_sgx_runtime::{
	Executive, ParentchainInstanceLitentry, ParentchainInstanceTargetA, ParentchainInstanceTargetB,
};
use itp_node_api::metadata::{provider::AccessNodeMetadata, NodeMetadataTrait};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_interface::{
	parentchain_pallet::ParentchainPalletInstancesInterface,
	runtime_upgrade::RuntimeUpgradeInterface,
	sudo_pallet::SudoPalletInterface,
	system_pallet::{SystemPalletAccountInterface, SystemPalletEventInterface},
	ExecuteCall, ExecuteGetter, InitState, ShardCreationInfo, ShardCreationQuery,
	StateCallInterface, StateGetterInterface, UpdateState,
};
use itp_stf_primitives::{
	error::StfError, traits::TrustedCallVerification, types::ShardIdentifier,
};
use itp_storage::storage_value_key;
use itp_types::{
	parentchain::{AccountId, ParentchainCall, ParentchainId},
	H256,
};
use itp_utils::stringify::account_id_to_string;
use log::*;
use sp_runtime::traits::StaticLookup;

impl<TCS, G, State, Runtime, AccountId> InitState<State, AccountId> for Stf<TCS, G, State, Runtime>
where
	State: SgxExternalitiesTrait + Debug,
	<State as SgxExternalitiesTrait>::SgxExternalitiesType: core::default::Default,
	Runtime: frame_system::Config<AccountId = AccountId> + pallet_balances::Config,
	<<Runtime as frame_system::Config>::Lookup as StaticLookup>::Source: From<AccountId>,
	AccountId: Encode,
{
	fn init_state(enclave_account: AccountId) -> State {
		debug!("initializing stf state, account id {}", account_id_to_string(&enclave_account));
		let mut state = State::new(Default::default());

		state.execute_with(|| {
			// Do not set genesis for pallets that are meant to be on-chain
			// use get_storage_hashes_to_update instead.

			sp_io::storage::set(&storage_value_key("Balances", "TotalIssuance"), &11u128.encode());
			sp_io::storage::set(&storage_value_key("Balances", "CreationFee"), &1u128.encode());
			sp_io::storage::set(&storage_value_key("Balances", "TransferFee"), &1u128.encode());
			sp_io::storage::set(
				&storage_value_key("Balances", "TransactionBaseFee"),
				&1u128.encode(),
			);
			sp_io::storage::set(
				&storage_value_key("Balances", "TransactionByteFee"),
				&1u128.encode(),
			);
			sp_io::storage::set(
				&storage_value_key("Balances", "ExistentialDeposit"),
				&1u128.encode(),
			);
		});

		#[cfg(feature = "test")]
		test_genesis_setup(&mut state);

		state.execute_with(|| {
			sp_io::storage::set(
				&storage_value_key("Sudo", ENCLAVE_ACCOUNT_KEY),
				&enclave_account.encode(),
			);

			if let Err(e) = create_enclave_self_account::<Runtime, AccountId>(enclave_account) {
				error!("Failed to initialize the enclave signer account: {:?}", e);
			}
		});

		trace!("Returning updated state: {:?}", state);
		state
	}
}

impl<TCS, G, State, Runtime>
	UpdateState<State, <State as SgxExternalitiesTrait>::SgxExternalitiesDiffType>
	for Stf<TCS, G, State, Runtime>
where
	State: SgxExternalitiesTrait + Debug,
	<State as SgxExternalitiesTrait>::SgxExternalitiesType: core::default::Default,
	<State as SgxExternalitiesTrait>::SgxExternalitiesDiffType:
		IntoIterator<Item = (Vec<u8>, Option<Vec<u8>>)>,
{
	fn apply_state_diff(
		state: &mut State,
		map_update: <State as SgxExternalitiesTrait>::SgxExternalitiesDiffType,
	) {
		state.execute_with(|| {
			map_update.into_iter().for_each(|(k, v)| {
				match v {
					Some(value) => sp_io::storage::set(&k, &value),
					None => sp_io::storage::clear(&k),
				};
			});
		});
	}

	fn storage_hashes_to_update_on_block(parentchain_id: &ParentchainId) -> Vec<Vec<u8>> {
		// Get all shards that are currently registered.
		match parentchain_id {
			ParentchainId::Litentry => vec![], // shards_key_hash() moved to stf_executor and is currently unused
			ParentchainId::TargetA => vec![],
			ParentchainId::TargetB => vec![],
		}
	}
}

impl<TCS, G, State, Runtime, NodeMetadataRepository>
	StateCallInterface<TCS, State, NodeMetadataRepository> for Stf<TCS, G, State, Runtime>
where
	TCS: PartialEq
		+ ExecuteCall<NodeMetadataRepository>
		+ Encode
		+ Decode
		+ Debug
		+ Clone
		+ Sync
		+ Send
		+ TrustedCallVerification,
	State: SgxExternalitiesTrait + Debug,
	NodeMetadataRepository: AccessNodeMetadata,
	NodeMetadataRepository::MetadataType: NodeMetadataTrait,
{
	type Error = TCS::Error;
	type Result = TCS::Result;

	fn execute_call(
		state: &mut State,
		shard: &ShardIdentifier,
		call: TCS,
		top_hash: H256,
		calls: &mut Vec<ParentchainCall>,
		node_metadata_repo: Arc<NodeMetadataRepository>,
	) -> Result<Self::Result, Self::Error> {
		state.execute_with(|| call.execute(shard, top_hash, calls, node_metadata_repo))
	}
}

impl<TCS, G, State, Runtime> StateGetterInterface<G, State> for Stf<TCS, G, State, Runtime>
where
	G: PartialEq + ExecuteGetter,
	State: SgxExternalitiesTrait + Debug,
{
	fn execute_getter(state: &mut State, getter: G) -> Option<Vec<u8>> {
		state.execute_with(|| getter.execute())
	}
}

impl<TCS, G, State, Runtime> ShardCreationQuery<State> for Stf<TCS, G, State, Runtime>
where
	State: SgxExternalitiesTrait + Debug,
{
	fn get_shard_creation_info(state: &mut State) -> ShardCreationInfo {
		state.execute_with(shard_creation_info)
	}
}

impl<TCS, G, State, Runtime> SudoPalletInterface<State> for Stf<TCS, G, State, Runtime>
where
	State: SgxExternalitiesTrait,
	Runtime: frame_system::Config + pallet_sudo::Config,
{
	type AccountId = Runtime::AccountId;

	fn get_root(state: &mut State) -> Self::AccountId {
		state.execute_with(|| pallet_sudo::Pallet::<Runtime>::key().expect("No root account"))
	}

	fn get_enclave_account(state: &mut State) -> Self::AccountId {
		state.execute_with(enclave_signer_account::<Self::AccountId>)
	}
}

impl<TCS, G, State, Runtime, AccountId> SystemPalletAccountInterface<State, AccountId>
	for Stf<TCS, G, State, Runtime>
where
	State: SgxExternalitiesTrait,
	Runtime: frame_system::Config<AccountId = AccountId>,
	AccountId: Encode,
{
	type Index = Runtime::Index;
	type AccountData = Runtime::AccountData;

	fn get_account_nonce(state: &mut State, account: &AccountId) -> Self::Index {
		state.execute_with(|| {
			let nonce = frame_system::Pallet::<Runtime>::account_nonce(account);
			debug!("Account {} nonce is {:?}", account_id_to_string(account), nonce);
			nonce
		})
	}

	fn get_account_data(state: &mut State, account: &AccountId) -> Self::AccountData {
		state.execute_with(|| frame_system::Pallet::<Runtime>::account(account).data)
	}
}

impl<TCS, G, State, Runtime> SystemPalletEventInterface<State> for Stf<TCS, G, State, Runtime>
where
	State: SgxExternalitiesTrait,
	Runtime: frame_system::Config,
{
	type EventRecord = frame_system::EventRecord<Runtime::RuntimeEvent, Runtime::Hash>;
	type EventIndex = u32; // For some reason this is not a pub type in frame_system
	type BlockNumber = Runtime::BlockNumber;
	type Hash = Runtime::Hash;

	fn get_events(state: &mut State) -> Vec<Box<Self::EventRecord>> {
		// Fixme: Not nice to have to call collect here, but we can't use impl Iterator<..>
		// in trait method return types yet, see:
		// https://rust-lang.github.io/impl-trait-initiative/RFCs/rpit-in-traits.html
		state.execute_with(|| frame_system::Pallet::<Runtime>::read_events_no_consensus().collect())
	}

	fn get_event_count(state: &mut State) -> Self::EventIndex {
		state.execute_with(|| frame_system::Pallet::<Runtime>::event_count())
	}

	fn get_event_topics(
		state: &mut State,
		topic: &Self::Hash,
	) -> Vec<(Self::BlockNumber, Self::EventIndex)> {
		state.execute_with(|| frame_system::Pallet::<Runtime>::event_topics(topic))
	}

	fn reset_events(state: &mut State) {
		state.execute_with(|| frame_system::Pallet::<Runtime>::reset_events())
	}
}

impl<TCS, G, State, Runtime, ParentchainHeader>
	ParentchainPalletInstancesInterface<State, ParentchainHeader> for Stf<TCS, G, State, Runtime>
where
	State: SgxExternalitiesTrait,
	Runtime: frame_system::Config<Header = ParentchainHeader, AccountId = AccountId>
		+ pallet_parentchain::Config<ParentchainInstanceLitentry>
		+ pallet_parentchain::Config<ParentchainInstanceTargetA>
		+ pallet_parentchain::Config<ParentchainInstanceTargetB>,
	<<Runtime as frame_system::Config>::Lookup as StaticLookup>::Source: From<AccountId>,
	ParentchainHeader: Debug,
{
	type Error = StfError;

	fn update_parentchain_litentry_block(
		state: &mut State,
		header: ParentchainHeader,
	) -> Result<(), Self::Error> {
		trace!("updating litentry parentchain block : {:?}", header);
		state.execute_with(|| {
			pallet_parentchain::Call::<Runtime, ParentchainInstanceLitentry>::set_block { header }
				.dispatch_bypass_filter(Runtime::RuntimeOrigin::root())
				.map_err(|e| {
					Self::Error::Dispatch(format!(
						"Update parentchain litentry block error: {:?}",
						e.error
					))
				})
		})?;
		Ok(())
	}

	fn update_parentchain_target_a_block(
		state: &mut State,
		header: ParentchainHeader,
	) -> Result<(), Self::Error> {
		trace!("updating target_a parentchain block: {:?}", header);
		state.execute_with(|| {
			pallet_parentchain::Call::<Runtime, ParentchainInstanceTargetA>::set_block { header }
				.dispatch_bypass_filter(Runtime::RuntimeOrigin::root())
				.map_err(|e| {
					Self::Error::Dispatch(format!(
						"Update parentchain target_a block error: {:?}",
						e.error
					))
				})
		})?;
		Ok(())
	}

	fn update_parentchain_target_b_block(
		state: &mut State,
		header: ParentchainHeader,
	) -> Result<(), Self::Error> {
		trace!("updating target_b parentchain block: {:?}", header);
		state.execute_with(|| {
			pallet_parentchain::Call::<Runtime, ParentchainInstanceTargetB>::set_block { header }
				.dispatch_bypass_filter(Runtime::RuntimeOrigin::root())
				.map_err(|e| {
					Self::Error::Dispatch(format!(
						"Update parentchain target_b block error: {:?}",
						e.error
					))
				})
		})?;
		Ok(())
	}

	fn set_creation_block(
		state: &mut State,
		header: ParentchainHeader,
		parentchain_id: ParentchainId,
	) -> Result<(), Self::Error> {
		state.execute_with(|| match parentchain_id {
			ParentchainId::Litentry => pallet_parentchain::Call::<
				Runtime,
				ParentchainInstanceLitentry,
			>::set_creation_block {
				header,
			}
			.dispatch_bypass_filter(Runtime::RuntimeOrigin::root())
			.map_err(|e| {
				Self::Error::Dispatch(format!("Init shard vault account error: {:?}", e.error))
			}),
			ParentchainId::TargetA => pallet_parentchain::Call::<
				Runtime,
				ParentchainInstanceTargetA,
			>::set_creation_block {
				header,
			}
			.dispatch_bypass_filter(Runtime::RuntimeOrigin::root())
			.map_err(|e| {
				Self::Error::Dispatch(format!("Init shard vault account error: {:?}", e.error))
			}),
			ParentchainId::TargetB => pallet_parentchain::Call::<
				Runtime,
				ParentchainInstanceTargetB,
			>::set_creation_block {
				header,
			}
			.dispatch_bypass_filter(Runtime::RuntimeOrigin::root())
			.map_err(|e| {
				Self::Error::Dispatch(format!("Init shard vault account error: {:?}", e.error))
			}),
		})?;
		Ok(())
	}
}

impl<TCS, G, State, Runtime> RuntimeUpgradeInterface<State> for Stf<TCS, G, State, Runtime>
where
	State: SgxExternalitiesTrait,
	Runtime: frame_system::Config,
{
	type Error = StfError;

	fn on_runtime_upgrade(state: &mut State) -> Result<(), Self::Error> {
		// Returns if the runtime was upgraded since the last time this function was called.
		let runtime_upgraded = || -> bool {
			let last = frame_system::LastRuntimeUpgrade::<Runtime>::get();
			let current =
				<<Runtime as frame_system::Config>::Version as frame_support::traits::Get<_>>::get(
				);

			if last.as_ref().map(|v| v.was_upgraded(&current)).unwrap_or(true) {
				frame_system::LastRuntimeUpgrade::<Runtime>::put(
					frame_system::LastRuntimeUpgradeInfo::from(current.clone()),
				);
				debug!("Do some migrations, last: {:?}, current: {:?}", last, current.spec_version);
				true
			} else {
				false
			}
		};

		state.execute_with(|| {
			if runtime_upgraded() {
				Executive::execute_on_runtime_upgrade();
			}
		});
		Ok(())
	}
}

/// Creates valid enclave account with a balance that is above the existential deposit.
/// !! Requires a root to be set.
fn create_enclave_self_account<Runtime, AccountId>(
	enclave_account: AccountId,
) -> Result<(), StfError>
where
	Runtime: frame_system::Config<AccountId = AccountId> + pallet_balances::Config,
	<<Runtime as frame_system::Config>::Lookup as StaticLookup>::Source: From<AccountId>,
	Runtime::Balance: From<u32>,
{
	pallet_balances::Call::<Runtime>::force_set_balance {
		who: enclave_account.into(),
		new_free: 1000.into(),
	}
	.dispatch_bypass_filter(Runtime::RuntimeOrigin::root())
	.map_err(|e| {
		StfError::Dispatch(format!("Set Balance for enclave signer account error: {:?}", e.error))
	})
	.map(|_| ())
}
