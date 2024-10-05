use crate::{AccountId, BTreeMap, Error, IDGraphs, OmniAccountIDGraph};
use lazy_static::lazy_static;

#[cfg(feature = "std")]
use std::sync::RwLock;
#[cfg(feature = "sgx")]
use std::sync::SgxRwLock as RwLock;

lazy_static! {
	static ref ID_GRAPHS: RwLock<IDGraphs> = RwLock::new(BTreeMap::new());
}

pub struct IDGraphsStore;

impl IDGraphsStore {
	pub fn get(&self, owner: AccountId) -> Result<OmniAccountIDGraph, Error> {
		let id_graph = ID_GRAPHS
			.read()
			.map_err(|_| {
				log::error!("[IDGraphsInMemoryRepository] Lock poisoning");
				Error::LockPoisoning
			})?
			.get(&owner)
			.cloned();

		id_graph.ok_or(Error::NotFound)
	}

	pub fn insert(&self, owner: AccountId, id_graph: OmniAccountIDGraph) -> Result<(), Error> {
		ID_GRAPHS
			.write()
			.map_err(|_| {
				log::error!("[IDGraphsInMemoryRepository] Lock poisoning");
				Error::LockPoisoning
			})?
			.insert(owner, id_graph);

		Ok(())
	}

	pub fn remove(&self, owner: AccountId) -> Result<(), Error> {
		ID_GRAPHS
			.write()
			.map_err(|_| {
				log::error!("[IDGraphsInMemoryRepository] Lock poisoning");
				Error::LockPoisoning
			})?
			.remove(&owner);

		Ok(())
	}

	pub fn load(&self, id_graphs: IDGraphs) -> Result<(), Error> {
		*ID_GRAPHS.write().map_err(|_| {
			log::error!("[IDGraphsInMemoryRepository] Lock poisoning");
			Error::LockPoisoning
		})? = id_graphs;

		Ok(())
	}
}
