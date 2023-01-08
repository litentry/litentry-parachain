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
extern crate sgx_tstd as std;

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

pub use parachain_core_primitives::IMPError as Error;
use std::format;

pub type Result<T> = core::result::Result<T, Error>;

pub(crate) fn from_hex_error(e: hex::FromHexError) -> Error {
	Error::DecodeHexFailed(parachain_core_primitives::ErrorString::truncate_from(
		format!("{:?}", e).as_bytes().to_vec(),
	))
}

pub(crate) fn from_data_provider_error(e: lc_data_providers::Error) -> Error {
	Error::HttpRequestFailed(parachain_core_primitives::ErrorString::truncate_from(
		format!("{:?}", e).as_bytes().to_vec(),
	))
}
