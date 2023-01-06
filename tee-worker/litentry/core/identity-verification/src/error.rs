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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use std::string::String;

pub type Result<T> = core::result::Result<T, Error>;

// identity verification errors
#[derive(Debug, thiserror::Error)]
pub enum Error {
	#[error("unexpected message")]
	UnexpectedMessage,
	#[error("wrong signature type")]
	WrongSignatureType,
	#[error("failed to verify substrate signature")]
	VerifySubstrateSignatureFailed,
	#[error("failed to recover substrate public key")]
	RecoverSubstratePubkeyFailed,
	#[error("failed to verify evm signature")]
	VerifyEvmSignatureFailed,
	#[error("failed to recover evm address")]
	RecoverEvmAddressFailed,
	#[error("Request error: {0}")]
	RequestError(String),
	#[error("Other error: {0}")]
	OtherError(String),
	#[error("Invalid identity")]
	InvalidIdentity,
	// #[error(transparent)]
	// Other(#[from] std::boxed::Box<dyn std::error::Error + Sync + Send + 'static>),
}
