use crate::{command_utils::get_worker_api_direct, trusted_cli::TrustedCli, Cli};
use codec::Decode;
use itc_rpc_client::direct_client::DirectApi;
use itp_rpc::{RpcRequest, RpcResponse, RpcReturnValue};
use itp_types::DirectRequestStatus;
use itp_utils::FromHexPrefixed;
use log::{debug, error, warn};
use my_node_runtime::Hash;

use std::sync::mpsc::channel;

#[derive(Parser)]
pub struct QueryStorageCommand {
	/// Storage Key
	module: String,

	storage: String,
}
impl QueryStorageCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_args: &TrustedCli) {
		let direct_api = get_worker_api_direct(cli);

		// let shard = read_shard(trusted_args).unwrap();

		// let key = storage_value_key("Parentchain", "Number");
		let key = storage_value_key(self.module.as_str(), self.storage.as_str());
		let key = format!("0x{}", hex::encode(key));

		let jsonrpc_call: String = RpcRequest::compose_jsonrpc_call(
			"state_getStorage".to_string(),
			vec![trusted_args.mrenclave.clone(), key],
		)
		.unwrap();

		debug!("setup sender and receiver");
		let (sender, receiver) = channel();
		direct_api.watch(jsonrpc_call, sender);

		debug!("waiting for rpc response");
		match receiver.recv() {
			Ok(response) => {
				let response: RpcResponse = serde_json::from_str(&response).unwrap();
				if let Ok(return_value) = RpcReturnValue::from_hex(&response.result) {
					warn!("return_value:{:?}", &return_value);
					match return_value.status {
						DirectRequestStatus::Ok => {
							println!("{}", hex::encode(return_value.value));
						},
						DirectRequestStatus::Error => {
							warn!("request status is error");
							if let Ok(value) = String::decode(&mut return_value.value.as_slice()) {
								warn!("[Error] {}", value);
							}
						},
						DirectRequestStatus::TrustedOperationStatus(status) => {
							warn!("request status is: {:?}", status);
							if let Ok(value) = Hash::decode(&mut return_value.value.as_slice()) {
								warn!("Trusted call {:?} is {:?}", value, status);
							}
						},
					}
				};
			},
			Err(e) => {
				error!("failed to receive rpc response: {:?}", e);
			},
		};
		direct_api.close().unwrap();
	}
}

pub fn storage_value_key(module_prefix: &str, storage_prefix: &str) -> Vec<u8> {
	let mut bytes = sp_core::twox_128(module_prefix.as_bytes()).to_vec();
	bytes.extend(&sp_core::twox_128(storage_prefix.as_bytes())[..]);
	bytes
}
