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

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
extern crate core;
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use musig2_sgx as musig2;
use std::{format, string::String, sync::Arc};

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

// enclave public key is used as signer identifier
pub type SignerId = [u8; 32];

use k256::SecretKey;
use musig2::{
	secp::Point, BinaryEncoding, CompactSignature, KeyAggContext, LiftedSignature, SecNonceSpices,
};
use std::{vec, vec::Vec};

use crate::CeremonyEvent::CeremonyEnded;
use codec::{Decode, Encode};
use itp_sgx_crypto::{key_repository::AccessKey, schnorr::Pair as SchnorrPair};
pub use k256::{elliptic_curve::sec1::FromEncodedPoint, PublicKey};
use log::{debug, error, info};
pub use musig2::{PartialSignature, PubNonce};
use std::collections::HashMap;

//TODO: this should be a hash of message
pub type CeremonyId = SignBitcoinPayload;
pub type SignersList = Vec<SignerId>;
pub type CeremonyRegistry<AK> = HashMap<CeremonyId, MuSig2Ceremony<AK>>;

#[derive(Debug)]
pub enum CeremonyError {
	NonceReceivingError(NonceReceivingErrorReason),
	PartialSignatureReceivingError(PartialSignatureReceivingErrorReason),
}

#[derive(Debug)]
pub enum NonceReceivingErrorReason {
	SignerNotFound,
	ContributionError,
	IncorrectRound,
	FirstRoundFinalizationError,
}

#[derive(Debug)]
pub enum PartialSignatureReceivingErrorReason {
	SignerNotFound,
	ContributionError,
	IncorrectRound,
	SecondRoundFinalizationError,
}

// events come from outside and are consumed by ceremony in tick fn
#[derive(Debug)]
pub enum CeremonyCommand {
	SaveNonce(SignerId, PubNonce),
	SavePartialSignature(SignerId, PartialSignature),
}

// commands are created by ceremony and executed by orchestrator
#[derive(Debug)]
pub enum CeremonyEvent {
	FirstRoundStarted(SignersList, CeremonyId, PubNonce),
	SecondRoundStarted(SignersList, CeremonyId, PartialSignature),
	CeremonyError(CeremonyError),
	CeremonyEnded([u8; 64]),
	CeremonyTimedOut,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SignBitcoinPayload {
	Derived(Vec<u8>),
	TaprootUnspendable(Vec<u8>),
	TaprootSpendable(Vec<u8>, [u8; 32]),
}

pub struct MuSig2Ceremony<AK: AccessKey<KeyType = SchnorrPair>> {
	payload: SignBitcoinPayload,
	//todo: move to layer above, ceremony should be communication agnostic
	aes_key: [u8; 32],
	me: SignerId,
	signers: Vec<(SignerId, PublicKey)>,
	commands: Vec<CeremonyCommand>,
	events: Vec<CeremonyEvent>,
	signing_key_access: Arc<AK>,
	first_round: Option<musig2::FirstRound>,
	second_round: Option<musig2::SecondRound<Vec<u8>>>,
	ticks_left: u8,
}

impl<AK: AccessKey<KeyType = SchnorrPair>> MuSig2Ceremony<AK> {
	// Creates new ceremony and starts first round
	pub fn new(
		me: SignerId,
		aes_key: [u8; 32],
		signers: Vec<(SignerId, PublicKey)>,
		payload: SignBitcoinPayload,
		commands: Vec<CeremonyCommand>,
		signing_key_access: Arc<AK>,
		ttl: u8,
	) -> Result<Self, String> {
		info!("Creating new ceremony with peers: {:?} and events {:?}", signers, commands);
		if signers.len() < 3 {
			return Err(format!("Not enough signers, minimum: {:?}, actual {:?}", 3, signers.len()))
		}
		// we are always the first key in the vector
		let my_index = signers.iter().position(|r| r.0 == me).ok_or("Could not determine index")?;
		let all_keys = signers.iter().map(|p| p.1).collect::<Vec<PublicKey>>();
		let key_context = match &payload {
			SignBitcoinPayload::TaprootSpendable(_, root_hash) =>
				KeyAggContext::new(all_keys.iter().map(|p| Point::from(*p)))
					.map_err(|e| format!("Key context creation error: {:?}", e))?
					.with_taproot_tweak(root_hash)
					.map_err(|e| format!("Key context creation error: {:?}", e))?,
			SignBitcoinPayload::TaprootUnspendable(_) =>
				KeyAggContext::new(all_keys.iter().map(|p| Point::from(*p)))
					.map_err(|e| format!("Key context creation error: {:?}", e))?
					.with_unspendable_taproot_tweak()
					.map_err(|e| format!("Key context creation error: {:?}", e))?,
			SignBitcoinPayload::Derived(_) =>
				KeyAggContext::new(all_keys.iter().map(|p| Point::from(*p)))
					.map_err(|e| format!("Key context creation error: {:?}", e))?,
		};

		let nonce_seed = random_seed();
		let first_round =
			musig2::FirstRound::new(key_context, nonce_seed, my_index, SecNonceSpices::new())
				.map_err(|e| format!("First round creation error: {:?}", e))?;

		let public_nonce = first_round.our_public_nonce();
		let events = vec![CeremonyEvent::FirstRoundStarted(
			signers.iter().filter(|e| e.0 != me).map(|s| s.0).collect(),
			payload.clone(),
			public_nonce,
		)];

		Ok(Self {
			payload,
			aes_key,
			me,
			signers,
			commands,
			events,
			signing_key_access,
			first_round: Some(first_round),
			second_round: None,
			ticks_left: ttl,
		})
	}

	pub fn tick(&mut self) -> Vec<CeremonyEvent> {
		debug!("Ceremony {:?} tick", self.get_id_ref());
		self.process_commands();
		self.ticks_left -= 1;

		if self.ticks_left == 0 {
			self.events.push(CeremonyEvent::CeremonyTimedOut);
		}

		self.events.drain(0..).collect()
	}

	pub fn save_event(&mut self, event: CeremonyCommand) {
		self.commands.push(event);
	}

	fn process_commands(&mut self) {
		let mut i = 0;
		let mut commands_to_execute = vec![];
		while i < self.commands.len() {
			if true {
				commands_to_execute.push(self.commands.remove(i));
			} else {
				i += 1;
			}
		}
		for command in commands_to_execute.into_iter() {
			debug!(
				"Processing ceremony command: {:?} for ceremony: {:?}",
				command,
				self.get_id_ref()
			);
			if let Err(e) = match command {
				CeremonyCommand::SaveNonce(signer, nonce) => self.receive_nonce(signer, nonce),
				CeremonyCommand::SavePartialSignature(signer, partial_signature) =>
					self.receive_partial_sign(signer, partial_signature),
			} {
				self.events.push(CeremonyEvent::CeremonyError(e));
			}
		}
	}

	// Saves signer's nonce
	fn receive_nonce(&mut self, signer: SignerId, nonce: PubNonce) -> Result<(), CeremonyError> {
		let peer_index =
			self.signers.iter().position(|p| p.0 == signer).ok_or(
				CeremonyError::NonceReceivingError(NonceReceivingErrorReason::SignerNotFound),
			)?;

		if let Some(ref mut r) = self.first_round {
			r.receive_nonce(peer_index, nonce).map_err(|e| {
				error!("Nonce receiving error: {:?}", e);
				CeremonyError::NonceReceivingError(NonceReceivingErrorReason::ContributionError)
			})?;
			if r.is_complete() {
				let secret_key = SecretKey::from_slice(
					&self
						.signing_key_access
						.retrieve_key()
						.map_err(|e| {
							error!("Nonce receiving error: {:?}", e);
							CeremonyError::NonceReceivingError(
								NonceReceivingErrorReason::FirstRoundFinalizationError,
							)
						})?
						.private_bytes(),
				)
				.map_err(|e| {
					error!("Nonce receiving error: {:?}", e);
					CeremonyError::NonceReceivingError(
						NonceReceivingErrorReason::FirstRoundFinalizationError,
					)
				})?;
				self.start_second_round(secret_key)?;
			}
			Ok(())
		} else {
			Err(CeremonyError::NonceReceivingError(NonceReceivingErrorReason::IncorrectRound))
		}
	}

	// Starts the second round
	fn start_second_round(&mut self, private_key: SecretKey) -> Result<(), CeremonyError> {
		let first_round = self
			.first_round
			.take()
			.ok_or(CeremonyError::NonceReceivingError(NonceReceivingErrorReason::IncorrectRound))?;

		let message = match &self.payload {
			SignBitcoinPayload::TaprootSpendable(message, _) => message.clone(),
			SignBitcoinPayload::Derived(message) => message.clone(),
			SignBitcoinPayload::TaprootUnspendable(message) => message.clone(),
		};
		let second_round = first_round.finalize(private_key, message).map_err(|e| {
			error!("Could not start second round: {:?}", e);
			CeremonyError::NonceReceivingError(
				NonceReceivingErrorReason::FirstRoundFinalizationError,
			)
		})?;

		let partial_signature: PartialSignature = second_round.our_signature();

		self.events.push(CeremonyEvent::SecondRoundStarted(
			self.signers.iter().filter(|e| e.0 != self.me).map(|s| s.0).collect(),
			self.get_id_ref().clone(),
			partial_signature,
		));
		self.second_round = Some(second_round);
		Ok(())
	}

	// Saves signer's partial signature
	pub fn receive_partial_sign(
		&mut self,
		signer: SignerId,
		partial_signature: impl Into<PartialSignature>,
	) -> Result<(), CeremonyError> {
		let peer_index = self.signers.iter().position(|p| p.0 == signer).ok_or(
			CeremonyError::PartialSignatureReceivingError(
				PartialSignatureReceivingErrorReason::SignerNotFound,
			),
		)?;

		if let Some(ref mut r) = self.second_round {
			r.receive_signature(peer_index, partial_signature).map_err(|e| {
				error!("Signature receiving error: {:?}", e);
				CeremonyError::PartialSignatureReceivingError(
					PartialSignatureReceivingErrorReason::ContributionError,
				)
			})?;
			if r.is_complete() {
				if let Some(r) = self.second_round.take() {
					let signature: CompactSignature = r
						.finalize::<LiftedSignature>()
						.map_err(|e| {
							error!("Could not finish second round: {:?}", e);
							CeremonyError::PartialSignatureReceivingError(
								PartialSignatureReceivingErrorReason::SecondRoundFinalizationError,
							)
						})?
						.compact();
					self.events.push(CeremonyEnded(signature.to_bytes()));
				}
			}
			Ok(())
		} else {
			Err(CeremonyError::PartialSignatureReceivingError(
				PartialSignatureReceivingErrorReason::IncorrectRound,
			))
		}
	}

	pub fn get_id_ref(&self) -> &CeremonyId {
		&self.payload
	}
	pub fn get_aes_key(&self) -> &[u8; 32] {
		&self.aes_key
	}
}

#[cfg(feature = "std")]
fn random_seed() -> [u8; 32] {
	use rand::{thread_rng, RngCore};

	let mut seed = [0u8; 32];
	let mut rand = thread_rng();
	rand.fill_bytes(&mut seed);
	seed
}

#[cfg(feature = "sgx")]
fn random_seed() -> [u8; 32] {
	use sgx_rand::{Rng, StdRng};
	let mut seed = [0u8; 32];
	let mut rand = StdRng::new().unwrap();
	rand.fill_bytes(&mut seed);
	seed
}

#[cfg(test)]
pub mod test {
	// use crate::MuSig2Ceremony;
	// use k256::{schnorr::SigningKey, PublicKey, SecretKey};
	// use musig2::{secp, secp::Scalar, BinaryEncoding, PubNonce, SecNonce};
	// use rand::{
	// rngs::{StdRng, ThreadRng},
	// thread_rng, RngCore,
	// };

	pub const MY_PRIV_KEY: [u8; 32] = [
		252, 240, 35, 85, 243, 83, 129, 54, 7, 155, 24, 114, 254, 0, 134, 251, 207, 83, 177, 9, 92,
		118, 222, 5, 202, 239, 188, 215, 132, 113, 127, 94,
	];

	pub const SIGNER_1: [u8; 32] = [1u8; 32];
	pub const SIGNER_1_PRIV_KEY: [u8; 32] = [
		42, 82, 57, 169, 208, 130, 125, 141, 62, 185, 167, 41, 142, 217, 252, 135, 158, 128, 44,
		129, 222, 71, 55, 86, 230, 183, 54, 111, 152, 83, 85, 155,
	];
	pub const SIGNER_1_SEC_NONCE: [u8; 64] = [
		57, 232, 181, 133, 43, 97, 251, 79, 229, 110, 26, 121, 197, 2, 249, 237, 222, 207, 129,
		232, 8, 227, 120, 202, 127, 61, 209, 41, 92, 54, 8, 91, 80, 31, 9, 126, 14, 137, 126, 143,
		98, 223, 254, 134, 9, 190, 5, 157, 133, 254, 18, 119, 117, 25, 65, 179, 35, 130, 156, 109,
		233, 51, 18, 32,
	];

	pub const SIGNER_2: [u8; 32] = [2u8; 32];
	pub const SIGNER_2_PRIV_KEY: [u8; 32] = [
		117, 130, 176, 36, 185, 53, 187, 61, 123, 86, 24, 38, 174, 143, 129, 73, 245, 210, 127,
		148, 115, 136, 32, 98, 62, 47, 26, 196, 57, 211, 171, 185,
	];
	pub const SIGNER_2_SEC_NONCE: [u8; 64] = [
		78, 229, 109, 189, 246, 169, 247, 85, 184, 199, 144, 135, 45, 60, 71, 109, 214, 121, 165,
		206, 185, 246, 120, 52, 228, 49, 155, 9, 160, 129, 171, 252, 69, 160, 122, 66, 151, 147,
		141, 118, 226, 189, 100, 94, 74, 163, 158, 245, 111, 99, 108, 202, 224, 110, 71, 106, 178,
		255, 89, 34, 16, 10, 195, 107,
	];

	// #[test]
	// fn it_should_initialize_new_instance() {
	// 	// when
	// 	let (message, ceremony) = init_sample_ceremony_in_first_round();
	//
	// 	// then
	// 	// assert!(
	// 	// 	matches!(ceremony.round, Round::First((ref peers, ref round)) if peers.contains(&SIGNER_1) && peers.contains(&SIGNER_2) && matches!(round, musig2::FirstRound { .. }))
	// 	// );
	// 	assert_eq!(ceremony.message, message);
	// 	// assert!(!ceremony.is_round_completed());
	// }

	// #[test]
	// fn it_should_not_finish_first_round_until_all_nonces_are_received() {
	// 	// given
	// 	let (message, mut ceremony) = init_sample_ceremony_in_first_round();
	//
	// 	// when
	// 	ceremony.receive_nonce(
	// 		SIGNER_1,
	// 		SecNonce::from_bytes(SIGNER_1_SEC_NONCE.as_slice()).unwrap().public_nonce(),
	// 	);
	// 	ceremony.receive_nonce(
	// 		SIGNER_2,
	// 		SecNonce::from_bytes(SIGNER_2_SEC_NONCE.as_slice()).unwrap().public_nonce(),
	// 	);
	//
	// 	// then
	// 	// assert!(
	// 	// 	matches!(ceremony.round, Round::First((ref peers, ref round)) if peers.contains(&SIGNER_1) && peers.contains(&SIGNER_2) && matches!(round, musig2::FirstRound { .. }) && round.holdouts().is_empty())
	// 	// );
	// 	// assert!(!ceremony.is_round_completed())
	// }

	// #[test]
	// fn it_should_not_finish_second_round_until_all_parital_signatures_are_received() {
	// 	// given
	// 	let (message, mut ceremony) = init_sample_ceremony_in_second_round();
	//
	// 	// when
	// 	ceremony.receive_partial_sign(
	// 		SIGNER_1,
	// 		Scalar::one(),
	// 	);
	//
	// 	// then
	// 	assert!(
	// 		matches!(ceremony.round, Round::First((ref peers, ref round)) if peers.contains(&SIGNER_1) && peers.contains(&SIGNER_2) && matches!(round, musig2::FirstRound { .. }) && round.holdouts().is_empty())
	// 	);
	// 	assert!(!ceremony.is_round_completed())
	// }

	// fn init_sample_ceremony_in_first_round() -> (String, MuSig2Ceremony<String>) {
	// 	let message = "TEST_MESSAGE".to_string();
	// 	let my_signing = SigningKey::from_bytes(&MY_PRIV_KEY).unwrap();
	// 	let my_public = PublicKey::from(my_signing.verifying_key());
	//
	// 	let signer_1_signing_key = SigningKey::from_bytes(&SIGNER_1_PRIV_KEY).unwrap();
	// 	let signer_1_public = PublicKey::from(signer_1_signing_key.verifying_key());
	//
	// 	let signer_2_signing_key = SigningKey::from_bytes(&SIGNER_2_PRIV_KEY).unwrap();
	// 	let signer_2_public = PublicKey::from(signer_2_signing_key.verifying_key());
	//
	// 	let ceremony = MuSig2Ceremony::new(
	// 		my_public,
	// 		vec![(SIGNER_1, signer_1_public), (SIGNER_2, signer_2_public)],
	// 		message.clone(),
	// 	);
	// 	(message, ceremony)
	// }

	// fn can_generate_nonces_if_all_keys_present
}
