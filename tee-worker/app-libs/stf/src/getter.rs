/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

use crate::IdentityManagement;
use codec::{Decode, Encode};
use ita_sgx_runtime::System;
use itp_stf_interface::ExecuteGetter;
use itp_stf_primitives::types::KeyPair;
use itp_utils::stringify::account_id_to_string;
use litentry_primitives::{LitentryMultiAddress, LitentryMultiSignature};
use log::*;
use std::prelude::v1::*;

#[cfg(feature = "evm")]
use ita_sgx_runtime::{AddressMapping, HashedAddressMapping};

#[cfg(feature = "evm")]
use crate::evm_helpers::{get_evm_account, get_evm_account_codes, get_evm_account_storages};

use itp_types::AccountId;
#[cfg(feature = "evm")]
use sp_core::{H160, H256};

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Getter {
	public(PublicGetter),
	trusted(TrustedGetterSigned),
}

impl From<PublicGetter> for Getter {
	fn from(item: PublicGetter) -> Self {
		Getter::public(item)
	}
}

impl From<TrustedGetterSigned> for Getter {
	fn from(item: TrustedGetterSigned) -> Self {
		Getter::trusted(item)
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum PublicGetter {
	some_value,
	nonce(LitentryMultiAddress),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TrustedGetter {
	free_balance(LitentryMultiAddress),
	reserved_balance(LitentryMultiAddress),
	#[cfg(feature = "evm")]
	evm_nonce(LitentryMultiAddress),
	#[cfg(feature = "evm")]
	evm_account_codes(LitentryMultiAddress, H160),
	#[cfg(feature = "evm")]
	evm_account_storages(LitentryMultiAddress, H160, H256),
	// litentry
	user_shielding_key(LitentryMultiAddress),
	id_graph(LitentryMultiAddress),
	id_graph_stats(LitentryMultiAddress),
}

impl TrustedGetter {
	pub fn sender_address(&self) -> &LitentryMultiAddress {
		match self {
			TrustedGetter::free_balance(sender_address) => sender_address,
			TrustedGetter::reserved_balance(sender_address) => sender_address,
			#[cfg(feature = "evm")]
			TrustedGetter::evm_nonce(sender_address) => sender_address,
			#[cfg(feature = "evm")]
			TrustedGetter::evm_account_codes(sender_address, _) => sender_address,
			#[cfg(feature = "evm")]
			TrustedGetter::evm_account_storages(sender_address, ..) => sender_address,
			// litentry
			TrustedGetter::user_shielding_key(sender_address, ..) => sender_address,
			TrustedGetter::id_graph(sender_address) => sender_address,
			TrustedGetter::id_graph_stats(sender_address) => sender_address,
		}
	}

	pub fn sign(&self, pair: &KeyPair) -> TrustedGetterSigned {
		let signature = pair.sign(self.encode().as_slice());
		TrustedGetterSigned { getter: self.clone(), signature }
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct TrustedGetterSigned {
	pub getter: TrustedGetter,
	pub signature: LitentryMultiSignature,
}

impl TrustedGetterSigned {
	pub fn new(getter: TrustedGetter, signature: LitentryMultiSignature) -> Self {
		TrustedGetterSigned { getter, signature }
	}

	pub fn verify_signature(&self) -> bool {
		self.signature
			.verify(self.getter.encode().as_slice(), self.getter.sender_address())
	}
}

impl ExecuteGetter for Getter {
	fn execute(self) -> Option<Vec<u8>> {
		match self {
			Getter::trusted(g) => g.execute(),
			Getter::public(g) => g.execute(),
		}
	}

	fn get_storage_hashes_to_update(self) -> Vec<Vec<u8>> {
		match self {
			Getter::trusted(g) => g.get_storage_hashes_to_update(),
			Getter::public(g) => g.get_storage_hashes_to_update(),
		}
	}
}

impl ExecuteGetter for TrustedGetterSigned {
	fn execute(self) -> Option<Vec<u8>> {
		match self.getter {
			TrustedGetter::free_balance(who) => {
				let account_id: AccountId = who.into();
				let info = System::account(&account_id);
				debug!("TrustedGetter free_balance");
				debug!("AccountInfo for {} is {:?}", account_id_to_string(&account_id), info);
				debug!("Account free balance is {}", info.data.free);
				Some(info.data.free.encode())
			},
			TrustedGetter::reserved_balance(who) => {
				let account_id: AccountId = who.into();
				let info = System::account(&account_id);
				debug!("TrustedGetter reserved_balance");
				debug!("AccountInfo for {} is {:?}", account_id_to_string(&account_id), info);
				debug!("Account reserved balance is {}", info.data.reserved);
				Some(info.data.reserved.encode())
			},
			#[cfg(feature = "evm")]
			TrustedGetter::evm_nonce(who) => {
				let evm_account = get_evm_account(&who.into());
				let evm_account = HashedAddressMapping::into_account_id(evm_account);
				let nonce = System::account_nonce(&evm_account);
				debug!("TrustedGetter evm_nonce");
				debug!("Account nonce is {}", nonce);
				Some(nonce.encode())
			},
			#[cfg(feature = "evm")]
			TrustedGetter::evm_account_codes(_who, evm_account) =>
			// TODO: This probably needs some security check if who == evm_account (or assosciated)
				if let Some(info) = get_evm_account_codes(&evm_account) {
					debug!("TrustedGetter Evm Account Codes");
					debug!("AccountCodes for {} is {:?}", evm_account, info);
					Some(info) // TOOD: encoded?
				} else {
					None
				},
			#[cfg(feature = "evm")]
			TrustedGetter::evm_account_storages(_who, evm_account, index) =>
			// TODO: This probably needs some security check if who == evm_account (or assosciated)
				if let Some(value) = get_evm_account_storages(&evm_account, &index) {
					debug!("TrustedGetter Evm Account Storages");
					debug!("AccountStorages for {} is {:?}", evm_account, value);
					Some(value.encode())
				} else {
					None
				},
			// litentry
			TrustedGetter::user_shielding_key(who) =>
				IdentityManagement::user_shielding_keys(&who).map(|key| key.encode()),
			TrustedGetter::id_graph(who) =>
				Some(IdentityManagement::get_id_graph(&who, usize::MAX).encode()),

			// TODO: we need to re-think it
			//       currently, _who is ignored meaning it's actually not a "trusted" getter.
			//       In fact, in the production no one should have access to the concrete identities
			//       but maybe it makes sense to get some statistic information
			// Disabled until it's resolved
			// Disabled the test `lit-id-graph-stats` too
			TrustedGetter::id_graph_stats(_who) => None,
		}
	}

	fn get_storage_hashes_to_update(self) -> Vec<Vec<u8>> {
		Vec::new()
	}
}

impl ExecuteGetter for PublicGetter {
	fn execute(self) -> Option<Vec<u8>> {
		match self {
			PublicGetter::some_value => Some(42u32.encode()),
			PublicGetter::nonce(address) => {
				let account_id: AccountId = address.into();
				let nonce = System::account_nonce(&account_id);
				debug!("PublicGetter nonce");
				debug!("Account nonce is {}", nonce);
				Some(nonce.encode())
			},
		}
	}

	fn get_storage_hashes_to_update(self) -> Vec<Vec<u8>> {
		Vec::new()
	}
}
