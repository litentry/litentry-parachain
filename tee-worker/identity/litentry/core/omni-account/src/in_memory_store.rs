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

use crate::{AccountId, BTreeMap, Error, MemberAccount, OmniAccounts, Vec};
use lazy_static::lazy_static;

#[cfg(feature = "std")]
use std::sync::RwLock;
#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

lazy_static! {
	static ref STORE: RwLock<OmniAccounts> = RwLock::new(BTreeMap::new());
}

pub struct InMemoryStore;

impl InMemoryStore {
	pub fn get(&self, owner: AccountId) -> Result<Vec<MemberAccount>, Error> {
		let omni_account_members = STORE
			.read()
			.map_err(|_| {
				log::error!("[InMemoryStore] Lock poisoning");
				Error::LockPoisoning
			})?
			.get(&owner)
			.cloned();

		omni_account_members.ok_or(Error::NotFound)
	}

	pub fn insert(&self, account_id: AccountId, members: Vec<MemberAccount>) -> Result<(), Error> {
		STORE
			.write()
			.map_err(|_| {
				log::error!("[InMemoryStore] Lock poisoning");
				Error::LockPoisoning
			})?
			.insert(account_id, members);

		Ok(())
	}

	pub fn remove(&self, account_id: AccountId) -> Result<(), Error> {
		STORE
			.write()
			.map_err(|_| {
				log::error!("[InMemoryStore] Lock poisoning");
				Error::LockPoisoning
			})?
			.remove(&account_id);

		Ok(())
	}

	pub fn load(&self, accounts: OmniAccounts) -> Result<(), Error> {
		*STORE.write().map_err(|_| {
			log::error!("[InMemoryStore] Lock poisoning");
			Error::LockPoisoning
		})? = accounts;

		Ok(())
	}
}
