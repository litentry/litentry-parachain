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

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {

	// Benchmark `create_identity`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	create_identity {
		let caller = whitelisted_caller();
		let shard = H256::from_slice(&TEST_MRENCLAVE);
		let encrypted_did = vec![1u8; 2048];
		let encrypted_metadata = Some(vec![1u8; 2048]);
	}: _(RawOrigin::Signed(caller), shard, encrypted_did, encrypted_metadata)
	verify {
		assert_last_event::<T>(Event::CreateIdentityRequested{ shard }.into());
	}

	// Benchmark `remove_identity`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	remove_identity {
		let caller = whitelisted_caller();
		let shard = H256::from_slice(&TEST_MRENCLAVE);
		let encrypted_did = vec![1u8; 2048];
	}: _(RawOrigin::Signed(caller), shard, encrypted_did )
	verify {
		assert_last_event::<T>(Event::RemoveIdentityRequested{ shard }.into());
	}

	// Benchmark `verify_identity`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	verify_identity {
		let caller = whitelisted_caller();
		let shard = H256::from_slice(&TEST_MRENCLAVE);
		let encrypted_did = vec![1u8; 2048];
		let encrypted_validation_data = vec![1u8; 2048];
	}: _(RawOrigin::Signed(caller), shard, encrypted_did, encrypted_validation_data )
	verify {
		assert_last_event::<T>(Event::VerifyIdentityRequested{ shard }.into());
	}

	// Benchmark `set_user_shielding_key`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	set_user_shielding_key {
		let caller = whitelisted_caller();
		let shard = H256::from_slice(&TEST_MRENCLAVE);
		let encrypted_key = vec![1u8; 2048];
	}: _(RawOrigin::Signed(caller), shard, encrypted_key )
	verify {
		assert_last_event::<T>(Event::SetUserShieldingKeyRequested{ shard }.into());
	}
}

impl_benchmark_test_suite!(IdentityManagement, crate::mock::new_test_ext(), crate::mock::Test,);
