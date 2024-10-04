use crate::{Hash, Identity, Vec};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::blake2_256;
use sp_runtime::{BoundedVec, RuntimeDebug};

#[derive(Encode, Decode, TypeInfo, Clone, PartialEq, Eq, RuntimeDebug)]
pub struct IDGraphMember {
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

pub trait IDGraphHash {
    fn hash(&self) -> Hash;
}

impl<T> IDGraphHash for BoundedVec<IDGraphMember, T> {
    fn hash(&self) -> Hash {
        let members_hashes: Vec<Hash> = self.iter().map(|member| member.hash).collect();
        Hash::from(blake2_256(&members_hashes.encode()))
    }
}

impl IDGraphHash for Vec<IDGraphMember> {
    fn hash(&self) -> Hash {
        let members_hashes: Vec<Hash> = self.iter().map(|member| member.hash).collect();
        Hash::from(blake2_256(&members_hashes.encode()))
    }
}
