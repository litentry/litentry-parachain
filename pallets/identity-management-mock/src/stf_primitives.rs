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

// The STF primitives for mocking, reference: app-libs/stf/src/lib.rs in tee-worker
//
// I stripped off the signing fn from KeyPair as the pallet itself
// only uses the SR25519 keys for verifying signatures, not signing.
// sign would require some system randomness if I understand it correctly, plus
// exposing the `full_crypto` feature from sp-core, which seems to cause linking/runtime issues
//
// some read: https://github.com/paritytech/substrate/issues/12009#issuecomment-1213006641

use crate::{Did, Metadata, UserShieldingKey};
use codec::{Decode, Encode, MaxEncodedLen};
use primitives::{BlockNumber, Index};
use sp_core::{crypto::AccountId32, ed25519, sr25519, H256};
use sp_runtime::{traits::Verify, MultiSignature};
use sp_std::prelude::*;

pub type Signature = MultiSignature;
pub type AuthorityId = <Signature as Verify>::Signer;
pub type AccountId = AccountId32;
pub type ShardIdentifier = H256;

#[derive(Clone)]
pub enum KeyPair {
	Sr25519(sr25519::Public),
	Ed25519(ed25519::Public),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, scale_info::TypeInfo)]
pub struct TrustedCallSigned {
	pub call: TrustedCall,
	pub nonce: Index,
	pub signature: Signature,
}

impl TrustedCallSigned {
	pub fn new(call: TrustedCall, nonce: Index, signature: Signature) -> Self {
		TrustedCallSigned { call, nonce, signature }
	}

	pub fn verify_signature(&self, mrenclave: &[u8; 32], shard: &ShardIdentifier) -> bool {
		let mut payload = self.call.encode();
		payload.append(&mut self.nonce.encode());
		payload.append(&mut mrenclave.encode());
		payload.append(&mut shard.encode());
		self.signature.verify(payload.as_slice(), self.call.account())
	}

	pub fn into_trusted_operation(self, direct: bool) -> TrustedOperation {
		match direct {
			true => TrustedOperation::direct_call(self),
			false => TrustedOperation::indirect_call(self),
		}
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, scale_info::TypeInfo)]
#[allow(non_camel_case_types)]
pub enum TrustedOperation {
	indirect_call(TrustedCallSigned),
	direct_call(TrustedCallSigned),
	get(Getter),
}

impl From<TrustedCallSigned> for TrustedOperation {
	fn from(item: TrustedCallSigned) -> Self {
		TrustedOperation::indirect_call(item)
	}
}

impl From<Getter> for TrustedOperation {
	fn from(item: Getter) -> Self {
		TrustedOperation::get(item)
	}
}

impl From<TrustedGetterSigned> for TrustedOperation {
	fn from(item: TrustedGetterSigned) -> Self {
		TrustedOperation::get(item.into())
	}
}

impl From<PublicGetter> for TrustedOperation {
	fn from(item: PublicGetter) -> Self {
		TrustedOperation::get(item.into())
	}
}

impl TrustedOperation {
	pub fn to_call(&self) -> Option<&TrustedCallSigned> {
		match self {
			TrustedOperation::direct_call(c) => Some(c),
			TrustedOperation::indirect_call(c) => Some(c),
			_ => None,
		}
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, scale_info::TypeInfo)]
#[allow(non_camel_case_types)]
pub enum Getter {
	public(PublicGetter),
	trusted(TrustedGetterSigned),
}

impl From<PublicGetter> for Getter {
	fn from(item: PublicGetter) -> Self {
		Getter::public(item)
	}
}

impl From<TrustedGetterSigned> for Getter {
	fn from(item: TrustedGetterSigned) -> Self {
		Getter::trusted(item)
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, scale_info::TypeInfo)]
#[allow(non_camel_case_types)]
pub enum PublicGetter {
	some_value,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, scale_info::TypeInfo)]
#[allow(non_camel_case_types)]
pub enum TrustedCall {
	set_shielding_key(AccountId, AccountId, UserShieldingKey),
	link_identity(AccountId, AccountId, Did, Option<Metadata>, BlockNumber),
	unlink_identity(AccountId, AccountId, Did),
	verify_identity(AccountId, AccountId, Did, BlockNumber),
}

impl TrustedCall {
	pub fn account(&self) -> &AccountId {
		match self {
			TrustedCall::set_shielding_key(account, _, _) => account,
			TrustedCall::link_identity(account, _, _, _, _) => account,
			TrustedCall::unlink_identity(account, _, _) => account,
			TrustedCall::verify_identity(account, _, _, _) => account,
		}
	}

	pub fn sign(
		&self,
		_pair: &KeyPair,
		_nonce: Index,
		_mrenclave: &[u8; 32],
		_shard: &ShardIdentifier,
	) -> TrustedCallSigned {
		todo!()
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, scale_info::TypeInfo)]
#[allow(non_camel_case_types)]
pub enum TrustedGetter {
	shielding_key(AccountId),
}

impl TrustedGetter {
	pub fn account(&self) -> &AccountId {
		match self {
			TrustedGetter::shielding_key(account) => account,
		}
	}

	pub fn sign(&self, _pair: &KeyPair) -> TrustedGetterSigned {
		todo!()
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, scale_info::TypeInfo)]
pub struct TrustedGetterSigned {
	pub getter: TrustedGetter,
	pub signature: Signature,
}

impl TrustedGetterSigned {
	pub fn new(getter: TrustedGetter, signature: Signature) -> Self {
		TrustedGetterSigned { getter, signature }
	}

	pub fn verify_signature(&self) -> bool {
		self.signature.verify(self.getter.encode().as_slice(), self.getter.account())
	}
}
