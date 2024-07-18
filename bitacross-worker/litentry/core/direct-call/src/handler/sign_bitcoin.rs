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
use bc_musig2_ceremony::{CeremonyCommand, PublicKey, SignersWithKeys};
use bc_relayer_registry::RelayerRegistryLookup;
use bc_signer_registry::SignerRegistryLookup;
use codec::Encode;
use parentchain_primitives::Identity;
use std::sync::Arc;

#[derive(Encode, Debug)]
pub enum SignBitcoinError {
	InvalidSigner,
	CeremonyError,
}

#[allow(clippy::too_many_arguments)]
pub fn handle<RRL: RelayerRegistryLookup, SR: SignerRegistryLookup, ER: EnclaveRegistryLookup>(
	signer: Identity,
	aes_key: [u8; 32],
	relayer_registry: &RRL,
	signer_registry: Arc<SR>,
	enclave_registry: &ER,
) -> Result<CeremonyCommand, SignBitcoinError> {
	if relayer_registry.contains_key(&signer)
		|| match &signer {
			Identity::Substrate(address) => enclave_registry.contains_key(address),
			_ => false,
		} {
		let signers: Result<SignersWithKeys, SignBitcoinError> = signer_registry
			.get_all()
			.iter()
			.map(|(address, pub_key)| {
				let public_key = PublicKey::from_sec1_bytes(pub_key)
					.map_err(|_| SignBitcoinError::CeremonyError)?;
				Ok((*address.as_ref(), public_key))
			})
			.collect();

		Ok(CeremonyCommand::InitCeremony(aes_key, signers?))
	} else {
		Err(SignBitcoinError::InvalidSigner)
	}
}

#[cfg(test)]
pub mod test {
	use crate::handler::sign_bitcoin::{handle, SignBitcoinError};
	use alloc::sync::Arc;
	use bc_enclave_registry::{EnclaveRegistry, EnclaveRegistryUpdater};
	use bc_relayer_registry::{RelayerRegistry, RelayerRegistryUpdater};
	use bc_signer_registry::{PubKey, SignerRegistryLookup};
	use itp_sgx_crypto::{key_repository::AccessKey, schnorr::Pair as SchnorrPair, Error};
	use parentchain_primitives::{Address32, Identity};
	use sp_core::{sr25519, Pair};

	struct SignersRegistryMock {}

	impl SignerRegistryLookup for SignersRegistryMock {
		fn contains_key(&self, _account: &Address32) -> bool {
			true
		}

		fn get_all(&self) -> Vec<(Address32, PubKey)> {
			vec![
				(
					Address32::from([0u8; 32]),
					[
						2, 58, 165, 169, 140, 84, 151, 130, 21, 185, 32, 243, 101, 89, 29, 51, 56,
						38, 233, 110, 219, 75, 23, 37, 81, 20, 189, 129, 185, 104, 46, 113, 33,
					],
				),
				(
					Address32::from([1u8; 32]),
					[
						2, 33, 158, 56, 188, 136, 36, 56, 255, 109, 228, 17, 179, 63, 196, 98, 40,
						57, 207, 209, 184, 120, 220, 9, 54, 115, 189, 207, 56, 230, 136, 48, 51,
					],
				),
				(
					Address32::from([2u8; 32]),
					[
						2, 167, 108, 241, 140, 166, 89, 112, 114, 58, 251, 60, 114, 93, 85, 16,
						221, 20, 31, 40, 78, 234, 124, 2, 156, 166, 18, 246, 230, 29, 49, 229, 58,
					],
				),
			]
		}
	}

	struct SignerAccess {}

	impl AccessKey for SignerAccess {
		type KeyType = SchnorrPair;

		fn retrieve_key(&self) -> itp_sgx_crypto::Result<Self::KeyType> {
			Err(Error::LockPoisoning)
		}
	}

	#[test]
	pub fn it_should_return_ok_for_relayer_signer() {
		// given
		let relayer_registry = RelayerRegistry::default();
		let enclave_registry = EnclaveRegistry::default();
		let alice_key_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let relayer_account = Identity::Substrate(alice_key_pair.public().into());
		relayer_registry.update(relayer_account.clone()).unwrap();
		let signers_registry = Arc::new(SignersRegistryMock {});

		// when
		let result = handle(
			relayer_account,
			[0u8; 32],
			&relayer_registry,
			signers_registry,
			&enclave_registry,
		);

		// then
		assert!(result.is_ok())
	}

	#[test]
	pub fn it_should_return_ok_for_enclave_signer() {
		// given
		let relayer_registry = RelayerRegistry::default();
		let enclave_registry = EnclaveRegistry::default();
		let alice_key_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let enclave_account = Identity::Substrate(alice_key_pair.public().into());
		enclave_registry.update(alice_key_pair.public().into(), "".to_string()).unwrap();
		let signers_registry = Arc::new(SignersRegistryMock {});

		// when
		let result = handle(
			enclave_account,
			[0u8; 32],
			&relayer_registry,
			signers_registry,
			&enclave_registry,
		);

		// then
		assert!(result.is_ok())
	}

	#[test]
	pub fn it_should_return_err_for_non_relayer_and_non_enclave_signer() {
		//given
		let relayer_registry = RelayerRegistry::default();
		let enclave_registry = EnclaveRegistry::default();

		let alice_key_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let non_relayer_account = Identity::Substrate(alice_key_pair.public().into());
		let signers_registry = Arc::new(SignersRegistryMock {});

		//when
		let result = handle(
			non_relayer_account,
			[0u8; 32],
			&relayer_registry,
			signers_registry,
			&enclave_registry,
		);

		//then
		assert!(matches!(result, Err(SignBitcoinError::InvalidSigner)))
	}
}
