// Copyright 2020-2022 Litentry Technologies GmbH.
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

use crate::{
	ensure,
	error::{Error, Result},
	get_expected_payload, AccountId, ToString,
};
use litentry_primitives::{
	ChallengeCode, Identity, IdentityHandle, IdentityMultiSignature, IdentityWebType,
	Web3CommonValidationData, Web3Network, Web3ValidationData,
};
use sp_core::{ed25519, sr25519};
use sp_io::{
	crypto::{
		ed25519_verify, secp256k1_ecdsa_recover, secp256k1_ecdsa_recover_compressed, sr25519_verify,
	},
	hashing::{blake2_256, keccak_256},
};

pub fn verify(
	who: AccountId,
	identity: Identity,
	code: ChallengeCode,
	web3: Web3ValidationData,
) -> Result<()> {
	match web3 {
		Web3ValidationData::Substrate(substrate_validation_data) =>
			verify_substrate_signature(&who, &identity, &code, &substrate_validation_data),
		Web3ValidationData::Evm(evm_validation_data) =>
			verify_evm_signature(&who, &identity, &code, &evm_validation_data),
	}
}

fn verify_substrate_signature(
	who: &AccountId,
	identity: &Identity,
	code: &ChallengeCode,
	validation_data: &Web3CommonValidationData,
) -> Result<()> {
	let msg = get_expected_payload(who, identity, code);

	ensure!(msg.as_slice() == validation_data.message.as_slice(), Error::UnexpectedMessage);

	let substrate_address = match &identity.web_type {
		IdentityWebType::Web3(Web3Network::Substrate(_)) => match &identity.handle {
			IdentityHandle::Address32(handle) => handle,
			_ => return Err(Error::WrongIdentityHanldeType),
		},
		_ => return Err(Error::WrongWeb3NetworkType),
	};

	match &validation_data.signature {
		IdentityMultiSignature::Sr25519(sig) => {
			ensure!(
				sr25519_verify(sig, &msg, &sr25519::Public(*substrate_address)),
				Error::VerifySubstrateSignatureFailed
			);
		},
		IdentityMultiSignature::Ed25519(sig) => {
			ensure!(
				ed25519_verify(sig, &msg, &ed25519::Public(*substrate_address)),
				Error::VerifySubstrateSignatureFailed
			);
		},
		// We can' use `ecdsa_verify` directly we don't have the raw 33-bytes publick key
		// instead we only have AccountId which is blake2_256(pubkey)
		IdentityMultiSignature::Ecdsa(sig) => {
			// see https://github.com/paritytech/substrate/blob/493b58bd4a475080d428ce47193ee9ea9757a808/primitives/runtime/src/traits.rs#L132
			let digest = blake2_256(&msg);
			let recovered_substrate_pubkey = secp256k1_ecdsa_recover_compressed(&sig.0, &digest)
				.map_err(|_| Error::RecoverSubstratePubkeyFailed)?;
			ensure!(
				&blake2_256(&recovered_substrate_pubkey) == substrate_address,
				Error::VerifySubstrateSignatureFailed
			);
		},
		_ => return Err(Error::WrongSignatureType),
	}
	Ok(())
}

fn verify_evm_signature(
	who: &AccountId,
	identity: &Identity,
	code: &ChallengeCode,
	validation_data: &Web3CommonValidationData,
) -> Result<()> {
	let msg = get_expected_payload(who, identity, code);
	let digest = compute_evm_msg_digest(&msg);
	if let IdentityMultiSignature::Ethereum(sig) = &validation_data.signature {
		let recovered_evm_address = recover_evm_address(&digest, sig.as_ref())
			.map_err(|_| Error::RecoverEvmAddressFailed)?;
		let evm_address = match &identity.web_type {
			IdentityWebType::Web3(Web3Network::Evm(_)) => match &identity.handle {
				IdentityHandle::Address20(handle) => handle,
				_ => return Err(Error::WrongIdentityHanldeType),
			},
			_ => return Err(Error::WrongWeb3NetworkType),
		};
		ensure!(&recovered_evm_address == evm_address, Error::VerifyEvmSignatureFailed);
	} else {
		return Err(Error::WrongSignatureType)
	}
	Ok(())
}

// we use an EIP-191 message has computing
fn compute_evm_msg_digest(message: &[u8]) -> [u8; 32] {
	let eip_191_message = [
		"\x19Ethereum Signed Message:\n".as_bytes(),
		message.len().to_string().as_bytes(),
		message,
	]
	.concat();
	keccak_256(&eip_191_message)
}

fn recover_evm_address(
	msg: &[u8; 32],
	sig: &[u8; 65],
) -> core::result::Result<[u8; 20], sp_io::EcdsaVerifyError> {
	let pubkey = secp256k1_ecdsa_recover(sig, msg)?;
	let hashed_pk = keccak_256(&pubkey);

	let mut addr = [0u8; 20];
	addr[..20].copy_from_slice(&hashed_pk[12..32]);
	Ok(addr)
}
