use crate::alloc::{fmt, format, string::String};
use codec::Encode;
use core::result::Result;
use lazy_static::lazy_static;
use litentry_primitives::{
	ErrorDetail, ErrorString, IntoErrorDetail, ParentchainAccountId as AccountId,
};
use lru::LruCache;
use std::num::NonZeroUsize;
#[cfg(feature = "std")]
use std::sync::RwLock;
#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

#[derive(Debug)]
pub enum VerificationCodeStoreError {
	LockPoisoning,
	Other(String),
}

impl fmt::Display for VerificationCodeStoreError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			VerificationCodeStoreError::LockPoisoning => write!(f, "Lock poisoning"),
			VerificationCodeStoreError::Other(msg) => write!(f, "{}", msg),
		}
	}
}

impl std::error::Error for VerificationCodeStoreError {}

impl IntoErrorDetail for VerificationCodeStoreError {
	fn into_error_detail(self) -> ErrorDetail {
		ErrorDetail::StfError(ErrorString::truncate_from(format!("{}", self).into()))
	}
}

lazy_static! {
	static ref STORE: RwLock<LruCache<String, String>> =
		RwLock::new(LruCache::new(NonZeroUsize::new(250).unwrap()));
}

pub struct VerificationCodeStore;

impl VerificationCodeStore {
	pub fn insert(
		account_id: AccountId,
		verification_code: String,
	) -> Result<(), VerificationCodeStoreError> {
		STORE
			.write()
			.map_err(|_| VerificationCodeStoreError::LockPoisoning)?
			.put(hex::encode(account_id.encode()), verification_code);
		Ok(())
	}

	pub fn get(account_id: &AccountId) -> Result<Option<String>, VerificationCodeStoreError> {
		let code = STORE
			.write()
			.map_err(|_| VerificationCodeStoreError::LockPoisoning)?
			.pop(hex::encode(account_id.encode()).as_str());
		Ok(code)
	}
}
