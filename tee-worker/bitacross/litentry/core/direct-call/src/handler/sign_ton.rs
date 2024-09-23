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
use codec::Encode;
use itp_sgx_crypto::key_repository::AccessKey;
use litentry_primitives::Identity;
use log::error;
use sp_core::{ed25519::Pair as Ed25519Pair, Pair};
use std::vec::Vec;

#[derive(Encode, Debug)]
pub enum SignTonError {
	InvalidSigner,
	SigningError,
}

pub fn handle<RRL: RelayerRegistryLookup, EKR: AccessKey<KeyType = Ed25519Pair>>(
	signer: Identity,
	msg: Vec<u8>,
	relayer_registry: &RRL,
	key_repository: &EKR,
) -> Result<[u8; 64], SignTonError> {
	if relayer_registry.contains_key(&signer) {
		let key = key_repository.retrieve_key().map_err(|e| {
			error!("Could not retrieve ton signing key: {}", e);
			SignTonError::SigningError
		})?;
		let sig = key.sign(&msg);
		Ok(sig.into())
	} else {
		Err(SignTonError::InvalidSigner)
	}
}

#[cfg(test)]
pub mod test {
	use crate::handler::sign_ton::handle;
	use bc_relayer_registry::{RelayerRegistry, RelayerRegistryUpdater};
	use itp_sgx_crypto::{ecdsa::Pair as EcdsaPair, mocks::KeyRepositoryMock};
	use k256::{ecdsa::SigningKey, elliptic_curve::rand_core};
	use litentry_primitives::Identity;
	use sp_core::{sr25519, Pair};

	#[test]
	pub fn it_should_return_ok_for_relayer_signer() {
		//given
		let relayer_registry = RelayerRegistry::default();
		let alice_key_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let relayer_account = Identity::Substrate(alice_key_pair.public().into());
		relayer_registry.update(relayer_account.clone()).unwrap();

		let signing_key = Pair::from_seed(&[
			135, 174, 141, 248, 62, 107, 189, 100, 181, 60, 54, 229, 76, 255, 248, 189, 240, 238,
			171, 149, 56, 144, 67, 122, 222, 52, 26, 118, 79, 121, 33, 37,
		]);

		let key_repository = KeyRepositoryMock::new(signing_key);

		//when
		let result =
			handle(relayer_account, Default::default(), &relayer_registry, &key_repository);

		//then
		assert!(result.is_ok())
	}

	#[test]
	pub fn it_should_return_err_for_non_relayer_signer() {
		//given
		let relayer_registry = RelayerRegistry::default();
		let alice_key_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let non_relayer_account = Identity::Substrate(alice_key_pair.public().into());

		let private = SigningKey::random(&mut rand_core::OsRng);
		let signing_key = Pair::from_seed(&[
			135, 174, 141, 248, 62, 107, 189, 100, 181, 60, 54, 229, 76, 255, 248, 189, 240, 238,
			171, 149, 56, 144, 67, 122, 222, 52, 26, 118, 79, 121, 33, 37,
		]);

		let key_repository = KeyRepositoryMock::new(signing_key);

		//when
		let result =
			handle(non_relayer_account, Default::default(), &relayer_registry, &key_repository);

		//then
		assert!(result.is_err())
	}
}
