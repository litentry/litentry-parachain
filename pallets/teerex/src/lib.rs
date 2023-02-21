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

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
	dispatch::{DispatchErrorWithPostInfo, DispatchResult, DispatchResultWithPostInfo},
	ensure,
	traits::{Currency, ExistenceRequirement, Get, OnTimestampSet},
};
use frame_system::{self, ensure_signed};
use sp_core::H256;
use sp_runtime::traits::SaturatedConversion;

#[cfg(not(feature = "skip-ias-check"))]
use sp_runtime::traits::CheckedSub;

use sp_std::{prelude::*, str};
use teerex_primitives::*;

use sgx_verify::{
	deserialize_enclave_identity, deserialize_tcb_info, extract_certs, verify_certificate_chain,
};

#[cfg(not(feature = "skip-ias-check"))]
use sgx_verify::{verify_dcap_quote, verify_ias_report, SgxReport};

pub use crate::weights::WeightInfo;
use teerex_primitives::SgxBuildMode;

// Disambiguate associated types
pub type AccountId<T> = <T as frame_system::Config>::AccountId;
pub type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountId<T>>>::Balance;

pub use pallet::*;

const MAX_RA_REPORT_LEN: usize = 5244;
const MAX_DCAP_QUOTE_LEN: usize = 5000;
const MAX_URL_LEN: usize = 256;
/// Maximum number of topics for the `publish_hash` call.
const TOPICS_LIMIT: usize = 5;
/// Maximum number of bytes for the `data` in the `publish_hash` call.
const DATA_LENGTH_LIMIT: usize = 100;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::config]
	pub trait Config: frame_system::Config + timestamp::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Currency: Currency<<Self as frame_system::Config>::AccountId>;
		type MomentsPerDay: Get<Self::Moment>;
		type WeightInfo: WeightInfo;
		/// origin to manage enclave and parameters
		type EnclaveAdminOrigin: EnsureOrigin<Self::RuntimeOrigin>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		AddedEnclave(T::AccountId, Vec<u8>),
		RemovedEnclave(T::AccountId),
		Forwarded(ShardIdentifier),
		ShieldFunds(Vec<u8>),
		UnshieldedFunds(T::AccountId),
		ProcessedParentchainBlock(T::AccountId, H256, H256, T::BlockNumber),
		SetHeartbeatTimeout(u64),
		UpdatedScheduledEnclave(u32, MrEnclave),
		RemovedScheduledEnclave(u32),
		/// An enclave with [mr_enclave] has published some [hash] with some metadata [data].
		PublishedHash {
			mr_enclave: MrEnclave,
			hash: H256,
			data: Vec<u8>,
		},
	}

	// Watch out: we start indexing with 1 instead of zero in order to
	// avoid ambiguity between Null and 0.
	#[pallet::storage]
	#[pallet::getter(fn enclave)]
	pub type EnclaveRegistry<T: Config> =
		StorageMap<_, Blake2_128Concat, u64, Enclave<T::AccountId, Vec<u8>>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn enclave_count)]
	pub type EnclaveCount<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn quoting_enclave)]
	pub type QuotingEnclaveRegistry<T: Config> = StorageValue<_, QuotingEnclave, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn tcb_info)]
	pub type TcbInfo<T: Config> =
		StorageMap<_, Blake2_128Concat, Fmspc, TcbInfoOnChain, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn enclave_index)]
	pub type EnclaveIndex<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn confirmed_calls)]
	pub type ExecutedCalls<T: Config> = StorageMap<_, Blake2_128Concat, H256, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn allow_sgx_debug_mode)]
	pub type AllowSGXDebugMode<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::type_value]
	pub fn HeartbeatTimeoutDefault<T: Config>() -> T::Moment {
		T::Moment::saturated_from::<u64>(172_800_000) // default 48h
	}

	// keep track of a list of scheduled/allowed enchalves, mainly used for enclave updates,
	// can only be modified by EnclaveAdminOrigin
	// sidechain_block_number -> expected MrEnclave
	//
	// about the first time enclave registration:
	// prior to `register_enclave` this map needs to be populated with (0, expected-mrenclave),
	// otherwise the registration will fail
	//
	// Theorectically we could always push the enclave in `register_enclave`, but the problem is
	// anyone could try to register it as long as the enclave is remotely attested:
	// see https://github.com/litentry/litentry-parachain/issues/1163
	// so we need an "enclave whitelist" anyway
	#[pallet::storage]
	#[pallet::getter(fn scheduled_enclave)]
	pub type ScheduledEnclave<T: Config> = StorageMap<_, Blake2_128Concat, u32, MrEnclave>;

	#[pallet::storage]
	#[pallet::getter(fn heartbeat_timeout)]
	pub type HeartbeatTimeout<T: Config> =
		StorageValue<_, T::Moment, ValueQuery, HeartbeatTimeoutDefault<T>>;

	#[pallet::genesis_config]
	#[cfg_attr(feature = "std", derive(Default))]
	pub struct GenesisConfig {
		pub allow_sgx_debug_mode: bool,
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			AllowSGXDebugMode::<T>::put(self.allow_sgx_debug_mode);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		// Needed for the conversion of `mr_enclave` to a `Hash`.
		// The condition holds for all known chains.
		<T as frame_system::Config>::Hash: From<[u8; 32]>,
	{
		// the integritee-service wants to register his enclave
		#[pallet::call_index(0)]
		#[pallet::weight((<T as Config>::WeightInfo::register_enclave(), DispatchClass::Normal, Pays::Yes))]
		pub fn register_enclave(
			origin: OriginFor<T>,
			ra_report: Vec<u8>,
			worker_url: Vec<u8>,
			shielding_key: Option<Vec<u8>>,
			vc_pubkey: Option<Vec<u8>>,
		) -> DispatchResultWithPostInfo {
			log::info!("teerex: called into runtime call register_enclave()");

			let sender = ensure_signed(origin)?;
			ensure!(ra_report.len() <= MAX_RA_REPORT_LEN, <Error<T>>::RaReportTooLong);
			ensure!(worker_url.len() <= MAX_URL_LEN, <Error<T>>::EnclaveUrlTooLong);
			log::info!("teerex: parameter length ok");

			#[cfg(not(feature = "skip-ias-check"))]
			let enclave = Self::verify_report(&sender, ra_report.clone()).map(|report| {
				log::debug!("[teerex] isv_enclave_quote = {:?}", report.metadata.isv_enclave_quote);

				Enclave::new(
					sender.clone(),
					report.mr_enclave,
					report.timestamp,
					worker_url.clone(),
					shielding_key,
					vc_pubkey,
					report.build_mode,
				)
			})?;

			#[cfg(not(feature = "skip-ias-check"))]
			if !<AllowSGXDebugMode<T>>::get() && enclave.sgx_mode == SgxBuildMode::Debug {
				log::error!("substraTEE_registry: debug mode is not allowed to attest!");
				return Err(<Error<T>>::SgxModeNotAllowed.into())
			}

			#[cfg(feature = "skip-ias-check")]
			log::warn!("[teerex]: Skipping remote attestation check. Only dev-chains are allowed to do this!");

			#[cfg(feature = "skip-ias-check")]
			let enclave = Enclave::new(
				sender.clone(),
				// insert mrenclave if the ra_report represents one, otherwise insert default
				<MrEnclave>::decode(&mut ra_report.as_slice()).unwrap_or_default(),
				<timestamp::Pallet<T>>::get().saturated_into(),
				worker_url.clone(),
				shielding_key,
				vc_pubkey,
				SgxBuildMode::default(),
			);

			// TODO: imagine this fn is not called for the first time (e.g. when worker restarts),
			//       should we check the current sidechain_blocknumber >= registered
			// sidechain_blocknumber?
			#[cfg(not(feature = "skip-scheduled-enclave-check"))]
			ensure!(
				ScheduledEnclave::<T>::iter_values().any(|m| m == enclave.mr_enclave),
				Error::<T>::EnclaveNotInSchedule
			);

			Self::add_enclave(&sender, &enclave)?;
			Self::deposit_event(Event::AddedEnclave(sender, worker_url));

			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight((<T as Config>::WeightInfo::unregister_enclave(), DispatchClass::Normal, Pays::Yes))]
		pub fn unregister_enclave(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			log::info!("teerex: called into runtime call unregister_enclave()");
			let sender = ensure_signed(origin)?;

			Self::remove_enclave(&sender)?;
			Self::deposit_event(Event::RemovedEnclave(sender));

			Ok(().into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight((<T as Config>::WeightInfo::call_worker(), DispatchClass::Normal, Pays::Yes))]
		pub fn call_worker(origin: OriginFor<T>, request: Request) -> DispatchResult {
			let _sender = ensure_signed(origin)?;
			log::info!("call_worker with {:?}", request);
			Self::deposit_event(Event::Forwarded(request.shard));
			Ok(())
		}

		/// The integritee worker calls this function for every processed parentchain_block to
		/// confirm a state update.
		#[pallet::call_index(3)]
		#[pallet::weight((<T as Config>::WeightInfo::confirm_processed_parentchain_block(), DispatchClass::Normal, Pays::Yes))]
		pub fn confirm_processed_parentchain_block(
			origin: OriginFor<T>,
			block_hash: H256,
			block_number: T::BlockNumber,
			trusted_calls_merkle_root: H256,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			Self::ensure_registered_enclave(&sender)?;
			log::debug!(
				"Processed parentchain block confirmed for mrenclave {:?}, block hash {:?}",
				sender,
				block_hash
			);

			let sender_index = <EnclaveIndex<T>>::get(sender.clone());
			let mut sender_enclave =
				<EnclaveRegistry<T>>::get(sender_index).ok_or(Error::<T>::EmptyEnclaveRegistry)?;
			sender_enclave.timestamp = <timestamp::Pallet<T>>::get().saturated_into();
			<EnclaveRegistry<T>>::insert(sender_index, sender_enclave);

			Self::deposit_event(Event::ProcessedParentchainBlock(
				sender,
				block_hash,
				trusted_calls_merkle_root,
				block_number,
			));
			Ok(().into())
		}

		/// Sent by a client who requests to get shielded funds managed by an enclave. For this
		/// on-chain balance is sent to the bonding_account of the enclave. The bonding_account does
		/// not have a private key as the balance on this account is exclusively managed from
		/// withing the pallet_teerex. Note: The bonding_account is bit-equivalent to the worker
		/// shard.
		#[pallet::call_index(4)]
		#[pallet::weight((1000, DispatchClass::Normal, Pays::No))]
		pub fn shield_funds(
			origin: OriginFor<T>,
			incognito_account_encrypted: Vec<u8>,
			amount: BalanceOf<T>,
			bonding_account: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			T::Currency::transfer(
				&sender,
				&bonding_account,
				amount,
				ExistenceRequirement::AllowDeath,
			)?;
			Self::deposit_event(Event::ShieldFunds(incognito_account_encrypted));
			Ok(().into())
		}

		/// Sent by enclaves only as a result of an `unshield` request from a client to an enclave.
		#[pallet::call_index(5)]
		#[pallet::weight((1000, DispatchClass::Normal, Pays::No))]
		pub fn unshield_funds(
			origin: OriginFor<T>,
			public_account: T::AccountId,
			amount: BalanceOf<T>,
			bonding_account: T::AccountId,
			call_hash: H256,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			Self::ensure_registered_enclave(&sender)?;
			let sender_enclave = Self::get_enclave(&sender)?;

			ensure!(
				sender_enclave.mr_enclave.encode() == bonding_account.encode(),
				<Error<T>>::WrongMrenclaveForBondingAccount
			);

			if !<ExecutedCalls<T>>::contains_key(call_hash) {
				log::info!("Executing unshielding call: {:?}", call_hash);
				T::Currency::transfer(
					&bonding_account,
					&public_account,
					amount,
					ExistenceRequirement::AllowDeath,
				)?;
				<ExecutedCalls<T>>::insert(call_hash, 0);
				Self::deposit_event(Event::UnshieldedFunds(public_account));
			} else {
				log::info!("Already executed unshielding call: {:?}", call_hash);
			}

			<ExecutedCalls<T>>::mutate(call_hash, |confirmations| *confirmations += 1);
			Ok(().into())
		}

		#[pallet::call_index(6)]
		#[pallet::weight((1000, DispatchClass::Normal, Pays::No))]
		pub fn set_heartbeat_timeout(
			origin: OriginFor<T>,
			timeout: u64,
		) -> DispatchResultWithPostInfo {
			T::EnclaveAdminOrigin::ensure_origin(origin)?;
			<HeartbeatTimeout<T>>::put(T::Moment::saturated_from(timeout));
			Self::deposit_event(Event::SetHeartbeatTimeout(timeout));
			Ok(().into())
		}

		#[pallet::call_index(7)]
		#[pallet::weight((<T as Config>::WeightInfo::register_dcap_enclave(), DispatchClass::Normal, Pays::Yes))]
		pub fn register_dcap_enclave(
			origin: OriginFor<T>,
			dcap_quote: Vec<u8>,
			worker_url: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			log::info!("teerex: called into runtime call register_dcap_enclave()");
			let sender = ensure_signed(origin)?;
			ensure!(dcap_quote.len() <= MAX_DCAP_QUOTE_LEN, <Error<T>>::RaReportTooLong);
			ensure!(worker_url.len() <= MAX_URL_LEN, <Error<T>>::EnclaveUrlTooLong);
			log::info!("teerex: parameter length ok");

			let dummy_shielding_key: Option<Vec<u8>> = Default::default();
			let dummy_vc_pubkey: Option<Vec<u8>> = Default::default();
			#[cfg(not(feature = "skip-ias-check"))]
			let enclave = Self::verify_dcap_quote(&sender, dcap_quote).map(|report| {
				Enclave::new(
					sender.clone(),
					report.mr_enclave,
					report.timestamp,
					worker_url.clone(),
					dummy_shielding_key,
					dummy_vc_pubkey,
					report.build_mode,
				)
			})?;

			#[cfg(not(feature = "skip-ias-check"))]
			if !<AllowSGXDebugMode<T>>::get() && enclave.sgx_mode == SgxBuildMode::Debug {
				log::error!("substraTEE_registry: debug mode is not allowed to attest!");
				return Err(<Error<T>>::SgxModeNotAllowed.into())
			}

			#[cfg(feature = "skip-ias-check")]
			log::warn!("[teerex]: Skipping remote attestation check. Only dev-chains are allowed to do this!");

			#[cfg(feature = "skip-ias-check")]
			let enclave = Enclave::new(
				sender.clone(),
				// insert mrenclave if the ra_report represents one, otherwise insert default
				<MrEnclave>::decode(&mut dcap_quote.as_slice()).unwrap_or_default(),
				<timestamp::Pallet<T>>::get().saturated_into(),
				worker_url.clone(),
				dummy_shielding_key,
				dummy_vc_pubkey,
				SgxBuildMode::default(),
			);

			Self::add_enclave(&sender, &enclave)?;
			Self::deposit_event(Event::AddedEnclave(sender, worker_url));
			Ok(().into())
		}

		#[pallet::call_index(8)]
		#[pallet::weight((1000, DispatchClass::Normal, Pays::No))]
		pub fn update_scheduled_enclave(
			origin: OriginFor<T>,
			sidechain_block_number: u32,
			mr_enclave: MrEnclave,
		) -> DispatchResultWithPostInfo {
			T::EnclaveAdminOrigin::ensure_origin(origin)?;
			ScheduledEnclave::<T>::insert(sidechain_block_number, mr_enclave);
			Self::deposit_event(Event::UpdatedScheduledEnclave(sidechain_block_number, mr_enclave));
			Ok(().into())
		}

		#[pallet::call_index(9)]
		#[pallet::weight((<T as Config>::WeightInfo::register_quoting_enclave(), DispatchClass::Normal, Pays::Yes))]
		pub fn register_quoting_enclave(
			origin: OriginFor<T>,
			enclave_identity: Vec<u8>,
			signature: Vec<u8>,
			certificate_chain: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			log::info!("teerex: called into runtime call register_quoting_enclave()");
			// Quoting enclaves are registered globally and not for a specific sender
			let _sender = ensure_signed(origin)?;
			let quoting_enclave =
				Self::verify_quoting_enclave(enclave_identity, signature, certificate_chain)?;
			<QuotingEnclaveRegistry<T>>::put(quoting_enclave);
			Ok(().into())
		}

		#[pallet::call_index(10)]
		#[pallet::weight((1000, DispatchClass::Normal, Pays::No))]
		pub fn remove_scheduled_enclave(
			origin: OriginFor<T>,
			sidechain_block_number: u32,
		) -> DispatchResultWithPostInfo {
			T::EnclaveAdminOrigin::ensure_origin(origin)?;
			ensure!(
				ScheduledEnclave::<T>::contains_key(sidechain_block_number),
				Error::<T>::ScheduledEnclaveNotExist
			);
			// remove
			ScheduledEnclave::<T>::remove(sidechain_block_number);
			Self::deposit_event(Event::RemovedScheduledEnclave(sidechain_block_number));
			Ok(().into())
		}

		#[pallet::call_index(11)]
		#[pallet::weight((<T as Config>::WeightInfo::register_dcap_enclave(), DispatchClass::Normal, Pays::Yes))]
		pub fn register_tcb_info(
			origin: OriginFor<T>,
			tcb_info: Vec<u8>,
			signature: Vec<u8>,
			certificate_chain: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			log::info!("teerex: called into runtime call register_tcb_info()");
			// TCB info is registered globally and not for a specific sender
			let _sender = ensure_signed(origin)?;
			let (fmspc, on_chain_info) =
				Self::verify_tcb_info(tcb_info, signature, certificate_chain)?;
			<TcbInfo<T>>::insert(fmspc, on_chain_info);
			Ok(().into())
		}

		/// Publish a hash as a result of an arbitrary enclave operation.
		///
		/// The `mrenclave` of the origin will be used as an event topic a client can subscribe to.
		/// `extra_topics`, if any, will be used as additional event topics.
		///
		/// `data` can be anything worthwhile publishing related to the hash. If it is a
		/// utf8-encoded string, the UIs will usually even render the text.
		#[pallet::call_index(12)]
		#[pallet::weight((<T as Config>::WeightInfo::publish_hash(), DispatchClass::Normal, Pays::Yes))]
		pub fn publish_hash(
			origin: OriginFor<T>,
			hash: H256,
			extra_topics: Vec<T::Hash>,
			data: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;
			Self::ensure_registered_enclave(&sender)?;
			let enclave = Self::get_enclave(&sender)?;

			ensure!(extra_topics.len() <= TOPICS_LIMIT, <Error<T>>::TooManyTopics);
			ensure!(data.len() <= DATA_LENGTH_LIMIT, <Error<T>>::DataTooLong);

			let mut topics = extra_topics;
			topics.push(enclave.mr_enclave.into());

			Self::deposit_event_indexed(
				&topics,
				Event::PublishedHash { mr_enclave: enclave.mr_enclave, hash, data },
			);

			Ok(().into())
		}
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Failed to decode enclave signer.
		EnclaveSignerDecodeError,
		/// Sender does not match attested enclave in report.
		SenderIsNotAttestedEnclave,
		/// Verifying RA report failed.
		RemoteAttestationVerificationFailed,
		RemoteAttestationTooOld,
		/// The enclave cannot attest, because its building mode is not allowed.
		SgxModeNotAllowed,
		/// The enclave is not registered.
		EnclaveIsNotRegistered,
		/// The bonding account doesn't match the enclave.
		WrongMrenclaveForBondingAccount,
		/// The shard doesn't match the enclave.
		WrongMrenclaveForShard,
		/// The worker url is too long.
		EnclaveUrlTooLong,
		/// The Remote Attestation report is too long.
		RaReportTooLong,
		/// No enclave is registered.
		EmptyEnclaveRegistry,
		/// Can not found the desired scheduled enclave.
		ScheduledEnclaveNotExist,
		/// Enclave not in the scheduled list, therefore unexpected.
		EnclaveNotInSchedule,
		/// The provided collateral data is invalid
		CollateralInvalid,
		/// The number of `extra_topics` passed to `publish_hash` exceeds the limit.
		TooManyTopics,
		/// The length of the `data` passed to `publish_hash` exceeds the limit.
		DataTooLong,
	}
}

impl<T: Config> Pallet<T> {
	pub fn add_enclave(
		sender: &T::AccountId,
		enclave: &Enclave<T::AccountId, Vec<u8>>,
	) -> DispatchResultWithPostInfo {
		let enclave_idx = if <EnclaveIndex<T>>::contains_key(sender) {
			log::info!("Updating already registered enclave");
			<EnclaveIndex<T>>::get(sender)
		} else {
			let enclaves_count = Self::enclave_count()
				.checked_add(1)
				.ok_or("[Teerex]: Overflow adding new enclave to registry")?;
			<EnclaveIndex<T>>::insert(sender, enclaves_count);
			<EnclaveCount<T>>::put(enclaves_count);
			enclaves_count
		};

		<EnclaveRegistry<T>>::insert(enclave_idx, enclave);
		Ok(().into())
	}

	fn remove_enclave(sender: &T::AccountId) -> DispatchResultWithPostInfo {
		ensure!(<EnclaveIndex<T>>::contains_key(sender), <Error<T>>::EnclaveIsNotRegistered);
		let index_to_remove = <EnclaveIndex<T>>::take(sender);

		let enclaves_count = Self::enclave_count();
		let new_enclaves_count = enclaves_count
			.checked_sub(1)
			.ok_or("[Teerex]: Underflow removing an enclave from the registry")?;

		Self::swap_and_pop(index_to_remove, new_enclaves_count + 1)?;
		<EnclaveCount<T>>::put(new_enclaves_count);

		Ok(().into())
	}

	pub(crate) fn get_enclave(
		sender: &T::AccountId,
	) -> Result<Enclave<T::AccountId, Vec<u8>>, Error<T>> {
		let sender_index = <EnclaveIndex<T>>::get(sender);
		<EnclaveRegistry<T>>::get(sender_index).ok_or(Error::<T>::EmptyEnclaveRegistry)
	}

	/// Our list implementation would introduce holes in out list if if we try to remove elements
	/// from the middle. As the order of the enclave entries is not important, we use the swap and
	/// pop method to remove elements from the registry.
	fn swap_and_pop(index_to_remove: u64, new_enclaves_count: u64) -> DispatchResultWithPostInfo {
		if index_to_remove != new_enclaves_count {
			let last_enclave = <EnclaveRegistry<T>>::get(new_enclaves_count)
				.ok_or(Error::<T>::EmptyEnclaveRegistry)?;
			<EnclaveRegistry<T>>::insert(index_to_remove, &last_enclave);
			<EnclaveIndex<T>>::insert(last_enclave.pubkey, index_to_remove);
		}

		<EnclaveRegistry<T>>::remove(new_enclaves_count);

		Ok(().into())
	}

	fn unregister_silent_workers(now: T::Moment) {
		let minimum = (now - Self::heartbeat_timeout()).saturated_into::<u64>();
		let silent_workers = <EnclaveRegistry<T>>::iter()
			.filter(|e| e.1.timestamp < minimum)
			.map(|e| e.1.pubkey);
		for index in silent_workers {
			let result = Self::remove_enclave(&index);
			match result {
				Ok(_) => {
					log::info!("Unregister enclave because silent worker : {:?}", index);
					Self::deposit_event(Event::RemovedEnclave(index.clone()));
				},
				Err(e) => {
					log::error!("Cannot unregister enclave : {:?}", e);
				},
			};
		}
	}

	/// Check if the sender is a registered enclave
	pub fn ensure_registered_enclave(
		account: &T::AccountId,
	) -> Result<(), DispatchErrorWithPostInfo> {
		ensure!(<EnclaveIndex<T>>::contains_key(account), <Error<T>>::EnclaveIsNotRegistered);
		Ok(())
	}

	/// Deposit a pallets teerex event with the corresponding topics.
	///
	/// Handles the conversion to the overarching event type.
	fn deposit_event_indexed(topics: &[T::Hash], event: Event<T>) {
		<frame_system::Pallet<T>>::deposit_event_indexed(
			topics,
			<T as Config>::RuntimeEvent::from(event).into(),
		)
	}

	#[cfg(not(feature = "skip-ias-check"))]
	fn verify_report(
		sender: &T::AccountId,
		ra_report: Vec<u8>,
	) -> Result<SgxReport, DispatchErrorWithPostInfo> {
		let report = verify_ias_report(&ra_report)
			.map_err(|_| <Error<T>>::RemoteAttestationVerificationFailed)?;
		log::info!("teerex: IAS report successfully verified");

		let enclave_signer = T::AccountId::decode(&mut &report.pubkey[..])
			.map_err(|_| <Error<T>>::EnclaveSignerDecodeError)?;

		ensure!(sender == &enclave_signer, <Error<T>>::SenderIsNotAttestedEnclave);

		// TODO: activate state checks as soon as we've fixed our setup #83
		// ensure!((report.status == SgxStatus::Ok) | (report.status ==
		// SgxStatus::ConfigurationNeeded),     "RA status is insufficient");
		// log::info!("teerex: status is acceptable");

		Self::ensure_timestamp_within_24_hours(report.timestamp)?;
		Ok(report)
	}

	#[cfg(not(feature = "skip-ias-check"))]
	fn verify_dcap_quote(
		sender: &T::AccountId,
		dcap_quote: Vec<u8>,
	) -> Result<SgxReport, DispatchErrorWithPostInfo> {
		let verification_time = <timestamp::Pallet<T>>::get();

		let qe = <QuotingEnclaveRegistry<T>>::get();
		let (fmspc, tcb_info, report) =
			verify_dcap_quote(&dcap_quote, verification_time.saturated_into(), &qe).map_err(
				|e| {
					log::warn!("verify_dcap_quote failed: {:?}", e);
					<Error<T>>::RemoteAttestationVerificationFailed
				},
			)?;

		log::info!("teerex: DCAP quote verified. FMSPC from quote: {:?}", fmspc);
		let tcb_info_on_chain = <TcbInfo<T>>::get(fmspc);
		ensure!(tcb_info_on_chain.verify_examinee(&tcb_info), "tcb_info is outdated");

		let enclave_signer = T::AccountId::decode(&mut &report.pubkey[..])
			.map_err(|_| <Error<T>>::EnclaveSignerDecodeError)?;
		ensure!(sender == &enclave_signer, <Error<T>>::SenderIsNotAttestedEnclave);

		// TODO: activate state checks as soon as we've fixed our setup #83
		// ensure!((report.status == SgxStatus::Ok) | (report.status ==
		// SgxStatus::ConfigurationNeeded),     "RA status is insufficient");
		// log::info!("teerex: status is acceptable");

		Ok(report)
	}

	fn verify_quoting_enclave(
		enclave_identity: Vec<u8>,
		signature: Vec<u8>,
		certificate_chain: Vec<u8>,
	) -> Result<QuotingEnclave, DispatchErrorWithPostInfo> {
		let verification_time: u64 = <timestamp::Pallet<T>>::get().saturated_into();
		let certs = extract_certs(&certificate_chain);
		ensure!(certs.len() >= 2, "Certificate chain must have at least two certificates");
		let intermediate_slices: Vec<&[u8]> = certs[1..].iter().map(Vec::as_slice).collect();
		let leaf_cert =
			verify_certificate_chain(&certs[0], &intermediate_slices, verification_time)?;
		let enclave_identity =
			deserialize_enclave_identity(&enclave_identity, &signature, &leaf_cert)?;

		if enclave_identity.is_valid(verification_time.try_into().unwrap()) {
			Ok(enclave_identity.to_quoting_enclave())
		} else {
			Err(<Error<T>>::CollateralInvalid.into())
		}
	}

	pub fn verify_tcb_info(
		tcb_info: Vec<u8>,
		signature: Vec<u8>,
		certificate_chain: Vec<u8>,
	) -> Result<(Fmspc, TcbInfoOnChain), DispatchErrorWithPostInfo> {
		let verification_time: u64 = <timestamp::Pallet<T>>::get().saturated_into();
		let certs = extract_certs(&certificate_chain);
		ensure!(certs.len() >= 2, "Certificate chain must have at least two certificates");
		let intermediate_slices: Vec<&[u8]> = certs[1..].iter().map(Vec::as_slice).collect();
		let leaf_cert =
			verify_certificate_chain(&certs[0], &intermediate_slices, verification_time)?;
		let tcb_info = deserialize_tcb_info(&tcb_info, &signature, &leaf_cert)?;
		if tcb_info.is_valid(verification_time.try_into().unwrap()) {
			Ok(tcb_info.to_chain_tcb_info())
		} else {
			Err(<Error<T>>::CollateralInvalid.into())
		}
	}

	#[cfg(not(feature = "skip-ias-check"))]
	fn ensure_timestamp_within_24_hours(report_timestamp: u64) -> DispatchResultWithPostInfo {
		let elapsed_time = <timestamp::Pallet<T>>::get()
			.checked_sub(&T::Moment::saturated_from(report_timestamp))
			.ok_or("Underflow while calculating elapsed time since report creation")?;

		if elapsed_time < T::MomentsPerDay::get() {
			Ok(().into())
		} else {
			Err(<Error<T>>::RemoteAttestationTooOld.into())
		}
	}
}

impl<T: Config> OnTimestampSet<T::Moment> for Pallet<T> {
	fn on_timestamp_set(moment: T::Moment) {
		Self::unregister_silent_workers(moment)
	}
}

mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
pub mod weights;
