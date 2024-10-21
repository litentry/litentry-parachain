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

use crate::{AccountId, Error, Header, MemberAccount, OmniAccounts, ParentchainId};
use alloc::{collections::btree_map::BTreeMap, sync::Arc, vec::Vec};
use frame_support::storage::storage_prefix;
use itp_ocall_api::EnclaveOnChainOCallApi;
use itp_storage::{decode_storage_key, extract_blake2_128concat_key};

pub trait GetAccountStoresRepository {
	fn get_all(&self) -> Result<OmniAccounts, Error>;
}

pub struct OmniAccountRepository<OCallApi: EnclaveOnChainOCallApi> {
	ocall_api: Arc<OCallApi>,
	header: Header,
}

impl<OCallApi: EnclaveOnChainOCallApi> OmniAccountRepository<OCallApi> {
	pub fn new(ocall_api: Arc<OCallApi>, header: Header) -> Self {
		Self { ocall_api, header }
	}

	pub fn set_header(&mut self, header: Header) {
		self.header = header;
	}
}

impl<OCallApi: EnclaveOnChainOCallApi> GetAccountStoresRepository
	for OmniAccountRepository<OCallApi>
{
	fn get_all(&self) -> Result<OmniAccounts, Error> {
		let account_store_key_prefix = storage_prefix(b"OmniAccount", b"AccountStore");
		let page_size = 300;
		let mut start_key: Option<Vec<u8>> = None;
		let mut omni_accounts: OmniAccounts = BTreeMap::new();

		loop {
			let storage_keys_paged = self
				.ocall_api
				.get_storage_keys_paged(
					account_store_key_prefix.into(),
					page_size,
					start_key.clone(),
					Some(&self.header),
				)
				.map_err(|_| Error::OCallApiError("Failed to get storage keys"))?;

			if storage_keys_paged.is_empty() {
				break
			}

			let account_store_storage_keys = storage_keys_paged
				.into_iter()
				.filter_map(decode_storage_key)
				.collect::<Vec<Vec<u8>>>();

			start_key = account_store_storage_keys.last().cloned();

			self.ocall_api
				.get_multiple_storages_verified(
					account_store_storage_keys,
					&self.header,
					&ParentchainId::Litentry,
				)
				.map_err(|_| Error::OCallApiError("Failed to get multiple storages"))?
				.into_iter()
				.filter_map(|entry| {
					let storage_key = decode_storage_key(entry.key)?;
					let account_id: AccountId = extract_blake2_128concat_key(&storage_key)?;
					let member_accounts: Vec<MemberAccount> = entry.value?;
					Some((account_id, member_accounts))
				})
				.for_each(|(account_id, member_accounts)| {
					omni_accounts.insert(account_id, member_accounts);
				});
		}

		Ok(omni_accounts)
	}
}
