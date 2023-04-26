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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use thiserror_sgx as thiserror;
	pub use url_sgx as url;
}

pub mod a1;
pub mod a10;
pub mod a11;
pub mod a2;
pub mod a3;
pub mod a4;
pub mod a5;
pub mod a6;
pub mod a7;
pub mod a8;

use litentry_primitives::{
	Assertion, ErrorDetail, ErrorString, EvmNetwork, Identity, IndexingNetwork, IndexingNetworks,
	IntoErrorDetail, ParameterString, ParentchainBlockNumber, SubstrateNetwork, VCMPError as Error,
	Web2Network, ASSERTION_FROM_DATE,
};
pub type Result<T> = core::result::Result<T, Error>;
