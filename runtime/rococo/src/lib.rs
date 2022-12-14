// Copyright 2020-2022 Litentry Technologies GmbH.
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
#![allow(clippy::identity_op)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

#[cfg(feature = "runtime-benchmarks")]
#[macro_use]
extern crate frame_benchmarking;

use codec::{Decode, Encode, MaxEncodedLen};
use cumulus_pallet_parachain_system::RelayNumberStrictlyIncreases;
use frame_support::{
	construct_runtime, ord_parameter_types, parameter_types,
	traits::{
		ConstU16, ConstU32, ConstU64, ConstU8, Contains, ContainsLengthBound, Everything,
		InstanceFilter, SortedMembers,
	},
	weights::{constants::RocksDbWeight, ConstantMultiplier, IdentityFee, Weight},
	PalletId, RuntimeDebug,
};
use frame_system::{EnsureRoot, EnsureSignedBy};
use hex_literal::hex;

// for TEE
pub use pallet_balances::Call as BalancesCall;
pub use pallet_sidechain;
pub use pallet_teeracle;
pub use pallet_teerex;

use sp_api::impl_runtime_apis;
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::KeyTypeId, OpaqueMetadata};
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
use sp_runtime::{
	create_runtime_str, generic, impl_opaque_keys,
	traits::{AccountIdConversion, AccountIdLookup, BlakeTwo256, Block as BlockT, ConvertInto},
	transaction_validity::{TransactionSource, TransactionValidity},
	ApplyExtrinsicResult,
};
pub use sp_runtime::{MultiAddress, Perbill, Percent, Permill};
use sp_std::prelude::*;
#[cfg(feature = "std")]
use sp_version::NativeVersion;
use sp_version::RuntimeVersion;
// XCM Imports
use xcm_executor::XcmExecutor;

pub use constants::currency::deposit;
pub use primitives::{opaque, Index, *};
pub use runtime_common::currency::*;
use runtime_common::{
	impl_runtime_transaction_payment_fees, prod_or_fast, BlockHashCount, BlockLength,
	CouncilInstance, CouncilMembershipInstance, EnsureEnclaveSigner, EnsureRootOrAllCouncil,
	EnsureRootOrAllTechnicalCommittee, EnsureRootOrHalfCouncil, EnsureRootOrHalfTechnicalCommittee,
	EnsureRootOrTwoThirdsCouncil, EnsureRootOrTwoThirdsTechnicalCommittee, NegativeImbalance,
	RuntimeBlockWeights, SlowAdjustingFeeUpdate, TechnicalCommitteeInstance,
	TechnicalCommitteeMembershipInstance, MAXIMUM_BLOCK_WEIGHT,
};
use xcm_config::{XcmConfig, XcmOriginToTransactDispatchOrigin};

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

pub mod asset_config;
pub mod constants;
#[cfg(test)]
mod tests;
pub mod weights;
pub mod xcm_config;

/// The address format for describing accounts.
pub type Address = MultiAddress<AccountId, ()>;

/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;

/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;

/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;

/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	frame_system::CheckNonZeroSender<Runtime>,
	frame_system::CheckSpecVersion<Runtime>,
	frame_system::CheckTxVersion<Runtime>,
	frame_system::CheckGenesis<Runtime>,
	frame_system::CheckEra<Runtime>,
	frame_system::CheckNonce<Runtime>,
	frame_system::CheckWeight<Runtime>,
	pallet_transaction_payment::ChargeTransactionPayment<Runtime>,
);

/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;

/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;

/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<
	Runtime,
	Block,
	frame_system::ChainContext<Runtime>,
	Runtime,
	// see https://github.com/paritytech/substrate/pull/10043
	//
	// With this type the hooks of pallets will be executed
	// in the order that they are declared in `construct_runtime!`
	// it was reverse order before.
	// See the comment before collation related pallets too.
	AllPalletsWithSystem,
>;

impl_opaque_keys! {
	pub struct SessionKeys {
		pub aura: Aura,
	}
}

/// This runtime version.
#[sp_version::runtime_version]
pub const VERSION: RuntimeVersion = RuntimeVersion {
	// Note:
	// It's important to match `rococo-parachain-runtime`, which is runtime pkg name
	spec_name: create_runtime_str!("rococo-parachain"),
	impl_name: create_runtime_str!("rococo-parachain"),
	authoring_version: 1,
	// same versioning-mechanism as polkadot:
	// last digit is used for minor updates, like 9110 -> 9111 in polkadot
	spec_version: 9115,
	impl_version: 0,
	apis: RUNTIME_API_VERSIONS,
	transaction_version: 1,
	state_version: 0,
};

/// A timestamp: milliseconds since the unix epoch.
pub type Moment = u64;

/// The version information used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion { runtime_version: VERSION, can_author_with: Default::default() }
}

parameter_types! {
	pub const Version: RuntimeVersion = VERSION;

	pub const SS58Prefix: u16 = 131;
}

impl frame_system::Config for Runtime {
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type Call = Call;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = AccountIdLookup<AccountId, ()>;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// The ubiquitous event type.
	type Event = Event;
	/// The ubiquitous origin type.
	type Origin = Origin;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// Runtime version.
	type Version = Version;
	/// Converts a module to an index of this module in the runtime.
	type PalletInfo = PalletInfo;
	/// The data to be stored in an account.
	type AccountData = pallet_balances::AccountData<Balance>;
	/// What to do if a new account is created.
	type OnNewAccount = ();
	/// What to do if an account is fully reaped from the system.
	type OnKilledAccount = ();
	/// The weight of database operations that the runtime can invoke.
	type DbWeight = RocksDbWeight;
	/// The basic call filter to use in dispatchable.
	type BaseCallFilter = BaseCallFilter;
	/// Weight information for the extrinsics of this pallet.
	type SystemWeightInfo = weights::frame_system::WeightInfo<Runtime>;
	/// Block & extrinsics weights: base values and limits.
	type BlockWeights = RuntimeBlockWeights;
	/// The maximum length of a block (in bytes).
	type BlockLength = BlockLength;
	/// This is used as an identifier of the chain. 42 is the generic substrate prefix.
	type SS58Prefix = SS58Prefix;
	/// The action to take on a Runtime Upgrade
	type OnSetCode = cumulus_pallet_parachain_system::ParachainSetCode<Self>;
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
	// One storage item; key size is 32; value is size 4+4+16+32 bytes = 56 bytes.
	pub const DepositBase: Balance = deposit(1, 88);
	// Additional storage item size of 32 bytes.
	pub const DepositFactor: Balance = deposit(0, 32);
}

impl pallet_multisig::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type DepositBase = DepositBase;
	type DepositFactor = DepositFactor;
	type MaxSignatories = ConstU16<100>;
	type WeightInfo = weights::pallet_multisig::WeightInfo<Runtime>;
}

/// The type used to represent the kinds of proxying allowed.
#[derive(
	Copy,
	Clone,
	Eq,
	PartialEq,
	Ord,
	PartialOrd,
	Encode,
	Decode,
	RuntimeDebug,
	MaxEncodedLen,
	scale_info::TypeInfo,
)]
pub enum ProxyType {
	/// Fully permissioned proxy. Can execute any call on behalf of _proxied_.
	Any,
	/// Can execute any call that does not transfer funds, including asset transfers.
	NonTransfer,
	/// Proxy with the ability to reject time-delay proxy announcements.
	CancelProxy,
	/// Collator selection proxy. Can execute calls related to collator selection mechanism.
	Collator,
	/// Governance
	Governance,
}

impl Default for ProxyType {
	fn default() -> Self {
		Self::Any
	}
}

impl InstanceFilter<Call> for ProxyType {
	fn filter(&self, c: &Call) -> bool {
		match self {
			ProxyType::Any => true,
			ProxyType::NonTransfer => !matches!(
				c,
				Call::Balances(..) | Call::Vesting(pallet_vesting::Call::vested_transfer { .. })
			),
			ProxyType::CancelProxy => matches!(
				c,
				Call::Proxy(pallet_proxy::Call::reject_announcement { .. }) |
					Call::Utility(..) | Call::Multisig(..)
			),
			ProxyType::Collator =>
				matches!(c, Call::ParachainStaking(..) | Call::Utility(..) | Call::Multisig(..)),
			ProxyType::Governance => matches!(
				c,
				Call::Democracy(..) |
					Call::Council(..) | Call::TechnicalCommittee(..) |
					Call::Treasury(..)
			),
		}
	}
	fn is_superset(&self, o: &Self) -> bool {
		match (self, o) {
			(x, y) if x == y => true,
			(ProxyType::Any, _) => true,
			(_, ProxyType::Any) => false,
			(ProxyType::NonTransfer, _) => true,
			_ => false,
		}
	}
}

parameter_types! {
	// One storage item; key size 32, value size 8; .
	pub const ProxyDepositBase: Balance = deposit(1, 8);
	// Additional storage item size of 33 bytes.
	pub const ProxyDepositFactor: Balance = deposit(0, 33);
	pub const AnnouncementDepositBase: Balance = deposit(1, 8);
	pub const AnnouncementDepositFactor: Balance = deposit(0, 66);
}

impl pallet_proxy::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type Currency = Balances;
	type ProxyType = ProxyType;
	type ProxyDepositBase = ProxyDepositBase;
	type ProxyDepositFactor = ProxyDepositFactor;
	type MaxProxies = ConstU32<32>;
	type WeightInfo = weights::pallet_proxy::WeightInfo<Runtime>;
	type MaxPending = ConstU32<32>;
	type CallHasher = BlakeTwo256;
	type AnnouncementDepositBase = AnnouncementDepositBase;
	type AnnouncementDepositFactor = AnnouncementDepositFactor;
}

impl pallet_timestamp::Config for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = Moment;
	type OnTimestampSet = Teerex;
	type MinimumPeriod = ConstU64<{ SLOT_DURATION / 2 }>;
	type WeightInfo = weights::pallet_timestamp::WeightInfo<Runtime>;
}

impl pallet_authorship::Config for Runtime {
	type FindAuthor = pallet_session::FindAccountFromAuthorIndex<Self, Aura>;
	type UncleGenerations = ConstU32<0>;
	type FilterUncle = ();
	type EventHandler = (ParachainStaking,);
}

parameter_types! {
	pub MaximumSchedulerWeight: Weight = Perbill::from_percent(80) *
		RuntimeBlockWeights::get().max_block;
}

impl pallet_scheduler::Config for Runtime {
	type Event = Event;
	type Origin = Origin;
	type PalletsOrigin = OriginCaller;
	type Call = Call;
	type MaximumWeight = MaximumSchedulerWeight;
	type ScheduleOrigin = EnsureRoot<AccountId>;
	type MaxScheduledPerBlock = ConstU32<50>;
	type WeightInfo = weights::pallet_scheduler::WeightInfo<Runtime>;
	type OriginPrivilegeCmp = frame_support::traits::EqualPrivilegeOnly;
	type PreimageProvider = Preimage;
	type NoPreimagePostponement = ();
}

parameter_types! {
	pub const PreimageMaxSize: u32 = 4096 * 1024;
	pub const PreimageBaseDeposit: Balance = 1 * DOLLARS;
}

impl pallet_preimage::Config for Runtime {
	type WeightInfo = weights::pallet_preimage::WeightInfo<Runtime>;
	type Event = Event;
	type Currency = Balances;
	type ManagerOrigin = EnsureRoot<AccountId>;
	type MaxSize = PreimageMaxSize;
	type BaseDeposit = PreimageBaseDeposit;
	type ByteDeposit = PreimageByteDeposit;
}

parameter_types! {
	pub const ExistentialDeposit: Balance = EXISTENTIAL_DEPOSIT;
}

impl pallet_balances::Config for Runtime {
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = weights::pallet_balances::WeightInfo<Runtime>;
	type MaxLocks = ConstU32<50>;
	type MaxReserves = ConstU32<50>;
	type ReserveIdentifier = [u8; 8];
}

impl pallet_utility::Config for Runtime {
	type Event = Event;
	type Call = Call;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = ();
}

parameter_types! {
	pub const TransactionByteFee: Balance = MILLICENTS / 10;
}
impl_runtime_transaction_payment_fees!(constants);

impl pallet_transaction_payment::Config for Runtime {
	type Event = Event;
	type OnChargeTransaction =
		pallet_transaction_payment::CurrencyAdapter<Balances, DealWithFees<Runtime>>;
	type WeightToFee = IdentityFee<Balance>;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type FeeMultiplierUpdate = SlowAdjustingFeeUpdate<Self>;
	type OperationalFeeMultiplier = ConstU8<5>;
}

parameter_types! {
	pub LaunchPeriod: BlockNumber = prod_or_fast!(10 * MINUTES, 5 * MINUTES, "ROCOCO_LAUNCHPERIOD");
	pub VotingPeriod: BlockNumber = prod_or_fast!(10 * MINUTES, 5 * MINUTES, "ROCOCO_VOTINGPERIOD");
	pub FastTrackVotingPeriod: BlockNumber = prod_or_fast!(8 * MINUTES, 2 * MINUTES, "ROCOCO_FASTTRACKVOTINGPERIOD");
	pub const InstantAllowed: bool = true;
	pub const MinimumDeposit: Balance = 100 * DOLLARS;
	pub EnactmentPeriod: BlockNumber = prod_or_fast!(5 * MINUTES, 2 * MINUTES, "ROCOCO_ENACTMENTPERIOD");
	pub CooloffPeriod: BlockNumber = prod_or_fast!(10 * MINUTES, 2 * MINUTES, "ROCOCO_COOLOFFPERIOD");
	pub const PreimageByteDeposit: Balance = deposit(0, 1);
}

impl pallet_democracy::Config for Runtime {
	type Proposal = Call;
	type Event = Event;
	type Currency = Balances;
	type EnactmentPeriod = EnactmentPeriod;
	type LaunchPeriod = LaunchPeriod;
	type VotingPeriod = VotingPeriod;
	type VoteLockingPeriod = EnactmentPeriod; // Same as EnactmentPeriod
	type MinimumDeposit = MinimumDeposit;
	/// A straight majority of the council can decide what their next motion is.
	type ExternalOrigin = EnsureRootOrHalfCouncil;
	/// A super-majority can have the next scheduled referendum be a straight majority-carries vote.
	type ExternalMajorityOrigin = EnsureRootOrTwoThirdsCouncil;
	/// A unanimous council can have the next scheduled referendum be a straight default-carries
	/// (NTB) vote.
	type ExternalDefaultOrigin = EnsureRootOrAllCouncil;
	/// Two thirds of the technical committee can have an ExternalMajority/ExternalDefault vote
	/// be tabled immediately and with a shorter voting/enactment period.
	type FastTrackOrigin = EnsureRootOrTwoThirdsTechnicalCommittee;
	type InstantOrigin = EnsureRootOrAllTechnicalCommittee;
	type InstantAllowed = InstantAllowed;
	type FastTrackVotingPeriod = FastTrackVotingPeriod;
	// To cancel a proposal which has been passed, 2/3 of the council must agree to it.
	type CancellationOrigin = EnsureRootOrTwoThirdsCouncil;
	// To cancel a proposal before it has been passed, the technical committee must be unanimous or
	// Root must agree.
	type CancelProposalOrigin = EnsureRootOrAllTechnicalCommittee;
	type BlacklistOrigin = EnsureRoot<AccountId>;
	// Any single technical committee member may veto a coming council proposal, however they can
	// only do it once and it lasts only for the cool-off period.
	type VetoOrigin = pallet_collective::EnsureMember<AccountId, TechnicalCommitteeInstance>;
	type CooloffPeriod = CooloffPeriod;
	type PreimageByteDeposit = PreimageByteDeposit;
	type OperationalPreimageOrigin = pallet_collective::EnsureMember<AccountId, CouncilInstance>;
	type Slash = Treasury;
	type Scheduler = Scheduler;
	type PalletsOrigin = OriginCaller;
	type MaxVotes = ConstU32<100>;
	type WeightInfo = weights::pallet_democracy::WeightInfo<Runtime>;
	type MaxProposals = ConstU32<100>;
}

parameter_types! {
	pub const CouncilMotionDuration: BlockNumber = 3 * DAYS;
	pub const CouncilDefaultMaxMembers: u32 = 100;
}

impl pallet_collective::Config<CouncilInstance> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = CouncilMotionDuration;
	type MaxProposals = ConstU32<100>;
	type MaxMembers = CouncilDefaultMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;
}

impl pallet_membership::Config<CouncilMembershipInstance> for Runtime {
	type Event = Event;
	type AddOrigin = EnsureRootOrTwoThirdsCouncil;
	type RemoveOrigin = EnsureRootOrTwoThirdsCouncil;
	type SwapOrigin = EnsureRootOrTwoThirdsCouncil;
	type ResetOrigin = EnsureRootOrTwoThirdsCouncil;
	type PrimeOrigin = EnsureRootOrTwoThirdsCouncil;
	type MembershipInitialized = Council;
	type MembershipChanged = Council;
	type MaxMembers = CouncilDefaultMaxMembers;
	type WeightInfo = weights::pallet_membership::WeightInfo<Runtime>;
}

parameter_types! {
	pub const TechnicalMotionDuration: BlockNumber = 3 * DAYS;
}

impl pallet_collective::Config<TechnicalCommitteeInstance> for Runtime {
	type Origin = Origin;
	type Proposal = Call;
	type Event = Event;
	type MotionDuration = TechnicalMotionDuration;
	type MaxProposals = ConstU32<100>;
	type MaxMembers = CouncilDefaultMaxMembers;
	type DefaultVote = pallet_collective::PrimeDefaultVote;
	type WeightInfo = weights::pallet_collective::WeightInfo<Runtime>;
}

impl pallet_membership::Config<TechnicalCommitteeMembershipInstance> for Runtime {
	type Event = Event;
	type AddOrigin = EnsureRootOrTwoThirdsCouncil;
	type RemoveOrigin = EnsureRootOrTwoThirdsCouncil;
	type SwapOrigin = EnsureRootOrTwoThirdsCouncil;
	type ResetOrigin = EnsureRootOrTwoThirdsCouncil;
	type PrimeOrigin = EnsureRootOrTwoThirdsCouncil;
	type MembershipInitialized = TechnicalCommittee;
	type MembershipChanged = TechnicalCommittee;
	type MaxMembers = CouncilDefaultMaxMembers;
	type WeightInfo = weights::pallet_membership::WeightInfo<Runtime>;
}

parameter_types! {
	pub const ProposalBond: Permill = Permill::from_percent(5);
	pub const ProposalBondMinimum: Balance = 1 * DOLLARS;
	pub const ProposalBondMaximum: Balance = 20 * DOLLARS;
	pub SpendPeriod: BlockNumber = prod_or_fast!(10 * MINUTES, 2 * MINUTES, "ROCOCO_SPENDPERIOD");
	pub const Burn: Permill = Permill::from_percent(0);
	pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");

	pub const TipCountdown: BlockNumber = DAYS;
	pub const TipFindersFee: Percent = Percent::from_percent(5);
	pub TipReportDepositBase: Balance = deposit(1, 0);
	pub BountyDepositBase: Balance = deposit(1, 0);
	pub const BountyDepositPayoutDelay: BlockNumber = 4 * DAYS;
	pub const BountyUpdatePeriod: BlockNumber = 35 * DAYS;
	pub const CuratorDepositMultiplier: Permill = Permill::from_percent(50);
	pub CuratorDepositMin: Balance = DOLLARS;
	pub CuratorDepositMax: Balance = 100 * DOLLARS;
	pub BountyValueMinimum: Balance = 5 * DOLLARS;
	pub DataDepositPerByte: Balance = deposit(0, 1);
	pub const MaximumReasonLength: u32 = 8192;
}

impl pallet_treasury::Config for Runtime {
	type PalletId = TreasuryPalletId;
	type Currency = Balances;
	type ApproveOrigin = EnsureRootOrTwoThirdsCouncil;
	type RejectOrigin = EnsureRootOrHalfCouncil;
	type Event = Event;
	type OnSlash = Treasury;
	type ProposalBond = ProposalBond;
	type ProposalBondMinimum = ProposalBondMinimum;
	type ProposalBondMaximum = ProposalBondMaximum;
	type SpendOrigin = frame_support::traits::NeverEnsureOrigin<Balance>;
	type SpendPeriod = SpendPeriod;
	type Burn = Burn;
	type BurnDestination = ();
	// Rcococo bounty enabled
	type SpendFunds = Bounties;
	type WeightInfo = ();
	type MaxApprovals = ConstU32<100>;
}

pub struct CouncilProvider;
impl SortedMembers<AccountId> for CouncilProvider {
	fn contains(who: &AccountId) -> bool {
		Council::is_member(who)
	}

	fn sorted_members() -> Vec<AccountId> {
		Council::members()
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn add(_: &AccountId) {
		unimplemented!()
	}
}

impl ContainsLengthBound for CouncilProvider {
	fn max_len() -> usize {
		CouncilDefaultMaxMembers::get() as usize
	}
	fn min_len() -> usize {
		0
	}
}

impl pallet_bounties::Config for Runtime {
	type Event = Event;
	type BountyDepositBase = BountyDepositBase;
	type BountyDepositPayoutDelay = BountyDepositPayoutDelay;
	type BountyUpdatePeriod = BountyUpdatePeriod;
	type BountyValueMinimum = BountyValueMinimum;
	type CuratorDepositMultiplier = CuratorDepositMultiplier;
	type CuratorDepositMin = CuratorDepositMin;
	type CuratorDepositMax = CuratorDepositMax;
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type WeightInfo = ();
	type ChildBountyManager = ();
}

impl pallet_tips::Config for Runtime {
	type Event = Event;
	type DataDepositPerByte = DataDepositPerByte;
	type MaximumReasonLength = MaximumReasonLength;
	type Tippers = CouncilProvider;
	type TipCountdown = TipCountdown;
	type TipFindersFee = TipFindersFee;
	type TipReportDepositBase = TipReportDepositBase;
	type WeightInfo = ();
}

impl pallet_sudo::Config for Runtime {
	type Call = Call;
	type Event = Event;
}

parameter_types! {
	pub const ReservedXcmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
	pub const ReservedDmpWeight: Weight = MAXIMUM_BLOCK_WEIGHT.saturating_div(4);
}

impl cumulus_pallet_parachain_system::Config for Runtime {
	type Event = Event;
	type OnSystemEvent = ();
	type SelfParaId = parachain_info::Pallet<Runtime>;
	type DmpMessageHandler = DmpQueue;
	type ReservedDmpWeight = ReservedDmpWeight;
	type OutboundXcmpMessageSource = XcmpQueue;
	type XcmpMessageHandler = XcmpQueue;
	type ReservedXcmpWeight = ReservedXcmpWeight;
	type CheckAssociatedRelayNumber = RelayNumberStrictlyIncreases;
}

impl parachain_info::Config for Runtime {}

impl cumulus_pallet_aura_ext::Config for Runtime {}

impl cumulus_pallet_xcmp_queue::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ChannelInfo = ParachainSystem;
	// We use pallet_xcm to confirm the version of xcm
	type VersionWrapper = PolkadotXcm;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
	type ControllerOrigin = EnsureRoot<AccountId>;
	type ControllerOriginConverter = XcmOriginToTransactDispatchOrigin;
	type WeightInfo = weights::cumulus_pallet_xcmp_queue::WeightInfo<Runtime>;
}

impl cumulus_pallet_dmp_queue::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
}

parameter_types! {
	pub const Offset: u32 = 0;
}

impl pallet_session::Config for Runtime {
	type Event = Event;
	type ValidatorId = <Self as frame_system::Config>::AccountId;
	// we don't have stash and controller, thus we don't need the convert as well.
	type ValidatorIdOf = ConvertInto;
	type ShouldEndSession = ParachainStaking;
	type NextSessionRotation = ParachainStaking;
	type SessionManager = ParachainStaking;
	// Essentially just Aura, but lets be pedantic.
	type SessionHandler = <SessionKeys as sp_runtime::traits::OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type WeightInfo = weights::pallet_session::WeightInfo<Runtime>;
}

impl pallet_aura::Config for Runtime {
	type AuthorityId = AuraId;
	type DisabledValidators = ();
	type MaxAuthorities = ConstU32<100_000>;
}

parameter_types! {
	/// Default fixed percent a collator takes off the top of due rewards
	pub const DefaultCollatorCommission: Perbill = Perbill::from_percent(0);
	/// Default percent of inflation set aside for parachain bond every round
	pub const DefaultParachainBondReservePercent: Percent = Percent::from_percent(0);
	pub const MinCollatorStk: Balance = 50 * DOLLARS;
	pub const MinCandidateStk: Balance = 50 * DOLLARS;
	pub const MinDelegation: Balance = 50 * DOLLARS;
	pub const MinDelegatorStk: Balance = 50 * DOLLARS;
}

impl pallet_parachain_staking::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type MonetaryGovernanceOrigin = EnsureRoot<AccountId>;
	/// Minimum round length is 2 minutes (10 * 12 second block times)
	type MinBlocksPerRound = ConstU32<{ 2 * MINUTES }>;
	/// Blocks per round
	type DefaultBlocksPerRound = ConstU32<{ 2 * MINUTES }>;
	/// Rounds before the collator leaving the candidates request can be executed
	type LeaveCandidatesDelay = ConstU32<28>;
	/// Rounds before the candidate bond increase/decrease can be executed
	type CandidateBondLessDelay = ConstU32<28>;
	/// Rounds before the delegator exit can be executed
	type LeaveDelegatorsDelay = ConstU32<28>;
	/// Rounds before the delegator revocation can be executed
	type RevokeDelegationDelay = ConstU32<28>;
	/// Rounds before the delegator bond increase/decrease can be executed
	type DelegationBondLessDelay = ConstU32<28>;
	/// Rounds before the reward is paid
	type RewardPaymentDelay = ConstU32<2>;
	/// Minimum collators selected per round, default at genesis and minimum forever after
	type MinSelectedCandidates = ConstU32<1>;
	/// Maximum top delegations per candidate
	type MaxTopDelegationsPerCandidate = ConstU32<1000>;
	/// Maximum bottom delegations per candidate
	type MaxBottomDelegationsPerCandidate = ConstU32<200>;
	/// Maximum delegations per delegator
	type MaxDelegationsPerDelegator = ConstU32<100>;
	type DefaultCollatorCommission = DefaultCollatorCommission;
	type DefaultParachainBondReservePercent = DefaultParachainBondReservePercent;
	/// Minimum stake required to become a collator
	type MinCollatorStk = MinCollatorStk;
	/// Minimum stake required to be reserved to be a candidate
	type MinCandidateStk = MinCandidateStk;
	/// Minimum stake required to be reserved to be a delegator
	type MinDelegation = MinDelegation;
	/// Minimum stake required to be reserved to be a delegator
	type MinDelegatorStk = MinDelegatorStk;
	type OnCollatorPayout = ();
	type OnNewRound = ();
	type WeightInfo = weights::pallet_parachain_staking::WeightInfo<Runtime>;
	type IssuanceAdapter = BridgeTransfer;
}

parameter_types! {
	pub const MinVestedTransfer: Balance = 10 * CENTS;
}

impl pallet_vesting::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type BlockNumberToBalance = ConvertInto;
	type MinVestedTransfer = MinVestedTransfer;
	type WeightInfo = ();
	// `VestingInfo` encode length is 36bytes. 28 schedules gets encoded as 1009 bytes, which is the
	// highest number of schedules that encodes less than 2^10.
	const MAX_VESTING_SCHEDULES: u32 = 28;
}

parameter_types! {
	pub const BridgeChainId: u8 = 3;
	pub const ProposalLifetime: BlockNumber = 50400; // ~7 days
	pub TreasuryAccount: AccountId = TreasuryPalletId::get().into_account_truncating();
}

impl pallet_bridge::Config for Runtime {
	type Event = Event;
	type BridgeCommitteeOrigin = EnsureRootOrHalfCouncil;
	type Proposal = Call;
	type BridgeChainId = BridgeChainId;
	type Currency = Balances;
	type ProposalLifetime = ProposalLifetime;
	type TreasuryAccount = TreasuryAccount;
	type WeightInfo = weights::pallet_bridge::WeightInfo<Runtime>;
}

parameter_types! {
	pub const MaximumIssuance: Balance = 20_000_000 * DOLLARS;
	// Ethereum LIT total issuance in parachain decimal form
	pub const ExternalTotalIssuance: Balance = 100_000_000 * DOLLARS;
	// bridge::derive_resource_id(1, &bridge::hashing::blake2_128(b"LIT"));
	pub const NativeTokenResourceId: [u8; 32] = hex!("00000000000000000000000000000063a7e2be78898ba83824b0c0cc8dfb6001");
}

pub struct TransferNativeAnyone;
impl SortedMembers<AccountId> for TransferNativeAnyone {
	fn sorted_members() -> Vec<AccountId> {
		vec![]
	}

	fn contains(_who: &AccountId) -> bool {
		true
	}

	#[cfg(feature = "runtime-benchmarks")]
	fn add(_: &AccountId) {
		unimplemented!()
	}
}

impl pallet_bridge_transfer::Config for Runtime {
	type Event = Event;
	type BridgeOrigin = pallet_bridge::EnsureBridge<Runtime>;
	type TransferNativeMembers = TransferNativeAnyone;
	type SetMaximumIssuanceOrigin = EnsureRootOrHalfCouncil;
	type NativeTokenResourceId = NativeTokenResourceId;
	type DefaultMaximumIssuance = MaximumIssuance;
	type ExternalTotalIssuance = ExternalTotalIssuance;
	type WeightInfo = weights::pallet_bridge_transfer::WeightInfo<Runtime>;
}

parameter_types! {
	pub const SlashPercent: Percent = Percent::from_percent(20);
}

impl pallet_drop3::Config for Runtime {
	type Event = Event;
	type PoolId = u64;
	type SetAdminOrigin = EnsureRootOrHalfCouncil;
	type Currency = Balances;
	type WeightInfo = weights::pallet_drop3::WeightInfo<Runtime>;
	type SlashPercent = SlashPercent;
	type MaximumNameLength = ConstU32<16>;
}

impl pallet_extrinsic_filter::Config for Runtime {
	type Event = Event;
	type UpdateOrigin = EnsureRootOrHalfTechnicalCommittee;
	#[cfg(feature = "tee-dev")]
	type NormalModeFilter = Everything;
	#[cfg(not(feature = "tee-dev"))]
	type NormalModeFilter = NormalModeFilter;
	type SafeModeFilter = SafeModeFilter;
	type TestModeFilter = Everything;
	type WeightInfo = weights::pallet_extrinsic_filter::WeightInfo<Runtime>;
}

parameter_types! {
	pub const MomentsPerDay: Moment = 86_400_000; // [ms/d]
	pub const MaxSilenceTime: Moment =172_800_000; // 48h
}

impl pallet_teerex::Config for Runtime {
	type Event = Event;
	type Currency = Balances;
	type MomentsPerDay = MomentsPerDay;
	type MaxSilenceTime = MaxSilenceTime;
	type WeightInfo = ();
}

impl pallet_sidechain::Config for Runtime {
	type Event = Event;
	type WeightInfo = ();
}

impl pallet_teeracle::Config for Runtime {
	type Event = Event;
	type WeightInfo = ();
	type MaxWhitelistedReleases = ConstU32<10>;
}

impl pallet_identity_management::Config for Runtime {
	type Event = Event;
	type WeightInfo = ();
	type TEECallOrigin = EnsureEnclaveSigner<Runtime>;
}

ord_parameter_types! {
	pub const ALICE: AccountId = sp_runtime::AccountId32::new(hex!["d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"]);
}

impl pallet_identity_management_mock::Config for Runtime {
	type Event = Event;
	type ManageWhitelistOrigin = EnsureRootOrAllCouncil;
	type MaxVerificationDelay = ConstU32<{ 30 * MINUTES }>;
	// intentionally use ALICE for the IMP mock
	type TEECallOrigin = EnsureSignedBy<ALICE, AccountId>;
}

impl pallet_vc_management::Config for Runtime {
	type Event = Event;
	type TEECallOrigin = EnsureEnclaveSigner<Runtime>;
}

impl runtime_common::BaseRuntimeRequirements for Runtime {}

impl runtime_common::ParaRuntimeRequirements for Runtime {}

construct_runtime! {
	pub enum Runtime where
		Block = Block,
		NodeBlock = opaque::Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		// Core
		System: frame_system = 0,
		Timestamp: pallet_timestamp = 1,
		Scheduler: pallet_scheduler = 2,
		Utility: pallet_utility = 3,
		Multisig: pallet_multisig = 4,
		Proxy: pallet_proxy = 5,
		Preimage: pallet_preimage = 6,

		// Token related
		Balances: pallet_balances = 10,
		Vesting: pallet_vesting = 11,
		TransactionPayment: pallet_transaction_payment = 12,
		Treasury: pallet_treasury = 13,

		// Governance
		Democracy: pallet_democracy = 21,
		Council: pallet_collective::<Instance1> = 22,
		CouncilMembership: pallet_membership::<Instance1> = 23,
		TechnicalCommittee: pallet_collective::<Instance2> = 24,
		TechnicalCommitteeMembership: pallet_membership::<Instance2> = 25,
		Bounties: pallet_bounties = 26,
		Tips: pallet_tips = 27,

		// Parachain
		ParachainSystem: cumulus_pallet_parachain_system = 30,
		ParachainInfo: parachain_info = 31,

		// Collator support
		// About the order of these 5 pallets, the comment in cumulus seems to be outdated.
		//
		// The main thing is Authorship looks for the block author (T::FindAuthor::find_author)
		// in its `on_initialize` hook -> Session::find_author, where Session::validators() is enquired.
		// Meanwhile Session could modify the validators storage in its `on_initialize` hook. If Session
		// comes after Authorship, the changes on validators() will only take effect in the next block.
		//
		// I assume it's the desired behavior though or it doesn't really matter.
		//
		// also see the comment above `AllPalletsWithSystem` and
		// https://github.com/litentry/litentry-parachain/issues/336
		Authorship: pallet_authorship = 40,
		//41 is for old CollatorSelection, replaced by ParachainSTaking
		Session: pallet_session = 42,
		Aura: pallet_aura = 43,
		AuraExt: cumulus_pallet_aura_ext = 44,
		ParachainStaking: pallet_parachain_staking = 45,

		// XCM helpers
		XcmpQueue: cumulus_pallet_xcmp_queue = 50,
		PolkadotXcm: pallet_xcm = 51,
		CumulusXcm: cumulus_pallet_xcm = 52,
		DmpQueue: cumulus_pallet_dmp_queue = 53,
		XTokens: orml_xtokens = 54,
		Tokens: orml_tokens = 55,

		// Rococo pallets
		ChainBridge: pallet_bridge = 60,
		BridgeTransfer: pallet_bridge_transfer = 61,
		Drop3: pallet_drop3 = 62,
		ExtrinsicFilter: pallet_extrinsic_filter = 63,
		IdentityManagement: pallet_identity_management = 64,
		AssetManager: pallet_asset_manager = 65,
		VCManagement: pallet_vc_management = 66,

		// TEE
		Teerex: pallet_teerex = 90,
		Sidechain: pallet_sidechain = 91,
		Teeracle: pallet_teeracle = 92,

		// Mock
		IdentityManagementMock: pallet_identity_management_mock = 100,

		// TMP
		Sudo: pallet_sudo = 255,
	}
}

pub struct BaseCallFilter;
impl Contains<Call> for BaseCallFilter {
	fn contains(call: &Call) -> bool {
		if matches!(
			call,
			Call::Sudo(_) |
				Call::System(_) | Call::Timestamp(_) |
				Call::ParachainSystem(_) |
				Call::ExtrinsicFilter(_) |
				Call::Multisig(_) |
				Call::Council(_) | Call::TechnicalCommittee(_)
		) {
			// always allow core calls
			return true
		}

		pallet_extrinsic_filter::Pallet::<Runtime>::contains(call)
	}
}

pub struct SafeModeFilter;
impl Contains<Call> for SafeModeFilter {
	fn contains(_call: &Call) -> bool {
		false
	}
}

pub struct NormalModeFilter;
impl Contains<Call> for NormalModeFilter {
	fn contains(call: &Call) -> bool {
		matches!(
			call,
			// Vesting::vest
			Call::Vesting(pallet_vesting::Call::vest { .. }) |
			// ChainBridge
			Call::ChainBridge(_) |
			// BridgeTransfer
			Call::BridgeTransfer(_) |
			// XTokens::transfer for normal users
			Call::XTokens(orml_xtokens::Call::transfer { .. }) |
			// memberships
			Call::CouncilMembership(_) |
			Call::TechnicalCommitteeMembership(_) |
			// democracy, we don't subdivide the calls, so we allow public proposals
			Call::Democracy(_) |
			// Utility
			Call::Utility(_) |
			// Seesion
			Call::Session(_) |
			// Balance
			Call::Balances(_) |
			// IMP Mock, only allowed on rococo for testing
			// we should use `tee-dev` branch if we want to test it on Litmus
			Call::IdentityManagementMock(_) |
			Call::IdentityManagement(_) |
			Call::VCManagement(_) |
			// ParachainStaking; Only the collator part
			Call::ParachainStaking(pallet_parachain_staking::Call::join_candidates { .. }) |
			Call::ParachainStaking(pallet_parachain_staking::Call::schedule_leave_candidates { .. }) |
			Call::ParachainStaking(pallet_parachain_staking::Call::execute_leave_candidates { .. }) |
			Call::ParachainStaking(pallet_parachain_staking::Call::cancel_leave_candidates { .. }) |
			Call::ParachainStaking(pallet_parachain_staking::Call::go_offline { .. }) |
			Call::ParachainStaking(pallet_parachain_staking::Call::go_online { .. }) |
			Call::ParachainStaking(pallet_parachain_staking::Call::candidate_bond_more { .. }) |
			Call::ParachainStaking(pallet_parachain_staking::Call::schedule_candidate_bond_less { .. }) |
			Call::ParachainStaking(pallet_parachain_staking::Call::execute_candidate_bond_less { .. }) |
			Call::ParachainStaking(pallet_parachain_staking::Call::cancel_candidate_bond_less { .. })
		)
	}
}

#[cfg(feature = "runtime-benchmarks")]
mod benches {
	define_benchmarks!(
		[frame_system, SystemBench::<Runtime>]
		[pallet_asset_manager, AssetManager]
		[pallet_balances, Balances]
		[pallet_timestamp, Timestamp]
		[pallet_utility, Utility]
		[pallet_treasury, Treasury]
		[pallet_democracy, Democracy]
		[pallet_collective, Council]
		[pallet_proxy, Proxy]
		[pallet_membership, CouncilMembership]
		[pallet_multisig, Multisig]
		[pallet_drop3, Drop3]
		[pallet_extrinsic_filter, ExtrinsicFilter]
		[pallet_scheduler, Scheduler]
		[pallet_preimage, Preimage]
		[pallet_session, SessionBench::<Runtime>]
		// Since this module benchmark times out, comment it out for now
		// https://github.com/litentry/litentry-parachain/actions/runs/3155868677/jobs/5134984739
		// [pallet_parachain_staking, ParachainStaking]
		[cumulus_pallet_xcmp_queue, XcmpQueue]
		[pallet_identity_management, IdentityManagement]
		[pallet_teerex, Teerex]
		[pallet_sidechain, Sidechain]
		[pallet_teeracle, Teeracle]
		[pallet_bridge,ChainBridge]
		[pallet_bridge_transfer,BridgeTransfer]
	);
}

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block);
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			OpaqueMetadata::new(Runtime::metadata().into())
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(
			extrinsic: <Block as BlockT>::Extrinsic,
		) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(block: Block, data: sp_inherents::InherentData) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(
			source: TransactionSource,
			tx: <Block as BlockT>::Extrinsic,
			block_hash: <Block as BlockT>::Hash,
		) -> TransactionValidity {
			Executive::validate_transaction(source, tx, block_hash)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn decode_session_keys(
			encoded: Vec<u8>,
		) -> Option<Vec<(Vec<u8>, KeyTypeId)>> {
			SessionKeys::decode_into_raw_public_keys(&encoded)
		}

		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			SessionKeys::generate(seed)
		}
	}

	impl sp_consensus_aura::AuraApi<Block, AuraId> for Runtime {
		fn slot_duration() -> sp_consensus_aura::SlotDuration {
			sp_consensus_aura::SlotDuration::from_millis(Aura::slot_duration())
		}

		fn authorities() -> Vec<AuraId> {
			Aura::authorities().into_inner()
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<Block, Balance> for Runtime {
		fn query_info(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
		fn query_fee_details(
			uxt: <Block as BlockT>::Extrinsic,
			len: u32,
		) -> pallet_transaction_payment::FeeDetails<Balance> {
			TransactionPayment::query_fee_details(uxt, len)
		}
	}

	impl cumulus_primitives_core::CollectCollationInfo<Block> for Runtime {
		fn collect_collation_info(header: &<Block as BlockT>::Header) -> cumulus_primitives_core::CollationInfo {
			ParachainSystem::collect_collation_info(header)
		}
	}

	#[cfg(feature = "try-runtime")]
	impl frame_try_runtime::TryRuntime<Block> for Runtime {
		fn on_runtime_upgrade() -> (Weight, Weight) {
			// NOTE: intentional unwrap: we don't want to propagate the error backwards, and want to
			// have a backtrace here. If any of the pre/post migration checks fail, we shall stop
			// right here and right now.
			log::info!("try-runtime::on_runtime_upgrade rococo.");
			let weight = Executive::try_runtime_upgrade().unwrap();
			(weight, RuntimeBlockWeights::get().max_block)
		}

		fn execute_block(block: Block, state_root_check: bool, select: frame_try_runtime::TryStateSelect) -> Weight {
			log::info!(
				target: "runtime::Rococo", "try-runtime: executing block #{} ({:?}) / root checks: {:?} / sanity-checks: {:?}",
				block.header.number,
				block.header.hash(),
				state_root_check,
				select,
			);
			Executive::try_execute_block(block, state_root_check, select).expect("try_execute_block failed")
		}
	}

	#[cfg(feature = "runtime-benchmarks")]
	impl frame_benchmarking::Benchmark<Block> for Runtime {
		fn benchmark_metadata(extra: bool) -> (
			Vec<frame_benchmarking::BenchmarkList>,
			Vec<frame_support::traits::StorageInfo>,
		) {
			use frame_benchmarking::{Benchmarking, BenchmarkList};
			use frame_support::traits::StorageInfoTrait;
			use frame_system_benchmarking::Pallet as SystemBench;
			use cumulus_pallet_session_benchmarking::Pallet as SessionBench;

			let mut list = Vec::<BenchmarkList>::new();
			list_benchmarks!(list, extra);

			let storage_info = AllPalletsWithSystem::storage_info();
			(list, storage_info)
		}

		fn dispatch_benchmark(
			config: frame_benchmarking::BenchmarkConfig
		) -> Result<Vec<frame_benchmarking::BenchmarkBatch>, sp_runtime::RuntimeString> {
			use frame_benchmarking::{Benchmarking, BenchmarkBatch, TrackedStorageKey};

			use frame_system_benchmarking::Pallet as SystemBench;
			impl frame_system_benchmarking::Config for Runtime {}

			use cumulus_pallet_session_benchmarking::Pallet as SessionBench;
			impl cumulus_pallet_session_benchmarking::Config for Runtime {}

			let whitelist: Vec<TrackedStorageKey> = vec![
				// Block Number
				hex!("26aa394eea5630e07c48ae0c9558cef702a5c1b19ab7a04f536c519aca4983ac").to_vec().into(),
				// Total Issuance
				hex!("c2261276cc9d1f8598ea4b6a74b15c2f57c875e4cff74148e4628f264b974c80").to_vec().into(),
				// Execution Phase
				hex!("26aa394eea5630e07c48ae0c9558cef7ff553b5a9862a516939d82b3d3d8661a").to_vec().into(),
				// Event Count
				hex!("26aa394eea5630e07c48ae0c9558cef70a98fdbe9ce6c55837576c60c7af3850").to_vec().into(),
				// System Events
				hex!("26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7").to_vec().into(),
			];

			let mut batches = Vec::<BenchmarkBatch>::new();
			let params = (&config, &whitelist);
			add_benchmarks!(params, batches);

			if batches.is_empty() { return Err("Benchmark not found for this pallet.".into()) }
			Ok(batches)
		}
	}
}

struct CheckInherents;

impl cumulus_pallet_parachain_system::CheckInherents<Block> for CheckInherents {
	fn check_inherents(
		block: &Block,
		relay_state_proof: &cumulus_pallet_parachain_system::RelayChainStateProof,
	) -> sp_inherents::CheckInherentsResult {
		let relay_chain_slot = relay_state_proof
			.read_slot()
			.expect("Could not read the relay chain slot from the proof");

		let inherent_data =
			cumulus_primitives_timestamp::InherentDataProvider::from_relay_chain_slot_and_duration(
				relay_chain_slot,
				sp_std::time::Duration::from_secs(6),
			)
			.create_inherent_data()
			.expect("Could not create the timestamp inherent data");

		inherent_data.check_extrinsics(block)
	}
}

cumulus_pallet_parachain_system::register_validate_block! {
	Runtime = Runtime,
	BlockExecutor = cumulus_pallet_aura_ext::BlockExecutor::<Runtime, Executive>,
	CheckInherents = CheckInherents,
}
