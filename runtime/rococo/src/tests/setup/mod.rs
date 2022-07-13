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

use frame_support::{
	traits::GenesisBuild,
	weights::{DispatchInfo, PostDispatchInfo, Weight},
};

pub use crate::{
	AccountId, AssetManager, Balance, Balances, Call, CumulusXcm, DmpQueue, Event, ExtrinsicFilter,
	Multisig, Origin, ParachainSystem, PolkadotXcm, Runtime, System, Tokens, TransactionByteFee,
	TransactionPayment, Treasury, Vesting, XTokens, XcmpQueue,
};
use runtime_common::currency::*;

pub const ALICE: [u8; 32] = [1u8; 32];
pub const BOB: [u8; 32] = [2u8; 32];
pub const CHARLIE: [u8; 32] = [3u8; 32];

pub use sp_std::cell::RefCell;
pub mod relay;
pub use pallet_balances::Call as BalancesCall;
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};

// TODO::Common folder for genreal utility function
pub(crate) fn last_event() -> Event {
	System::events().pop().expect("Event expected").event
}

pub(crate) fn alice() -> AccountId {
	AccountId::from(ALICE)
}

pub(crate) fn bob() -> AccountId {
	AccountId::from(BOB)
}

pub(crate) fn charlie() -> AccountId {
	AccountId::from(CHARLIE)
}

pub const PARA_A_USER_INITIAL_BALANCE: u128 = 500_000 * UNIT;
pub const PARA_B_USER_INITIAL_BALANCE: u128 = 600_000 * UNIT;

pub struct ExtBuilder {
	balances: Vec<(AccountId, Balance)>,
	parachain_id: u32,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { balances: vec![], parachain_id: 1 }
	}
}

impl ExtBuilder {
	pub fn balances(mut self, balances: Vec<(AccountId, Balance)>) -> Self {
		self.balances = balances;
		self
	}

	#[allow(dead_code)]
	pub fn parachain_id(mut self, parachain_id: u32) -> Self {
		self.parachain_id = parachain_id;
		self
	}

	pub fn build(self) -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();
		pallet_balances::GenesisConfig::<Runtime> {
			balances: self
				.balances
				.into_iter()
				.map(|(account_id, initial_balance)| (account_id, initial_balance))
				.collect::<Vec<_>>(),
		}
		.assimilate_storage(&mut t)
		.unwrap();

		let parachain_info_config =
			parachain_info::GenesisConfig { parachain_id: self.parachain_id.into() };
		<parachain_info::GenesisConfig as GenesisBuild<Runtime, _>>::assimilate_storage(
			&parachain_info_config,
			&mut t,
		)
		.unwrap();

		<pallet_xcm::GenesisConfig as frame_support::traits::GenesisBuild<Runtime>>::assimilate_storage(
			&pallet_xcm::GenesisConfig { safe_xcm_version: Some(2) },
			&mut t,
		)
		.unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

/// create a transaction info struct from weight. Handy to avoid building the whole struct.
pub fn info_from_weight(w: Weight) -> DispatchInfo {
	// pays_fee: Pays::Yes -- class: DispatchClass::Normal
	DispatchInfo { weight: w, ..Default::default() }
}

pub fn post_info_from_weight(w: Weight) -> PostDispatchInfo {
	PostDispatchInfo { actual_weight: Some(w), pays_fee: Default::default() }
}

pub fn run_with_system_weight<F>(w: Weight, mut assertions: F)
where
	F: FnMut(),
{
	let mut t: sp_io::TestExternalities = frame_system::GenesisConfig::default()
		.build_storage::<Runtime>()
		.unwrap()
		.into();
	t.execute_with(|| {
		System::set_block_consumed_resources(w, 0);
		assertions()
	});
}

decl_test_parachain! {
	pub struct ParaA {
		Runtime = Runtime,
		XcmpMessageHandler = XcmpQueue,
		DmpMessageHandler = DmpQueue,
		new_ext = ExtBuilder::default()
		.balances(vec![
			// fund Alice
			(alice(), PARA_A_USER_INITIAL_BALANCE),
		]).parachain_id(1).build(),
	}
}

decl_test_parachain! {
	pub struct ParaB {
		Runtime = Runtime,
		XcmpMessageHandler = XcmpQueue,
		DmpMessageHandler = DmpQueue,
		new_ext = ExtBuilder::default()
		.balances(vec![
			// fund BOB
			(bob(), PARA_B_USER_INITIAL_BALANCE),
		]).parachain_id(2).build(),
	}
}

pub fn relay_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<relay::Runtime>()
		.unwrap();

	pallet_balances::GenesisConfig::<relay::Runtime> { balances: vec![] }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| relay::System::set_block_number(1));
	ext
}

decl_test_relay_chain! {
	pub struct Relay {
		Runtime = relay::Runtime,
		XcmConfig = relay::XcmConfig,
		new_ext = relay_ext(),
	}
}

decl_test_network! {
	pub struct TestNet {
		relay_chain = Relay,
		parachains = vec![
			(1, ParaA),
			(2, ParaB),
		],
	}
}
