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

use crate::{StfError, StfResult, ENCLAVE_ACCOUNT_KEY};

use ring::{
	aead::{Aad, BoundKey, Nonce, NonceSequence, SealingKey, UnboundKey, AES_256_GCM},
	error::Unspecified,
};

use codec::{Decode, Encode};
use itp_storage::{storage_double_map_key, storage_map_key, storage_value_key, StorageHasher};
use itp_utils::stringify::account_id_to_string;
use litentry_primitives::{
	AesOutput, ChallengeCode, UserShieldingKeyType, USER_SHIELDING_KEY_NONCE_LEN,
};
use log::*;
use std::prelude::v1::*;

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate rand_sgx as rand;

use rand::Rng;

pub fn get_storage_value<V: Decode>(
	storage_prefix: &'static str,
	storage_key_name: &'static str,
) -> Option<V> {
	let key = storage_value_key(storage_prefix, storage_key_name);
	get_storage_by_key_hash(key)
}

pub fn get_storage_map<K: Encode, V: Decode + Clone>(
	storage_prefix: &'static str,
	storage_key_name: &'static str,
	map_key: &K,
	hasher: &StorageHasher,
) -> Option<V> {
	let key = storage_map_key::<K>(storage_prefix, storage_key_name, map_key, hasher);
	get_storage_by_key_hash(key)
}

pub fn get_storage_double_map<K: Encode, Q: Encode, V: Decode + Clone>(
	storage_prefix: &'static str,
	storage_key_name: &'static str,
	first: &K,
	first_hasher: &StorageHasher,
	second: &Q,
	second_hasher: &StorageHasher,
) -> Option<V> {
	let key = storage_double_map_key::<K, Q>(
		storage_prefix,
		storage_key_name,
		first,
		first_hasher,
		second,
		second_hasher,
	);
	get_storage_by_key_hash(key)
}

/// Get value in storage.
pub fn get_storage_by_key_hash<V: Decode>(key: Vec<u8>) -> Option<V> {
	if let Some(value_encoded) = sp_io::storage::get(&key) {
		if let Ok(value) = Decode::decode(&mut value_encoded.as_slice()) {
			Some(value)
		} else {
			error!("could not decode state for key {:x?}", key);
			None
		}
	} else {
		info!("key not found in state {:x?}", key);
		None
	}
}

/// Get the AccountInfo key where the account is stored.
pub fn account_key_hash<AccountId: Encode>(account: &AccountId) -> Vec<u8> {
	storage_map_key("System", "Account", account, &StorageHasher::Blake2_128Concat)
}

pub fn enclave_signer_account<AccountId: Decode>() -> AccountId {
	get_storage_value("Sudo", ENCLAVE_ACCOUNT_KEY).expect("No enclave account")
}

/// Ensures an account is a registered enclave account.
pub fn ensure_enclave_signer_account<AccountId: Encode + Decode + PartialEq>(
	account: &AccountId,
) -> StfResult<()> {
	let expected_enclave_account: AccountId = enclave_signer_account();
	if &expected_enclave_account == account {
		Ok(())
	} else {
		error!(
			"Expected enclave account {}, but found {}",
			account_id_to_string(&expected_enclave_account),
			account_id_to_string(account)
		);
		Err(StfError::RequireEnclaveSignerAccount)
	}
}

pub fn set_block_number(block_number: u32) {
	sp_io::storage::set(&storage_value_key("System", "Number"), &block_number.encode());
}

// Litentry
pub fn aes_encrypt_default(key: &UserShieldingKeyType, data: &[u8]) -> AesOutput {
	let mut in_out = data.to_vec();

	let nonce = RingAeadNonceSequence::new();
	let aad = b"";
	let unbound_key = UnboundKey::new(&AES_256_GCM, key.as_slice()).unwrap();
	let mut sealing_key = SealingKey::new(unbound_key, nonce.clone());
	sealing_key.seal_in_place_append_tag(Aad::from(aad), &mut in_out).unwrap();

	AesOutput { ciphertext: in_out.to_vec(), aad: aad.to_vec(), nonce: nonce.nonce }
}

#[cfg(feature = "mockserver")]
pub fn generate_challenge_code() -> ChallengeCode {
	// Hard Code ChallengeCode for mockserver test
	// rand::thread_rng().gen::<ChallengeCode>()
	// hex: 0x08685a3823d512fad5d277f102ae1808
	let code: ChallengeCode = [8, 104, 90, 56, 35, 213, 18, 250, 213, 210, 119, 241, 2, 174, 24, 8];
	code
}

#[cfg(not(feature = "mockserver"))]
pub fn generate_challenge_code() -> ChallengeCode {
	rand::thread_rng().gen::<ChallengeCode>()
}

#[derive(Clone)]
pub struct RingAeadNonceSequence {
	pub nonce: [u8; USER_SHIELDING_KEY_NONCE_LEN],
}

impl RingAeadNonceSequence {
	fn new() -> RingAeadNonceSequence {
		RingAeadNonceSequence { nonce: [0u8; USER_SHIELDING_KEY_NONCE_LEN] }
	}
}

impl NonceSequence for RingAeadNonceSequence {
	fn advance(&mut self) -> Result<Nonce, Unspecified> {
		let nonce = Nonce::assume_unique_for_key(self.nonce);

		// FIXME: in function `ring::rand::sysrand::fill': undefined reference to `syscall'
		// let mut nonce_vec = vec![0; USER_SHIELDING_KEY_NONCE_LEN];
		// let rand = SystemRandom::new();
		// rand.fill(&mut nonce_vec).unwrap();
		let nonce_vec = rand::thread_rng().gen::<[u8; USER_SHIELDING_KEY_NONCE_LEN]>();

		self.nonce.copy_from_slice(&nonce_vec[0..USER_SHIELDING_KEY_NONCE_LEN]);

		Ok(nonce)
	}
}
