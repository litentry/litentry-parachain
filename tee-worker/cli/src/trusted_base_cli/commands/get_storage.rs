use crate::{command_utils::get_worker_api_direct, trusted_cli::TrustedCli, Cli};
use codec::Decode;
use frame_metadata::{RuntimeMetadata, StorageEntryType, StorageHasher};
use ita_sgx_runtime::Runtime;
use itc_rpc_client::direct_client::{DirectApi, DirectClient};
use itp_rpc::{RpcRequest, RpcResponse, RpcReturnValue};
use itp_types::DirectRequestStatus;
use itp_utils::FromHexPrefixed;
use log::{debug, error, warn};
use my_node_runtime::Hash;
use scale_value::{scale::TypeId, Value};
use sp_application_crypto::scale_info::TypeDef;
use std::{format, sync::mpsc::channel};

/// Usage:
///    Plain Storage: ./integritee-cli trusted --mrenclave $mrenclave get-storage Parentchain Number
///    Map Storage: ./integritee-cli trusted --mrenclave $mrenclave get-storage System Account 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
///    Double Map Storage: ./integritee-cli trusted --mrenclave $mrenclave get-storage IdentityManagement ChallengeCodes 0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d 0x0200246d6f636b5f75736572
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
		if let Some(v) = get_storage_value(
			direct_api,
			mrenclave,
			self.pallet_name.as_str(),
			self.storage_name.as_str(),
			&self.keys,
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
	storage_entry_keys: &[String],
) -> Option<Value<TypeId>> {
	let metadata = Runtime::metadata();

	let metadata = match metadata.1 {
		RuntimeMetadata::V14(meta) => meta,
		_ => panic!("Invalid metadata"),
	};

	let pallet = metadata
		.pallets
		.iter()
		.find(|pallet| pallet.name == pallet_name)
		.unwrap()
		.clone();
	let storage = pallet.storage.unwrap();

	let storage_entry = storage
		.entries
		.iter()
		.find(|storage| storage.name == storage_name)
		.unwrap()
		.clone();

	let storage_entry_keys: Vec<Vec<u8>> = storage_entry_keys
		.iter()
		.map(|v| {
			if v.starts_with("0x") {
				hex::decode(v.strip_prefix("0x").unwrap().as_bytes()).unwrap()
			} else {
				hex::decode(v.as_bytes()).unwrap()
			}
		})
		.collect();

	let mut entry_bytes: Vec<u8> = vec![];

	write_storage_address_root_bytes(pallet_name, storage_name, &mut entry_bytes);

	let return_ty_id = match storage_entry.ty {
		StorageEntryType::Plain(ty) =>
			if !storage_entry_keys.is_empty() {
				panic!("Wrong Number Of Keys, expected: 0");
			} else {
				ty.id()
			},
		StorageEntryType::Map { hashers, key, value } => {
			let ty = metadata.types.resolve(key.id()).unwrap();
			// If the key is a tuple, we encode each value to the corresponding tuple type.
			// If the key is not a tuple, encode a single value to the key type.
			let type_ids = match ty.type_def() {
				TypeDef::Tuple(tuple) => tuple.fields().iter().map(|f| f.id()).collect(),
				_other => {
					vec![key.id()]
				},
			};
			if hashers.len() != storage_entry_keys.len() {
				panic!("Wrong Number Of Keys, expected: {}", hashers.len());
			}
			for ((key, _type_id), hasher) in storage_entry_keys.iter().zip(type_ids).zip(hashers) {
				hash_bytes(key.as_slice(), &hasher, &mut entry_bytes);
			}
			println!("value name:{:?}", metadata.types.resolve(value.id()).unwrap());
			value.id()
		},
	};

	if let Some(value) = send_get_storage_request(direct_api, mrenclave, &entry_bytes) {
		println!("type_id, {}, value: {:?}", return_ty_id, value.clone());
		let mut bytes = if value.is_empty() { &storage_entry.default[..] } else { &value[..] };
		Some(scale_value::scale::decode_as_type(&mut bytes, return_ty_id, &metadata.types).unwrap())
	} else {
		None
	}
}

fn send_get_storage_request(
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

fn write_storage_address_root_bytes(pallet_name: &str, storage_name: &str, out: &mut Vec<u8>) {
	out.extend(sp_core_hashing::twox_128(pallet_name.as_bytes()));
	out.extend(sp_core_hashing::twox_128(storage_name.as_bytes()));
}

/// Take some SCALE encoded bytes and a [`StorageHasher`] and hash the bytes accordingly.
fn hash_bytes(input: &[u8], hasher: &StorageHasher, bytes: &mut Vec<u8>) {
	match hasher {
		StorageHasher::Identity => bytes.extend(input),
		StorageHasher::Blake2_128 => bytes.extend(sp_core_hashing::blake2_128(input)),
		StorageHasher::Blake2_128Concat => {
			bytes.extend(sp_core_hashing::blake2_128(input));
			bytes.extend(input);
		},
		StorageHasher::Blake2_256 => bytes.extend(sp_core_hashing::blake2_256(input)),
		StorageHasher::Twox128 => bytes.extend(sp_core_hashing::twox_128(input)),
		StorageHasher::Twox256 => bytes.extend(sp_core_hashing::twox_256(input)),
		StorageHasher::Twox64Concat => {
			bytes.extend(sp_core_hashing::twox_64(input));
			bytes.extend(input);
		},
	}
}

#[test]
fn identity() {
	let a = hex::decode("0200246d6f636b5f75736572").unwrap();
	let a = litentry_primitives::Identity::decode(&mut a.as_slice()).unwrap();
	println!("{:?}", a);
	// let metadata = Metadata::try_from(Runtime::metadata()).unwrap();

	// let storage_entry_key = Value::from_bytes(a);
	// println!("{:?}", metadata.resolve_type(37));
	// println!("");
	// let mut input: Vec<u8> = Vec::new();
	// storage_entry_key.encode_with_metadata(37, &metadata, &mut input).unwrap();
	let direct_api = DirectClient::new("wss://localhost:2000".to_string());
	if let Some(v) = get_storage_value(
		direct_api.clone(),
		"6FNtLbzq4NE25gpQJ9vBab2YBNdNZahumJdWwuCV2oD".to_string(),
		"IdentityManagement",
		"ChallengeCodes",
		&vec![
			"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_string(),
			"0x0200246d6f636b5f75736572".to_string(),
		],
	){
			println!("{}", v);
	}

	// if let Some(v) = get_storage_value(
	// 	direct_api,
	// 	"6FNtLbzq4NE25gpQJ9vBab2YBNdNZahumJdWwuCV2oD".to_string(),
	// 	"System",
	// 	"Account",
	// 	&vec!["0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d".to_string()]
	// ){
	// 		println!("{}", v);
	// }

	// // let mrenclave = trusted_args.mrenclave.clone();
	// let storage_entry_keys: Vec<Value> = vec![
	// 	"0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d",
	// 	"0x0200246d6f636b5f75736572",
	// ]
	// .iter()
	// .map(|v| {
	// 	let v = if v.starts_with("0x") {
	// 		v.strip_prefix("0x").unwrap().as_bytes()
	// 	} else {
	// 		v.as_bytes()
	// 	};
	// 	Value::from_bytes(hex::decode(v).unwrap())
	// })
	// .collect();

	// if let Some(v) = get_storage_value(
	// 	direct_api,
	// 	"6FNtLbzq4NE25gpQJ9vBab2YBNdNZahumJdWwuCV2oD".to_string(),
	// 	"IdentityManagement",
	// 	"ChallengeCodes",
	// 	storage_entry_keys,
	// ) {
	// 	println!("{}", v);
	// } else {
	// 	println!("None");
	// }
}
