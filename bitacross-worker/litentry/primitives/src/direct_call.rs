// Copyright 2020-2023 Trust Computing GmbH.
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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::{blake2_256, ShardIdentifier};
use codec::{Decode, Encode};
use itp_stf_primitives::types::KeyPair;
use sp_core::crypto::AccountId32;
use sp_runtime::{traits::Verify, MultiSignature};
use std::vec::Vec;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct DirectCallSigned {
	pub call: DirectCall,
	pub signature: MultiSignature,
}

impl DirectCallSigned {
	pub fn verify_signature(&self, mrenclave: &[u8; 32], shard: &ShardIdentifier) -> bool {
		let mut payload = self.call.encode();
		payload.append(&mut mrenclave.encode());
		payload.append(&mut shard.encode());

		// make it backwards compatible for now - will deprecate the old way later
		self.signature.verify(blake2_256(&payload).as_slice(), self.call.signer())
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub enum DirectCall {
	SignBitcoin(AccountId32, Vec<u8>),
	SignEthereum(AccountId32, Vec<u8>),
}

impl DirectCall {
	pub fn signer(&self) -> &AccountId32 {
		match self {
			Self::SignBitcoin(signer, ..) => signer,
			Self::SignEthereum(signer, ..) => signer,
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
