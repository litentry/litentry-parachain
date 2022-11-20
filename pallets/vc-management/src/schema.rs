use crate::{vc_context::Status, Config};
use codec::{Decode, Encode, MaxEncodedLen};
use sp_runtime::{traits::ConstU32, BoundedVec};
use scale_info::TypeInfo;

type MaxStringLength = ConstU32<64>;
pub type ContentString = BoundedVec<u8, MaxStringLength>;

#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, TypeInfo,MaxEncodedLen)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct VCSchema<T: Config> {
	pub id_str: ContentString,
	// the schema author
	pub author: T::AccountId,
	// schema content
	pub content: ContentString,
	// status of the Schema
	pub status: Status,
}

impl<T: Config> VCSchema<T> {
	pub fn new(sid: Vec<u8>, author: T::AccountId, scontent: Vec<u8>) -> Self {
		let id_str: ContentString = sid.clone().try_into().expect("schema id is too long");
		let content: ContentString =
			scontent.clone().try_into().expect("schema content is too long");

		Self { id_str, author, content, status: Status::Active }
	}
}
