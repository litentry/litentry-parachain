// Copyright 2020-2022 Litentry Technologies GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

//! Benchmarking setup for pallet-identity-management

use super::*;

#[allow(unused)]
use crate::Pallet as IdentityManagement;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_core::H256;
use sp_std::vec;

const TEST_MRENCLAVE: [u8; 32] = [2u8; 32];

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {

	// Benchmark `link_identity`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	link_identity {
		let caller = whitelisted_caller();
		let shard = H256::from_slice(&TEST_MRENCLAVE);
		let encrypted_data = vec![1u8; 2048];
	}: _(RawOrigin::Signed(caller), shard, encrypted_data )
	verify {
		assert_last_event::<T>(Event::LinkIdentityRequested.into());
	}

	// Benchmark `unlink_identity`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	unlink_identity {
		let caller = whitelisted_caller();
		let shard = H256::from_slice(&TEST_MRENCLAVE);
		let encrypted_data = vec![1u8; 2048];
	}: _(RawOrigin::Signed(caller), shard, encrypted_data )
	verify {
		assert_last_event::<T>(Event::UnlinkIdentityRequested.into());
	}

	// Benchmark `verify_identity`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	verify_identity {
		let caller = whitelisted_caller();
		let shard = H256::from_slice(&TEST_MRENCLAVE);
		let encrypted_data = vec![1u8; 2048];
	}: _(RawOrigin::Signed(caller), shard, encrypted_data )
	verify {
		assert_last_event::<T>(Event::VerifyIdentityRequested.into());
	}

	// Benchmark `set_user_shielding_key`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	set_user_shielding_key {
		let caller = whitelisted_caller();
		let shard = H256::from_slice(&TEST_MRENCLAVE);
		let encrypted_data = vec![1u8; 2048];
	}: _(RawOrigin::Signed(caller), shard, encrypted_data )
	verify {
		assert_last_event::<T>(Event::SetShieldingKeyRequested.into());
	}
}

impl_benchmark_test_suite!(IdentityManagement, crate::mock::new_test_ext(), crate::mock::Test,);
