use crate::{AccountId, Error, Hash, Header, OmniAccountMembers, OmniAccounts, ParentchainId};
use alloc::vec::Vec;
use codec::Encode;
use frame_support::storage::storage_prefix;
use itp_ocall_api::EnclaveOnChainOCallApi;
use itp_storage::{
	decode_storage_key, extract_blake2_128concat_key, storage_map_key, StorageHasher,
};
use litentry_primitives::{GetAccountStoreHash, MemberAccount};
use sp_core::blake2_256;

pub trait GetAccountStoresRepository {
	fn get_by_owner(&self, owner: AccountId) -> Result<OmniAccountMembers, Error>;
	fn get(&self) -> Result<OmniAccounts, Error>;
}

pub struct OmniAccountRepository<OCallApi: EnclaveOnChainOCallApi> {
	ocall_api: OCallApi,
	header: Header,
}

impl<OCallApi: EnclaveOnChainOCallApi> OmniAccountRepository<OCallApi> {
	pub fn new(ocall_api: OCallApi, header: Header) -> Self {
		Self { ocall_api, header }
	}

	pub fn set_header(&mut self, header: Header) {
		self.header = header;
	}
}

impl<OCallApi: EnclaveOnChainOCallApi> GetAccountStoresRepository
	for OmniAccountRepository<OCallApi>
{
	fn get_by_owner(&self, owner: AccountId) -> Result<OmniAccountMembers, Error> {
		let storage_key = storage_map_key(
			"OmniAccount",
			"AccountStore",
			&owner,
			&StorageHasher::Blake2_128Concat,
		);
		let storage_entry = self
			.ocall_api
			.get_storage_verified(storage_key, &self.header, &ParentchainId::Litentry)
			.map_err(|_| Error::OCallApiError("Failed to get storage"))?;
		let member_accounts: Vec<MemberAccount> =
			storage_entry.value().to_owned().ok_or(Error::NotFound)?;
		let account_store_hash = member_accounts.hash();

		Ok(OmniAccountMembers { member_accounts, hash: account_store_hash })
	}

	fn get(&self) -> Result<OmniAccounts, Error> {
		let account_store_key_prefix = storage_prefix(b"OmniAccount", b"AccountStore");
		let account_store_storage_keys_response = self
			.ocall_api
			.get_storage_keys(account_store_key_prefix.into())
			.map_err(|_| Error::OCallApiError("Failed to get storage keys"))?;
		let account_store_storage_keys = account_store_storage_keys_response
			.into_iter()
			.filter_map(decode_storage_key)
			.collect::<Vec<Vec<u8>>>();
		let omni_accounts: OmniAccounts = self
			.ocall_api
			.get_multiple_storages_verified(
				account_store_storage_keys,
				&self.header,
				&ParentchainId::Litentry,
			)
			.map_err(|_| Error::OCallApiError("Failed to get multiple storages"))?
			.into_iter()
			.filter_map(|entry| {
				// TODO: double check this
				let storage_key = decode_storage_key(entry.key)?;
				let account_id: AccountId = extract_blake2_128concat_key(&storage_key)?;
				let member_accounts: Vec<MemberAccount> = entry.value?;
				let account_store_hash = member_accounts.hash();
				let omni_account_members =
					OmniAccountMembers { member_accounts, hash: account_store_hash };
				Some((account_id, omni_account_members))
			})
			.collect();

		Ok(omni_accounts)
	}
}
