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

use codec::{Decode, Encode, Error, Input};
use sp_runtime::OpaqueExtrinsic;
use sp_std::vec::Vec;
use substrate_api_client::{GenericSignedExtra, PlainTip, UncheckedExtrinsicV4};

use crate::parentchain::{Address, Balance, Index, Signature as PairSignature};

pub type ParentchainPlainTip = PlainTip<Balance>;
pub type ParentchainSignedExtra = GenericSignedExtra<ParentchainPlainTip, Index>;

/// Same function as in primitives::generic. Needed to be copied as it is private there.
fn encode_with_vec_prefix<T: Encode, F: Fn(&mut Vec<u8>)>(encoder: F) -> Vec<u8> {
	let size = sp_std::mem::size_of::<T>();
	let reserve = match size {
		0..=0b0011_1111 => 1,
		0b0100_0000..=0b0011_1111_1111_1111 => 2,
		_ => 4,
	};
	let mut v = Vec::with_capacity(reserve + size);
	v.resize(reserve, 0);
	encoder(&mut v);

	// need to prefix with the total length to ensure it's binary compatible with
	// Vec<u8>.
	let mut length: Vec<()> = Vec::new();
	length.resize(v.len() - reserve, ());
	length.using_encoded(|s| {
		v.splice(0..reserve, s.iter().cloned());
	});

	v
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct OpaqueExtrinsicWithStatus {
	pub xt: OpaqueExtrinsic,
	pub status: bool,
}

impl Encode for OpaqueExtrinsicWithStatus {
	fn encode(&self) -> Vec<u8> {
		encode_with_vec_prefix::<Self, _>(|v| {
			self.xt.encode_to(v);
			self.status.encode_to(v);
		})
	}
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ParentchainUncheckedExtrinsicWithStatus<Call> {
	pub xt: UncheckedExtrinsicV4<Address, Call, PairSignature, ParentchainSignedExtra>,
	pub status: bool,
}

impl<Call> Decode for ParentchainUncheckedExtrinsicWithStatus<Call>
where
	UncheckedExtrinsicV4<Address, Call, PairSignature, ParentchainSignedExtra>: Decode,
{
	fn decode<I: Input>(input: &mut I) -> Result<Self, Error> {
		// This is a little more complicated than usual since the binary format must be compatible
		// with substrate's generic `Vec<u8>` type. Basically this just means accepting that there
		// will be a prefix of vector length (we don't need
		// to use this).
		let _length_do_not_remove_me_see_above: Vec<()> = Decode::decode(input)?;

		Ok(ParentchainUncheckedExtrinsicWithStatus::<Call> {
			xt: Decode::decode(input)?,
			status: Decode::decode(input)?,
		})
	}
}
