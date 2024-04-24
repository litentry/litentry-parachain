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
use std::{collections::HashMap, string::String, sync::Arc};

use litentry_primitives::ParentchainAccountId as AccountId;

lazy_static! {
	static ref TWITTER_CODE_VERIFIER_STORE: Arc<RwLock<HashMap<String, (String, String)>>> =
		Arc::new(RwLock::new(HashMap::new()));
}

pub struct OAuthStore;

impl OAuthStore {
	pub fn save_data(account_id: AccountId, code: String, state: String) -> Result<(), String> {
		TWITTER_CODE_VERIFIER_STORE
			.write()
			.map_err(|_| String::from("Lock poisoning"))?
			.insert(hex::encode(account_id.encode()), (code, state));
		Ok(())
	}

	pub fn get_data(account_id: &AccountId) -> Result<Option<(String, String)>, String> {
		let data = TWITTER_CODE_VERIFIER_STORE
			.write()
			.map_err(|_| String::from("Lock poisoning"))?
			.remove(hex::encode(account_id.encode()).as_str());
		Ok(data)
	}
}
