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

use tungstenite::Message;

use crate::{
	encrypt::{Encryptor, RsaPrivateKey},
	test::{
		fixtures::test_server_config_provider::TestServerConfigProvider,
		mocks::web_socket_handler_mock::WebSocketHandlerMock,
	},
	TungsteniteWsServer,
};
use std::{string::String, sync::Arc};

pub type TestServer = TungsteniteWsServer<WebSocketHandlerMock, TestServerConfigProvider>;

pub fn create_server(
	handler_responses: Vec<String>,
	port: u16,
) -> (
	Arc<TestServer>,
	Arc<WebSocketHandlerMock>,
	impl Fn(Message) -> Message + Clone + Send + Sync,
	Vec<u8>,
) {
	let config_provider = Arc::new(TestServerConfigProvider {});
	let handler = Arc::new(WebSocketHandlerMock::from_response_sequence(handler_responses));

	let server_addr_string = format!("127.0.0.1:{}", port);

	static PRIV_KEY: once_cell::sync::Lazy<Arc<RsaPrivateKey>> =
		once_cell::sync::Lazy::new(|| Arc::new(RsaPrivateKey::new().unwrap()));
	let pubkey = PRIV_KEY.to_public_key();
	let (decryptor, key) = Encryptor::export(&pubkey).unwrap();
	let decryptor = Arc::new(decryptor);

	let server = Arc::new(TungsteniteWsServer::new(
		server_addr_string,
		config_provider,
		handler.clone(),
		PRIV_KEY.clone(),
	));
	(
		server,
		handler,
		move |msg| match msg {
			Message::Binary(msg) =>
				Message::Text(String::from_utf8(decryptor.decrypt(&msg).unwrap()).unwrap()),
			_ => msg,
		},
		key,
	)
}
