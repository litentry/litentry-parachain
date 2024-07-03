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

use bc_relayer_registry::RelayerRegistryLookup;
use itp_sgx_crypto::key_repository::AccessKey;
use log::error;
use parentchain_primitives::Identity;
use std::sync::Arc;

#[cfg(feature = "std")]
use std::sync::Mutex;

use bc_musig2_ceremony::{
	CeremonyCommandsRegistry, CeremonyRegistry, MuSig2Ceremony, PublicKey, SignBitcoinError,
	SignersWithKeys,
};
use bc_signer_registry::SignerRegistryLookup;
use itp_sgx_crypto::schnorr::Pair as SchnorrPair;

use bc_musig2_ceremony::SignBitcoinPayload;
#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;

#[allow(clippy::too_many_arguments)]
pub fn handle<
	RRL: RelayerRegistryLookup,
	SR: SignerRegistryLookup,
	AK: AccessKey<KeyType = SchnorrPair>,
>(
	signer: Identity,
	payload: SignBitcoinPayload,
	aes_key: [u8; 32],
	relayer_registry: &RRL,
	ceremony_registry: Arc<Mutex<CeremonyRegistry<AK>>>,
	ceremony_commands: Arc<Mutex<CeremonyCommandsRegistry>>,
	signer_registry: Arc<SR>,
	enclave_key_pub: &[u8; 32],
	signer_access_key: Arc<AK>,
) -> Result<(), SignBitcoinError> {
	if relayer_registry.contains_key(signer) {
		let mut registry = ceremony_registry.lock().map_err(|_| SignBitcoinError::CeremonyError)?;
		// ~1 minute (1 tick ~ 1 ms)
		let ceremony_tick_to_live = 60_000;

		let signers: Result<SignersWithKeys, SignBitcoinError> = signer_registry
			.get_all()
			.iter()
			.map(|(address, pub_key)| {
				let public_key = PublicKey::from_sec1_bytes(pub_key)
					.map_err(|_| SignBitcoinError::CeremonyError)?;
				Ok((*address.as_ref(), public_key))
			})
			.collect();

		if registry.contains_key(&payload) {
			return Err(SignBitcoinError::CeremonyError)
		}

		let pending_commands = ceremony_commands
			.lock()
			.map_err(|_| SignBitcoinError::CeremonyError)?
			.remove(&payload)
			.unwrap_or_default();
		let ceremony = MuSig2Ceremony::new(
			*enclave_key_pub,
			aes_key,
			signers?,
			payload.clone(),
			pending_commands.into_iter().map(|c| c.command).collect(),
			signer_access_key,
			ceremony_tick_to_live,
		)
		.map_err(|e| {
			error!("Could not start ceremony, error: {:?}", e);
			SignBitcoinError::CeremonyError
		})?;
		registry.insert(payload, ceremony);

		Ok(())
	} else {
		Err(SignBitcoinError::InvalidSigner)
	}
}

#[cfg(test)]
pub mod test {
	use crate::handler::sign_bitcoin::{handle, SignBitcoinError};
	use alloc::sync::Arc;
	use bc_musig2_ceremony::{CeremonyCommandsRegistry, CeremonyRegistry, SignBitcoinPayload};
	use bc_relayer_registry::{RelayerRegistry, RelayerRegistryUpdater};
	use bc_signer_registry::{PubKey, SignerRegistryLookup};
	use codec::alloc::sync::Mutex;
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
		let alice_key_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let relayer_account = Identity::Substrate(alice_key_pair.public().into());
		relayer_registry.update(relayer_account.clone()).unwrap();
		let ceremony_registry = Arc::new(Mutex::new(CeremonyRegistry::new()));
		let ceremony_commands_registry = Arc::new(Mutex::new(CeremonyCommandsRegistry::new()));
		let signers_registry = Arc::new(SignersRegistryMock {});
		let signer_access_key = Arc::new(SignerAccess {});

		// when
		let result = handle(
			relayer_account,
			SignBitcoinPayload::Derived(vec![]),
			[0u8; 32],
			&relayer_registry,
			ceremony_registry,
			ceremony_commands_registry,
			signers_registry,
			&[0u8; 32],
			signer_access_key,
		);

		// then
		assert!(result.is_ok())
	}

	#[test]
	pub fn it_should_return_err_for_non_relayer_signer() {
		//given
		let relayer_registry = RelayerRegistry::default();
		let alice_key_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let non_relayer_account = Identity::Substrate(alice_key_pair.public().into());
		let ceremony_registry = Arc::new(Mutex::new(CeremonyRegistry::new()));
		let ceremony_commands_registry = Arc::new(Mutex::new(CeremonyCommandsRegistry::new()));
		let signers_registry = Arc::new(SignersRegistryMock {});
		let signer_access_key = Arc::new(SignerAccess {});

		//when
		let result = handle(
			non_relayer_account,
			SignBitcoinPayload::Derived(vec![]),
			[0u8; 32],
			&relayer_registry,
			ceremony_registry,
			ceremony_commands_registry,
			signers_registry,
			&alice_key_pair.public().0,
			signer_access_key,
		);

		//then
		assert!(matches!(result, Err(SignBitcoinError::InvalidSigner)))
	}

	#[test]
	pub fn it_should_return_err_for_existing_ceremony() {
		// given
		let relayer_registry = RelayerRegistry::default();
		let alice_key_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let relayer_account = Identity::Substrate(alice_key_pair.public().into());
		relayer_registry.update(relayer_account.clone()).unwrap();
		let ceremony_registry = Arc::new(Mutex::new(CeremonyRegistry::new()));
		let ceremony_commands_registry = Arc::new(Mutex::new(CeremonyCommandsRegistry::new()));
		let signers_registry = Arc::new(SignersRegistryMock {});
		let signer_access_key = Arc::new(SignerAccess {});

		// when
		handle(
			relayer_account.clone(),
			SignBitcoinPayload::Derived(vec![]),
			[0u8; 32],
			&relayer_registry,
			ceremony_registry.clone(),
			ceremony_commands_registry.clone(),
			signers_registry.clone(),
			&[0u8; 32],
			signer_access_key.clone(),
		)
		.unwrap();

		let result = handle(
			relayer_account,
			SignBitcoinPayload::Derived(vec![]),
			[0u8; 32],
			&relayer_registry,
			ceremony_registry,
			ceremony_commands_registry,
			signers_registry,
			&[0u8; 32],
			signer_access_key,
		);

		// then
		assert!(matches!(result, Err(SignBitcoinError::CeremonyError)))
	}
}
