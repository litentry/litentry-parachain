use frame_support::{
	traits::{GenesisBuild, OnFinalize, OnIdle, OnInitialize},
	weights::{DispatchInfo, PostDispatchInfo, Weight},
};
use sp_runtime::MultiAddress;
// use frame_support::{
// 	parameter_types,
// 	traits::{FindAuthor, Get},
// 	weights::{
// 		DispatchClass, DispatchInfo, PostDispatchInfo, Weight, WeightToFeeCoefficient,
// 		WeightToFeeCoefficients, WeightToFeePolynomial,
// 	},
// 	PalletId,
// };

pub use runtime::{
	AccountId, Balance, Balances, Call, Runtime, System, TransactionPayment, Treasury,
};
pub const ALICE: [u8; 32] = [1u8; 32];
pub const BOB: [u8; 32] = [2u8; 32];

// pub const ALICE_ACCOUNT: AccountId = AccountId([4u8; 32]);

pub use pallet_balances::Call as BalancesCall;
// pub const CALL: &<Runtime as frame_system::Config>::Call =
// 	&Call::Balances(BalancesCall::transfer {
//         dest: MultiAddress::Id(ALICE_ACCOUNT),
//         value: 69 });

pub struct ExtBuilder {
	balance_factor: u64,
	base_weight: u64,
	byte_fee: u64,
	weight_to_fee: u64,
	balances: Vec<(AccountId, Balance)>,
	parachain_id: u32,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self {
			balances: vec![],
			parachain_id: 2008,
			balance_factor: 1,
			base_weight: 0,
			byte_fee: 1,
			weight_to_fee: 1,
		}
	}
}

impl ExtBuilder {
	pub fn base_weight(mut self, base_weight: u64) -> Self {
		self.base_weight = base_weight;
		self
	}
	pub fn balance_factor(mut self, factor: u64) -> Self {
		self.balance_factor = factor;
		self
	}
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
