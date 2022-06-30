use super::{
	weights, xcm_config::CurrencyId, AccountId, Amount, AssetId, AssetManager, Balance, Balances,
	Event, Runtime, TreasuryPalletId,
};
use frame_support::{
	parameter_types,
	traits::{ConstU32, Contains},
};
use runtime_common::EnsureRootOrHalfCouncil;
use sp_runtime::traits::AccountIdConversion;
use sp_std::prelude::*;

pub fn get_all_module_accounts() -> Vec<AccountId> {
	// Add whitelist here, usually this is the system account like treasury
	vec![]
}

parameter_types! {
	pub LitTreasuryAccount: AccountId = TreasuryPalletId::get().into_account_truncating();
}

pub struct DustRemovalWhitelist;
impl Contains<AccountId> for DustRemovalWhitelist {
	fn contains(a: &AccountId) -> bool {
		get_all_module_accounts().contains(a)
	}
}

pub type ReserveIdentifier = [u8; 8];

impl orml_tokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	// Get this info from pallet_asset_manager metadata
	type ExistentialDeposits = AssetManager;
	type OnDust = orml_tokens::TransferDust<Runtime, LitTreasuryAccount>;
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = ReserveIdentifier;
	type DustRemovalWhitelist = DustRemovalWhitelist;
	type WeightInfo = ();
	type OnNewTokenAccount = ();
	type OnKilledTokenAccount = ();
}

impl pallet_asset_manager::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type AssetId = AssetId;
	type ForeignAssetType = CurrencyId;
	type ForeignAssetModifierOrigin = EnsureRootOrHalfCouncil;
	type Currency = Balances;
	type WeightInfo = weights::pallet_asset_manager::WeightInfo<Runtime>;
}
