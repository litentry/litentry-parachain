// Copyright 2020-2024 Trust Computing GmbH.
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

use crate::handler::nonce_share::NonceShareError::InvalidSigner;
use bc_enclave_registry::EnclaveRegistryLookup;
use bc_musig2_ceremony::{CeremonyCommand, CeremonyId, PubNonce};
use codec::Encode;
use litentry_primitives::Identity;
use log::debug;
use std::sync::Arc;

#[derive(Encode, Debug)]
pub enum NonceShareError {
	InvalidSigner,
	InvalidNonce,
}

pub fn handle<ER: EnclaveRegistryLookup>(
	signer: Identity,
	ceremony_id: &CeremonyId,
	payload: [u8; 66],
	enclave_registry: Arc<ER>,
) -> Result<CeremonyCommand, NonceShareError> {
	debug!("Received nonce share from: {:?} for ceremony {:?}", signer, ceremony_id);
	let is_valid_signer = match signer {
		Identity::Substrate(address) => enclave_registry.contains_key(&address),
		_ => false,
	};
	if !is_valid_signer {
		return Err(InvalidSigner)
	}

	let nonce =
		PubNonce::from_bytes(payload.as_slice()).map_err(|_| NonceShareError::InvalidNonce)?;

	match signer {
		Identity::Substrate(address) => Ok(CeremonyCommand::SaveNonce(*address.as_ref(), nonce)),
		_ => Err(InvalidSigner),
	}
}

#[cfg(test)]
pub mod test {
	use crate::handler::nonce_share::{handle, NonceShareError};
	use alloc::sync::Arc;
	use bc_enclave_registry::{EnclaveRegistry, EnclaveRegistryUpdater};
	use bc_musig2_ceremony::SignBitcoinPayload;
	use itp_sgx_crypto::{key_repository::AccessKey, schnorr::Pair as SchnorrPair, Error};
	use litentry_primitives::Identity;
	use sp_core::{sr25519, Pair};

	struct SignerAccess {}

	impl AccessKey for SignerAccess {
		type KeyType = SchnorrPair;

		fn retrieve_key(&self) -> itp_sgx_crypto::Result<Self::KeyType> {
			Err(Error::LockPoisoning)
		}
	}

	#[test]
	pub fn it_should_return_ok_for_enclave_signer() {
		// given
		let alice_key_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let signer_account = Identity::Substrate(alice_key_pair.public().into());
		let ceremony_id = SignBitcoinPayload::Derived(vec![]);
		let enclave_registry = Arc::new(EnclaveRegistry::default());
		let _ =
			enclave_registry.update(alice_key_pair.public().into(), "localhost:2000".to_string());

		// when
		let result = handle(
			signer_account,
			&ceremony_id,
			[
				2, 121, 190, 102, 126, 249, 220, 187, 172, 85, 160, 98, 149, 206, 135, 11, 7, 2,
				155, 252, 219, 45, 206, 40, 217, 89, 242, 129, 91, 22, 248, 23, 152, 3, 45, 226,
				102, 38, 40, 201, 11, 3, 245, 231, 32, 40, 78, 181, 47, 247, 215, 31, 66, 132, 246,
				39, 182, 138, 133, 61, 120, 199, 142, 31, 254, 147,
			],
			enclave_registry,
		);

		// then
		assert!(result.is_ok())
	}

	#[test]
	pub fn it_should_return_err_for_non_enclave_signer() {
		// given
		let alice_key_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let signer_account = Identity::Substrate(alice_key_pair.public().into());
		let ceremony_id = SignBitcoinPayload::Derived(vec![]);
		let enclave_registry = Arc::new(EnclaveRegistry::default());

		// when
		let result = handle(
			signer_account,
			&ceremony_id,
			[
				2, 121, 190, 102, 126, 249, 220, 187, 172, 85, 160, 98, 149, 206, 135, 11, 7, 2,
				155, 252, 219, 45, 206, 40, 217, 89, 242, 129, 91, 22, 248, 23, 152, 3, 45, 226,
				102, 38, 40, 201, 11, 3, 245, 231, 32, 40, 78, 181, 47, 247, 215, 31, 66, 132, 246,
				39, 182, 138, 133, 61, 120, 199, 142, 31, 254, 147,
			],
			enclave_registry,
		);

		// then
		assert!(matches!(result, Err(NonceShareError::InvalidSigner)))
	}
}
