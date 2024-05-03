// Copyright 2020-2024 Trust Computing GmbH.
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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use futures_sgx as futures;
	pub use http_req_sgx as http_req;
	pub use http_sgx as http;
	pub use thiserror_sgx as thiserror;
}

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

use frame_support::pallet_prelude::*;
use lc_stf_task_sender::Web2IdentityVerificationRequest;

pub mod web2;

mod error;
use error::{Error, Result};
use lc_data_providers::DataProviderConfig;

pub fn verify(r: &Web2IdentityVerificationRequest, config: &DataProviderConfig) -> Result<()> {
	web2::verify(&r.who, &r.identity, &r.raw_msg, &r.validation_data, config)
}
