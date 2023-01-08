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

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{traits::ConstU32, BoundedVec};

pub type MaxStringLength = ConstU32<100>;
pub type ErrorString = BoundedVec<u8, MaxStringLength>;

// Identity Management Pallet Error
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum IMPError {
	// UTF8Error,
	DecodeHexFailed(ErrorString),

	HttpRequestFailed(ErrorString),

	// identity verification errors
	InvalidIdentity,
	WrongWeb2Handle,
	UnexpectedMessage,
	WrongIdentityHandleType,
	WrongSignatureType,
	VerifySubstrateSignatureFailed,
	RecoverSubstratePubkeyFailed,
	VerifyEvmSignatureFailed,
	RecoverEvmAddressFailed,
}

impl frame_support::traits::PalletError for IMPError {
	// max_encoded_len
	const MAX_ENCODED_SIZE: usize = 1;
}

// Verified Credential(VC) Management Pallet Error
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum VCMPError {
	HttpRequestFailed(ErrorString),
	// Assertion
	Assertion1Failed,
	Assertion2Failed,
	Assertion3Failed,
	Assertion4Failed,
	Assertion5Failed,
	Assertion7Failed,
}
