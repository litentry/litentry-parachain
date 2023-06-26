// Copyright 2020-2023 Litentry Technologies GmbH.
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
	ensure, get_expected_raw_message, get_expected_wrapped_message, AccountId, Error, Result,
	ToString,
};
use itp_utils::stringify::account_id_to_string;
use litentry_primitives::{
	ChallengeCode, ErrorDetail, Identity, IdentityMultiSignature, Web3CommonValidationData,
	Web3ValidationData,
};
use log::*;
use sp_core::{ed25519, sr25519};
use sp_io::{
	crypto::{
		ed25519_verify, secp256k1_ecdsa_recover, secp256k1_ecdsa_recover_compressed, sr25519_verify,
	},
	hashing::{blake2_256, keccak_256},
};

pub fn verify(
	who: &AccountId,
	identity: &Identity,
	code: &ChallengeCode,
	data: &Web3ValidationData,
) -> Result<()> {
	debug!("verify web3 identity, who: {}", account_id_to_string(&who));
	match data {
		Web3ValidationData::Substrate(substrate_validation_data) =>
			verify_substrate_signature(who, identity, code, substrate_validation_data),
		Web3ValidationData::Evm(evm_validation_data) =>
			verify_evm_signature(who, identity, code, evm_validation_data),
	}
}

fn verify_substrate_signature(
	who: &AccountId,
	identity: &Identity,
	code: &ChallengeCode,
	validation_data: &Web3CommonValidationData,
) -> Result<()> {
	let raw_msg = get_expected_raw_message(who, identity, code);
	let wrapped_msg = get_expected_wrapped_message(raw_msg.clone());

	ensure!(
		raw_msg.as_slice() == validation_data.message.as_slice(),
		Error::VerifyIdentityFailed(ErrorDetail::UnexpectedMessage)
	);
	let substrate_address = if let Identity::Substrate { address, .. } = identity {
		address.as_ref()
	} else {
		return Err(Error::VerifyIdentityFailed(ErrorDetail::InvalidIdentity))
	};

	// we accept both the raw_msg's signature and the wrapped_msg's signature
	ensure!(
		verify_substrate_signature_internal(
			&raw_msg,
			&validation_data.signature,
			substrate_address
		) || verify_substrate_signature_internal(
			&wrapped_msg,
			&validation_data.signature,
			substrate_address
		),
		Error::VerifyIdentityFailed(ErrorDetail::VerifySubstrateSignatureFailed)
	);
	Ok(())
}

fn verify_substrate_signature_internal(
	msg: &[u8],
	signature: &IdentityMultiSignature,
	address: &[u8; 32],
) -> bool {
	match signature {
		IdentityMultiSignature::Sr25519(sig) =>
			sr25519_verify(sig, msg, &sr25519::Public(*address)),
		IdentityMultiSignature::Ed25519(sig) =>
			ed25519_verify(sig, msg, &ed25519::Public(*address)),
		// We can' use `ecdsa_verify` directly we don't have the raw 33-bytes publick key
		// instead we only have AccountId which is blake2_256(pubkey)
		IdentityMultiSignature::Ecdsa(sig) => {
			// see https://github.com/paritytech/substrate/blob/493b58bd4a475080d428ce47193ee9ea9757a808/primitives/runtime/src/traits.rs#L132
			let digest = blake2_256(msg);
			if let Ok(recovered_substrate_pubkey) =
				secp256k1_ecdsa_recover_compressed(&sig.0, &digest)
			{
				&blake2_256(&recovered_substrate_pubkey) == address
			} else {
				false
			}
		},
		_ => false,
	}
}

fn verify_evm_signature(
	who: &AccountId,
	identity: &Identity,
	code: &ChallengeCode,
	validation_data: &Web3CommonValidationData,
) -> Result<()> {
	let msg = get_expected_raw_message(who, identity, code);
	let digest = compute_evm_msg_digest(&msg);
	let evm_address = if let Identity::Evm { address, .. } = identity {
		address
	} else {
		return Err(Error::VerifyIdentityFailed(ErrorDetail::InvalidIdentity))
	};
	if let IdentityMultiSignature::Ethereum(sig) = &validation_data.signature {
		let recovered_evm_address = recover_evm_address(&digest, sig.as_ref())
			.map_err(|_| Error::VerifyIdentityFailed(ErrorDetail::RecoverEvmAddressFailed))?;
		ensure!(
			&recovered_evm_address == evm_address.as_ref(),
			Error::VerifyIdentityFailed(ErrorDetail::VerifyEvmSignatureFailed)
		);
	} else {
		return Err(Error::VerifyIdentityFailed(ErrorDetail::WrongSignatureType))
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
