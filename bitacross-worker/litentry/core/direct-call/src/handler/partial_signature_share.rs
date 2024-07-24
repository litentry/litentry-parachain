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

use bc_enclave_registry::EnclaveRegistryLookup;
use codec::Encode;
use log::debug;
use parentchain_primitives::Identity;
use std::{collections::HashMap, sync::Arc};

#[cfg(feature = "std")]
use std::sync::Mutex;

use crate::handler::partial_signature_share::PartialSignatureShareError::{
	InvalidSignature, SignatureSaveError,
};
use bc_musig2_ceremony::{CeremonyCommand, CeremonyId, MuSig2Ceremony, PartialSignature};
use itp_sgx_crypto::{key_repository::AccessKey, schnorr::Pair as SchnorrPair};
#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;

#[derive(Encode, Debug)]
pub enum PartialSignatureShareError {
	InvalidSigner,
	SignatureSaveError,
	InvalidSignature,
}

pub fn handle<ER: EnclaveRegistryLookup, AK: AccessKey<KeyType = SchnorrPair>>(
	signer: Identity,
	ceremony_id: CeremonyId,
	signature: [u8; 32],
	ceremony_registry: Arc<Mutex<HashMap<CeremonyId, MuSig2Ceremony<AK>>>>,
	enclave_registry: Arc<ER>,
) -> Result<(), PartialSignatureShareError> {
	debug!("Received partial signature share from: {:?} for ceremony {:?}", signer, ceremony_id);
	let is_valid_signer = match signer {
		Identity::Substrate(address) => enclave_registry.contains_key(&address),
		_ => false,
	};
	if !is_valid_signer {
		return Err(PartialSignatureShareError::InvalidSigner)
	}
	let mut registry = ceremony_registry.lock().map_err(|_| SignatureSaveError)?;
	match signer {
		Identity::Substrate(address) =>
			if let Some(ceremony) = registry.get_mut(&ceremony_id) {
				ceremony.save_event(CeremonyCommand::SavePartialSignature(
					*address.as_ref(),
					PartialSignature::from_slice(&signature).map_err(|_| InvalidSignature)?,
				));
			},
		_ => return Err(PartialSignatureShareError::InvalidSigner),
	}

	Ok(())
}

#[cfg(test)]
pub mod test {

	use crate::handler::partial_signature_share::{
		handle, PartialSignatureShareError, SchnorrPair,
	};
	use alloc::sync::Arc;
	use bc_enclave_registry::{EnclaveRegistry, EnclaveRegistryUpdater};
	use bc_musig2_ceremony::{CeremonyRegistry, SignBitcoinPayload};
	use codec::alloc::sync::Mutex;
	use itp_sgx_crypto::{key_repository::AccessKey, Error};
	use parentchain_primitives::Identity;
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
		let ceremony_registry = Arc::new(Mutex::new(CeremonyRegistry::<SignerAccess>::new()));
		let enclave_registry = Arc::new(EnclaveRegistry::default());
		let _ =
			enclave_registry.update(alice_key_pair.public().into(), "localhost:2000".to_string());

		// when
		let result = handle(
			signer_account,
			ceremony_id,
			[
				137, 19, 147, 124, 98, 243, 46, 98, 24, 93, 239, 14, 218, 117, 49, 69, 110, 245,
				176, 150, 209, 28, 241, 70, 195, 172, 198, 5, 12, 146, 251, 228,
			],
			ceremony_registry,
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
		let ceremony_registry = Arc::new(Mutex::new(CeremonyRegistry::<SignerAccess>::new()));
		let enclave_registry = Arc::new(EnclaveRegistry::default());

		// when
		let result = handle(
			signer_account,
			ceremony_id,
			[
				137, 19, 147, 124, 98, 243, 46, 98, 24, 93, 239, 14, 218, 117, 49, 69, 110, 245,
				176, 150, 209, 28, 241, 70, 195, 172, 198, 5, 12, 146, 251, 228,
			],
			ceremony_registry,
			enclave_registry,
		);

		// then
		assert!(matches!(result, Err(PartialSignatureShareError::InvalidSigner)))
	}
}
