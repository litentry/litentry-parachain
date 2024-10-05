use crate::{
	AccountId, Error, Hash, Header, IDGraph, IDGraphMember, IDGraphs, Identity, OmniAccountIDGraph,
	ParentchainId,
};
use alloc::vec::Vec;
use codec::Encode;
use frame_support::storage::storage_prefix;
use itp_ocall_api::EnclaveOnChainOCallApi;
use itp_storage::{
	decode_storage_key, extract_blake2_128concat_key, storage_map_key, StorageHasher,
};
use sp_core::blake2_256;

// TODO: get this from core_primitives after the release-v0.9.19 branch has been updated
pub trait IDGraphHash {
	fn hash(&self) -> Hash;
}
impl IDGraphHash for Vec<IDGraphMember> {
	fn hash(&self) -> Hash {
		let members_hashes: Vec<Hash> = self.iter().map(|member| member.hash).collect();
		Hash::from(blake2_256(&members_hashes.encode()))
	}
}

pub trait GetIDGraphsRepository {
	fn get_by_owner(
		&self,
		block_header: Header,
		owner: Identity,
	) -> Result<OmniAccountIDGraph, Error>;
	fn get(&self, block_header: Header) -> Result<IDGraphs, Error>;
}

pub struct IDGraphsRepository<OCallApi: EnclaveOnChainOCallApi> {
	ocall_api: OCallApi,
}

impl<OCallApi: EnclaveOnChainOCallApi> IDGraphsRepository<OCallApi> {
	pub fn new(ocall_api: OCallApi) -> Self {
		Self { ocall_api }
	}
}

impl<OCallApi: EnclaveOnChainOCallApi> GetIDGraphsRepository for IDGraphsRepository<OCallApi> {
	fn get_by_owner(
		&self,
		block_header: Header,
		owner: Identity,
	) -> Result<OmniAccountIDGraph, Error> {
		let storage_key =
			storage_map_key("OmniAccount", "IDGraphs", &owner, &StorageHasher::Blake2_128Concat);
		let storage_entry = self
			.ocall_api
			.get_storage_verified(storage_key, &block_header, &ParentchainId::Litentry)
			.map_err(|_| Error::OCallApiError("Failed to get storage"))?;
		let id_graph: Vec<IDGraphMember> =
			storage_entry.value().to_owned().ok_or(Error::NotFound)?;
		let id_graph_hash = id_graph.hash();

		Ok(OmniAccountIDGraph { graph: id_graph, hash: id_graph_hash })
	}

	fn get(&self, block_header: Header) -> Result<IDGraphs, Error> {
		let id_graphs_key_prefix = storage_prefix(b"OmniAccount", b"IDGraphs");
		let id_graphs_storage_keys_response = self
			.ocall_api
			.get_storage_keys(id_graphs_key_prefix.into())
			.map_err(|_| Error::OCallApiError("Failed to get storage keys"))?;
		let id_graphs_storage_keys = id_graphs_storage_keys_response
			.into_iter()
			.filter_map(decode_storage_key)
			.collect::<Vec<Vec<u8>>>();
		let id_graphs: IDGraphs = self
			.ocall_api
			.get_multiple_storages_verified(
				id_graphs_storage_keys,
				&block_header,
				&ParentchainId::Litentry,
			)
			.map_err(|_| Error::OCallApiError("Failed to get multiple storages"))?
			.into_iter()
			.filter_map(|entry| {
				// TODO: double check this
				let storage_key = decode_storage_key(entry.key)?;
				let account_id: AccountId = extract_blake2_128concat_key(&storage_key)?;
				let id_graph: IDGraph = entry.value?;
				let id_graph_hash = id_graph.hash();
				let omni_account_id_graph =
					OmniAccountIDGraph { graph: id_graph, hash: id_graph_hash };
				Some((account_id, omni_account_id_graph))
			})
			.collect();

		Ok(id_graphs)
	}
}
