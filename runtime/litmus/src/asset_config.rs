use super::{
	xcm_config::CurrencyId, AccountId, Amount, AssetId, AssetManager, Balance, Balances, Event, MaxLocks,
	Runtime, TreasuryPalletId,
};
use frame_support::{parameter_types, traits::Contains};
use frame_system::EnsureRoot;
use orml_traits::parameter_type_with_key;
use sp_runtime::traits::AccountIdConversion;
use sp_std::prelude::*;

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
	// Get this info from pallet_asset_manager metadata
	type ExistentialDeposits = AssetManager;
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
