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

use bc_musig2_ceremony::{CeremonyCommand, CeremonyId, MuSig2Ceremony, PartialSignature};
use itp_sgx_crypto::{key_repository::AccessKey, schnorr::Pair as SchnorrPair};
#[cfg(feature = "sgx")]
use std::sync::SgxMutex as Mutex;

pub fn handle<ER: EnclaveRegistryLookup, AK: AccessKey<KeyType = SchnorrPair>>(
	signer: Identity,
	ceremony_id: CeremonyId,
	signature: [u8; 32],
	ceremony_registry: Arc<Mutex<HashMap<CeremonyId, MuSig2Ceremony<AK>>>>,
	enclave_registry: Arc<ER>,
) -> Result<Vec<u8>, String> {
	log::info!("Partial signature share call for ceremony {:?}", ceremony_id);
	let is_valid_signer = match signer {
		Identity::Substrate(address) => enclave_registry.contains_key(&address),
		_ => false,
	};
	if !is_valid_signer {
		return Err("Signer is not a valid enclave".to_string())
	}
	let mut registry = ceremony_registry.lock().unwrap();
	match signer {
		Identity::Substrate(address) =>
			if let Some(ceremony) = registry.get_mut(&ceremony_id) {
				ceremony.save_event(CeremonyCommand::SavePartialSignature(
					*address.as_ref(),
					PartialSignature::from_slice(&signature).unwrap(),
				));
			},
		_ => return Err("Signer is not of substrate type".to_string()),
	}

	Ok(vec![])
}
