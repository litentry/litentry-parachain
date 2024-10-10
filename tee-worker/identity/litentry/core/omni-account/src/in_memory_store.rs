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

use crate::{AccountId, BTreeMap, Error, OmniAccountMembers, OmniAccounts};
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
	pub fn get(&self, owner: AccountId) -> Result<OmniAccountMembers, Error> {
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

	pub fn insert(
		&self,
		owner: AccountId,
		omni_account_members: OmniAccountMembers,
	) -> Result<(), Error> {
		STORE
			.write()
			.map_err(|_| {
				log::error!("[InMemoryStore] Lock poisoning");
				Error::LockPoisoning
			})?
			.insert(owner, omni_account_members);

		Ok(())
	}

	pub fn remove(&self, owner: AccountId) -> Result<(), Error> {
		STORE
			.write()
			.map_err(|_| {
				log::error!("[InMemoryStore] Lock poisoning");
				Error::LockPoisoning
			})?
			.remove(&owner);

		Ok(())
	}

	pub fn load(&self, omni_accounts: OmniAccounts) -> Result<(), Error> {
		*STORE.write().map_err(|_| {
			log::error!("[InMemoryStore] Lock poisoning");
			Error::LockPoisoning
		})? = omni_accounts;

		Ok(())
	}
}
