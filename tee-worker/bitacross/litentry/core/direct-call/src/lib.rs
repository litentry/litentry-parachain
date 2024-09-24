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
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use bc_musig2_ceremony::SignBitcoinPayload;
use codec::{Decode, Encode};
use itp_stf_primitives::types::KeyPair;
use litentry_primitives::{Identity, LitentryMultiSignature, ShardIdentifier};
use sp_io::hashing::blake2_256;
use std::vec::Vec;

pub mod handler;

pub type PrehashedEthereumMessage = [u8; 32];

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct DirectCallSigned {
	pub call: DirectCall,
	pub signature: LitentryMultiSignature,
}

impl DirectCallSigned {
	pub fn verify_signature(&self, mrenclave: &[u8; 32], shard: &ShardIdentifier) -> bool {
		let mut payload = self.call.encode();
		payload.append(&mut mrenclave.encode());
		payload.append(&mut shard.encode());

		self.signature.verify(blake2_256(&payload).as_slice(), self.call.signer())
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub enum DirectCall {
	SignBitcoin(Identity, SignBitcoinPayload),
	SignEthereum(Identity, PrehashedEthereumMessage),
	SignTon(Identity, Vec<u8>),
	CheckSignBitcoin(Identity),
}

impl DirectCall {
	pub fn signer(&self) -> &Identity {
		match self {
			Self::SignBitcoin(signer, ..) => signer,
			Self::SignEthereum(signer, ..) => signer,
			Self::SignTon(signer, ..) => signer,
			Self::CheckSignBitcoin(signer) => signer,
		}
	}

	pub fn sign(
		&self,
		pair: &KeyPair,
		mrenclave: &[u8; 32],
		shard: &ShardIdentifier,
	) -> DirectCallSigned {
		let mut payload = self.encode();
		payload.append(&mut mrenclave.encode());
		payload.append(&mut shard.encode());

		DirectCallSigned {
			call: self.clone(),
			signature: pair.sign(blake2_256(&payload).as_slice()),
		}
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct CeremonyRoundCallSigned {
	pub call: CeremonyRoundCall,
	pub signature: LitentryMultiSignature,
}

impl CeremonyRoundCallSigned {
	pub fn verify_signature(&self, mrenclave: &[u8; 32], shard: &ShardIdentifier) -> bool {
		let mut payload = self.call.encode();
		payload.append(&mut mrenclave.encode());
		payload.append(&mut shard.encode());

		self.signature.verify(blake2_256(&payload).as_slice(), self.call.signer())
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub enum CeremonyRoundCall {
	NonceShare(Identity, SignBitcoinPayload, [u8; 66]),
	PartialSignatureShare(Identity, SignBitcoinPayload, [u8; 32]),
	KillCeremony(Identity, SignBitcoinPayload),
}

impl CeremonyRoundCall {
	pub fn signer(&self) -> &Identity {
		match self {
			Self::NonceShare(signer, ..) => signer,
			Self::PartialSignatureShare(signer, ..) => signer,
			Self::KillCeremony(signer, ..) => signer,
		}
	}

	pub fn sign(
		&self,
		pair: &KeyPair,
		mrenclave: &[u8; 32],
		shard: &ShardIdentifier,
	) -> CeremonyRoundCallSigned {
		let mut payload = self.encode();
		payload.append(&mut mrenclave.encode());
		payload.append(&mut shard.encode());

		CeremonyRoundCallSigned {
			call: self.clone(),
			signature: pair.sign(blake2_256(&payload).as_slice()),
		}
	}
}
