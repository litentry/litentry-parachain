/// Traits for pallet-halving-mint
use frame_support::pallet_prelude::Weight;

pub trait OnTokenMinted<AccountId, Balance> {
	fn token_minted(beneficiary: AccountId, amount: Balance) -> Weight;
}

impl<AccountId, Balance> OnTokenMinted<AccountId, Balance> for () {
	fn token_minted(_beneficiary: AccountId, _amount: Balance) -> Weight {
		Weight::zero()
	}
}
