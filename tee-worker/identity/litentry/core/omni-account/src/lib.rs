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

pub extern crate alloc;

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

mod repository;
pub use repository::*;

mod in_memory_store;
pub use in_memory_store::InMemoryStore;

use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use itp_types::parentchain::{AccountId, Header, ParentchainId};
use litentry_primitives::MemberAccount;

pub type OmniAccounts = BTreeMap<AccountId, Vec<MemberAccount>>;

#[derive(Debug)]
pub enum Error {
	LockPoisoning,
	OCallApiError(&'static str),
}
