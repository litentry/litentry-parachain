use core::time::Duration;

use lc_vc_task_sender::{VCRequest, VcRequestSender};

use super::*;
use crate::mock::*;

#[test]
pub fn test_signle_vc_requests_with_empty_id_graph() {
	execute_with_vc_task(|top_calls_receiver, rpc_calls_receiver| {
		let sender = VcRequestSender::default();
		let vc_request = create_vc_request(vec![Assertion::A1]);
		sender.send(VCRequest { request: vc_request }).unwrap();

		std::thread::sleep(Duration::from_secs(2));

		top_calls_receiver.try_iter().for_each(|a| {
			println!("Received Top Call");
		});

		rpc_calls_receiver.try_iter().for_each(|a| {
			decrypt_aes_request(a);
		});

		loop {}
	});
}

#[test]
pub fn test_duplicate_assertions() {
	init_vc_task();
	let top_calls_receiver = init_global_mock_author_api().unwrap();
	let rpc_calls_receiver = init_global_rpc_api().unwrap();
	println!("Finished initialising VC Task handler");
	let sender = VcRequestSender::default();
	let vc_request = create_vc_request(vec![Assertion::A1, Assertion::A6, Assertion::A1]);
	sender.send(VCRequest { request: vc_request }).unwrap();

	std::thread::sleep(Duration::from_secs(2));

	top_calls_receiver.try_iter().for_each(|a| {
		// println!("Received RPC response");
	});

	rpc_calls_receiver.try_iter().for_each(|a| {
		decrypt_aes_request(a);
	});
}

#[test]
pub fn test_no_eligible_identity() {
	init_vc_task();
	let top_calls_receiver = init_global_mock_author_api().unwrap();
	let rpc_calls_receiver = init_global_rpc_api().unwrap();
	println!("Finished initialising VC Task handler");
	let sender = VcRequestSender::default();
	let vc_request = create_vc_request(vec![Assertion::A6]);
	sender.send(VCRequest { request: vc_request }).unwrap();

	// Sleep to receive responses
	std::thread::sleep(Duration::from_secs(10));

	top_calls_receiver.try_iter().for_each(|a| {
		// println!("Received RPC response");
	});

	// We should receive one response only
	let response = rpc_calls_receiver.recv_timeout(Duration::from_secs(100)).unwrap();
	assert!(assert_is_err(String::from("No eligible identity"), response));

	println!("This test ends");
}

pub fn test_invalid_mrenclave() {}

pub fn test_invalid_shard() {}

pub fn test_failed_to_verify_signature() {}

pub fn test_failed_decrypting_payload() {}

pub fn test_request_batch_vc() {}

pub fn test_sending_wrong_trusted_call() {}
