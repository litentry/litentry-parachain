use frame_support::dispatch::Weight;
use pns_registrar::{price_oracle, redeem_code, registrar, registry};

// TODO: More accurate gas fees

pub struct WeightInfo;

impl price_oracle::WeightInfo for WeightInfo {
	fn set_exchange_rate() -> Weight {
		10_000
	}

	fn set_base_price(_len: u32) -> Weight {
		10_000
	}

	fn set_rent_price(_len: u32) -> Weight {
		10_000
	}
}

impl registrar::WeightInfo for WeightInfo {
	fn mint_subname(_len: u32) -> Weight {
		10_000
	}

	fn register(_len: u32) -> Weight {
		10_000
	}

	fn renew(_len: u32) -> Weight {
		10_000
	}

	fn set_owner() -> Weight {
		10_000
	}

	fn reclaimed() -> Weight {
		10_000
	}

	fn add_reserved() -> Weight {
		10_000
	}

	fn remove_reserved() -> Weight {
		10_000
	}
}

impl registry::WeightInfo for WeightInfo {
	fn approve(_approve: bool) -> Weight {
		10_000
	}

	fn approval_for_all(_approve: bool) -> Weight {
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
}

impl redeem_code::WeightInfo for WeightInfo {
	fn mint_redeem(_len: Option<u32>) -> Weight {
		10_000
	}

	fn name_redeem(_len: u32) -> Weight {
		10_000
	}

	fn name_redeem_any(_len: u32) -> Weight {
		10_000
	}
}

impl pns_registrar::origin::WeightInfo for WeightInfo {
	fn set_origin(_approve: bool) -> Weight {
		10_000
	}
}
