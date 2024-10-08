use crate::{Hash, Identity};
use alloc::vec::Vec;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core_hashing::blake2_256;
use sp_runtime::{BoundedVec, RuntimeDebug};

#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct MemberAccount {
	pub id: MemberIdentity,
	pub hash: Hash,
}

#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, RuntimeDebug)]
pub enum MemberIdentity {
	Public(Identity),
	Private(Vec<u8>),
}

impl MemberIdentity {
	pub fn is_public(&self) -> bool {
		matches!(self, Self::Public(..))
	}
}

impl From<Identity> for MemberIdentity {
	fn from(identity: Identity) -> Self {
		Self::Public(identity)
	}
}

pub trait GetAccountStoreHash {
	fn hash(&self) -> Hash;
}

impl<T> GetAccountStoreHash for BoundedVec<MemberAccount, T> {
	fn hash(&self) -> Hash {
		let hashes: Vec<Hash> = self.iter().map(|member| member.hash).collect();
		hashes.using_encoded(blake2_256).into()
	}
}

impl GetAccountStoreHash for Vec<MemberAccount> {
	fn hash(&self) -> Hash {
		let hashes: Vec<Hash> = self.iter().map(|member| member.hash).collect();
		hashes.using_encoded(blake2_256).into()
	}
}
