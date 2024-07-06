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

use crate::error::{Error, ServiceResult};
use itp_settings::files::{
	ENCLAVE_REGISTRY_FILE, LITENTRY_PARENTCHAIN_LIGHT_CLIENT_DB_PATH, RELAYER_REGISTRY_FILE,
	SHARDS_PATH, SIGNER_REGISTRY_FILE, TARGET_A_PARENTCHAIN_LIGHT_CLIENT_DB_PATH,
	TARGET_B_PARENTCHAIN_LIGHT_CLIENT_DB_PATH,
};
use std::{fs, path::Path};

#[cfg(feature = "link-binary")]
pub(crate) use needs_enclave::{
	generate_shielding_key_file, generate_signing_key_file, init_shard, initialize_shard_and_keys,
	migrate_shard,
};

#[cfg(feature = "link-binary")]
mod needs_enclave {
	use crate::error::{Error, ServiceResult};
	use codec::Encode;
	use itp_enclave_api::{enclave_base::EnclaveBase, Enclave};
	use itp_settings::files::{
		LITENTRY_PARENTCHAIN_LIGHT_CLIENT_DB_PATH, SHARDS_PATH, SHIELDING_KEY_FILE,
		SIGNING_KEY_FILE, TARGET_A_PARENTCHAIN_LIGHT_CLIENT_DB_PATH,
		TARGET_B_PARENTCHAIN_LIGHT_CLIENT_DB_PATH,
	};
	use itp_types::ShardIdentifier;
	use log::*;
	use std::{fs, fs::File, path::Path};

	/// Initializes the shard and generates the key files.
	pub(crate) fn initialize_shard_and_keys(
		enclave: &Enclave,
		shard_identifier: &ShardIdentifier,
	) -> ServiceResult<()> {
		println!("[+] Initialize the shard");
		init_shard(enclave, shard_identifier);

		let pubkey = enclave.get_ecc_signing_pubkey().unwrap();
		debug!("Enclave signing key (public) raw: {:?}", pubkey);
		let pubkey = enclave.get_rsa_shielding_pubkey().unwrap();
		debug!("Enclave shielding key (public) raw (may be overwritten later): {:?}", pubkey);
		Ok(())
	}

	pub(crate) fn init_shard(enclave: &Enclave, shard_identifier: &ShardIdentifier) {
		use base58::ToBase58;

		match enclave.init_shard(shard_identifier.encode()) {
			Err(e) => {
				println!(
					"Failed to initialize shard {:?}: {:?}",
					shard_identifier.0.to_base58(),
					e
				);
			},
			Ok(_) => {
				println!("Successfully initialized shard {:?}", shard_identifier.0.to_base58());
			},
		}
	}

	pub(crate) fn migrate_shard(enclave: &Enclave, &new_shard: &ShardIdentifier) {
		match enclave.migrate_shard(new_shard.encode()) {
			Err(e) => {
				panic!("Failed to migrate shard {:?}. {:?}", new_shard, e);
			},
			Ok(_) => {
				println!("Shard {:?} migrated Successfully", new_shard);
			},
		}
	}

	pub(crate) fn generate_signing_key_file(enclave: &Enclave) {
		info!("*** Get the signing key from the TEE\n");
		let pubkey = enclave.get_ecc_signing_pubkey().unwrap();
		debug!("[+] Signing key raw: {:?}", pubkey);
		match fs::write(SIGNING_KEY_FILE, pubkey) {
			Err(x) => {
				error!("[-] Failed to write '{}'. {}", SIGNING_KEY_FILE, x);
			},
			_ => {
				println!("[+] File '{}' written successfully", SIGNING_KEY_FILE);
			},
		}
	}

	pub(crate) fn generate_shielding_key_file(enclave: &Enclave) {
		info!("*** Get the public key from the TEE\n");
		let pubkey = enclave.get_rsa_shielding_pubkey().unwrap();
		let file = File::create(SHIELDING_KEY_FILE).unwrap();
		match serde_json::to_writer(file, &pubkey) {
			Err(x) => {
				error!("[-] Failed to write '{}'. {}", SHIELDING_KEY_FILE, x);
			},
			_ => {
				println!("[+] File '{}' written successfully", SHIELDING_KEY_FILE);
			},
		}
	}
}

/// backs up shard directory and restores it after cleaning shards directory
pub(crate) fn remove_old_shards(root_dir: &Path, new_shard_name: &str) {
	let shard_backup = root_dir.join("shard_backup");
	let shard_dir = root_dir.join(SHARDS_PATH).join(new_shard_name);

	fs::rename(shard_dir.clone(), shard_backup.clone()).expect("Failed to backup shard");
	remove_dir_if_it_exists(root_dir, SHARDS_PATH).expect("Failed to remove shards directory");
	fs::create_dir_all(root_dir.join(SHARDS_PATH)).expect("Failed to create shards directory");
	fs::rename(shard_backup, shard_dir).expect("Failed to restore shard");
}

/// Purge all worker files from `dir`.
pub(crate) fn purge_files_from_dir(dir: &Path) -> ServiceResult<()> {
	println!("[+] Performing a clean reset of the worker");

	println!("[+] Purge all files from previous runs");
	purge_files(dir)?;

	Ok(())
}

/// Purge all worker files in a given path.
fn purge_files(root_directory: &Path) -> ServiceResult<()> {
	remove_dir_if_it_exists(root_directory, SHARDS_PATH)?;

	remove_dir_if_it_exists(root_directory, LITENTRY_PARENTCHAIN_LIGHT_CLIENT_DB_PATH)?;
	remove_dir_if_it_exists(root_directory, TARGET_A_PARENTCHAIN_LIGHT_CLIENT_DB_PATH)?;
	remove_dir_if_it_exists(root_directory, TARGET_B_PARENTCHAIN_LIGHT_CLIENT_DB_PATH)?;

	remove_file_if_it_exists(root_directory, RELAYER_REGISTRY_FILE)?;
	remove_file_if_it_exists(root_directory, ENCLAVE_REGISTRY_FILE)?;
	remove_file_if_it_exists(root_directory, SIGNER_REGISTRY_FILE)?;
	Ok(())
}

fn remove_dir_if_it_exists(root_directory: &Path, dir_name: &str) -> ServiceResult<()> {
	let directory_path = root_directory.join(dir_name);
	if directory_path.exists() {
		fs::remove_dir_all(directory_path).map_err(|e| Error::Custom(e.into()))?;
	}
	Ok(())
}

fn remove_file_if_it_exists(root_directory: &Path, file_name: &str) -> ServiceResult<()> {
	let file = root_directory.join(file_name);
	if file.exists() {
		fs::remove_file(file).map_err(|e| Error::Custom(e.into()))?;
	}
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;
	use itp_settings::files::{
		SHARDS_PATH, SIGNER_REGISTRY_FILE, TARGET_A_PARENTCHAIN_LIGHT_CLIENT_DB_PATH,
	};
	use std::{fs, path::PathBuf};

	#[test]
	fn purge_files_deletes_all_relevant_files() {
		let test_directory_handle =
			TestDirectoryHandle::new(PathBuf::from("test_purge_files_deletes_all_relevant_files"));
		let root_directory = test_directory_handle.path();

		let shards_path = root_directory.join(SHARDS_PATH);
		fs::create_dir_all(&shards_path).unwrap();
		fs::File::create(&shards_path.join("state_1.bin")).unwrap();
		fs::File::create(&shards_path.join("state_2.bin")).unwrap();

		fs::File::create(&root_directory.join(RELAYER_REGISTRY_FILE)).unwrap();
		fs::File::create(&root_directory.join(ENCLAVE_REGISTRY_FILE)).unwrap();
		fs::File::create(&root_directory.join(SIGNER_REGISTRY_FILE)).unwrap();

		fs::create_dir_all(&root_directory.join(LITENTRY_PARENTCHAIN_LIGHT_CLIENT_DB_PATH))
			.unwrap();
		fs::create_dir_all(&root_directory.join(TARGET_A_PARENTCHAIN_LIGHT_CLIENT_DB_PATH))
			.unwrap();
		fs::create_dir_all(&root_directory.join(TARGET_B_PARENTCHAIN_LIGHT_CLIENT_DB_PATH))
			.unwrap();

		purge_files(&root_directory).unwrap();

		assert!(!shards_path.exists());
		assert!(!root_directory.join(LITENTRY_PARENTCHAIN_LIGHT_CLIENT_DB_PATH).exists());
		assert!(!root_directory.join(TARGET_A_PARENTCHAIN_LIGHT_CLIENT_DB_PATH).exists());
		assert!(!root_directory.join(TARGET_B_PARENTCHAIN_LIGHT_CLIENT_DB_PATH).exists());
		assert!(!root_directory.join(RELAYER_REGISTRY_FILE).exists());
		assert!(!root_directory.join(ENCLAVE_REGISTRY_FILE).exists());
		assert!(!root_directory.join(SIGNER_REGISTRY_FILE).exists());
	}

	#[test]
	fn purge_files_succeeds_when_no_files_exist() {
		let test_directory_handle = TestDirectoryHandle::new(PathBuf::from(
			"test_purge_files_succeeds_when_no_files_exist",
		));
		let root_directory = test_directory_handle.path();

		assert!(purge_files(&root_directory).is_ok());
	}

	#[test]
	fn test_remove_old_shards() {
		let test_directory_handle = TestDirectoryHandle::new(PathBuf::from("test_backup_shard"));
		let root_directory = test_directory_handle.path();
		let shard_1_name = "test_shard_1";
		let shard_2_name = "test_shard_2";

		let shard_1_dir = root_directory.join(SHARDS_PATH).join(shard_1_name);
		fs::create_dir_all(&shard_1_dir).unwrap();
		fs::File::create(shard_1_dir.join("test_state.bin")).unwrap();
		fs::File::create(shard_1_dir.join("test_state_2.bin")).unwrap();

		let shard_2_dir = root_directory.join(SHARDS_PATH).join(shard_2_name);
		fs::create_dir_all(&shard_2_dir).unwrap();
		fs::File::create(shard_2_dir.join("test_state.bin")).unwrap();

		assert!(root_directory.join(SHARDS_PATH).join(shard_2_name).exists());

		remove_old_shards(root_directory, shard_1_name);

		assert!(root_directory.join(SHARDS_PATH).join(shard_1_name).exists());
		assert_eq!(
			fs::read_dir(root_directory.join(SHARDS_PATH).join(shard_1_name))
				.expect("Failed to read shard directory")
				.count(),
			2
		);
		assert!(!root_directory.join(SHARDS_PATH).join(shard_2_name).exists());
	}

	/// Directory handle to automatically initialize a directory
	/// and upon dropping the reference, removing it again.
	struct TestDirectoryHandle {
		path: PathBuf,
	}

	impl TestDirectoryHandle {
		pub fn new(path: PathBuf) -> Self {
			let test_path = std::env::current_dir().unwrap().join(&path);
			fs::create_dir_all(&test_path).unwrap();
			TestDirectoryHandle { path: test_path }
		}

		pub fn path(&self) -> &PathBuf {
			&self.path
		}
	}

	impl Drop for TestDirectoryHandle {
		fn drop(&mut self) {
			if self.path.exists() {
				fs::remove_dir_all(&self.path).unwrap();
			}
		}
	}
}
