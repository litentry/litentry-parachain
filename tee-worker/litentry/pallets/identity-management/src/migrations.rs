use frame_support::{
	traits::{GetStorageVersion, PalletInfoAccess, StorageVersion},
	weights::Weight,
};


/// Just example how to write migration
pub fn migrate_to_v1<T: crate::Config, P: GetStorageVersion + PalletInfoAccess>() -> Weight {
	let on_chain_storage_version = <P as GetStorageVersion>::on_chain_storage_version();
	if on_chain_storage_version < 1 {
		log::warn!("test migrate");
		StorageVersion::new(1).put::<P>();
	}
	Weight::zero()
}
