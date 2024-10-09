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
