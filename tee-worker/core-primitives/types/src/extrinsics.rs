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

use codec::Encode;
use sp_runtime::OpaqueExtrinsic;
use sp_std::vec::Vec;
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

// #[cfg(test)]
// mod tests {
// 	use crate::extrinsics::OpaqueExtrinsicWithStatus;
// 	use itp_api_client_types::ParentchainUncheckedExtrinsicWithStatus;
// 	use sp_core::{hexdisplay::HexDisplay, Pair, H256 as Hash};
// 	use sp_runtime::{generic::Era, testing::sr25519, MultiSignature, OpaqueExtrinsic};
// 	use substrate_api_client::{
// 		BaseExtrinsicParams, PlainTip, PlainTipExtrinsicParams, PlainTipExtrinsicParamsBuilder,
// 		SubstrateDefaultSignedExtra, UncheckedExtrinsicV4,
// 	};
//
// 	#[test]
// 	fn encode_decode_works() {
// 		let msg = &b"test-message"[..];
// 		let (pair, _) = sr25519::Pair::generate();
// 		let signature = pair.sign(msg);
// 		let multi_sig = MultiSignature::from(signature);
// 		let account: AccountId = pair.public().into();
// 		let tx_params =
// 			PlainTipExtrinsicParamsBuilder::new().era(Era::mortal(8, 0), Hash::from([0u8; 32]));
//
// 		let default_extra = BaseExtrinsicParams::new(0, 0, 2, Hash::from([0u8; 32]), tx_params);
// 		let xt = UncheckedExtrinsicV4::new_signed(
// 			vec![1, 1, 1],
// 			account.into(),
// 			multi_sig,
// 			default_extra.signed_extra(),
// 		);
// 		let unchecked_with_status =
// 			ParentchainUncheckedExtrinsicWithStatus { xt: xt.clone(), status: true };
// 		let op_xt = OpaqueExtrinsic::from_bytes(xt.encode().as_slice()).unwrap();
// 		let op_xt_with_status = OpaqueExtrinsicWithStatus { raw: op_xt, status: true };
// 		assert_eq!(with_status, Decode::decode(&mut op_xt_with_status.encode().as_slice()).unwrap())
// 	}
// }
