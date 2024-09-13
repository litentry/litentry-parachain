use crate::alloc::string::String;
use codec::Encode;
use core::result::Result;
use lazy_static::lazy_static;
use litentry_primitives::ParentchainAccountId as AccountId;
use lru::LruCache;
use std::num::NonZeroUsize;
#[cfg(feature = "std")]
use std::sync::RwLock;
#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

lazy_static! {
	static ref STORE: RwLock<LruCache<String, String>> =
		RwLock::new(LruCache::new(NonZeroUsize::new(250).unwrap()));
}

pub struct VerificationCodeStore;

impl VerificationCodeStore {
	pub fn insert(account_id: AccountId, verification_code: String) -> Result<(), String> {
		STORE
			.write()
			.map_err(|_| String::from("Lock poisoning"))?
			.put(hex::encode(account_id.encode()), verification_code);
		Ok(())
	}

	pub fn get(account_id: &AccountId) -> Result<Option<String>, String> {
		let code = STORE
			.write()
			.map_err(|_| String::from("Lock poisoning"))?
			.pop(hex::encode(account_id.encode()).as_str());
		Ok(code)
	}
}
