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

use crate::{AccountId, BTreeMap, BlockNumber, Error, MemberAccount, OmniAccounts, Vec};
use lazy_static::lazy_static;

#[cfg(feature = "std")]
use std::sync::RwLock;
#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

lazy_static! {
	static ref STORE: RwLock<OmniAccounts> = RwLock::new(BTreeMap::new());
	static ref STORE_BLOCK_HEIGHT: RwLock<BlockNumber> = RwLock::new(0);
}

pub struct InMemoryStore;

impl InMemoryStore {
	pub fn set_block_height(block_number: BlockNumber) -> Result<(), Error> {
		*STORE_BLOCK_HEIGHT.write().map_err(|_| {
			log::error!("[InMemoryStore] Lock poisoning");
			Error::LockPoisoning
		})? = block_number;

		Ok(())
	}

	pub fn get_block_height() -> Result<BlockNumber, Error> {
		let block_number = *STORE_BLOCK_HEIGHT.read().map_err(|_| {
			log::error!("[InMemoryStore] Lock poisoning");
			Error::LockPoisoning
		})?;

		Ok(block_number)
	}

	pub fn get(account_id: AccountId) -> Result<Option<Vec<MemberAccount>>, Error> {
		let omni_account_members = STORE
			.read()
			.map_err(|_| {
				log::error!("[InMemoryStore] Lock poisoning");
				Error::LockPoisoning
			})?
			.get(&account_id)
			.cloned();

		Ok(omni_account_members)
	}

	pub fn insert(account_id: AccountId, members: Vec<MemberAccount>) -> Result<(), Error> {
		STORE
			.write()
			.map_err(|_| {
				log::error!("[InMemoryStore] Lock poisoning");
				Error::LockPoisoning
			})?
			.insert(account_id, members);

		Ok(())
	}

	pub fn load(accounts: OmniAccounts) -> Result<(), Error> {
		*STORE.write().map_err(|_| {
			log::error!("[InMemoryStore] Lock poisoning");
			Error::LockPoisoning
		})? = accounts;

		Ok(())
	}
}
