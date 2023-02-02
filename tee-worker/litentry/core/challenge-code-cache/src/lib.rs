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
#![feature(assert_matches)]

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use std::sync::SgxRwLock as RwLock;

#[cfg(feature = "std")]
use std::sync::RwLock;

use lazy_static::lazy_static;
use litentry_primitives::{ChallengeCode, Identity};
use std::{collections::HashMap, sync::Arc, vec::Vec};

lazy_static! {
	/// Global instance of a challenge cache
	///
	/// Concurrent access is managed internally, using RW locks
	pub static ref GLOBAL_CHALLENGE_CODE_CACHE: Arc<ChallengeCodeCache> = Default::default();
}

/// Local challenge_code cache
/// This struct is for testing
#[derive(Default)]
pub struct ChallengeCodeCache {
	codes: RwLock<HashMap<Vec<u8>, Vec<u8>>>,
	enable: RwLock<bool>,
}

impl ChallengeCodeCache {
	pub fn insert_challenge_code(&self, identity: Identity, code: ChallengeCode) {
		if self.enable.read().map_or(false, |r| *r) {
			if let Ok(mut codes_lock) = self.codes.write() {
				log::debug!("cache challenge_code: {:?}, code:{:?}", identity, code);
				codes_lock.insert(identity.flat(), code.to_vec());
			}
		}
	}

	pub fn enable(&self) {
		if let Ok(w) = self.enable.write() {
			let mut enable_lock = w;
			*enable_lock = true;
		}
	}

	pub fn is_enabled(&self) -> bool {
		self.enable.read().map_or(false, |r| *r)
	}

	pub fn get_challenge_code(&self, identity: Vec<u8>) -> Option<Vec<u8>> {
		if let Ok(codes) = self.codes.read() {
			return codes.get(&identity).cloned()
		}
		None
	}
}
