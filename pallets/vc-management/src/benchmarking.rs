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

use crate::Pallet as VCManagement;
#[allow(unused)]
use core_primitives::{AesOutput, ErrorDetail, VCMPError};
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use sp_core::H256;
use sp_std::vec;

const TEST_MRENCLAVE: [u8; 32] = [2u8; 32];
const USER_SEED: u32 = 9966;
const VC_HASH: H256 = H256::zero();
const VC_INDEX: H256 = H256::zero();

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	// Benchmark `request_vc`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	request_vc {
		let account: T::AccountId = frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		let shard = H256::from_slice(&TEST_MRENCLAVE);
		let assertion = Assertion::A1;
	}: _(RawOrigin::Signed(account.clone()), shard, assertion.clone())
	verify{
		assert_last_event::<T>(Event::VCRequested{ account, shard, assertion }.into());
	}

	// Benchmark `disable_vc`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	disable_vc {
		let account: T::AccountId = frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		let assertion = Assertion::A1;
		let vc = AesOutput::default();
		let req_ext_hash = H256::default();
		VCManagement::<T>::vc_issued(RawOrigin::Signed(account.clone()).into(), account.clone(), assertion, VC_INDEX, VC_HASH, vc, req_ext_hash)?;
	}: _(RawOrigin::Signed(account.clone()), VC_INDEX)
	verify{
		assert_last_event::<T>(Event::VCDisabled{ account, index: VC_HASH }.into());
	}

	// Benchmark `revoke_vc`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	revoke_vc {
		let account: T::AccountId = frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		let assertion = Assertion::A1;
		let vc = AesOutput::default();
		let req_ext_hash = H256::default();
		VCManagement::<T>::vc_issued(RawOrigin::Signed(account.clone()).into(), account.clone(), assertion, VC_INDEX, VC_HASH, vc, req_ext_hash)?;
	}: _(RawOrigin::Signed(account.clone()), VC_INDEX)
	verify{
		assert_last_event::<T>(Event::VCRevoked{ account, index: VC_HASH }.into());
	}

	// Benchmark `vc_issued`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	vc_issued {
		let account: T::AccountId = frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		let assertion = Assertion::A1;
		let vc = AesOutput::default();
		let req_ext_hash = H256::default();
	}: _(RawOrigin::Signed(account.clone()), account.clone(), assertion.clone(), VC_INDEX, VC_HASH, vc.clone(), req_ext_hash)
	verify{
		assert_last_event::<T>(Event::VCIssued{ account, assertion, index: VC_INDEX, vc, req_ext_hash}.into());
	}

	// Benchmark `some_error`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	some_error {
		let account: T::AccountId = frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		let detail = ErrorDetail::WrongWeb2Handle;
		let assertion = Assertion::A1;
		let error = VCMPError::RequestVCFailed(assertion.clone(), detail.clone());
		let req_ext_hash = H256::default();
	}: _(RawOrigin::Signed(account.clone()), Some(account.clone()), error, req_ext_hash)
	verify {
		assert_last_event::<T>(Event::RequestVCFailed { account: Some(account), assertion, detail, req_ext_hash }.into())
	}

	// Benchmark `set_admin`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	set_admin {
		let account: T::AccountId = frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		let new_schema_admin: T::AccountId = frame_benchmarking::account("TEST_A", 0u32, USER_SEED + 1);
	}: _(RawOrigin::Signed(account.clone()), new_schema_admin.clone())
	verify {
		assert_last_event::<T>(Event::AdminChanged { old_admin: None, new_admin: Some(new_schema_admin) }.into())
	}

	// Benchmark `add_schema`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	add_schema {
		let account: T::AccountId = frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		VCManagement::<T>::set_admin(RawOrigin::Root.into(), account.clone())?;
		let id: Vec<u8> = vec![1, 2, 3, 4];
		let content: Vec<u8> = vec![5, 6, 7, 8];
		let shard = H256::from_slice(&TEST_MRENCLAVE);
	}: _(RawOrigin::Signed(account.clone()), shard, id, content)
	verify {
		assert_last_event::<T>(Event::SchemaIssued { account, shard, index: 0 }.into())
	}

	// Benchmark `disable_schema`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	disable_schema {
		let account: T::AccountId = frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		VCManagement::<T>::set_admin(RawOrigin::Root.into(), account.clone())?;
		let id: Vec<u8> = vec![1, 2, 3, 4];
		let content: Vec<u8> = vec![5, 6, 7, 8];
		let shard = H256::from_slice(&TEST_MRENCLAVE);
		VCManagement::<T>::add_schema(RawOrigin::Signed(account.clone()).into(), shard, id, content)?;
	}: _(RawOrigin::Signed(account.clone()), shard, 0)
	verify {
		assert_last_event::<T>(Event::SchemaDisabled { account, shard, index: 0 }.into())
	}

	// Benchmark `activate_schema`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	activate_schema {
		let account: T::AccountId = frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		VCManagement::<T>::set_admin(RawOrigin::Root.into(), account.clone())?;
		let id: Vec<u8> = vec![1, 2, 3, 4];
		let content: Vec<u8> = vec![5, 6, 7, 8];
		let shard = H256::from_slice(&TEST_MRENCLAVE);
		VCManagement::<T>::add_schema(RawOrigin::Signed(account.clone()).into(), shard, id, content)?;
		VCManagement::<T>::disable_schema(RawOrigin::Signed(account.clone()).into(), shard, 0)?;
	}: _(RawOrigin::Signed(account.clone()), shard, 0)
	verify {
		assert_last_event::<T>(Event::SchemaActivated { account, shard, index: 0 }.into())
	}

	// Benchmark `revoke_schema`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	revoke_schema {
		let account: T::AccountId = frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		VCManagement::<T>::set_admin(RawOrigin::Root.into(), account.clone())?;
		let id: Vec<u8> = vec![1, 2, 3, 4];
		let content: Vec<u8> = vec![5, 6, 7, 8];
		let shard = H256::from_slice(&TEST_MRENCLAVE);
		VCManagement::<T>::add_schema(RawOrigin::Signed(account.clone()).into(), shard, id, content)?;
	}: _(RawOrigin::Signed(account.clone()), shard, 0)
	verify {
		assert_last_event::<T>(Event::SchemaRevoked { account, shard, index: 0 }.into())
	}

	// Benchmark `add_vc_registry_item`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	add_vc_registry_item {
		let account: T::AccountId = frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		VCManagement::<T>::set_admin(RawOrigin::Root.into(), account.clone())?;
		let assertion = Assertion::A1;
	}: _(RawOrigin::Signed(account.clone()), VC_INDEX, account.clone(), assertion.clone(), VC_HASH)
	verify {
		assert_last_event::<T>(Event::VCRegistryItemAdded { account, assertion, index: VC_INDEX }.into())
	}

	// Benchmark `remove_vc_registry_item`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of encrypted_data size.
	remove_vc_registry_item {
		let account: T::AccountId = frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		VCManagement::<T>::set_admin(RawOrigin::Root.into(), account.clone())?;
		let assertion = Assertion::A1;
		VCManagement::<T>::add_vc_registry_item(RawOrigin::Signed(account.clone()).into(), VC_INDEX, account.clone(), assertion, VC_HASH)?;
	}: _(RawOrigin::Signed(account), VC_INDEX)
	verify {
		assert_last_event::<T>(Event::VCRegistryItemRemoved { index: VC_INDEX }.into())
	}

	// Benchmark `clear_vc_registry`.
	clear_vc_registry {
		let x in 0..100u32;
		let account: T::AccountId = frame_benchmarking::account("TEST_A", 0u32, USER_SEED);
		VCManagement::<T>::set_admin(RawOrigin::Root.into(), account.clone())?;
		let assertion = Assertion::A1;
		for i in 0..x {
			let seed = USER_SEED - i;
			let candidate:T::AccountId = frame_benchmarking::account("TEST_A", 0u32, seed);
			VCManagement::<T>::add_vc_registry_item(RawOrigin::Signed(account.clone()).into(), H256::from_low_u64_be(seed.into()), candidate.clone(), assertion.clone(), VC_HASH)?;
		}
	}: _(RawOrigin::Signed(account))
	verify {
		assert_last_event::<T>(Event::VCRegistryCleared.into())
	}
}

impl_benchmark_test_suite!(VCManagement, crate::mock::new_test_ext(), crate::mock::Test,);
