use bc_enclave_registry::EnclaveRegistryLookup;
use parentchain_primitives::Identity;
use std::{
	collections::HashMap,
	string::{String, ToString},
	sync::Arc,
	vec,
	vec::Vec,
};

#[cfg(feature = "std")]
use std::sync::Mutex;

use bc_musig2_ceremony::{CeremonyCommand, CeremonyId, CeremonyRegistry, PubNonce};
use itp_sgx_crypto::{key_repository::AccessKey, schnorr::Pair as SchnorrPair};
#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;

pub fn handle<ER: EnclaveRegistryLookup, AK: AccessKey<KeyType = SchnorrPair>>(
	signer: Identity,
	ceremony_id: CeremonyId,
	payload: [u8; 66],
	ceremony_registry: Arc<Mutex<CeremonyRegistry<AK>>>,
	ceremony_events: Arc<Mutex<HashMap<CeremonyId, Vec<CeremonyCommand>>>>,
	enclave_registry: Arc<ER>,
) -> Result<Vec<u8>, String> {
	let is_valid_signer = match signer {
		Identity::Substrate(address) => enclave_registry.contains_key(&address),
		_ => false,
	};
	if !is_valid_signer {
		return Err("Signer is not a valid enclave".to_string())
	}

	match signer {
		Identity::Substrate(address) => {
			let mut registry = ceremony_registry.lock().unwrap();
			if let Some(ceremony) = registry.get_mut(&ceremony_id) {
				ceremony.save_event(CeremonyCommand::SaveNonce(
					*address.as_ref(),
					PubNonce::from_bytes(payload.as_slice()).unwrap(),
				));
			} else {
				std::println!("Ceremony not found, saving events...");
				let mut events = ceremony_events.lock().unwrap();
				if let Some(events) = events.get_mut(&ceremony_id) {
					std::println!("Events found, appending...");
					events.push(CeremonyCommand::SaveNonce(
						*address.as_ref(),
						PubNonce::from_bytes(payload.as_slice()).unwrap(),
					));
				} else {
					std::println!("Events not found, creating...");
					events.insert(
						ceremony_id,
						vec![CeremonyCommand::SaveNonce(
							*address.as_ref(),
							PubNonce::from_bytes(payload.as_slice()).unwrap(),
						)],
					);
				}
			}
		},
		_ => return Err("Signer is not of substrate type".to_string()),
	}
	Ok(vec![])
}
