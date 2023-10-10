use super::*;
use mock::*;

use codec::Decode;
use ita_stf::TrustedCall;
use itp_stf_executor::mocks::StfEnclaveSignerMock;
use itp_test::mock::{
	handle_state_mock::HandleStateMock, onchain_mock::OnchainMock,
	shielding_crypto_mock::ShieldingCryptoMock,
};
use lc_stf_task_sender::stf_task_sender::{SendStfRequest, StfRequestSender};
use litentry_primitives::Assertion;

#[test]
fn test_threadpool_behaviour() {
	let shielding_key = ShieldingCryptoMock::default();
	let author_mock: AuthorApiMock<H256, H256> = AuthorApiMock::default();
	let stf_enclave_signer_mock = StfEnclaveSignerMock::default();
	let handle_state_mock = HandleStateMock::default();
	let onchain_mock = OnchainMock::default();
	let context = StfTaskContext::new(
		shielding_key.clone(),
		author_mock.into(),
		stf_enclave_signer_mock.into(),
		handle_state_mock.into(),
		onchain_mock.into(),
	);
	let _handle = std::thread::spawn(move || {
		run_stf_task_receiver(Arc::new(context)).unwrap();
	});

	let sender = StfRequestSender::default();
	let receiver = init_global_mock_author_api().unwrap();

	sender.send_stf_request(construct_assertion_request(Assertion::A1)).unwrap();
	sender.send_stf_request(construct_assertion_request(Assertion::A6)).unwrap();

	// As you see in the expected output, We receive A6 first even though A1 is requested first and is put to sleep
	let mut expected_output: Vec<Assertion> = vec![Assertion::A6, Assertion::A1];

	while let Ok(ext) = receiver.recv() {
		let decrypted = shielding_key.decrypt(&ext).unwrap();
		let decoded: TrustedOperation = Decode::decode(&mut decrypted.as_ref()).unwrap();
		match decoded {
			TrustedOperation::direct_call(trusted_call_signed) =>
				if let TrustedCall::request_vc_callback(_, _, assertion, ..) =
					trusted_call_signed.call
				{
					println!("Received Request VC Callback for: {:?}", assertion);
					assert_eq!(expected_output.remove(0), assertion);
				},
			_ => {
				// Do nothing
			},
		}
		if expected_output.len() == 0 {
			break
		}
	}
}
