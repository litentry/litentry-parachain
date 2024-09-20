/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG
	Copyright (C) 2017-2019 Baidu, Inc. All Rights Reserved.

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

use crate::{
	initialization::global_components::{
		GLOBAL_BITCOIN_KEY_REPOSITORY_COMPONENT, GLOBAL_ETHEREUM_KEY_REPOSITORY_COMPONENT,
		GLOBAL_SIGNING_KEY_REPOSITORY_COMPONENT,
	},
	rpc::worker_api_direct::public_api_rpc_handler,
	test::{
		fixtures::components::create_ocall_api,
		mocks::types::{TestOCallApi, TestSigner},
	},
	Hash,
};
use bc_signer_registry::{PubKey, SignerRegistryLookup};
use codec::{Decode, Encode};
use ita_stf::{Getter, PublicGetter};
use itc_direct_rpc_server::{
	create_determine_watch, rpc_connection_registry::ConnectionRegistry,
	rpc_ws_handler::RpcWsHandler,
};
use itc_parentchain_test::ParentchainHeaderBuilder;
use itc_tls_websocket_server::{ConnectionToken, WebSocketMessageHandler};
use itp_component_container::ComponentGetter;
use itp_rpc::{Id, RpcRequest, RpcReturnValue};
use itp_sgx_crypto::get_rsa3072_repository;
use itp_sgx_temp_dir::TempDir;
use itp_stf_executor::{getter_executor::GetterExecutor, mocks::GetStateMock};
use itp_stf_state_observer::mock::ObserveStateMock;
use itp_top_pool_author::mocks::AuthorApiMock;
use itp_types::{DirectRequestStatus, RsaRequest, ShardIdentifier};
use itp_utils::{FromHexPrefixed, ToHexPrefixed};
use litentry_primitives::{Address32, Identity};
use sp_core::Pair;
use std::{string::ToString, sync::Arc, vec::Vec};

struct SignerRegistryMock {}

impl SignerRegistryLookup for SignerRegistryMock {
	fn contains_key(&self, _account: &Address32) -> bool {
		true
	}
	fn get_all(&self) -> Vec<(Address32, PubKey)> {
		vec![]
	}
}

pub fn state_get_mrenclave_works() {
	type TestState = u64;

	let temp_dir = TempDir::with_prefix("get_state_request_works").unwrap();

	let connection_registry = Arc::new(ConnectionRegistry::<Hash, ConnectionToken>::new());
	let watch_extractor = Arc::new(create_determine_watch::<Hash>());
	let rsa_repository = get_rsa3072_repository(temp_dir.path().to_path_buf()).unwrap();

	let mr_enclave = [1; 32];

	let ocall_api = TestOCallApi::default().with_mr_enclave(mr_enclave.clone());

	let state: TestState = 78234u64;
	let state_observer = Arc::new(ObserveStateMock::<TestState>::new(state));
	let getter_executor =
		Arc::new(GetterExecutor::<_, GetStateMock<TestState>, Getter>::new(state_observer));
	let top_pool_author = Arc::new(AuthorApiMock::default());
	let signer_lookup = Arc::new(SignerRegistryMock {});

	let io_handler = public_api_rpc_handler(
		top_pool_author,
		getter_executor,
		Arc::new(rsa_repository),
		ocall_api.into(),
		GLOBAL_SIGNING_KEY_REPOSITORY_COMPONENT.get().unwrap(),
		GLOBAL_BITCOIN_KEY_REPOSITORY_COMPONENT.get().unwrap(),
		GLOBAL_ETHEREUM_KEY_REPOSITORY_COMPONENT.get().unwrap(),
		signer_lookup,
	);
	let rpc_handler = Arc::new(RpcWsHandler::new(io_handler, watch_extractor, connection_registry));

	let request_string = RpcRequest::compose_jsonrpc_call(
		Id::Text("1".to_string()),
		"state_getMrenclave".to_string(),
		vec![],
	)
	.unwrap();

	let response_string =
		rpc_handler.handle_message(ConnectionToken(1), request_string).unwrap().unwrap();

	assert!(!response_string.is_empty());

	const EXPECTED_HEX_RETURN_VALUE: &str =
		"0x8001010101010101010101010101010101010101010101010101010101010101010000";
	assert!(response_string.contains(EXPECTED_HEX_RETURN_VALUE));
	let rpc_return_value = RpcReturnValue::from_hex(EXPECTED_HEX_RETURN_VALUE).unwrap();
	assert_eq!(rpc_return_value.status, DirectRequestStatus::Ok);
	let decoded_value: [u8; 32] = Decode::decode(&mut rpc_return_value.value.as_slice()).unwrap();
	assert_eq!(decoded_value, mr_enclave);
}

pub fn get_state_request_works() {
	type TestState = u64;

	let temp_dir = TempDir::with_prefix("get_state_request_works").unwrap();

	let connection_registry = Arc::new(ConnectionRegistry::<Hash, ConnectionToken>::new());
	let watch_extractor = Arc::new(create_determine_watch::<Hash>());
	let rsa_repository = get_rsa3072_repository(temp_dir.path().to_path_buf()).unwrap();

	let signer = TestSigner::from_seed(b"42315678901234567890123456789012");
	let header = ParentchainHeaderBuilder::default().build();

	let ocall_api = create_ocall_api(&header, &signer);

	let state: TestState = 78234u64;
	let state_observer = Arc::new(ObserveStateMock::<TestState>::new(state));
	let getter_executor =
		Arc::new(GetterExecutor::<_, GetStateMock<TestState>, Getter>::new(state_observer));
	let top_pool_author = Arc::new(AuthorApiMock::default());
	let signer_lookup = Arc::new(SignerRegistryMock {});

	let io_handler = public_api_rpc_handler(
		top_pool_author,
		getter_executor,
		Arc::new(rsa_repository),
		ocall_api,
		GLOBAL_SIGNING_KEY_REPOSITORY_COMPONENT.get().unwrap(),
		GLOBAL_BITCOIN_KEY_REPOSITORY_COMPONENT.get().unwrap(),
		GLOBAL_ETHEREUM_KEY_REPOSITORY_COMPONENT.get().unwrap(),
		signer_lookup,
	);
	let rpc_handler = Arc::new(RpcWsHandler::new(io_handler, watch_extractor, connection_registry));

	let getter =
		Getter::public(PublicGetter::nonce(Identity::Substrate(Address32::from([0u8; 32]))));

	let request = RsaRequest::new(ShardIdentifier::default(), getter.encode());

	let request_string = RpcRequest::compose_jsonrpc_call(
		Id::Text("1".to_string()),
		"state_executeGetter".to_string(),
		vec![request.to_hex()],
	)
	.unwrap();

	let response_string =
		rpc_handler.handle_message(ConnectionToken(1), request_string).unwrap().unwrap();

	assert!(!response_string.is_empty());

	const EXPECTED_HEX_RETURN_VALUE: &str = "0x2801209a310100000000000000";
	assert!(response_string.contains(EXPECTED_HEX_RETURN_VALUE));
	let rpc_return_value = RpcReturnValue::from_hex(EXPECTED_HEX_RETURN_VALUE).unwrap();
	assert_eq!(rpc_return_value.status, DirectRequestStatus::Ok);
	let decoded_value: Option<Vec<u8>> =
		Option::decode(&mut rpc_return_value.value.as_slice()).unwrap();
	assert_eq!(decoded_value, Some(state.encode()));
}
