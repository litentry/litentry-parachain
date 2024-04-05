#[cfg(feature = "sgx")]
use thiserror_sgx as thiserror;

#[cfg(feature = "std")]
use std::sync::RwLock;
#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

#[cfg(feature = "std")]
use hex;
#[cfg(feature = "sgx")]
use hex_sgx as hex;

use codec::Encode;
use core::result::Result;
use lazy_static::lazy_static;
use std::{collections::HashMap, format, string::String, sync::Arc};

use litentry_primitives::ParentchainAccountId as AccountId;

lazy_static! {
	static ref TWITTER_CODE_VERIFIER_STORE: Arc<RwLock<HashMap<String, String>>> =
		Arc::new(RwLock::new(HashMap::new()));
}

pub struct CodeVerifierStore;

impl CodeVerifierStore {
	pub fn save_code(account_id: AccountId, code: String) -> Result<(), String> {
		TWITTER_CODE_VERIFIER_STORE
			.write()
			.map_err(|_| format!("Lock poisoning"))?
			.insert(hex::encode(account_id.encode()), code);
		Ok(())
	}

	pub fn get_code(account_id: &AccountId) -> Result<Option<String>, String> {
		let code = TWITTER_CODE_VERIFIER_STORE
			.read()
			.map_err(|_| format!("Lock poisoning"))?
			.get(hex::encode(account_id.encode()).as_str())
			.cloned();

		Ok(code)
	}
}
