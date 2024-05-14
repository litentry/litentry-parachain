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

use crate::error::{Error, Result};
use itp_ocall_api::EnclaveOnChainOCallApi;
use itp_types::{parentchain::ParentchainId, AccountId, WorkerType};
use lc_teebag_storage::{TeebagStorage, TeebagStorageKeys};
use sp_core::H256;
use sp_runtime::traits::Header as HeaderT;
use sp_std::prelude::Vec;

pub trait ValidateerFetch {
	fn current_validateers<Header: HeaderT<Hash = H256>>(
		&self,
		latest_header: &Header,
	) -> Result<Vec<AccountId>>;

	fn validateer_count<Header: HeaderT<Hash = H256>>(&self, latest_header: &Header)
		-> Result<u64>;
}

impl<OnchainStorage: EnclaveOnChainOCallApi> ValidateerFetch for OnchainStorage {
	fn current_validateers<Header: HeaderT<Hash = H256>>(
		&self,
		header: &Header,
	) -> Result<Vec<AccountId>> {
		let identifiers = self
			.get_storage_verified(
				TeebagStorage::enclave_identifier(WorkerType::Identity),
				header,
				&ParentchainId::Litentry,
			)?
			.into_tuple()
			.1
			.ok_or_else(|| Error::Other("Could not get validateer list from chain"))?;
		Ok(identifiers)
	}

	fn validateer_count<Header: HeaderT<Hash = H256>>(&self, header: &Header) -> Result<u64> {
		Ok(self.current_validateers::<Header>(header)?.len() as u64)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use itc_parentchain_test::ParentchainHeaderBuilder;
	use itp_test::mock::onchain_mock::{validateer_set, OnchainMock};

	#[test]
	pub fn get_validateer_count_works() {
		let header = ParentchainHeaderBuilder::default().build();
		let mock = OnchainMock::default().add_validateer_set(&header, None);
		assert_eq!(mock.validateer_count(&header).unwrap(), 4u64);
	}

	#[test]
	pub fn get_validateer_set_works() {
		let header = ParentchainHeaderBuilder::default().build();
		let mock = OnchainMock::default().add_validateer_set(&header, None);
		let validateers = validateer_set();
		assert_eq!(mock.current_validateers(&header).unwrap(), validateers);
	}
}
