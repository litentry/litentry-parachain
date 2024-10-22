use alloc::string::String;
use codec::{Decode, Encode};
use ita_stf::{LitentryMultiSignature, TrustedCall};
use itp_types::parentchain::Index as ParentchainIndex;
use lc_identity_verification::VerificationCodeStore;
use lc_omni_account::InMemoryStore as OmniAccountStore;
use litentry_hex_utils::hex_encode;
use litentry_primitives::{MemberAccount, ShardIdentifier};
use sp_core::{
	blake2_256,
	crypto::{AccountId32, UncheckedFrom},
	ed25519,
};

type VerificationCode = String;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub enum TCAuthentication {
	Web3(LitentryMultiSignature),
	Email(VerificationCode),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct TrustedCallAuthenticated {
	pub call: TrustedCall,
	pub nonce: ParentchainIndex,
	pub authentication: TCAuthentication,
}

impl TrustedCallAuthenticated {
	pub fn new(
		call: TrustedCall,
		nonce: ParentchainIndex,
		authentication: TCAuthentication,
	) -> Self {
		Self { call, nonce, authentication }
	}
}

impl Default for TrustedCallAuthenticated {
	fn default() -> Self {
		Self {
			call: TrustedCall::noop(AccountId32::unchecked_from([0u8; 32].into()).into()),
			nonce: 0,
			authentication: TCAuthentication::Web3(LitentryMultiSignature::Ed25519(
				ed25519::Signature::unchecked_from([0u8; 64]),
			)),
		}
	}
}

pub fn verify_tca_web3_authentication(
	tca: TrustedCallAuthenticated,
	mrenclave: &[u8; 32],
	shard: &ShardIdentifier,
) -> bool {
	match tca.authentication {
		TCAuthentication::Web3(signature) => {
			let mut payload = tca.call.encode();
			payload.append(&mut tca.nonce.encode());
			payload.append(&mut mrenclave.encode());
			payload.append(&mut shard.encode());

			// The signature should be valid in either case:
			// 1. blake2_256(payload)
			// 2. Signature Prefix + blake2_256(payload)

			let hashed = blake2_256(&payload);

			let prettified_msg_hash = tca.call.signature_message_prefix() + &hex_encode(&hashed);
			let prettified_msg_hash = prettified_msg_hash.as_bytes();

			// Most common signatures variants by clients are verified first (4 and 2).
			signature.verify(prettified_msg_hash, tca.call.sender_identity())
				|| signature.verify(&hashed, tca.call.sender_identity())
		},
		TCAuthentication::Email(_) => false,
	}
}

pub fn verify_tca_email_authentication(tca: TrustedCallAuthenticated) -> bool {
	match tca.authentication {
		TCAuthentication::Web3(_) => false,
		TCAuthentication::Email(verification_code) => {
			match OmniAccountStore::get_omni_account(tca.call.sender_identity().hash()) {
				Ok(Some(account_id)) => {
					let identity_hash = tca.call.sender_identity().hash();
					match VerificationCodeStore::get(&account_id, identity_hash) {
						Ok(Some(code)) => code == verification_code,
						_ => false,
					}
				},
				_ => false,
			}
		},
	}
}
