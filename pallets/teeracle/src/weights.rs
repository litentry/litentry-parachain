/*
Copyright 2021 Integritee AG

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

*/
use frame_support::weights::Weight;
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_exchange.
pub trait WeightInfo {
	fn add_to_whitelist() -> Weight;
	fn remove_from_whitelist() -> Weight;
	fn update_exchange_rate() -> Weight;
	fn update_oracle() -> Weight;
}

pub struct IntegriteeWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for IntegriteeWeight<T> {
	fn add_to_whitelist() -> Weight {
		Weight::from_ref_time(46_200_000)
	}
	fn remove_from_whitelist() -> Weight {
		Weight::from_ref_time(46_200_000)
	}
	fn update_exchange_rate() -> Weight {
		Weight::from_ref_time(46_200_000)
	}
	fn update_oracle() -> Weight {
		Weight::from_ref_time(46_200_000)
	}
}
// For tests
impl WeightInfo for () {
	fn add_to_whitelist() -> Weight {
		Weight::from_ref_time(46_200_000)
	}
	fn remove_from_whitelist() -> Weight {
		Weight::from_ref_time(46_200_000)
	}
	fn update_exchange_rate() -> Weight {
		Weight::from_ref_time(46_200_000)
	}
	fn update_oracle() -> Weight {
		Weight::from_ref_time(46_200_000)
	}
}
