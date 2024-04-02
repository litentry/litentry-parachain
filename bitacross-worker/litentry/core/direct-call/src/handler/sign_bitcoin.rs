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
use parentchain_primitives::Identity;
use std::{
	collections::HashMap,
	string::{String, ToString},
	sync::Arc,
	vec::Vec,
};

#[cfg(feature = "std")]
use std::sync::Mutex;

use bc_musig2_ceremony::{
	CeremonyCommand, CeremonyId, CeremonyRegistry, MuSig2Ceremony, PublicKey,
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
	ceremony_events: Arc<Mutex<HashMap<CeremonyId, Vec<CeremonyCommand>>>>,
	signer_registry: Arc<SR>,
	enclave_key_pub: &[u8; 32],
	signer_access_key: Arc<AK>,
) -> Result<[u8; 64], String> {
	if relayer_registry.contains_key(signer) {
		let mut registry = ceremony_registry.lock().unwrap();
		let ceremony_tick_to_live = 15;

		let peers = signer_registry
			.get_all()
			.iter()
			.map(|(address, pub_key)| {
				(*address.as_ref(), PublicKey::from_sec1_bytes(pub_key).unwrap())
			})
			.collect();

		let events = ceremony_events.lock().unwrap().remove(&payload).unwrap_or_default();
		let ceremony = MuSig2Ceremony::new(
			*enclave_key_pub,
			aes_key,
			peers,
			payload.clone(),
			events,
			signer_access_key,
			ceremony_tick_to_live,
		)?;
		registry.insert(payload, ceremony);

		Ok([0; 64])
	} else {
		Err("Unauthorized: Signer is not a valid relayer".to_string())
	}
}

#[cfg(test)]
pub mod test {
	// use crate::handler::sign_bitcoin::handle;
	// use bc_relayer_registry::{RelayerRegistry, RelayerRegistryUpdater};
	// use itp_sgx_crypto::mocks::KeyRepositoryMock;
	// use k256::elliptic_curve::rand_core;
	// use parentchain_primitives::Identity;
	// use sp_core::{sr25519, Pair};

	// #[test]
	// pub fn it_should_return_ok_for_relayer_signer() {
	// 	//given
	// 	let relayer_registry = RelayerRegistry::default();
	// 	let alice_key_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
	// 	let relayer_account = Identity::Substrate(alice_key_pair.public().into());
	// 	relayer_registry.update(relayer_account.clone()).unwrap();
	//
	// 	let private = k256::schnorr::SigningKey::random(&mut rand_core::OsRng);
	// 	let signing_key = itp_sgx_crypto::schnorr::Pair::new(private);
	//
	// 	let key_repository = KeyRepositoryMock::new(signing_key);
	//
	// 	//when
	// 	let result = handle(relayer_account, vec![], &relayer_registry, &key_repository);
	//
	// 	//then
	// 	assert!(result.is_ok())
	// }

	// #[test]
	// pub fn it_should_return_err_for_non_relayer_signer() {
	// 	//given
	// 	let relayer_registry = RelayerRegistry::default();
	// 	let alice_key_pair = sr25519::Pair::from_string("//Alice", None).unwrap();
	// 	let non_relayer_account = Identity::Substrate(alice_key_pair.public().into());
	//
	// 	let private = k256::schnorr::SigningKey::random(&mut rand_core::OsRng);
	// 	let signing_key = itp_sgx_crypto::schnorr::Pair::new(private);
	//
	// 	let key_repository = KeyRepositoryMock::new(signing_key);
	//
	// 	//when
	// 	let result = handle(non_relayer_account, vec![], &relayer_registry, &key_repository);
	//
	// 	//then
	// 	assert!(result.is_err())
	// }
}
