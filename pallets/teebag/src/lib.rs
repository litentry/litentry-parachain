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

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::too_many_arguments)]

use codec::Decode;
use frame_support::{
	dispatch::{DispatchErrorWithPostInfo, DispatchResult, DispatchResultWithPostInfo},
	ensure,
	pallet_prelude::*,
	traits::Get,
};
use frame_system::pallet_prelude::*;
use sp_core::H256;
use sp_runtime::traits::{CheckedSub, SaturatedConversion};
use sp_std::{prelude::*, str};

mod sgx_verify;
pub use sgx_verify::{
	deserialize_enclave_identity, deserialize_tcb_info, extract_certs,
	extract_tcb_info_from_raw_dcap_quote, verify_certificate_chain, verify_dcap_quote,
	verify_ias_report, SgxReport,
};

pub use pallet::*;

mod types;
pub use types::*;

mod quoting_enclave;
pub use quoting_enclave::*;

mod tcb;
pub use tcb::*;

const MAX_RA_REPORT_LEN: usize = 5244;
const MAX_DCAP_QUOTE_LEN: usize = 5000;
const MAX_URL_LEN: usize = 256;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_timestamp::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type MomentsPerDay: Get<Self::Moment>;
		/// The origin who can set the admin account
		type SetAdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		/// Maximum number of enclave identifiers allowed to be registered for a specific
		/// `worker_type`
		#[pallet::constant]
		type MaxEnclaveIdentifier: Get<u32>;
	}

	// TODO: maybe add more sidechain lifecycle events
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ModeSet {
			new_mode: OperationalMode,
		},
		AdminSet {
			new_admin: Option<T::AccountId>,
		},
		EnclaveAdded {
			who: T::AccountId,
			worker_type: WorkerType,
			url: Vec<u8>,
		},
		EnclaveRemoved {
			who: T::AccountId,
		},
		OpaqueTaskPosted {
			shard: ShardIdentifier,
		},
		ParentchainBlockProcessed {
			who: T::AccountId,
			block_number: T::BlockNumber,
			block_hash: H256,
			task_merkle_root: H256,
		},
		SidechainBlockFinalized {
			who: T::AccountId,
			sidechain_block_number: SidechainBlockNumber,
		},
		ScheduledEnclaveSet {
			worker_type: WorkerType,
			sidechain_block_number: SidechainBlockNumber,
			mrenclave: MrEnclave,
		},
		ScheduledEnclaveRemoved {
			worker_type: WorkerType,
			sidechain_block_number: SidechainBlockNumber,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// This operation needs the admin permission
		RequireAdminOrRoot,
		/// Failed to decode enclave signer.
		EnclaveSignerDecodeError,
		/// Sender does not match attested enclave in report.
		SenderIsNotAttestedEnclave,
		/// Verifying RA report failed.
		RemoteAttestationVerificationFailed,
		/// RA report is too old.
		RemoteAttestationTooOld,
		/// Invalid attestion type, e.g., an `Ignore` type under non-dev mode
		InvalidAttestationType,
		/// The enclave cannot attest, because its building mode is not allowed.
		InvalidSgxMode,
		/// The enclave doesn't exist.
		EnclaveNotExist,
		/// The enclave identifier doesn't exist.
		EnclaveIdentifierNotExist,
		/// The enclave identifier already exists.
		EnclaveIdentifierAlreadyExist,
		/// The worker type is unexpected, becuase e.g. when we try to re-register an
		/// existing enclave with a differnet worker type
		UnexpectedWorkerType,
		/// The shard doesn't match the enclave.
		WrongMrenclaveForShard,
		/// The worker url is too long.
		EnclaveUrlTooLong,
		/// The raw attestation data is too long.
		AttestationTooLong,
		/// The worker mode is unexpected, because e.g. a non-sidechain worker calls
		/// sidechain specific extrinsic
		UnexpectedWorkerMode,
		/// Can not found the desired scheduled enclave.
		ScheduledEnclaveNotExist,
		/// Enclave not in the scheduled list, therefore unexpected.
		EnclaveNotInSchedule,
		/// The provided collateral data is invalid.
		CollateralInvalid,
		/// MaxEnclaveIdentifier overflow.
		MaxEnclaveIdentifierOverflow,
		/// A proposed block is unexpected.
		ReceivedUnexpectedSidechainBlock,
		/// The value for the next finalization candidate is invalid.
		InvalidNextFinalizationCandidateBlockNumber,
	}

	#[pallet::storage]
	#[pallet::getter(fn admin)]
	pub type Admin<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn mode)]
	pub type Mode<T: Config> = StorageValue<_, OperationalMode, ValueQuery>;

	// records the enclave identifier list for each worker_type
	// T::AccountId is used as identifier as it's unique for each enclave
	#[pallet::storage]
	#[pallet::getter(fn enclave_identifier)]
	pub type EnclaveIdentifier<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		WorkerType,
		BoundedVec<T::AccountId, T::MaxEnclaveIdentifier>,
		ValueQuery,
	>;

	// registry that holds all registered enclaves, using T::AccountId as the key
	// having `worker_type` and `mrenclave` in each `Enclave` instance might seem a bit redundant,
	// but it increases flexibility where we **could** allow the same type of worker to have
	// distinct mrenclaves: e.g. when multiple versions of enclave are permitted in TEE cluster.
	//
	// It simplifies the lookup a bit too, otherwise we might need several storages.
	#[pallet::storage]
	#[pallet::getter(fn enclave_registry)]
	pub type EnclaveRegistry<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, Enclave, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn allow_sgx_debug_mode)]
	pub type AllowSGXDebugMode<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn quoting_enclave_registry)]
	pub type QuotingEnclaveRegistry<T: Config> = StorageValue<_, QuotingEnclave, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn tcb_info)]
	pub type TcbInfo<T: Config> =
		StorageMap<_, Blake2_128Concat, Fmspc, TcbInfoOnChain, ValueQuery>;

	// keep track of a list of scheduled/allowed enchalves, mainly used for enclave updates,
	// can only be modified by AdminOrigin
	// (worker_type, sidechain_block_number) -> expected MrEnclave
	//
	// about the first time enclave registration:
	// prior to `register_enclave` this map needs to be populated with ((worker_type, 0),
	// expected-mrenclave), otherwise the registration will fail
	//
	// For NON-sidechain worker_type, we still use this storage to whitelist mrenclave, in this case
	// the `SidechainBlockNumber` is ignored - you could always set it to 0.
	//
	// Theorectically we could always push the enclave in `register_enclave`, but we want to
	// limit the mrenclave that can be registered, as the parachain is supposed to process enclaves
	// with specific mrenclaves.
	#[pallet::storage]
	#[pallet::getter(fn scheduled_enclave)]
	pub type ScheduledEnclave<T: Config> =
		StorageMap<_, Blake2_128Concat, (WorkerType, SidechainBlockNumber), MrEnclave>;

	#[pallet::storage]
	#[pallet::getter(fn latest_sidechain_block_confirmation)]
	pub type LatestSidechainBlockConfirmation<T: Config> =
		StorageMap<_, Blake2_128Concat, ShardIdentifier, SidechainBlockConfirmation, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn sidechain_block_finalization_candidate)]
	pub type SidechainBlockFinalizationCandidate<T: Config> =
		StorageMap<_, Blake2_128Concat, ShardIdentifier, SidechainBlockNumber, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub allow_sgx_debug_mode: bool,
		pub admin: Option<T::AccountId>,
		pub mode: OperationalMode,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { allow_sgx_debug_mode: false, admin: None, mode: OperationalMode::Production }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
		fn build(&self) {
			AllowSGXDebugMode::<T>::put(self.allow_sgx_debug_mode);
			Mode::<T>::put(self.mode);
			if let Some(ref admin) = self.admin {
				Admin::<T>::put(admin);
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		// Needed for the conversion of `mrenclave` to a `Hash`.
		// The condition holds for all known chains.
		<T as frame_system::Config>::Hash: From<[u8; 32]>,
	{
		/// Set the admin account
		///
		/// Weights should be 2 DB writes: 1 for mode and 1 for event
		#[pallet::call_index(0)]
		#[pallet::weight((2 * T::DbWeight::get().write, DispatchClass::Normal, Pays::No))]
		pub fn set_admin(
			origin: OriginFor<T>,
			new_admin: T::AccountId,
		) -> DispatchResultWithPostInfo {
			T::SetAdminOrigin::ensure_origin(origin)?;
			Admin::<T>::put(new_admin.clone());
			Self::deposit_event(Event::AdminSet { new_admin: Some(new_admin) });
			Ok(Pays::No.into())
		}

		/// Set the mode
		///
		/// Weights should be 2 DB writes: 1 for mode and 1 for event
		#[pallet::call_index(1)]
		#[pallet::weight((2 * T::DbWeight::get().write, DispatchClass::Normal, Pays::Yes))]
		pub fn set_mode(
			origin: OriginFor<T>,
			new_mode: OperationalMode,
		) -> DispatchResultWithPostInfo {
			Self::ensure_admin_or_root(origin)?;
			Mode::<T>::put(new_mode);
			Self::deposit_event(Event::ModeSet { new_mode });
			Ok(().into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight((195_000_000, DispatchClass::Normal, Pays::No))]
		pub fn force_add_enclave(
			origin: OriginFor<T>,
			who: T::AccountId,
			enclave: Enclave,
		) -> DispatchResultWithPostInfo {
			Self::ensure_admin_or_root(origin)?;
			Self::add_enclave(&who, &enclave)?;
			Ok(Pays::No.into())
		}

		#[pallet::call_index(3)]
		#[pallet::weight((195_000_000, DispatchClass::Normal, Pays::No))]
		pub fn force_remove_enclave(
			origin: OriginFor<T>,
			who: T::AccountId,
		) -> DispatchResultWithPostInfo {
			Self::ensure_admin_or_root(origin)?;
			Self::remove_enclave(&who)?;
			Ok(Pays::No.into())
		}

		#[pallet::call_index(4)]
		#[pallet::weight((195_000_000, DispatchClass::Normal, Pays::No))]
		pub fn force_remove_enclave_by_mrenclave(
			origin: OriginFor<T>,
			mrenclave: MrEnclave,
		) -> DispatchResultWithPostInfo {
			Self::ensure_admin_or_root(origin)?;
			let accounts = EnclaveRegistry::<T>::iter()
				.filter_map(
					|(who, enclave)| {
						if enclave.mrenclave == mrenclave {
							Some(who)
						} else {
							None
						}
					},
				)
				.collect::<Vec<T::AccountId>>();

			for who in accounts.into_iter() {
				Self::remove_enclave(&who)?;
			}
			Ok(Pays::No.into())
		}

		#[pallet::call_index(5)]
		#[pallet::weight((195_000_000, DispatchClass::Normal, Pays::No))]
		pub fn force_remove_enclave_by_worker_type(
			origin: OriginFor<T>,
			worker_type: WorkerType,
		) -> DispatchResultWithPostInfo {
			Self::ensure_admin_or_root(origin)?;
			let accounts = EnclaveRegistry::<T>::iter()
				.filter_map(
					|(who, enclave)| {
						if enclave.worker_type == worker_type {
							Some(who)
						} else {
							None
						}
					},
				)
				.collect::<Vec<T::AccountId>>();

			for who in accounts.into_iter() {
				Self::remove_enclave(&who)?;
			}

			Ok(Pays::No.into())
		}

		#[pallet::call_index(6)]
		#[pallet::weight((195_000_000, DispatchClass::Normal, Pays::No))]
		pub fn set_scheduled_enclave(
			origin: OriginFor<T>,
			worker_type: WorkerType,
			sidechain_block_number: SidechainBlockNumber,
			mrenclave: MrEnclave,
		) -> DispatchResultWithPostInfo {
			Self::ensure_admin_or_root(origin)?;
			ScheduledEnclave::<T>::insert((worker_type, sidechain_block_number), mrenclave);
			Self::deposit_event(Event::ScheduledEnclaveSet {
				worker_type,
				sidechain_block_number,
				mrenclave,
			});
			Ok(().into())
		}

		#[pallet::call_index(7)]
		#[pallet::weight((195_000_000, DispatchClass::Normal, Pays::No))]
		pub fn remove_scheduled_enclave(
			origin: OriginFor<T>,
			worker_type: WorkerType,
			sidechain_block_number: SidechainBlockNumber,
		) -> DispatchResultWithPostInfo {
			Self::ensure_admin_or_root(origin)?;
			ensure!(
				ScheduledEnclave::<T>::contains_key((worker_type, sidechain_block_number)),
				Error::<T>::ScheduledEnclaveNotExist
			);
			ScheduledEnclave::<T>::remove((worker_type, sidechain_block_number));
			Self::deposit_event(Event::ScheduledEnclaveRemoved {
				worker_type,
				sidechain_block_number,
			});
			Ok(().into())
		}

		#[pallet::call_index(8)]
		#[pallet::weight((195_000_000, DispatchClass::Normal, Pays::Yes))]
		pub fn register_enclave(
			origin: OriginFor<T>,
			worker_type: WorkerType,
			attestation: Vec<u8>,
			worker_url: Vec<u8>,
			shielding_pubkey: Option<Vec<u8>>,
			vc_pubkey: Option<Vec<u8>>,
			attestation_type: AttestationType,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			ensure!(worker_url.len() <= MAX_URL_LEN, Error::<T>::EnclaveUrlTooLong);

			let mut enclave = Enclave::new(worker_type)
				.with_url(worker_url)
				.with_shielding_pubkey(shielding_pubkey)
				.with_vc_pubkey(vc_pubkey)
				.with_attestation_type(attestation_type);

			match attestation_type {
				AttestationType::Ignore => {
					ensure!(
						Self::mode() == OperationalMode::Development,
						Error::<T>::InvalidAttestationType
					);
					enclave.mrenclave =
						<MrEnclave>::decode(&mut attestation.as_slice()).unwrap_or_default();
					enclave.last_seen_timestamp = Self::now().saturated_into();
					enclave.sgx_build_mode = SgxBuildMode::default();
				},
				AttestationType::Ias => {
					let report = Self::verify_ias(&sender, attestation)?;
					enclave.mrenclave = report.mr_enclave;
					enclave.last_seen_timestamp = report.timestamp;
					enclave.sgx_build_mode = report.build_mode;
				},
				AttestationType::Dcap(_) => {
					let report = Self::verify_dcap(&sender, attestation)?;
					enclave.mrenclave = report.mr_enclave;
					enclave.last_seen_timestamp = report.timestamp;
					enclave.sgx_build_mode = report.build_mode;
				},
			};

			match Self::mode() {
				OperationalMode::Production | OperationalMode::Maintenance => {
					if !Self::allow_sgx_debug_mode() &&
						enclave.sgx_build_mode == SgxBuildMode::Debug
					{
						return Err(Error::<T>::InvalidSgxMode.into())
					}
					ensure!(
						ScheduledEnclave::<T>::iter_values().any(|m| m == enclave.mrenclave),
						Error::<T>::EnclaveNotInSchedule
					);
				},
				OperationalMode::Development => (),
			};
			Self::add_enclave(&sender, &enclave)?;
			Ok(().into())
		}

		#[pallet::call_index(9)]
		#[pallet::weight((195_000_000, DispatchClass::Normal, Pays::Yes))]
		pub fn unregister_enclave(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			Self::remove_enclave(&sender)?;
			Ok(().into())
		}

		#[pallet::call_index(10)]
		#[pallet::weight((195_000_000, DispatchClass::Normal, Pays::No))]
		pub fn register_quoting_enclave(
			origin: OriginFor<T>,
			enclave_identity: Vec<u8>,
			signature: Vec<u8>,
			certificate_chain: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			// Quoting enclaves are registered globally and not for a specific sender
			let _ = ensure_signed(origin)?;
			let quoting_enclave =
				Self::verify_quoting_enclave(enclave_identity, signature, certificate_chain)?;
			<QuotingEnclaveRegistry<T>>::put(quoting_enclave);
			Ok(().into())
		}

		#[pallet::call_index(11)]
		#[pallet::weight((195_000_000, DispatchClass::Normal, Pays::No))]
		pub fn register_tcb_info(
			origin: OriginFor<T>,
			tcb_info: Vec<u8>,
			signature: Vec<u8>,
			certificate_chain: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			// TCB info is registered globally and not for a specific sender
			let _ = ensure_signed(origin)?;
			let (fmspc, on_chain_info) =
				Self::verify_tcb_info(tcb_info, signature, certificate_chain)?;
			TcbInfo::<T>::insert(fmspc, on_chain_info);
			Ok(().into())
		}

		// ===============================================================================
		// Following extrinsics are for runtime communication between parachain and worker
		// ===============================================================================

		#[pallet::call_index(20)]
		#[pallet::weight((195_000_000, DispatchClass::Normal, Pays::Yes))]
		pub fn post_opaque_task(origin: OriginFor<T>, request: RsaRequest) -> DispatchResult {
			let _ = ensure_signed(origin)?;
			Self::deposit_event(Event::OpaqueTaskPosted { shard: request.shard });
			Ok(())
		}

		#[pallet::call_index(21)]
		#[pallet::weight((195_000_000, DispatchClass::Normal, Pays::No))]
		pub fn parentchain_block_processed(
			origin: OriginFor<T>,
			block_hash: H256,
			block_number: T::BlockNumber,
			task_merkle_root: H256,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			let mut enclave =
				EnclaveRegistry::<T>::get(&sender).ok_or(Error::<T>::EnclaveNotExist)?;
			enclave.last_seen_timestamp = Self::now().saturated_into();
			Self::deposit_event(Event::ParentchainBlockProcessed {
				who: sender,
				block_number,
				block_hash,
				task_merkle_root,
			});
			Ok(().into())
		}

		#[pallet::call_index(22)]
		#[pallet::weight((195_000_000, DispatchClass::Normal, Pays::No))]
		pub fn sidechain_block_imported(
			origin: OriginFor<T>,
			shard: ShardIdentifier,
			block_number: u64,
			next_finalization_candidate_block_number: u64,
			block_header_hash: H256,
		) -> DispatchResultWithPostInfo {
			let confirmation = SidechainBlockConfirmation { block_number, block_header_hash };

			let sender = ensure_signed(origin)?;
			let mut sender_enclave =
				EnclaveRegistry::<T>::get(&sender).ok_or(Error::<T>::EnclaveNotExist)?;

			ensure!(
				sender_enclave.mrenclave.as_ref() == shard.as_ref(),
				Error::<T>::WrongMrenclaveForShard
			);

			ensure!(
				sender_enclave.worker_mode == WorkerMode::Sidechain,
				Error::<T>::UnexpectedWorkerMode
			);

			sender_enclave.last_seen_timestamp = Self::now().saturated_into();

			// Simple logic for now: only accept blocks from first registered enclave.
			let primary_enclave_identifier =
				EnclaveIdentifier::<T>::get(sender_enclave.worker_type)
					.get(0)
					.cloned()
					.ok_or(Error::<T>::EnclaveIdentifierNotExist)?;
			if sender != primary_enclave_identifier {
				log::debug!(
					"Ignore block confirmation from non primary enclave identifier: {:?}, primary: {:?}",
					sender, primary_enclave_identifier
				);
				return Ok(().into())
			}

			let block_number = confirmation.block_number;
			let finalization_candidate_block_number =
				SidechainBlockFinalizationCandidate::<T>::try_get(shard).unwrap_or(1);

			ensure!(
				block_number == finalization_candidate_block_number,
				Error::<T>::ReceivedUnexpectedSidechainBlock
			);
			ensure!(
				next_finalization_candidate_block_number > finalization_candidate_block_number,
				Error::<T>::InvalidNextFinalizationCandidateBlockNumber
			);

			SidechainBlockFinalizationCandidate::<T>::insert(
				shard,
				next_finalization_candidate_block_number,
			);

			Self::finalize_block(sender, shard, confirmation);
			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
	fn ensure_admin_or_root(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
		ensure!(
			ensure_root(origin.clone()).is_ok() || Some(ensure_signed(origin)?) == Self::admin(),
			Error::<T>::RequireAdminOrRoot
		);
		Ok(().into())
	}

	fn add_enclave_identifier(worker_type: WorkerType, who: &T::AccountId) -> Result<(), Error<T>> {
		EnclaveIdentifier::<T>::try_mutate(worker_type, |v| {
			ensure!(!v.contains(who), Error::<T>::EnclaveIdentifierAlreadyExist);
			v.try_push(who.clone()).map_err(|_| Error::<T>::MaxEnclaveIdentifierOverflow)
		})
	}

	pub fn add_enclave(sender: &T::AccountId, enclave: &Enclave) -> Result<(), Error<T>> {
		match EnclaveRegistry::<T>::get(sender) {
			Some(old_enclave) => ensure!(
				old_enclave.worker_type == enclave.worker_type,
				Error::<T>::UnexpectedWorkerType
			),
			None => Self::add_enclave_identifier(enclave.worker_type, sender)?,
		};
		EnclaveRegistry::<T>::insert(sender, enclave);
		Self::deposit_event(Event::<T>::EnclaveAdded {
			who: sender.clone(),
			worker_type: enclave.worker_type,
			url: enclave.url.clone(),
		});
		Ok(())
	}

	fn remove_enclave(sender: &T::AccountId) -> DispatchResultWithPostInfo {
		let enclave = EnclaveRegistry::<T>::get(sender).ok_or(Error::<T>::EnclaveNotExist)?;

		EnclaveIdentifier::<T>::try_mutate(enclave.worker_type, |v| {
			ensure!(v.contains(sender), Error::<T>::EnclaveIdentifierNotExist);
			v.retain(|e| e != sender);
			Ok::<(), DispatchErrorWithPostInfo>(())
		})?;

		EnclaveRegistry::<T>::remove(sender);
		Self::deposit_event(Event::<T>::EnclaveRemoved { who: sender.clone() });
		Ok(().into())
	}

	pub fn enclave_count(worker_type: WorkerType) -> u32 {
		EnclaveIdentifier::<T>::get(worker_type).iter().count() as u32
	}

	fn verify_ias(
		sender: &T::AccountId,
		ra_report: Vec<u8>,
	) -> Result<SgxReport, DispatchErrorWithPostInfo> {
		ensure!(ra_report.len() <= MAX_RA_REPORT_LEN, Error::<T>::AttestationTooLong);
		let report = verify_ias_report(&ra_report)
			.map_err(|_| Error::<T>::RemoteAttestationVerificationFailed)?;

		let enclave_signer = T::AccountId::decode(&mut &report.pubkey[..])
			.map_err(|_| Error::<T>::EnclaveSignerDecodeError)?;

		ensure!(sender == &enclave_signer, Error::<T>::SenderIsNotAttestedEnclave);

		Self::ensure_timestamp_within_24_hours(report.timestamp)?;
		Ok(report)
	}

	fn verify_dcap(
		sender: &T::AccountId,
		dcap_quote: Vec<u8>,
	) -> Result<SgxReport, DispatchErrorWithPostInfo> {
		ensure!(dcap_quote.len() <= MAX_DCAP_QUOTE_LEN, Error::<T>::AttestationTooLong);
		let timestamp = Self::now();
		let qe = <QuotingEnclaveRegistry<T>>::get();
		let (fmspc, tcb_info, report) =
			verify_dcap_quote(&dcap_quote, timestamp.saturated_into(), &qe).map_err(|e| {
				log::warn!("verify_dcap_quote failed: {:?}", e);
				Error::<T>::RemoteAttestationVerificationFailed
			})?;

		let tcb_info_on_chain = <TcbInfo<T>>::get(fmspc);
		ensure!(tcb_info_on_chain.verify_examinee(&tcb_info), "tcb_info is outdated");

		let enclave_signer = T::AccountId::decode(&mut &report.pubkey[..])
			.map_err(|_| Error::<T>::EnclaveSignerDecodeError)?;
		ensure!(sender == &enclave_signer, Error::<T>::SenderIsNotAttestedEnclave);

		Ok(report)
	}

	fn verify_quoting_enclave(
		enclave_identity: Vec<u8>,
		signature: Vec<u8>,
		certificate_chain: Vec<u8>,
	) -> Result<QuotingEnclave, DispatchErrorWithPostInfo> {
		let verification_time: u64 = Self::now().saturated_into();
		let certs = extract_certs(&certificate_chain);
		ensure!(certs.len() >= 2, "Certificate chain must have at least two certificates");
		let intermediate_slices: Vec<webpki::types::CertificateDer> =
			certs[1..].iter().map(|c| c.as_slice().into()).collect();
		let leaf_cert_der = webpki::types::CertificateDer::from(certs[0].as_slice());
		let leaf_cert = webpki::EndEntityCert::try_from(&leaf_cert_der)
			.map_err(|_| "Failed to parse leaf certificate")?;
		verify_certificate_chain(&leaf_cert, &intermediate_slices, verification_time)?;
		let enclave_identity =
			deserialize_enclave_identity(&enclave_identity, &signature, &leaf_cert)?;

		if enclave_identity.is_valid(verification_time.try_into().unwrap()) {
			Ok(enclave_identity.to_quoting_enclave())
		} else {
			Err(Error::<T>::CollateralInvalid.into())
		}
	}

	pub fn verify_tcb_info(
		tcb_info: Vec<u8>,
		signature: Vec<u8>,
		certificate_chain: Vec<u8>,
	) -> Result<(Fmspc, TcbInfoOnChain), DispatchErrorWithPostInfo> {
		let verification_time: u64 = Self::now().saturated_into();
		let certs = extract_certs(&certificate_chain);
		ensure!(certs.len() >= 2, "Certificate chain must have at least two certificates");
		let intermediate_slices: Vec<webpki::types::CertificateDer> =
			certs[1..].iter().map(|c| c.as_slice().into()).collect();
		let leaf_cert_der = webpki::types::CertificateDer::from(certs[0].as_slice());
		let leaf_cert = webpki::EndEntityCert::try_from(&leaf_cert_der)
			.map_err(|_| "Failed to parse leaf certificate")?;
		verify_certificate_chain(&leaf_cert, &intermediate_slices, verification_time)?;
		let tcb_info = deserialize_tcb_info(&tcb_info, &signature, &leaf_cert)?;
		if tcb_info.is_valid(verification_time.try_into().unwrap()) {
			Ok(tcb_info.to_chain_tcb_info())
		} else {
			Err(Error::<T>::CollateralInvalid.into())
		}
	}

	fn ensure_timestamp_within_24_hours(report_timestamp: u64) -> DispatchResultWithPostInfo {
		let elapsed_time = Self::now()
			.checked_sub(&T::Moment::saturated_from(report_timestamp))
			.ok_or("Underflow while calculating elapsed time since report creation")?;

		if elapsed_time < T::MomentsPerDay::get() {
			Ok(().into())
		} else {
			Err(Error::<T>::RemoteAttestationTooOld.into())
		}
	}

	fn finalize_block(
		sender: T::AccountId,
		shard: ShardIdentifier,
		confirmation: SidechainBlockConfirmation,
	) {
		LatestSidechainBlockConfirmation::<T>::insert(shard, confirmation);
		Self::deposit_event(Event::SidechainBlockFinalized {
			who: sender,
			sidechain_block_number: confirmation.block_number,
		});
	}

	fn now() -> T::Moment {
		pallet_timestamp::Pallet::<T>::now()
	}
}

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
