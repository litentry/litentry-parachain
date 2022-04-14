use super::xcm_config::CurrencyId;
use super::{
    AccountId, Amount, AssetId, Balance, Balances, Event, MaxLocks, Runtime, TreasuryPalletId,
};
use orml_traits::{parameter_type_with_key};
use frame_support::parameter_types;
use frame_support::traits::Contains;
use frame_system::EnsureRoot;
use sp_std::prelude::*;
use sp_runtime::traits::AccountIdConversion;

pub fn get_all_module_accounts() -> Vec<AccountId> {
	vec![]
}


// Replace this in our asset_manager pallet
parameter_type_with_key! {
	pub ExistentialDeposits: |currency_id: AssetId| -> Balance {
		match currency_id {
			_ => 0,
		}
	};
}

parameter_types! {
	pub LitTreasuryAccount: AccountId = TreasuryPalletId::get().into_account();
}

pub struct DustRemovalWhitelist;
impl Contains<AccountId> for DustRemovalWhitelist {
	fn contains(a: &AccountId) -> bool {
		get_all_module_accounts().contains(a)
	}
}

impl orml_tokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type Amount = Amount;
	type CurrencyId = AssetId;
	// Get this info from pallet_asset_manager
	type ExistentialDeposits = ExistentialDeposits;
	type OnDust = orml_tokens::TransferDust<Runtime, LitTreasuryAccount>;
	type MaxLocks = MaxLocks;
	type DustRemovalWhitelist = DustRemovalWhitelist;
	type WeightInfo = ();
}

impl pallet_asset_manager::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type AssetId = AssetId;
	type ForeignAssetType = CurrencyId;
	type ForeignAssetModifierOrigin = EnsureRoot<AccountId>;
	type Currency = Balances;
	type WeightInfo = ();
}

