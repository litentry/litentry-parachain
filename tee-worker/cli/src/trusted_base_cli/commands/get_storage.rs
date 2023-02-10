use crate::{command_utils::get_worker_api_direct, trusted_cli::TrustedCli, Cli};
use codec::Decode;
use ita_sgx_runtime::Runtime;
use itc_rpc_client::direct_client::{DirectApi, DirectClient};
use itp_rpc::{RpcRequest, RpcResponse, RpcReturnValue};
use itp_types::DirectRequestStatus;
use itp_utils::FromHexPrefixed;
use log::{debug, error, warn};
use my_node_runtime::Hash;
use sp_application_crypto::scale_info::form::PortableForm;
use std::sync::mpsc::channel;
use subxt::{
	self,
	dynamic::{DecodedValue, DecodedValueThunk, Value},
	ext::frame_metadata::StorageEntryType,
	metadata::DecodeWithMetadata,
	storage::{dynamic, utils, StorageAddress},
	Metadata,
};

/// Usage:
///    Plain Storage: ./integritee-cli trusted --mrenclave $mrenclave get-storage Parentchain Number
///    Map Storage: ./integritee-cli trusted --mrenclave $mrenclave get-storage System Account 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
#[derive(Parser)]
pub struct GetStorageCommand {
	/// Pallet Name
	pallet_name: String,

	/// Storage Name
	storage_name: String,

	/// Stroage Key
	keys: Vec<String>,
}
impl GetStorageCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_args: &TrustedCli) {
		let direct_api = get_worker_api_direct(cli);
		let mrenclave = trusted_args.mrenclave.clone();
		let storage_entry_keys: Vec<Value> = self
			.keys
			.iter()
			.map(|v| {
				let v = if v.starts_with("0x") {
					v.strip_prefix("0x").unwrap().as_bytes()
				} else {
					v.as_bytes()
				};
				Value::from_bytes(hex::decode(v).unwrap())
			})
			.collect();
		if let Some(v) = get_storage_value(
			direct_api,
			mrenclave,
			self.pallet_name.as_str(),
			self.storage_name.as_str(),
			storage_entry_keys,
		) {
			println!("{}", v);
		} else {
			println!("None");
		}
	}
}

fn get_storage_value(
	direct_api: DirectClient,
	mrenclave: String,
	pallet_name: &str,
	storage_name: &str,
	storage_entry_keys: Vec<Value>,
) -> Option<DecodedValue> {
	let addr = dynamic(pallet_name, storage_name, storage_entry_keys);
	let metadata = Metadata::try_from(Runtime::metadata()).unwrap();
	let mut entry_bytes: Vec<u8> = vec![];

	utils::write_storage_address_root_bytes(&addr, &mut entry_bytes);
	debug!("storage_root: 0x{}", hex::encode(entry_bytes.clone()));

	addr.append_entry_bytes(&metadata, &mut entry_bytes).unwrap();
	debug!("storage_entry_key: 0x{}", hex::encode(&entry_bytes));

	if let Some(value) = send_request(direct_api, mrenclave, &entry_bytes) {
		let pallet_metadata = metadata.pallet(pallet_name).unwrap();
		let storage_metadata = pallet_metadata.storage(storage_name).unwrap();
		let return_ty_id = return_type_from_storage_entry_type(&storage_metadata.ty);

		let mut bytes = if value.is_empty() { &storage_metadata.default[..] } else { &value[..] };
		let val =
			DecodedValueThunk::decode_with_metadata(&mut bytes, return_ty_id, &metadata).unwrap();

		Some(val.to_value().unwrap())
	} else {
		None
	}
}

fn send_request(
	direct_api: DirectClient,
	mrenclave: String,
	storage_entry_key: &Vec<u8>,
) -> Option<Vec<u8>> {
	let (sender, receiver) = channel();

	let jsonrpc_call: String = RpcRequest::compose_jsonrpc_call(
		"state_getStorage".to_string(),
		vec![mrenclave, format!("0x{}", hex::encode(storage_entry_key))],
	)
	.unwrap();

	direct_api.watch(jsonrpc_call, sender);

	debug!("waiting for rpc response");

	let value = match receiver.recv() {
		Ok(response) => {
			let response: RpcResponse = serde_json::from_str(&response).unwrap();
			if let Ok(return_value) = RpcReturnValue::from_hex(&response.result) {
				match return_value.status {
					DirectRequestStatus::Ok => Some(return_value.value),
					DirectRequestStatus::Error => {
						warn!("request status is error");
						if let Ok(value) = String::decode(&mut return_value.value.as_slice()) {
							warn!("[Error] {}", value);
						}
						None
					},
					DirectRequestStatus::TrustedOperationStatus(status) => {
						warn!("request status is: {:?}", status);
						if let Ok(value) = Hash::decode(&mut return_value.value.as_slice()) {
							warn!("Trusted call {:?} is {:?}", value, status);
						}
						None
					},
				}
			} else {
				None
			}
		},
		Err(e) => {
			error!("failed to receive rpc response: {:?}", e);
			None
		},
	};
	direct_api.close().unwrap();
	value
}

/// Fetch the return type out of a [`StorageEntryType`].
fn return_type_from_storage_entry_type(entry: &StorageEntryType<PortableForm>) -> u32 {
	match entry {
		StorageEntryType::Plain(ty) => ty.id(),
		StorageEntryType::Map { value, .. } => value.id(),
	}
}
