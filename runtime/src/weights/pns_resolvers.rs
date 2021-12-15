use frame_support::dispatch::Weight;

// TODO: More accurate gas fees

pub struct WeightInfo;

impl pns_resolvers::WeightInfo for WeightInfo {
	fn set_text(_content_len: usize) -> Weight {
		10_000
	}

	fn set_account() -> Weight {
		10_000
	}
}
