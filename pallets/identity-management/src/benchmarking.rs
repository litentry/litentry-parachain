// Copyright 2020-2023 Litentry Technologies GmbH.
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

use crate::Pallet as IdentityManagement;
#[allow(unused)]
use core_primitives::{AesOutput, ErrorDetail, IMPError};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, BenchmarkError};
use frame_support::traits::EnsureOrigin;
use frame_system::RawOrigin;
use sp_core::H256;
use sp_std::vec;

use test_utils::ias::consts::TEST8_MRENCLAVE;
const USER_SEED: u32 = 9966;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	// Benchmark `add_delegatee`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	add_delegatee {
		let account: T::AccountId =  frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
	}: _(RawOrigin::Root, account.clone())
	verify{
		assert!(Delegatee::<T>::contains_key(account));
	}
	// Benchmark `remove_delegatee`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	remove_delegatee {
		let account: T::AccountId =  frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		IdentityManagement::<T>::add_delegatee(RawOrigin::Root.into(), account.clone())?;
	}: _(RawOrigin::Root, account.clone())
	verify{
		assert!(!Delegatee::<T>::contains_key(account));
	}
	// Benchmark `create_identity`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	create_identity {
		let caller: T::AccountId =  frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		let shard = H256::from_slice(&TEST8_MRENCLAVE);
		let encrypted_did = vec![1u8; 2048];
		let encrypted_metadata = Some(vec![1u8; 2048]);
	}: _(RawOrigin::Signed(caller.clone()), shard, caller.clone(), encrypted_did, encrypted_metadata)
	verify {
		assert_last_event::<T>(Event::CreateIdentityRequested{ shard }.into());
	}

	// Benchmark `remove_identity`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	remove_identity {
		let caller: T::AccountId =  frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		let shard = H256::from_slice(&TEST8_MRENCLAVE);
		let encrypted_did = vec![1u8; 2048];
		let encrypted_metadata = Some(vec![1u8; 2048]);
		IdentityManagement::<T>::create_identity(RawOrigin::Signed(caller.clone()).into(), shard, caller.clone(), encrypted_did.clone(), encrypted_metadata)?;
	}: _(RawOrigin::Signed(caller), shard, encrypted_did)
	verify {
		assert_last_event::<T>(Event::RemoveIdentityRequested{ shard }.into());
	}

	// Benchmark `verify_identity`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	verify_identity {
		let caller: T::AccountId =  frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		let shard = H256::from_slice(&TEST8_MRENCLAVE);
		let encrypted_did = vec![1u8; 2048];
		let encrypted_validation_data = vec![1u8; 2048];
	}: _(RawOrigin::Signed(caller), shard, encrypted_did, encrypted_validation_data)
	verify {
		assert_last_event::<T>(Event::VerifyIdentityRequested{ shard }.into());
	}

	// Benchmark `set_user_shielding_key`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	set_user_shielding_key {
		let caller: T::AccountId =  frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		let shard = H256::from_slice(&TEST8_MRENCLAVE);
		let encrypted_key = vec![1u8; 2048];
	}: _(RawOrigin::Signed(caller), shard, encrypted_key)
	verify {
		assert_last_event::<T>(Event::SetUserShieldingKeyRequested{ shard }.into());
	}

	// Benchmark `user_shielding_key_set`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	user_shielding_key_set {
		let req_ext_hash = H256::default();
		let id_graph = AesOutput::default();
		let call_origin = T::TEECallOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
		let account: T::AccountId =  frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
	}: _<T::RuntimeOrigin>(call_origin, account.clone(), id_graph.clone(), req_ext_hash)
	verify {
		assert_last_event::<T>(Event::UserShieldingKeySet { account, id_graph, req_ext_hash }.into());
	}

	// Benchmark `identity_created`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	identity_created {
		let req_ext_hash = H256::default();
		let identity = AesOutput::default();
		let code = AesOutput::default();
		let call_origin = T::TEECallOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
		let account: T::AccountId =  frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
	}: _<T::RuntimeOrigin>(call_origin, account.clone(), identity.clone(), code.clone(), req_ext_hash)
	verify {
		assert_last_event::<T>(Event::IdentityCreated { account, identity, code, req_ext_hash }.into());
	}

	// Benchmark `identity_removed`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	identity_removed {
		let req_ext_hash = H256::default();
		let identity = AesOutput::default();
		let call_origin = T::TEECallOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
		let account: T::AccountId =  frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
	}: _<T::RuntimeOrigin>(call_origin, account.clone(), identity.clone(), req_ext_hash)
	verify {
		assert_last_event::<T>(Event::IdentityRemoved { account, identity, req_ext_hash }.into());
	}

	// Benchmark `identity_verified`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	identity_verified {
		let req_ext_hash = H256::default();
		let identity = AesOutput::default();
		let id_graph = AesOutput::default();
		let call_origin = T::TEECallOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
		let account: T::AccountId =  frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
	}: _<T::RuntimeOrigin>(call_origin, account.clone(), identity.clone(), id_graph.clone(), req_ext_hash)
	verify {
		assert_last_event::<T>(Event::IdentityVerified { account, identity, id_graph, req_ext_hash }.into());
	}

	// Benchmark `some_error`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	some_error {
		let call_origin = T::TEECallOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
		let account: T::AccountId =  frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		let detail = ErrorDetail::WrongWeb2Handle;
		let error = IMPError::VerifyIdentityFailed(detail.clone());
		let req_ext_hash = H256::default();
	}: _<T::RuntimeOrigin>(call_origin, Some(account.clone()), error, req_ext_hash)
	verify {
		assert_last_event::<T>(Event::VerifyIdentityFailed { account: Some(account), detail, req_ext_hash }.into())
	}
}

#[cfg(test)]
mod tests {
	pub fn new_test_ext() -> sp_io::TestExternalities {
		let t = frame_system::GenesisConfig::default()
			.build_storage::<crate::mock::Test>()
			.unwrap();
		sp_io::TestExternalities::new(t)
	}
}

impl_benchmark_test_suite!(
	IdentityManagement,
	crate::benchmarking::tests::new_test_ext(),
	crate::mock::Test,
);
