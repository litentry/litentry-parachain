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

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{traits::ConstU32, BoundedVec};

pub type MaxStringLength = ConstU32<100>;
pub type ErrorString = BoundedVec<u8, MaxStringLength>;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum ErrorDetail {
	// error when importing the parentchain blocks and executing indirect calls
	ImportError,
	// generic error when executing STF, the `ErrorString` should indicate the actual reasons
	StfError(ErrorString),
	// error when sending stf request to the receiver
	SendStfRequestFailed,
	ChallengeCodeNotFound,
	// errors when verifying identities
	DecodeHexPayloadFailed(ErrorString),
	HttpRequestFailed(ErrorString),
	InvalidIdentity,
	WrongWeb2Handle,
	UnexpectedMessage,
	WrongSignatureType,
	VerifySubstrateSignatureFailed,
	VerifyEvmSignatureFailed,
	RecoverEvmAddressFailed,
}

// Identity Management Pallet Error
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum IMPError {
	// errors when executing individual error
	SetUserShieldingKeyFailed(ErrorDetail),
	CreateIdentityFailed(ErrorDetail),
	RemoveIdentityFailed(ErrorDetail),
	VerifyIdentityFailed(ErrorDetail),
	// scheduled encalve import error
	ImportScheduledEnclaveFailed,

	// should be unreached, but just to be on the safe side
	// we should classify the error if we ever get this
	UnclassifiedError(ErrorDetail),
}

impl frame_support::traits::PalletError for IMPError {
	// max_encoded_len
	const MAX_ENCODED_SIZE: usize = 1;
}

// Verified Credential(VC) Management Pallet Error
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum VCMPError {
	HttpRequestFailed(ErrorString),
	// Indirect call handling errors when importing parachain blocks
	RequestVCHandlingFailed,
	// tee stf error
	StfError(ErrorString),
	// UTF8Error
	ParseError,
	// Assertion
	Assertion1Failed,
	Assertion2Failed,
	Assertion3Failed,
	Assertion4Failed,
	Assertion5Failed,
	Assertion6Failed,
	Assertion7Failed,
	Assertion8Failed,
	Assertion10Failed,
	Assertion11Failed,
	// should be unreached, but just to be on the safe side
	// we should classify the error if we ever get this
	// UnclassifiedError(ErrorDetail),
}
