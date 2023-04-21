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
// along with Litentry. If not, see <https://www.gnu.org/licenses/>.

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

use litentry_primitives::{ErrorDetail, ErrorString, IntoErrorDetail};
use std::{boxed::Box, string::String};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
	#[error("Invalid Credential")]
	InvalidCredential,
	#[error("Empty Credential Type")]
	EmptyCredentialType,
	#[error("Empty Credential Issuer")]
	EmptyCredentialIssuer,
	#[error("Empty Credential Subject")]
	EmptyCredentialSubject,
	#[error("Empty Issuance Block Number")]
	EmptyIssuanceBlockNumber,
	#[error("Empty Proof Block Number")]
	EmptyProofBlockNumber,
	#[error("Invalid Proof")]
	InvalidProof,
	#[error("Credential Is Too Long")]
	CredentialIsTooLong,
	#[error("Parse Error: {0}")]
	ParseError(String),
	#[error("Unsupported Assertion")]
	UnsupportedAssertion,
	#[error("Runtime Error: {0}")]
	RuntimeError(String),
	#[error(transparent)]
	Other(#[from] Box<dyn std::error::Error + Sync + Send + 'static>),
}

impl IntoErrorDetail for Error {
	fn into_error_detail(self) -> ErrorDetail {
		ErrorDetail::StfError(ErrorString::truncate_from(format!("{}", self).into()))
	}
}
