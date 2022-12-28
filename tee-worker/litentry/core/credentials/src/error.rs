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
// along with Litentry. If not, see <https://www.gnu.org/licenses/>.

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

use std::string::String;

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
	#[error("Empty Credential Proof")]
	EmptyCredentialProof,
	#[error("Empty Credential Type")]
	EmptyCredentialType,
	#[error("Empty Credential Issuer")]
	EmptyCredentialIssuer,
	#[error("Empty Credential Subject")]
	EmptyCredentialSubject,
	#[error("Empty Issuance Date")]
	EmptyIssuanceDate,
	#[error("Pass Error: {0}")]
	ParseError(String),
	#[error("Unsupported Assertion")]
	UnsupportedAssertion,
	#[error("Runtime Error: {0}")]
	RuntimeError(String),
	#[error(transparent)]
	Json(#[from] serde_json::Error),
}
