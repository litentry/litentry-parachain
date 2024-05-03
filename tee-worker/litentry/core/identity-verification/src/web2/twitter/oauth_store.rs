use codec::Encode;
use core::result::Result;
use lazy_static::lazy_static;
use lru::LruCache;
#[cfg(feature = "std")]
use std::sync::RwLock;
#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;
use std::{num::NonZeroUsize, string::String};

use litentry_primitives::ParentchainAccountId as AccountId;

lazy_static! {
	static ref STORE: RwLock<LruCache<String, (String, String)>> =
		RwLock::new(LruCache::new(NonZeroUsize::new(250).unwrap()));
}

pub struct OAuthStore;

impl OAuthStore {
	pub fn save_data(account_id: AccountId, code: String, state: String) -> Result<(), String> {
		STORE
			.write()
			.map_err(|_| String::from("Lock poisoning"))?
			.put(hex::encode(account_id.encode()), (code, state));
		Ok(())
	}

	pub fn get_data(account_id: &AccountId) -> Result<Option<(String, String)>, String> {
		let data = STORE
			.write()
			.map_err(|_| String::from("Lock poisoning"))?
			.pop(hex::encode(account_id.encode()).as_str());
		Ok(data)
	}
}
