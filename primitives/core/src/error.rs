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

use crate::Assertion;
use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_runtime::{
	traits::{ConstU32, Printable},
	BoundedVec, DispatchError, DispatchErrorWithPostInfo,
};

pub type MaxStringLength = ConstU32<100>;
pub type ErrorString = BoundedVec<u8, MaxStringLength>;

// enum to reflect the error detail from TEE-worker processing
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum ErrorDetail {
	// error when importing the parentchain blocks and executing indirect calls
	ImportError,
	// the direct or indirect request comes from an unauthorized signer
	UnauthorizedSigner,
	// generic error when executing STF, the `ErrorString` should indicate the actual reason
	StfError(ErrorString),
	// error when sending stf request to the receiver fails
	SendStfRequestFailed,
	// error when the user shielding key can not be found
	UserShieldingKeyNotFound,
	// generic parse error, can be caused by UTF8/JSON serde..
	ParseError,
	// errors when communicating with data provider, e.g. HTTP error
	DataProviderError(ErrorString),
	InvalidIdentity,
	WrongWeb2Handle,
	UnexpectedMessage,
	WrongSignatureType,
	VerifyWeb3SignatureFailed,
	RecoverEvmAddressFailed,
	Web3NetworkOutOfBounds,
}

// We could have used Into<ErrorDetail>, but we want it to be more explicit, similar to `into_iter`
pub trait IntoErrorDetail {
	fn into_error_detail(self) -> ErrorDetail;
}

// `From` is implemented for `DispatchError` and `DispatchErrorWithPostInfo` on the top level,
// because we know it can only happen during stf execution in enclave
impl From<DispatchError> for ErrorDetail {
	fn from(e: DispatchError) -> Self {
		ErrorDetail::StfError(ErrorString::truncate_from(
			<DispatchError as Into<&'static str>>::into(e).into(),
		))
	}
}

impl<T> From<DispatchErrorWithPostInfo<T>> for ErrorDetail
where
	T: Eq + PartialEq + Clone + Copy + Encode + Decode + Printable,
{
	fn from(e: DispatchErrorWithPostInfo<T>) -> Self {
		ErrorDetail::StfError(ErrorString::truncate_from(
			<DispatchErrorWithPostInfo<T> as Into<&'static str>>::into(e).into(),
		))
	}
}

// Identity Management Pallet Error
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum IMPError {
	// errors when executing individual error
	SetUserShieldingKeyFailed(ErrorDetail),
	LinkIdentityFailed(ErrorDetail),
	DeactivateIdentityFailed(ErrorDetail),
	ActivateIdentityFailed(ErrorDetail),
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
	RequestVCFailed(Assertion, ErrorDetail),
	// should be unreached, but just to be on the safe side
	// we should classify the error if we ever get this
	UnclassifiedError(ErrorDetail),
}
