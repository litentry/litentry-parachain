use super::{Pallet as Teebag, *};
use frame_benchmarking::v2::*;
use frame_support::assert_ok;
use frame_system::RawOrigin;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

fn create_test_enclaves<T: Config>(n: u32, mrenclave: MrEnclave) {
	for i in 0..n {
		let who: T::AccountId = account("who", i, 1);
		let test_enclave = Enclave::new(WorkerType::Identity).with_mrenclave(mrenclave.clone());
		assert_ok!(Teebag::<T>::add_enclave(&who, &test_enclave));
	}
}

#[benchmarks(
    where <T as frame_system::Config>::Hash: From<[u8; 32]>
)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn force_add_enclave() {
		let who: T::AccountId = account("who", 1, 1);
		let test_enclave = Enclave::new(WorkerType::Identity);

		#[extrinsic_call]
		_(RawOrigin::Root, who.clone(), test_enclave.clone());

		assert_eq!(Teebag::<T>::enclave_count(WorkerType::Identity), 1);
		assert_eq!(EnclaveRegistry::<T>::get(who.clone()).unwrap(), test_enclave.clone());
		assert_last_event::<T>(
			Event::EnclaveAdded {
				who,
				worker_type: test_enclave.worker_type,
				url: test_enclave.url.clone(),
			}
			.into(),
		)
	}

	#[benchmark]
	fn force_remove_enclave() {
		let who: T::AccountId = account("who", 1, 1);
		let test_enclave = Enclave::new(WorkerType::Identity);
		Teebag::<T>::add_enclave(&who, &test_enclave);
		assert_eq!(Teebag::<T>::enclave_count(WorkerType::Identity), 1);

		#[extrinsic_call]
		_(RawOrigin::Root, who.clone());

		assert_eq!(Teebag::<T>::enclave_count(WorkerType::Identity), 0);
		assert_eq!(EnclaveRegistry::<T>::get(who.clone()), None);
		assert_last_event::<T>(Event::EnclaveRemoved { who }.into())
	}

	impl_benchmark_test_suite!(Teebag, super::mock::new_bench_ext(), super::mock::Test);
}
