use core::time::Duration;

use lc_vc_task_sender::{VCRequest, VcRequestSender};
use litentry_primitives::AesRequest;

use super::*;
use crate::mock::*;

#[test]
pub fn test_signle_vc_requests_with_empty_id_graph() {
	execute_with_vc_task(|top_calls_receiver, rpc_calls_receiver| {
		let sender = VcRequestSender::default();
		let vc_request = create_vc_request(vec![Assertion::A1], [0_u8; 32], H256::zero());
		sender.send(VCRequest { request: vc_request }).unwrap();

		// This should have valid VC Response
		let response = rpc_calls_receiver.recv_timeout(Duration::from_secs(100)).unwrap();
	});
}

#[test]
pub fn test_duplicate_assertions() {
	execute_with_vc_task(|top_calls_receiver, rpc_calls_receiver| {
		let sender = VcRequestSender::default();
		let vc_request =
			create_vc_request(vec![Assertion::A1, Assertion::A1], [0_u8; 32], H256::zero());
		sender.send(VCRequest { request: vc_request }).unwrap();

		let response = rpc_calls_receiver.recv_timeout(Duration::from_secs(100)).unwrap();
		assert!(assert_is_err(String::from("Duplicate assertion request"), response));

		// This should have the valid VC response
		// TODO: Assert Valid VC response (Only being able to decode the string)
		let response = rpc_calls_receiver.recv_timeout(Duration::from_secs(100)).unwrap();
	});
}

#[test]
pub fn test_no_eligible_identity() {
	execute_with_vc_task(|top_calls_receiver, rpc_calls_receiver| {
		let sender = VcRequestSender::default();
		let vc_request = create_vc_request(vec![Assertion::A6], [0_u8; 32], H256::zero());
		sender.send(VCRequest { request: vc_request }).unwrap();

		let response = rpc_calls_receiver.recv_timeout(Duration::from_secs(100)).unwrap();
		assert!(assert_is_err(String::from("No eligible identity"), response));
	});
}

#[test]
pub fn test_invalid_mrenclave() {
	execute_with_vc_task(|top_calls_receiver, rpc_calls_receiver| {
		let sender = VcRequestSender::default();
		let vc_request = create_vc_request(vec![Assertion::A1], [1_u8; 32], H256::zero());
		sender.send(VCRequest { request: vc_request }).unwrap();

		let response = rpc_calls_receiver.recv_timeout(Duration::from_secs(100)).unwrap();
		assert!(assert_is_err(String::from("Failed to verify sig"), response));
	});
}

#[test]
pub fn test_invalid_shard() {
	execute_with_vc_task(|top_calls_receiver, rpc_calls_receiver| {
		let sender = VcRequestSender::default();
		let vc_request = create_vc_request(vec![Assertion::A1], [1_u8; 32], H256::random());
		sender.send(VCRequest { request: vc_request }).unwrap();

		let response = rpc_calls_receiver.recv_timeout(Duration::from_secs(100)).unwrap();
		assert!(assert_is_err(String::from("Failed to verify sig"), response));
	});
}

#[test]
pub fn test_failed_decoding_payload() {
	execute_with_vc_task(|top_calls_receiver, rpc_calls_receiver| {
		let sender = VcRequestSender::default();
		let vc_request: AesRequest = Default::default();
		sender.send(VCRequest { request: vc_request }).unwrap();

		let response = rpc_calls_receiver.recv_timeout(Duration::from_secs(100)).unwrap();
		assert!(assert_is_err(String::from("Failed to decode request payload"), response));
	});
}

#[test]
pub fn test_sending_wrong_trusted_call() {
	execute_with_vc_task(|top_calls_receiver, rpc_calls_receiver| {
		let sender = VcRequestSender::default();
		let request: AesRequest = create_noop_trusted_call();
		sender.send(VCRequest { request }).unwrap();

		let response = rpc_calls_receiver.recv_timeout(Duration::from_secs(100)).unwrap();
		assert!(assert_is_err(
			String::from("Wrong trusted call. Expect request_batch_vc "),
			response
		));
	});
}
