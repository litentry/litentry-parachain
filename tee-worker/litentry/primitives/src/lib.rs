// Copyright 2020-2023 Litentry Technologies GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

mod assertion;
mod enclave_quote;
mod ethereum_signature;
mod identity;
mod validation_data;

pub use ethereum_signature::*;
pub use identity::*;
pub use parentchain_primitives::{
	AccountId, AesOutput, Balance as ParentchainBalance, BlockNumber as ParentchainBlockNumber,
	Hash as ParentchainHash, Header as ParentchainHeader, Signature as ParentchainSignature,
	UserShieldingKeyType, MINUTES, USER_SHIELDING_KEY_LEN, USER_SHIELDING_KEY_NONCE_LEN,
	USER_SHIELDING_KEY_TAG_LEN,
};

use ring::{
	aead::{Aad, BoundKey, Nonce, NonceSequence, SealingKey, UnboundKey, AES_256_GCM},
	error::Unspecified,
};

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate rand_sgx as rand;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use chrono_sgx as chrono;
}

use rand::Rng;

pub use assertion::*;
pub use enclave_quote::*;
pub use validation_data::*;

pub const CHALLENGE_CODE_SIZE: usize = 16;
pub type ChallengeCode = [u8; CHALLENGE_CODE_SIZE];

// Returns the default if any error happens
// We don't propagate the error to upper level as this function is used in too many places,
// it's too verbose to handle them all and pass back to the parentchain as events.
// We rely on the parentchain event consumers to handle them correctly (and they kind of
// have to, because they'll find all fields are 0)
pub fn aes_encrypt_default(key: &UserShieldingKeyType, data: &[u8]) -> AesOutput {
	let mut in_out = data.to_vec();

	let nonce = RingAeadNonceSequence::new();
	let aad = b"";
	if let Ok(unbound_key) = UnboundKey::new(&AES_256_GCM, key.as_slice()) {
		let mut sealing_key = SealingKey::new(unbound_key, nonce.clone());
		if sealing_key.seal_in_place_append_tag(Aad::from(aad), &mut in_out).is_ok() {
			return AesOutput { ciphertext: in_out.to_vec(), aad: aad.to_vec(), nonce: nonce.nonce }
		}
	}

	AesOutput::default()
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
