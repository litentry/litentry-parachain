use frame_support::dispatch::Weight;
use pns_registrar::{price_oracle, redeem_code, registrar, registry};

// TODO: More accurate gas fees

pub struct WeightInfo;

impl price_oracle::WeightInfo for WeightInfo {
	fn set_price() -> Weight {
		10_000
	}
}

impl registrar::WeightInfo for WeightInfo {
	fn mint_subname() -> Weight {
		10_000
	}

	fn register() -> Weight {
		10_000
	}

	fn renew() -> Weight {
		10_000
	}

	fn set_owner() -> Weight {
		10_000
	}

	fn reclaimed() -> Weight {
		10_000
	}

	fn add_blacklist() -> Weight {
		10_000
	}

	fn remove_blacklist() -> Weight {
		10_000
	}
}

impl registry::WeightInfo for WeightInfo {
	fn set_approval_for_all() -> Weight {
		10_000
	}

	fn set_resolver() -> Weight {
		10_000
	}

	fn destroy() -> Weight {
		10_000
	}

	fn set_official() -> Weight {
		10_000
	}

	fn add_manger() -> Weight {
		10_000
	}

	fn remove_manger() -> Weight {
		10_000
	}
}

impl redeem_code::WeightInfo for WeightInfo {
	fn mint_redeem(_len: Option<u32>) -> Weight {
		10_000
	}

	fn name_redeem() -> Weight {
		10_000
	}

	fn name_redeem_any() -> Weight {
		10_000
	}
}
