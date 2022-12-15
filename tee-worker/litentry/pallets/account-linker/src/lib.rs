//! # AccountLinker Pallet
//!
//! The AccountLinker pallet provides functionality for linking a Litentry account to account at
//! other networks. (currently support Ethereum (BSC), BTC and Substrate based address)
//!
//! ## Overview
//!
//! The AccountLinker pallet stores the linking relation between Litentry accounts and accounts at other
//! networks. It also offers extrinscs for user to update the linking relation. For each linking relation,
//! user may choose to freshly link new account or replace an existing linked account with a new provided one.
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//! * `link_eth` - Link an Ethereum address to a Litentry account providing a proof signature
//! from the private key of that Ethereum address.
//! * `link_btc` - Link an BTC address to a Litentry account providing a proof signature
//! from the private key of that BTC address.
//! * `link_sub` - Initiate a link request to link a substrate based address to Litentry address
//!
//! [`Call`]: ./enum.Call.html
//! [`Config`]: ./trait.Config.html

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;

mod util_eth;
pub mod weights;

type EthAddress = [u8; 20];
// rsv signature
type Signature = [u8; 65];

#[frame_support::pallet]
pub mod pallet {
	use crate::*;
	use codec::Encode;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::{ensure_signed, pallet_prelude::*};
	use sp_core::{ed25519, sr25519};
	use sp_std::prelude::*;

	use weights::WeightInfo;
	pub const EXPIRING_BLOCK_NUMBER_MAX: u32 = 10 * 60 * 24 * 30; // 30 days for 6s per block
	pub const MAX_ETH_LINKS: usize = 3;
	pub const MAX_BTC_LINKS: usize = 3;
	pub const MAX_SUB_LINKS: usize = 3;

	#[derive(Encode, Decode, Clone, Debug, Copy, Eq, PartialEq, TypeInfo)]
	pub enum NetworkType {
		Kusama,
		Polkadot,
		KusamaParachain(u32),
		PolkadotParachain(u32),
	}

	#[derive(Encode, Decode, Clone, Debug, Eq, PartialEq, TypeInfo)]
	pub enum MultiSignature {
		Sr25519Signature([u8; 64]),
		Ed25519Signature([u8; 64]),
		EcdsaSignature([u8; 65]),
	}

	#[derive(Encode, Decode, Clone, Debug, Eq, PartialEq, TypeInfo)]
	pub struct LinkedSubAccount<AccountId> {
		network_type: NetworkType,
		account_id: AccountId,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Ethereum address successfully linked. \[Lintentry account, Ethereum account\]
		EthAddressLinked(T::AccountId, Vec<u8>),
		/// BTC address successfully linked. \[Lintentry account, BTC account\]
		BtcAddressLinked(T::AccountId, Vec<u8>),
		/// Substrate based address successfully linked. \[Lintentry account, substrate account\]
		SubAddressLinked(T::AccountId, LinkedSubAccount<T::AccountId>),
	}

	#[pallet::error]
	pub enum Error<T> {
		// Cannot recover the signature
		EcdsaRecoverFailure,
		// Link request expired
		LinkRequestExpired,
		// Provided address mismatch the address recovered from signature recovery
		UnexpectedAddress,
		// Unexpected ethereum message length error
		UnexpectedEthMsgLength,
		// Invalid BTC address to link
		InvalidBTCAddress,
		// Expiration block number is too far away from now
		InvalidExpiringBlockNumber,
		// Can't get layer one block number
		LayerOneBlockNumberNotAvailable,
		// Signature is wrong
		WrongSignature,
		// Expected AccountId is [u8; 32]
		UnexpectedAccountId,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn eth_addresses)]
	pub(super) type EthereumLink<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<EthAddress>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn btc_addresses)]
	pub(super) type BitcoinLink<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Vec<u8>>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn sub_addresses)]
	pub(super) type SubLink<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Vec<LinkedSubAccount<T::AccountId>>,
		ValueQuery,
	>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Link an Ethereum address to a Litentry account providing a proof signature from the private key
		/// of that Ethereum address. The extrinsic supposed to be executed in the sgx.
		///
		/// The runtime needs to ensure that a malicious index can be handled correctly.
		/// Currently, when vec.len > MAX_ETH_LINKS, replacement will always happen at the final index.
		/// Otherwise it will use the next new slot unless index is valid against a currently available slot.
		///
		/// Parameters:
		/// - `account`: The Litentry address that is to be linked
		/// - `index`: The index of the linked Ethereum address that the user wants to replace with.
		/// - `addr_expected`: The intended Ethereum address to link to the origin's Litentry address
		/// - `layer_one_block_number`: The current layer one block number
		/// - `expiring_block_number`: The block number after which this link request will expire
		/// - `sig`: The rsv-signature generated by the private key of the addr_expected
		///
		/// Emits `EthAddressLinked` event when successful.
		#[pallet::weight(T::WeightInfo::link_eth())]
		pub fn link_eth(
			origin: OriginFor<T>,
			account: T::AccountId,
			index: u32,
			addr_expected: EthAddress,
			layer_one_block_number: T::BlockNumber,
			expiring_block_number: T::BlockNumber,
			sig: Signature,
		) -> DispatchResultWithPostInfo {
			// in sgx runtime, the account who want to link ethereum address don't have the balance to
			// submit extrinsic, the origin could be the root account
			let _ = ensure_signed(origin)?;
			Self::do_link_eth(
				account,
				index,
				addr_expected,
				expiring_block_number,
				layer_one_block_number,
				sig,
			)
		}

		/// Link a substrate based address to a Litentry address
		///
		/// The runtime needs to ensure that a malicious index can be handled correctly.
		/// Currently, when vec.len > MAX_ETH_LINKS, replacement will always happen at the final index.
		/// Otherwise it will use the next new slot unless index is valid against a currently available slot.
		///
		/// Parameters:
		/// - `account`: The Litentry address that is to be linked
		///
		/// Emits `SubAddressLinked` event when successful.
		// TODO will update weight when do the benchmark testing

		#[allow(clippy::too_many_arguments)]
		#[pallet::weight(T::WeightInfo::link_eth())]
		pub fn link_sub(
			origin: OriginFor<T>,
			account: T::AccountId,
			index: u32,
			network_type: NetworkType,
			linked_account: T::AccountId,
			layer_one_block_number: T::BlockNumber,
			expiring_block_number: T::BlockNumber,
			sig: MultiSignature,
		) -> DispatchResultWithPostInfo {
			// in sgx runtime, the account who want to link ethereum address don't have the balance to
			// submit extrinsic, the origin could be the root account
			let _ = ensure_signed(origin)?;
			Self::do_link_sub(
				account,
				index,
				network_type,
				linked_account,
				expiring_block_number,
				layer_one_block_number,
				sig,
			)
		}
	}

	impl<T: Config> Pallet<T> {
		/// Assemble the message that the user has signed
		/// Format: "Link Litentry: " + Litentry account + expiring block number
		fn generate_eth_raw_message(
			account: &T::AccountId,
			expiring_block_number: T::BlockNumber,
		) -> Vec<u8> {
			let mut bytes = b"Link Litentry: ".encode();
			let mut account_vec = account.encode();
			let mut expiring_block_number_vec = expiring_block_number.encode();

			bytes.append(&mut account_vec);
			bytes.append(&mut expiring_block_number_vec);
			bytes
		}

		/// Assemble the message that the user has signed
		/// Format: "Link Litentry: " + network_type + Litentry account + expiring block number
		fn generate_sub_raw_message(
			account: &T::AccountId,
			network_type: NetworkType,
			expiring_block_number: T::BlockNumber,
		) -> Vec<u8> {
			let mut bytes = b"Link Litentry: ".encode();
			let mut network_type_vec = network_type.encode();
			let mut account_vec = account.encode();
			let mut expiring_block_number_vec = expiring_block_number.encode();

			bytes.append(&mut network_type_vec);
			bytes.append(&mut account_vec);
			bytes.append(&mut expiring_block_number_vec);
			bytes
		}

		pub fn do_link_eth(
			account: T::AccountId,
			index: u32,
			addr_expected: EthAddress,
			expiring_block_number: T::BlockNumber,
			layer_one_blocknumber: T::BlockNumber,
			sig: Signature,
		) -> DispatchResultWithPostInfo {
			ensure!(expiring_block_number > layer_one_blocknumber, Error::<T>::LinkRequestExpired);
			ensure!(
				(expiring_block_number - layer_one_blocknumber)
					< T::BlockNumber::from(EXPIRING_BLOCK_NUMBER_MAX),
				Error::<T>::InvalidExpiringBlockNumber
			);

			let bytes = Self::generate_eth_raw_message(&account, expiring_block_number);

			let hash =
				util_eth::eth_data_hash(bytes).map_err(|_| Error::<T>::UnexpectedEthMsgLength)?;

			let mut msg = [0u8; 32];
			msg[..32].copy_from_slice(&hash[..32]);

			let addr =
				util_eth::addr_from_sig(msg, sig).map_err(|_| Error::<T>::EcdsaRecoverFailure)?;
			ensure!(addr == addr_expected, Error::<T>::UnexpectedAddress);

			EthereumLink::<T>::mutate(&account, |addrs| {
				let index = index as usize;
				// NOTE: allow linking `MAX_ETH_LINKS` eth addresses.
				if (index >= addrs.len()) && (addrs.len() != MAX_ETH_LINKS) {
					addrs.push(addr);
				} else if (index >= addrs.len()) && (addrs.len() == MAX_ETH_LINKS) {
					addrs[MAX_ETH_LINKS - 1] = addr;
				} else {
					addrs[index] = addr;
				}
			});

			Self::deposit_event(Event::EthAddressLinked(account, addr.to_vec()));

			Ok(().into())
		}

		pub fn do_link_sub(
			account: T::AccountId,
			index: u32,
			network_type: NetworkType,
			linked_account: T::AccountId,
			expiring_block_number: T::BlockNumber,
			layer_one_blocknumber: T::BlockNumber,
			multi_sig: MultiSignature,
		) -> DispatchResultWithPostInfo {
			ensure!(expiring_block_number > layer_one_blocknumber, Error::<T>::LinkRequestExpired);
			ensure!(
				(expiring_block_number - layer_one_blocknumber)
					< T::BlockNumber::from(EXPIRING_BLOCK_NUMBER_MAX),
				Error::<T>::InvalidExpiringBlockNumber
			);

			let bytes = Self::generate_sub_raw_message(
				&linked_account,
				network_type,
				expiring_block_number,
			);

			// get the public key
			let account_vec = linked_account.encode();
			ensure!(account_vec.len() == 32, Error::<T>::UnexpectedAccountId);

			let mut public_key = [0u8; 32];
			public_key[..32].copy_from_slice(&account_vec[..32]);

			// verify signature according to encryption type
			match multi_sig {
				MultiSignature::Sr25519Signature(sig) => {
					let msg = sp_io::hashing::keccak_256(&bytes);

					ensure!(
						sp_io::crypto::sr25519_verify(
							&sr25519::Signature(sig),
							&msg,
							&sr25519::Public(public_key)
						),
						Error::<T>::WrongSignature
					);
				},
				MultiSignature::Ed25519Signature(sig) => {
					let msg = sp_io::hashing::keccak_256(&bytes);

					ensure!(
						sp_io::crypto::ed25519_verify(
							&ed25519::Signature(sig),
							&msg,
							&ed25519::Public(public_key)
						),
						Error::<T>::WrongSignature
					);
				},
				MultiSignature::EcdsaSignature(sig) => {
					let msg = sp_io::hashing::blake2_256(&bytes);

					let recovered_public_key =
						sp_io::crypto::secp256k1_ecdsa_recover_compressed(&sig, &msg)
							.map_err(|_| Error::<T>::UnexpectedAddress)?;
					let hashed_pk = sp_io::hashing::blake2_256(&recovered_public_key);

					ensure!(public_key == hashed_pk, Error::<T>::WrongSignature);
				},
			}

			let new_address = LinkedSubAccount { network_type, account_id: linked_account };

			// insert new linked account into storage
			SubLink::<T>::mutate(&account, |addrs| {
				let index = index as usize;
				if (index >= addrs.len()) && (addrs.len() != MAX_SUB_LINKS) {
					addrs.push(new_address.clone());
				} else if (index >= addrs.len()) && (addrs.len() == MAX_SUB_LINKS) {
					addrs[MAX_SUB_LINKS - 1] = new_address.clone();
				} else {
					addrs[index] = new_address.clone();
				}
			});

			Self::deposit_event(Event::SubAddressLinked(account, new_address));

			Ok(().into())
		}
	}
}
