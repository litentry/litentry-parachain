use frame_support::traits::{GenesisBuild, OnFinalize, OnIdle, OnInitialize};
use runtime::{AccountId, Balance, Runtime, System};

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
		// let evm_genesis_accounts = evm_genesis(vec![]);

		let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

		// GenesisBuild::<Runtime>::assimilate_storage().unwrap();

		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}
