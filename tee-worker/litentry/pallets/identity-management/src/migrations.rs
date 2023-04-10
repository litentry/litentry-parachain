use frame_support::{
	storage::migration,
	traits::{GetStorageVersion, PalletInfoAccess, StorageVersion},
	weights::Weight,
	Hashable,
};
use litentry_primitives::UserShieldingKeyType;

// This is just an example of how to write a custom migration
pub fn migrate_to_v1<T: crate::Config, P: GetStorageVersion + PalletInfoAccess>() -> Weight {
	let on_chain_storage_version = <P as GetStorageVersion>::on_chain_storage_version();
	if on_chain_storage_version < 1 {
		log::info!("Doing migrations now for IMT version {:?}", on_chain_storage_version);
		StorageVersion::new(1).put::<P>();
	}
	Weight::zero()
}

pub fn migrate_to_v2<T: crate::Config, P: GetStorageVersion + PalletInfoAccess>() -> Weight {
	let on_chain_storage_version = <P as GetStorageVersion>::on_chain_storage_version();
	if on_chain_storage_version < 2 {
		log::info!("Doing migrations now for IMT version {:?}", on_chain_storage_version);
		StorageVersion::new(2).put::<P>();

		// '//Charlie' -> 0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22
		let mut charlie = [0; 32];
		hex::decode_to_slice(
			"90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22",
			&mut charlie,
		)
		.unwrap();

		// set Charlie's shielding key to [2u8; 32]
		// verifiable via:
		// ```
		// ./bin/integritee-cli trusted --mrenclave $mrenclave get-storage IdentityManagement UserShieldingKeys 90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22
		// ```
		migration::put_storage_value::<UserShieldingKeyType>(
			"IdentityManagement".as_bytes(),
			"UserShieldingKeys".as_bytes(),
			&charlie.blake2_128_concat(),
			[2u8; 32],
		);
	}
	Weight::zero()
}
