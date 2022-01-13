use frame_support::weights::{DispatchInfo, PostDispatchInfo, Weight};

pub use runtime::{
	AccountId, Balance, Balances, Call, Runtime, System, TransactionPayment, Treasury,
};
pub const ALICE: [u8; 32] = [1u8; 32];
pub const BOB: [u8; 32] = [2u8; 32];
pub use pallet_balances::Call as BalancesCall;

pub struct ExtBuilder {
	balances: Vec<(AccountId, Balance)>,
	parachain_id: u32,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { balances: vec![], parachain_id: 2008 }
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
				.clone()
				.into_iter()
				.map(|(account_id, initial_balance)| (account_id, initial_balance))
				.collect::<Vec<_>>(),
		}
		.assimilate_storage(&mut t)
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
