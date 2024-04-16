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

		// This should have valid VC Response
		let response = rpc_calls_receiver.recv_timeout(Duration::from_secs(100)).unwrap();
	});
}

#[test]
pub fn test_duplicate_assertions() {
	execute_with_vc_task(|top_calls_receiver, rpc_calls_receiver| {
		let sender = VcRequestSender::default();
		let vc_request = create_vc_request(vec![Assertion::A1, Assertion::A1]);
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
		let vc_request = create_vc_request(vec![Assertion::A6]);
		sender.send(VCRequest { request: vc_request }).unwrap();

		let response = rpc_calls_receiver.recv_timeout(Duration::from_secs(100)).unwrap();
		assert!(assert_is_err(String::from("No eligible identity"), response));
	});
}

pub fn test_invalid_mrenclave() {}

pub fn test_invalid_shard() {}

pub fn test_failed_to_verify_signature() {}

pub fn test_failed_decrypting_payload() {}

pub fn test_request_batch_vc() {}

pub fn test_sending_wrong_trusted_call() {}
