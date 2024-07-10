#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use itp_extrinsics_factory::CreateExtrinsics;
use itp_ocall_api::EnclaveOnChainOCallApi;
use itp_types::parentchain::ParentchainId;
use lc_parachain_extrinsic_task_sender::init_parachain_extrinsic_sender_storage;
use log::*;
use std::{format, string::String, sync::Arc, time, vec};

const MAX_BATCH_SIZE: usize = 500;
const BATCH_EXTRINSIC_INTERVAL: time::Duration = time::Duration::from_secs(6);

pub fn run_parachain_extrinsic_task_receiver<ExtrinsicsFactory, OCallApi>(
	api: Arc<OCallApi>,
	extrinsic_factory: Arc<ExtrinsicsFactory>,
) -> Result<(), String>
where
	ExtrinsicsFactory: CreateExtrinsics + Send + Sync + 'static,
	OCallApi: EnclaveOnChainOCallApi,
{
	let task_receiver = init_parachain_extrinsic_sender_storage().map_err(|e| {
		format!("Failed to initialize parachain extrinsic task sender storage: {:?}", e)
	})?;
	let mut calls = vec::Vec::new();

	loop {
		let start_time = time::Instant::now();
		while start_time.elapsed() < BATCH_EXTRINSIC_INTERVAL {
			if let Ok(call) = task_receiver.recv() {
				calls.push(call);
			}
			if calls.len() == MAX_BATCH_SIZE {
				break
			}
		}
		if !calls.is_empty() {
			let extrinsic =
				match extrinsic_factory.create_batch_extrinsic(std::mem::take(&mut calls), None) {
					Ok(extrinsic) => extrinsic,
					Err(e) => {
						error!("Failed to create extrinsic: {:?}", e);
						continue
					},
				};
			if api
				.send_to_parentchain(vec![extrinsic], &ParentchainId::Litentry, false)
				.is_err()
			{
				error!("Failed to send extrinsic to parentchain");
			}
		}
	}
}
