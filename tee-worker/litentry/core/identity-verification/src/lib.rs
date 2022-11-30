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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use futures_sgx as futures;
	pub use hex_sgx as hex;
	pub use http_req_sgx as http_req;
	pub use http_sgx as http;
	pub use thiserror_sgx as thiserror;
	pub use url_sgx as url;
}

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

use codec::Encode;
use frame_support::pallet_prelude::*;
use sp_core::blake2_256;
// this should be ita_stf::AccountId, but we use itp_types to avoid cyclic dep
use itp_types::AccountId;
use litentry_primitives::{ChallengeCode, Identity};
use sp_std::vec::Vec;
use std::string::ToString;

pub mod web2;
pub mod web3;

pub mod error;

// verification message format: <challeng-code> + <litentry-AccountId32> + <Identity>,
// where <> means SCALE-encoded
pub fn get_expected_payload(who: &AccountId, identity: &Identity, code: &ChallengeCode) -> Vec<u8> {
	let mut payload = code.encode();
	payload.append(&mut who.encode());
	payload.append(&mut identity.encode());
	blake2_256(payload.as_slice()).to_vec()
}
