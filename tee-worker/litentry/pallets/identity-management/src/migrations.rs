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

use crate::{IDGraphs, IdentityContext};
use frame_support::{
	traits::{GetStorageVersion, PalletInfoAccess, StorageVersion},
	weights::Weight,
};

// This is just an example of how to write a custom migration
pub fn migrate_to_v1<T: crate::Config, P: GetStorageVersion + PalletInfoAccess>() -> Weight {
	let on_chain_storage_version = <P as GetStorageVersion>::on_chain_storage_version();
	if on_chain_storage_version != 0 {
		return Weight::zero()
	}
	log::info!("Doing migrations now for IMT version {:?}", on_chain_storage_version);
	StorageVersion::new(1).put::<P>();
	Weight::from_all(1)
}

pub fn migrate_to_v2<T: crate::Config, P: GetStorageVersion + PalletInfoAccess>() -> Weight {
	let on_chain_storage_version = <P as GetStorageVersion>::on_chain_storage_version();
	if on_chain_storage_version != 1 {
		return Weight::zero()
	}
	log::info!("Doing migrations now for IMT version {:?}", on_chain_storage_version);
	StorageVersion::new(2).put::<P>();

	// '//Charlie' -> 0x90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22
	let mut charlie = [0; 32];
	hex::decode_to_slice(
		"90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22",
		&mut charlie,
	)
	.unwrap();

	// Update after P-174:
	// the following code snippet only serves as an example based on the old codebase, it doesn't
	// work now as we don't have the storage item anymore, thus commented out.
	//
	// set Charlie's shielding key to [2u8; 32]
	// verifiable via:
	// ```
	// ./bin/litentry-cli trusted --mrenclave $mrenclave get-storage IdentityManagement UserShieldingKeys 90b5ab205c6974c9ea841be688864633dc9ca8a357843eeacf2314649965fe22
	// ```
	// migration::put_storage_value::<RequestAesKey>(
	// 	"IdentityManagement".as_bytes(),
	// 	"UserShieldingKeys".as_bytes(),
	// 	&charlie.blake2_128_concat(),
	// 	[2u8; 32],
	// );
	Weight::zero()
}

mod v2 {
	use crate::{BlockNumberOf, Config, IdentityStatus, Pallet, Web3Network};
	use codec::{Decode, Encode};
	use frame_support::{pallet_prelude::OptionQuery, storage_alias, Blake2_128Concat};
	use litentry_primitives::Identity;
	use scale_info::TypeInfo;
	use sp_std::vec::Vec;

	// The context associated with the (litentry-account, did) pair
	#[derive(Clone, Eq, PartialEq, Debug, Encode, Decode, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	#[codec(mel_bound())]
	pub struct IdentityContext<T: Config> {
		// the sidechain block number at which the identity is linked
		pub link_block: BlockNumberOf<T>,
		// a list of web3 networks on which the identity should be used
		pub web3networks: Vec<Web3Network>,
		// the identity status
		pub status: IdentityStatus,
	}

	#[storage_alias]
	pub type IDGraphs<T: Config> = StorageDoubleMap<
		Pallet<T>,
		Blake2_128Concat,
		Identity,
		Blake2_128Concat,
		Identity,
		IdentityContext<T>,
		OptionQuery,
	>;
}

pub fn migrate_to_v3<T: crate::Config, P: GetStorageVersion + PalletInfoAccess>() -> Weight {
	let on_chain_storage_version = <P as GetStorageVersion>::on_chain_storage_version();
	if on_chain_storage_version != 2 {
		return Weight::zero()
	}

	log::info!("Doing migrations now for IMT version {:?}", on_chain_storage_version);
	StorageVersion::new(2).put::<P>();

	let mut count = 0;
	for (who, identity, context) in v2::IDGraphs::<T>::drain() {
		IDGraphs::<T>::insert(
			&who,
			&identity,
			IdentityContext { link_block: context.link_block, status: context.status },
		);
		count += 1;
	}

	Weight::from_all(count + 1)
}
