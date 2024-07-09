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

use k256::SecretKey;
use musig2::{
	secp::Point, verify_single, BinaryEncoding, CompactSignature, KeyAggContext, LiftedSignature,
	SecNonceSpices,
};
use std::{vec, vec::Vec};

use crate::CeremonyEvent::CeremonyEnded;
use codec::{Decode, Encode};
use itp_sgx_crypto::{key_repository::AccessKey, schnorr::Pair as SchnorrPair};
pub use k256::{elliptic_curve::sec1::FromEncodedPoint, PublicKey};
use litentry_primitives::RequestAesKey;
use log::{debug, error, info, trace};
use musig2::secp::Scalar;
pub use musig2::{PartialSignature, PubNonce};
use std::collections::HashMap;

pub type CeremonyId = SignBitcoinPayload;
pub type SignaturePayload = Vec<u8>;
pub type Signers = Vec<SignerId>;
pub type CeremonyRegistry<AK> = HashMap<CeremonyId, MuSig2Ceremony<AK>>;
pub type CeremonyCommandsRegistry = HashMap<CeremonyId, Vec<PendingCeremonyCommand>>;
// enclave public key is used as signer identifier
pub type SignerId = [u8; 32];
pub type SignersWithKeys = Vec<(SignerId, PublicKey)>;

pub struct PendingCeremonyCommand {
	pub ticks_left: u32,
	pub command: CeremonyCommand,
}

impl PendingCeremonyCommand {
	pub fn tick(&mut self) {
		self.ticks_left -= 1;
	}
}

#[derive(Encode, Debug)]
pub enum SignBitcoinError {
	InvalidSigner,
	CeremonyError,
}

#[derive(Debug, Eq, PartialEq)]
pub enum CeremonyError {
	NonceReceivingError(NonceReceivingErrorReason),
	PartialSignatureReceivingError(PartialSignatureReceivingErrorReason),
}

#[derive(Debug, Eq, PartialEq)]
pub enum NonceReceivingErrorReason {
	SignerNotFound,
	ContributionError,
	IncorrectRound,
	FirstRoundFinalizationError,
}

#[derive(Debug, Eq, PartialEq)]
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

// commands are created by ceremony and executed by runner
#[derive(Debug, Eq, PartialEq)]
pub enum CeremonyEvent {
	FirstRoundStarted(Signers, CeremonyId, PubNonce),
	SecondRoundStarted(Signers, CeremonyId, PartialSignature),
	CeremonyError(Signers, CeremonyError, RequestAesKey),
	CeremonyEnded([u8; 64], RequestAesKey),
	CeremonyTimedOut(Signers, RequestAesKey),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, Hash)]
pub enum SignBitcoinPayload {
	Derived(SignaturePayload),
	TaprootUnspendable(SignaturePayload),
	TaprootSpendable(SignaturePayload, [u8; 32]),
	WithTweaks(SignaturePayload, Vec<([u8; 32], bool)>),
}

pub fn generate_aggregated_public_key(mut public_keys: Vec<PublicKey>) -> PublicKey {
	public_keys.sort();
	KeyAggContext::new(public_keys).unwrap().aggregated_pubkey()
}

pub struct MuSig2Ceremony<AK: AccessKey<KeyType = SchnorrPair>> {
	payload: SignBitcoinPayload,
	// P-713: move to layer above, ceremony should be communication agnostic
	aes_key: RequestAesKey,
	me: SignerId,
	signers: SignersWithKeys,
	commands: Vec<CeremonyCommand>,
	events: Vec<CeremonyEvent>,
	signing_key_access: Arc<AK>,
	first_round: Option<musig2::FirstRound>,
	second_round: Option<musig2::SecondRound<SignaturePayload>>,
	ticks_left: u32,
	agg_key: Option<PublicKey>,
}

impl<AK: AccessKey<KeyType = SchnorrPair>> MuSig2Ceremony<AK> {
	// Creates new ceremony and starts first round
	pub fn new(
		me: SignerId,
		aes_key: RequestAesKey,
		mut signers: SignersWithKeys,
		payload: SignBitcoinPayload,
		commands: Vec<CeremonyCommand>,
		signing_key_access: Arc<AK>,
		ttl: u32,
	) -> Result<Self, String> {
		info!("Creating new ceremony {:?} and events {:?}", payload, commands);
		if signers.len() < 3 {
			return Err(format!("Not enough signers, minimum: {:?}, actual {:?}", 3, signers.len()))
		}

		signers.sort_by_key(|k| k.1);
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
			SignBitcoinPayload::WithTweaks(_, tweaks) => {
				let mut prepared_tweaks = vec![];
				for (tweak_bytes, is_x_only) in tweaks.iter() {
					let scalar: Scalar = tweak_bytes.try_into().map_err(|e| {
						format!("Key context creation error, could not parse scalar: {:?}", e)
					})?;
					prepared_tweaks.push((scalar, *is_x_only));
				}
				KeyAggContext::new(all_keys.iter().map(|p| Point::from(*p)))
					.map_err(|e| format!("Key context creation error: {:?}", e))?
					.with_tweaks(prepared_tweaks)
					.map_err(|e| format!("Key context creation error: {:?}", e))?
			},
		};

		info!(
			"Ceremony aggregated public key: {:?}",
			key_context.aggregated_pubkey::<PublicKey>().to_sec1_bytes().to_vec()
		);
		let agg_key_copy = key_context.aggregated_pubkey::<PublicKey>();
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
			agg_key: Some(agg_key_copy),
		})
	}

	pub fn tick(&mut self) -> Vec<CeremonyEvent> {
		trace!("Ceremony {:?} tick", self.get_id_ref());
		self.process_commands();
		self.ticks_left -= 1;

		if self.ticks_left == 0 {
			self.events.push(CeremonyEvent::CeremonyTimedOut(
				self.signers.iter().filter(|e| e.0 != self.me).map(|s| s.0).collect(),
				self.aes_key,
			));
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
			if match self.commands.get(i).unwrap() {
				CeremonyCommand::SaveNonce(_, _) => self.first_round.is_some(),
				CeremonyCommand::SavePartialSignature(_, _) => self.second_round.is_some(),
			} {
				commands_to_execute.push(self.commands.remove(i));
			} else {
				debug!(
					"Skipping ceremony command {:?} processing due to incorrect round",
					self.commands.get(i).unwrap()
				);
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
				self.events.push(CeremonyEvent::CeremonyError(
					self.signers.iter().filter(|e| e.0 != self.me).map(|s| s.0).collect(),
					e,
					self.aes_key,
				));
			}
		}
	}

	// Saves signer's nonce
	fn receive_nonce(&mut self, signer: SignerId, nonce: PubNonce) -> Result<(), CeremonyError> {
		info!("Saving nonce from signer: {:?}", signer);
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
			SignBitcoinPayload::WithTweaks(message, _) => message.clone(),
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
	fn receive_partial_sign(
		&mut self,
		signer: SignerId,
		partial_signature: impl Into<PartialSignature>,
	) -> Result<(), CeremonyError> {
		info!("Saving partial signature from signer: {:?}", signer);
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

					info!("Ceremony {:?} has ended", self.get_id_ref());
					info!("Aggregated public key {:?}", self.agg_key.unwrap().to_sec1_bytes());
					info!("Signature {:?}", signature.to_bytes());

					let message = match &self.payload {
						SignBitcoinPayload::Derived(p) => p,
						SignBitcoinPayload::TaprootUnspendable(p) => p,
						SignBitcoinPayload::TaprootSpendable(p, _) => p,
						SignBitcoinPayload::WithTweaks(p, _) => p,
					};

					info!("Message {:?}", message);

					info!("Verification result: ");
					match verify_single(self.agg_key.unwrap(), signature, message) {
						Ok(_) => info!("OK!"),
						Err(_) => info!("NOK!"),
					};
					self.events.push(CeremonyEnded(signature.to_bytes(), self.aes_key));
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
	pub fn get_aes_key(&self) -> &RequestAesKey {
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
	use crate::{
		CeremonyCommand, CeremonyError, CeremonyEvent, MuSig2Ceremony, NonceReceivingErrorReason,
		SignBitcoinPayload, SignerId, SignersWithKeys,
	};
	use alloc::sync::Arc;
	use itp_sgx_crypto::{key_repository::AccessKey, schnorr::Pair as SchnorrPair};
	use k256::{
		elliptic_curve::PublicKey,
		schnorr::{signature::Keypair, SigningKey},
		sha2::digest::Mac,
	};
	use litentry_primitives::RequestAesKey;
	use musig2::{secp::MaybeScalar, SecNonce};

	pub const MY_SIGNER_ID: SignerId = [0u8; 32];

	fn my_priv_key() -> SigningKey {
		SigningKey::from_bytes(&[
			252, 240, 35, 85, 243, 83, 129, 54, 7, 155, 24, 114, 254, 0, 134, 251, 207, 83, 177, 9,
			92, 118, 222, 5, 202, 239, 188, 215, 132, 113, 127, 94,
		])
		.unwrap()
	}

	fn signer1_priv_key() -> SigningKey {
		SigningKey::from_bytes(&[
			42, 82, 57, 169, 208, 130, 125, 141, 62, 185, 167, 41, 142, 217, 252, 135, 158, 128,
			44, 129, 222, 71, 55, 86, 230, 183, 54, 111, 152, 83, 85, 155,
		])
		.unwrap()
	}

	pub const SIGNER_1_ID: SignerId = [1u8; 32];
	pub const SIGNER_1_SEC_NONCE: [u8; 64] = [
		57, 232, 181, 133, 43, 97, 251, 79, 229, 110, 26, 121, 197, 2, 249, 237, 222, 207, 129,
		232, 8, 227, 120, 202, 127, 61, 209, 41, 92, 54, 8, 91, 80, 31, 9, 126, 14, 137, 126, 143,
		98, 223, 254, 134, 9, 190, 5, 157, 133, 254, 18, 119, 117, 25, 65, 179, 35, 130, 156, 109,
		233, 51, 18, 32,
	];

	pub const SIGNER_2_ID: SignerId = [2u8; 32];

	fn signer2_priv_key() -> SigningKey {
		SigningKey::from_bytes(&[
			117, 130, 176, 36, 185, 53, 187, 61, 123, 86, 24, 38, 174, 143, 129, 73, 245, 210, 127,
			148, 115, 136, 32, 98, 62, 47, 26, 196, 57, 211, 171, 185,
		])
		.unwrap()
	}

	pub const SIGNER_2_SEC_NONCE: [u8; 64] = [
		78, 229, 109, 189, 246, 169, 247, 85, 184, 199, 144, 135, 45, 60, 71, 109, 214, 121, 165,
		206, 185, 246, 120, 52, 228, 49, 155, 9, 160, 129, 171, 252, 69, 160, 122, 66, 151, 147,
		141, 118, 226, 189, 100, 94, 74, 163, 158, 245, 111, 99, 108, 202, 224, 110, 71, 106, 178,
		255, 89, 34, 16, 10, 195, 107,
	];

	fn signers_with_keys() -> SignersWithKeys {
		vec![
			(MY_SIGNER_ID, PublicKey::from(my_priv_key().verifying_key())),
			(SIGNER_1_ID, PublicKey::from(signer1_priv_key().verifying_key())),
			(SIGNER_2_ID, PublicKey::from(signer2_priv_key().verifying_key())),
		]
	}

	fn save_signer1_nonce_cmd() -> CeremonyCommand {
		CeremonyCommand::SaveNonce(
			SIGNER_1_ID,
			SecNonce::from_bytes(&SIGNER_1_SEC_NONCE).unwrap().public_nonce(),
		)
	}

	fn save_signer2_nonce_cmd() -> CeremonyCommand {
		CeremonyCommand::SaveNonce(
			SIGNER_2_ID,
			SecNonce::from_bytes(&SIGNER_2_SEC_NONCE).unwrap().public_nonce(),
		)
	}

	fn save_signer1_partial_sign_cmd() -> CeremonyCommand {
		CeremonyCommand::SavePartialSignature(SIGNER_1_ID, MaybeScalar::Zero)
	}

	fn unknown_signer_nonce_cmd() -> CeremonyCommand {
		CeremonyCommand::SaveNonce(
			[10u8; 32],
			SecNonce::from_bytes(&SIGNER_2_SEC_NONCE).unwrap().public_nonce(),
		)
	}

	pub const SAMPLE_REQUEST_AES_KEY: RequestAesKey = [0u8; 32];
	pub const SAMPLE_SIGNATURE_PAYLOAD: [u8; 32] = [0u8; 32];

	struct MockedSigningKeyAccess {
		signing_key: SigningKey,
	}

	impl AccessKey for MockedSigningKeyAccess {
		type KeyType = SchnorrPair;

		fn retrieve_key(&self) -> itp_sgx_crypto::Result<Self::KeyType> {
			Ok(SchnorrPair::new(self.signing_key.clone()))
		}
	}

	#[test]
	fn it_should_create_ceremony_without_pending_commands() {
		// given
		let signing_key_access = MockedSigningKeyAccess { signing_key: my_priv_key() };

		// when
		let result = MuSig2Ceremony::new(
			MY_SIGNER_ID,
			SAMPLE_REQUEST_AES_KEY.clone(),
			signers_with_keys(),
			SignBitcoinPayload::Derived(SAMPLE_SIGNATURE_PAYLOAD.to_vec()),
			vec![],
			Arc::new(signing_key_access),
			10,
		);

		// then
		assert!(result.is_ok())
	}

	#[test]
	fn it_should_prevent_from_creating_ceremony_without_sufficient_signers() {
		// given
		let signing_key_access = MockedSigningKeyAccess { signing_key: my_priv_key() };

		// when
		let result = MuSig2Ceremony::new(
			MY_SIGNER_ID,
			SAMPLE_REQUEST_AES_KEY.clone(),
			signers_with_keys()[0..1].to_vec(),
			SignBitcoinPayload::Derived(SAMPLE_SIGNATURE_PAYLOAD.to_vec()),
			vec![],
			Arc::new(signing_key_access),
			10,
		);

		// then
		assert!(result.is_err());
	}

	#[test]
	fn it_should_timeout_after_certain_ticks() {
		// given
		let signing_key_access = MockedSigningKeyAccess { signing_key: my_priv_key() };

		let ceremony_id = SignBitcoinPayload::Derived(SAMPLE_SIGNATURE_PAYLOAD.to_vec());
		let mut ceremony = MuSig2Ceremony::new(
			MY_SIGNER_ID,
			SAMPLE_REQUEST_AES_KEY.clone(),
			signers_with_keys(),
			ceremony_id.clone(),
			vec![],
			Arc::new(signing_key_access),
			10,
		)
		.unwrap();

		// when
		(0..9).for_each(|_| {
			ceremony.tick();
		});
		let events = ceremony.tick();

		// then
		assert!(matches!(events.get(0), Some(CeremonyEvent::CeremonyTimedOut(_, _))));
		assert_eq!(events.len(), 1)
	}

	#[test]
	fn newly_created_ceremony_without_commands_should_produce_first_round_started_event_after_tick()
	{
		// given
		let signing_key_access = MockedSigningKeyAccess { signing_key: my_priv_key() };
		let ceremony_id = SignBitcoinPayload::Derived(SAMPLE_SIGNATURE_PAYLOAD.to_vec());
		let mut ceremony = MuSig2Ceremony::new(
			MY_SIGNER_ID,
			SAMPLE_REQUEST_AES_KEY.clone(),
			signers_with_keys(),
			ceremony_id.clone(),
			vec![],
			Arc::new(signing_key_access),
			10,
		)
		.unwrap();

		// when
		let events = ceremony.tick();

		assert!(
			matches!(events.get(0), Some(CeremonyEvent::FirstRoundStarted(ref signers, ref ev_ceremony_id, ref _pub_nonce)) if *signers == vec![SIGNER_1_ID, SIGNER_2_ID] && *ev_ceremony_id == ceremony_id)
		);
		assert_eq!(events.len(), 1)
	}

	#[test]
	fn newly_created_ceremony_with_not_all_nonce_commands_should_produce_only_first_round_started_event_after_tick(
	) {
		// given
		let signing_key_access = MockedSigningKeyAccess { signing_key: my_priv_key() };
		let ceremony_id = SignBitcoinPayload::Derived(SAMPLE_SIGNATURE_PAYLOAD.to_vec());
		let mut ceremony = MuSig2Ceremony::new(
			MY_SIGNER_ID,
			SAMPLE_REQUEST_AES_KEY.clone(),
			signers_with_keys(),
			ceremony_id.clone(),
			vec![save_signer1_nonce_cmd()],
			Arc::new(signing_key_access),
			10,
		)
		.unwrap();

		// when
		let events = ceremony.tick();

		assert!(
			matches!(events.get(0), Some(CeremonyEvent::FirstRoundStarted(ref signers, ref ev_ceremony_id, ref _pub_nonce)) if *signers == vec![SIGNER_1_ID, SIGNER_2_ID] && *ev_ceremony_id == ceremony_id)
		);
		assert_eq!(events.len(), 1)
	}

	#[test]
	fn newly_created_ceremony_with_all_nonce_commands_should_produce_first_and_second_round_started_events_after_tick(
	) {
		// given
		let signing_key_access = MockedSigningKeyAccess { signing_key: my_priv_key() };
		let ceremony_id = SignBitcoinPayload::Derived(SAMPLE_SIGNATURE_PAYLOAD.to_vec());
		let mut ceremony = MuSig2Ceremony::new(
			MY_SIGNER_ID,
			SAMPLE_REQUEST_AES_KEY.clone(),
			signers_with_keys(),
			ceremony_id.clone(),
			vec![
				save_signer1_partial_sign_cmd(),
				save_signer1_nonce_cmd(),
				save_signer2_nonce_cmd(),
			],
			Arc::new(signing_key_access),
			10,
		)
		.unwrap();

		// when
		let events = ceremony.tick();

		assert!(
			matches!(events.get(0), Some(CeremonyEvent::FirstRoundStarted(ref signers, ref ev_ceremony_id, ref _pub_nonce)) if *signers == vec![SIGNER_1_ID, SIGNER_2_ID] && *ev_ceremony_id == ceremony_id)
		);
		assert!(
			matches!(events.get(1), Some(CeremonyEvent::SecondRoundStarted(ref signers, ref ev_ceremony_id, ref _partial_signature)) if *signers == vec![SIGNER_1_ID, SIGNER_2_ID] && *ev_ceremony_id == ceremony_id)
		);
		assert_eq!(events.len(), 2)
	}

	#[test]
	fn newly_created_ceremony_with_unknown_signer_command_should_produce_first_round_started_event_and_error_event_after_tick(
	) {
		// given
		let signing_key_access = MockedSigningKeyAccess { signing_key: my_priv_key() };
		let ceremony_id = SignBitcoinPayload::Derived(SAMPLE_SIGNATURE_PAYLOAD.to_vec());
		let mut ceremony = MuSig2Ceremony::new(
			MY_SIGNER_ID,
			SAMPLE_REQUEST_AES_KEY.clone(),
			signers_with_keys(),
			ceremony_id.clone(),
			vec![unknown_signer_nonce_cmd()],
			Arc::new(signing_key_access),
			10,
		)
		.unwrap();

		// when
		let events = ceremony.tick();

		assert!(
			matches!(events.get(0), Some(CeremonyEvent::FirstRoundStarted(ref signers, ref ev_ceremony_id, ref _pub_nonce)) if *signers == vec![SIGNER_1_ID, SIGNER_2_ID] && *ev_ceremony_id == ceremony_id)
		);
		assert!(matches!(
			events.get(1),
			Some(CeremonyEvent::CeremonyError(
				_,
				CeremonyError::NonceReceivingError(NonceReceivingErrorReason::SignerNotFound),
				_
			))
		));
		assert_eq!(events.len(), 2)
	}
}

#[cfg(feature = "sgx-test")]
pub mod sgx_tests {
	use super::*;
	use crate::{
		generate_aggregated_public_key, CeremonyEvent, MuSig2Ceremony, SignBitcoinPayload,
	};
	use alloc::sync::Arc;
	use k256::schnorr::SigningKey;
	use musig2::verify_single;

	pub const MY_SIGNER_ID: SignerId = [0u8; 32];
	pub const SIGNER_1_ID: SignerId = [1u8; 32];
	pub const SIGNER_2_ID: SignerId = [2u8; 32];
	pub const SAMPLE_REQUEST_AES_KEY: RequestAesKey = [0u8; 32];
	pub const SAMPLE_SIGNATURE_PAYLOAD: [u8; 32] = [0u8; 32];

	struct MockedSigningKeyAccess {
		pub signing_key: SigningKey,
	}

	impl AccessKey for MockedSigningKeyAccess {
		type KeyType = SchnorrPair;

		fn retrieve_key(&self) -> itp_sgx_crypto::Result<Self::KeyType> {
			Ok(SchnorrPair::new(self.signing_key.clone()))
		}
	}

	pub fn test_full_flow_with_3_ceremonies() {
		// given
		let ceremony_id = SignBitcoinPayload::Derived(SAMPLE_SIGNATURE_PAYLOAD.to_vec());
		//my signer
		let my_signer_key_access = MockedSigningKeyAccess { signing_key: my_priv_key() };
		let mut my_ceremony = MuSig2Ceremony::new(
			MY_SIGNER_ID,
			SAMPLE_REQUEST_AES_KEY.clone(),
			signers_with_keys(),
			ceremony_id.clone(),
			vec![],
			Arc::new(my_signer_key_access),
			10,
		)
		.unwrap();
		// signer 1
		let signer1_key_access = MockedSigningKeyAccess { signing_key: signer1_priv_key() };
		let mut signer1_ceremony = MuSig2Ceremony::new(
			SIGNER_1_ID,
			SAMPLE_REQUEST_AES_KEY.clone(),
			signers_with_keys(),
			ceremony_id.clone(),
			vec![],
			Arc::new(signer1_key_access),
			10,
		)
		.unwrap();
		// signer 2
		let signer2_key_access = MockedSigningKeyAccess { signing_key: signer2_priv_key() };
		let mut signer2_ceremony = MuSig2Ceremony::new(
			SIGNER_2_ID,
			SAMPLE_REQUEST_AES_KEY.clone(),
			signers_with_keys(),
			ceremony_id.clone(),
			vec![],
			Arc::new(signer2_key_access),
			10,
		)
		.unwrap();

		// when

		// first tick (performs first round in this case)
		let mut my_ceremony_events = my_ceremony.tick();
		let mut signer1_ceremony_events = signer1_ceremony.tick();
		let mut signer2_ceremony_events = signer2_ceremony.tick();

		let my_ceremony_first_round_started_ev = my_ceremony_events.get(0).unwrap();
		let signer1_ceremony_first_round_started_ev = signer1_ceremony_events.get(0).unwrap();
		let signer2_ceremony_first_round_started_ev = signer2_ceremony_events.get(0).unwrap();

		match my_ceremony_first_round_started_ev {
			CeremonyEvent::FirstRoundStarted(_, _, nonce) => {
				signer1_ceremony.receive_nonce(MY_SIGNER_ID, nonce.clone()).unwrap();
				signer2_ceremony.receive_nonce(MY_SIGNER_ID, nonce.clone()).unwrap();
			},
			_ => {},
		}

		match signer1_ceremony_first_round_started_ev {
			CeremonyEvent::FirstRoundStarted(_, _, nonce) => {
				my_ceremony.receive_nonce(SIGNER_1_ID, nonce.clone()).unwrap();
				signer2_ceremony.receive_nonce(SIGNER_1_ID, nonce.clone()).unwrap();
			},
			_ => {},
		}

		match signer2_ceremony_first_round_started_ev {
			CeremonyEvent::FirstRoundStarted(_, _, nonce) => {
				my_ceremony.receive_nonce(SIGNER_2_ID, nonce.clone()).unwrap();
				signer1_ceremony.receive_nonce(SIGNER_2_ID, nonce.clone()).unwrap();
			},
			_ => {},
		}

		// second tick (performs second round in this case)
		my_ceremony_events = my_ceremony.tick();
		signer1_ceremony_events = signer1_ceremony.tick();
		signer2_ceremony_events = signer2_ceremony.tick();

		let my_ceremony_second_round_started_ev = my_ceremony_events.get(0).unwrap();
		let signer1_ceremony_second_round_started_ev = signer1_ceremony_events.get(0).unwrap();
		let signer2_ceremony_second_round_started_ev = signer2_ceremony_events.get(0).unwrap();

		match my_ceremony_second_round_started_ev {
			CeremonyEvent::SecondRoundStarted(_, _, partial_sign) => {
				signer1_ceremony
					.receive_partial_sign(MY_SIGNER_ID, partial_sign.clone())
					.unwrap();
				signer2_ceremony
					.receive_partial_sign(MY_SIGNER_ID, partial_sign.clone())
					.unwrap();
			},
			_ => {},
		}

		match signer1_ceremony_second_round_started_ev {
			CeremonyEvent::SecondRoundStarted(_, _, partial_sign) => {
				my_ceremony.receive_partial_sign(SIGNER_1_ID, partial_sign.clone()).unwrap();
				signer2_ceremony
					.receive_partial_sign(SIGNER_1_ID, partial_sign.clone())
					.unwrap();
			},
			_ => {},
		}

		match signer2_ceremony_second_round_started_ev {
			CeremonyEvent::SecondRoundStarted(_, _, partial_sign) => {
				my_ceremony.receive_partial_sign(SIGNER_2_ID, partial_sign.clone()).unwrap();
				signer1_ceremony
					.receive_partial_sign(SIGNER_2_ID, partial_sign.clone())
					.unwrap();
			},
			_ => {},
		}

		// third tick (finalizes ceremony and produces final signature in this case)
		my_ceremony_events = my_ceremony.tick();
		signer1_ceremony_events = signer1_ceremony.tick();
		signer2_ceremony_events = signer2_ceremony.tick();

		let my_ceremony_ceremony_ended_ev = my_ceremony_events.get(0).unwrap();
		let signer1_ceremony_ceremony_ended_ev = signer1_ceremony_events.get(0).unwrap();
		let signer2_ceremony_ceremony_ended_ev = signer2_ceremony_events.get(0).unwrap();

		let my_ceremony_final_signature = match my_ceremony_ceremony_ended_ev {
			CeremonyEvent::CeremonyEnded(signature, _) => signature.clone(),
			_ => {
				panic!("Ceremony should be ended")
			},
		};

		let signer1_ceremony_final_signature = match signer1_ceremony_ceremony_ended_ev {
			CeremonyEvent::CeremonyEnded(signature, _) => signature.clone(),
			_ => {
				panic!("Ceremony should be ended")
			},
		};

		let signer2_ceremony_final_signature = match signer2_ceremony_ceremony_ended_ev {
			CeremonyEvent::CeremonyEnded(signature, _) => signature.clone(),
			_ => {
				panic!("Ceremony should be ended")
			},
		};

		assert_eq!(my_ceremony_final_signature, signer1_ceremony_final_signature);
		assert_eq!(my_ceremony_final_signature, signer2_ceremony_final_signature);

		// let signature =
		// 	k256::schnorr::Signature::try_from(signer1_ceremony_final_signature.as_slice())
		// 		.unwrap();
		let agg_key =
			generate_aggregated_public_key(signers_with_keys().iter().map(|sk| sk.1).collect());
		// let ver_key = k256::schnorr::VerifyingKey::try_from(agg_key).unwrap();

		// this pass
		verify_single(agg_key, signer1_ceremony_final_signature, SAMPLE_SIGNATURE_PAYLOAD).unwrap();

		// this not pass
		// ver_key.verify(&SAMPLE_SIGNATURE_PAYLOAD, &signature).unwrap()
	}

	fn signers_with_keys() -> SignersWithKeys {
		vec![
			(MY_SIGNER_ID, k256::elliptic_curve::PublicKey::from(my_priv_key().verifying_key())),
			(
				SIGNER_1_ID,
				k256::elliptic_curve::PublicKey::from(signer1_priv_key().verifying_key()),
			),
			(
				SIGNER_2_ID,
				k256::elliptic_curve::PublicKey::from(signer2_priv_key().verifying_key()),
			),
		]
	}

	fn my_priv_key() -> SigningKey {
		SigningKey::from_bytes(&[
			252, 240, 35, 85, 243, 83, 129, 54, 7, 155, 24, 114, 254, 0, 134, 251, 207, 83, 177, 9,
			92, 118, 222, 5, 202, 239, 188, 215, 132, 113, 127, 94,
		])
		.unwrap()
	}

	fn signer1_priv_key() -> SigningKey {
		SigningKey::from_bytes(&[
			42, 82, 57, 169, 208, 130, 125, 141, 62, 185, 167, 41, 142, 217, 252, 135, 158, 128,
			44, 129, 222, 71, 55, 86, 230, 183, 54, 111, 152, 83, 85, 155,
		])
		.unwrap()
	}

	fn signer2_priv_key() -> SigningKey {
		SigningKey::from_bytes(&[
			117, 130, 176, 36, 185, 53, 187, 61, 123, 86, 24, 38, 174, 143, 129, 73, 245, 210, 127,
			148, 115, 136, 32, 98, 62, 47, 26, 196, 57, 211, 171, 185,
		])
		.unwrap()
	}
}
