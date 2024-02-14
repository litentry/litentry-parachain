use bc_relayer_registry::RelayerRegistryLookup;
use itp_sgx_crypto::{key_repository::AccessKey, schnorr::Pair};
use parentchain_primitives::Identity;
use std::{
	string::{String, ToString},
	vec::Vec,
};

pub fn handle<RRL: RelayerRegistryLookup, BKR: AccessKey<KeyType = Pair>>(
	signer: Identity,
	payload: Vec<u8>,
	relayer_registry: &RRL,
	key_repository: &BKR,
) -> Result<[u8; 64], String> {
	if relayer_registry.contains_key(signer) {
		let key = key_repository.retrieve_key().unwrap();
		Ok(key.sign(&payload).unwrap())
	} else {
		Err("Unauthorized: Signer is not a valid relayer".to_string())
	}
}

#[cfg(test)]
pub mod test {
	use crate::handler::sign_bitcoin::handle;
	use bc_relayer_registry::{RelayerRegistry, RelayerRegistryUpdater};
	use itp_sgx_crypto::mocks::KeyRepositoryMock;
	use k256::elliptic_curve::rand_core;
	use parentchain_primitives::{Address32, Identity};
	use sp_core::{crypto::Ss58Codec, sr25519 as sr25519_core, sr25519, Pair};

	#[test]
	pub fn it_should_return_ok_for_relayer_signer() {
		//given
		let relayer_registry = RelayerRegistry::default();
		let alice_key_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let relayer_account = Identity::Substrate(alice_key_pair.public().into());
		relayer_registry.update(relayer_account.clone()).unwrap();

		let private = k256::schnorr::SigningKey::random(&mut rand_core::OsRng);
		let signing_key = itp_sgx_crypto::schnorr::Pair::new(private);

		let key_repository = KeyRepositoryMock::new(signing_key);

		//when
		let result = handle(relayer_account, vec![], &relayer_registry, &key_repository);

		//then
		assert!(result.is_ok())
	}

	#[test]
	pub fn it_should_return_err_for_non_relayer_signer() {
		//given
		let relayer_registry = RelayerRegistry::default();
		let alice_key_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
		let non_relayer_account = Identity::Substrate(alice_key_pair.public().into());

		let private = k256::schnorr::SigningKey::random(&mut rand_core::OsRng);
		let signing_key = itp_sgx_crypto::schnorr::Pair::new(private);

		let key_repository = KeyRepositoryMock::new(signing_key);

		//when
		let result = handle(non_relayer_account, vec![], &relayer_registry, &key_repository);

		//then
		assert!(result.is_err())
	}
}
