use super::*;
use mock::*;

use codec::Decode;
use ita_stf::{TrustedCall, TrustedCallSigned};
use itp_stf_executor::mocks::StfEnclaveSignerMock;
use itp_test::mock::{
	handle_state_mock::HandleStateMock, onchain_mock::OnchainMock,
	shielding_crypto_mock::ShieldingCryptoMock,
};
use itp_top_pool_author::mocks::AuthorApiMock;
use lc_stf_task_sender::stf_task_sender::{SendStfRequest, StfRequestSender};
use litentry_primitives::Assertion;
use std::sync::mpsc;

#[test]
fn test_threadpool_behaviour() {
	let shielding_key = ShieldingCryptoMock::default();
	let author_mock: AuthorApiMock<H256, H256> = AuthorApiMock::default();
	let stf_enclave_signer_mock = StfEnclaveSignerMock::default();
	let handle_state_mock = HandleStateMock::default();
	let onchain_mock = OnchainMock::default();
	let (sender, recv) = mpsc::sync_channel(1000);
	let context = StfTaskContext::new(
		shielding_key.clone(),
		author_mock.into(),
		stf_enclave_signer_mock.into(),
		handle_state_mock.into(),
		onchain_mock.into(),
		Arc::new(sender),
	);
	let _handle = std::thread::spawn(move || {
		run_stf_task_receiver(Arc::new(context)).unwrap();
	});

	let sender = StfRequestSender::default();

	// Sleep in order to initialize the components
	std::thread::sleep(core::time::Duration::from_secs(2));

	sender.send_stf_request(construct_assertion_request(Assertion::A1)).unwrap();
	sender.send_stf_request(construct_assertion_request(Assertion::A6)).unwrap();

	let receiver = init_global_mock_author_api().unwrap();
	// As you see in the expected output, We receive A6 first even though A1 is requested first and is put to sleep
	let mut expected_output: Vec<Assertion> = vec![Assertion::A6, Assertion::A1];

	let timeout_duration = core::time::Duration::from_secs(30);
	let start_time = std::time::Instant::now();

	while let Ok(ext) = receiver.recv() {
		let decrypted = shielding_key.decrypt(&ext).unwrap();
		let decoded: TrustedOperation = Decode::decode(&mut decrypted.as_ref()).unwrap();
		if let TrustedOperation::direct_call(TrustedCallSigned {
			call: TrustedCall::request_vc_callback(_, _, assertion, ..),
			..
		}) = decoded
		{
			assert_eq!(expected_output.remove(0), assertion);
		}
		if expected_output.len() == 0 {
			break
		}

		// Timeout condition
		if start_time.elapsed() > timeout_duration {
			assert!(false, "Test exceeded the 60-second timeout");
		}
	}
}
