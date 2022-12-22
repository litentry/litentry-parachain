/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

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

//! Teerex pallet benchmarking

#![cfg(any(test, feature = "runtime-benchmarks"))]

use super::*;

use crate::Pallet as Teerex;
use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;
use sp_runtime::traits::CheckedConversion;
use sp_std::vec;
use test_utils::{
	get_signer,
	ias::{consts::*, setups::*},
};

fn ensure_not_skipping_ra_check() {
	#[cfg(not(test))]
	if cfg!(feature = "skip-ias-check") {
		panic!("Benchmark does not allow the `skip-ias-check` flag.");
	};
}

fn generate_accounts<T: Config>(amount: u32) -> Vec<T::AccountId> {
	(0..amount).map(|n| account("dummy name", n, n)).collect()
}

fn add_enclaves_to_registry<T: Config>(accounts: &[T::AccountId]) {
	for a in accounts.iter() {
		Teerex::<T>::add_enclave(
			a,
			&Enclave::test_enclave(a.clone()).with_mr_enclave(TEST4_SETUP.mrenclave),
		)
		.unwrap();
	}
}

benchmarks! {
	// Note: The storage-map structure has the following complexity for updating:
	//   DB Reads: O(1) Encoding: O(1) DB Writes: O(1)
	//
	// Hence, it does not matter how many other enclaves are registered for the benchmark.


	// Benchmark `register_enclave` with the worst possible conditions:
	// * remote attestation is valid
	// * enclave already exists
	where_clause {  where T::AccountId: From<[u8; 32]> }
	register_enclave {
		ensure_not_skipping_ra_check();
		timestamp::Pallet::<T>::set_timestamp(TEST4_SETUP.timestamp.checked_into().unwrap());
		let signer: T::AccountId = get_signer(TEST4_SETUP.signer_pub);

		// simply register the enclave before to make sure it already
		// exists when running the benchmark
		Teerex::<T>::register_enclave(
			RawOrigin::Signed(signer.clone()).into(),
			TEST4_SETUP.cert.to_vec(),
			URL.to_vec()
		).unwrap();

	}: _(RawOrigin::Signed(signer), TEST4_SETUP.cert.to_vec(), URL.to_vec())
	verify {
		assert_eq!(Teerex::<T>::enclave_count(), 1);
	}

	// Benchmark `unregister_enclave` enclave with the worst possible conditions:
	// * enclave exists
	// * enclave is not the most recently registered enclave
	unregister_enclave {
		let enclave_count = 3;
		let accounts: Vec<T::AccountId> = generate_accounts::<T>(enclave_count);
		add_enclaves_to_registry::<T>(&accounts);

	}: _(RawOrigin::Signed(accounts[0].clone()))
	verify {
		assert!(!crate::EnclaveIndex::<T>::contains_key(&accounts[0]));
		assert_eq!(Teerex::<T>::enclave_count(), enclave_count as u64 - 1);
	}

	// Benchmark `call_worker`. There are no worst conditions. The benchmark showed that
	// execution time is constant irrespective of cyphertext size.
	call_worker {
		let accounts: Vec<T::AccountId> = generate_accounts::<T>(1);
		let req = Request { shard:H256::from_slice(&TEST4_SETUP.mrenclave), cyphertext: vec![1u8; 2000]};
	}: _(RawOrigin::Signed(accounts[0].clone()), req)

	// Benchmark `confirm_processed_parentchain_block` with the worst possible conditions:
	// * sender enclave is registered
	confirm_processed_parentchain_block {
		let accounts: Vec<T::AccountId> = generate_accounts::<T>(1);
		add_enclaves_to_registry::<T>(&accounts);

		let block_hash: H256 = [2; 32].into();
		let merkle_root: H256 = [4; 32].into();
		let block_number: u32 = 0;

	}: _(RawOrigin::Signed(accounts[0].clone()), block_hash, block_number.into(), merkle_root)
}

#[cfg(test)]
use crate::{Config, Pallet as PalletModule};

#[cfg(test)]
use frame_benchmarking::impl_benchmark_test_suite;
use test_utils::ias::TestEnclave;

#[cfg(test)]
impl_benchmark_test_suite!(PalletModule, crate::mock::new_test_ext(), crate::mock::Test,);
