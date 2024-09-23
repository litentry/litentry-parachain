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

use core_primitives::AccountId;
use cumulus_primitives_core::{InteriorMultiLocation, ParaId};
use frame_support::{match_types, parameter_types};
use polkadot_runtime_parachains::origin;
use xcm::latest::prelude::{Here, MultiLocation, NetworkId, Parachain, X1};
use xcm_builder::{
	AccountId32Aliases, ChildParachainAsNative, ChildParachainConvertsVia,
	CurrencyAdapter as XcmCurrencyAdapter, IsConcrete, SignedAccountId32AsNative,
	SovereignSignedViaLocation,
};

parameter_types! {
	pub KsmLocation: MultiLocation = Here.into();
	pub const KusamaNetwork: Option<NetworkId> = Some(NetworkId::Kusama);
	pub UniversalLocation: InteriorMultiLocation = Here;
}

match_types! {
	pub type OnlyParachains: impl Contains<MultiLocation> = {
		MultiLocation { parents: 0, interior: X1(Parachain(_)) }
	};
}

pub type SovereignAccountOf =
	(ChildParachainConvertsVia<ParaId, AccountId>, AccountId32Aliases<KusamaNetwork, AccountId>);

pub type LocalAssetTransactor<R> = XcmCurrencyAdapter<
	pallet_balances::Pallet<R>,
	IsConcrete<KsmLocation>,
	SovereignAccountOf,
	AccountId,
	(),
>;

pub type LocalOriginConverter<O> = (
	SovereignSignedViaLocation<SovereignAccountOf, O>,
	ChildParachainAsNative<origin::Origin, O>,
	SignedAccountId32AsNative<KusamaNetwork, O>,
);

#[macro_export]
macro_rules! decl_test_relay_chain_runtime {
	() => {
		use frame_support::{
            traits::{ConstU128, ConstU32, ConstU64, Everything, Nothing},
            weights::{Weight, IdentityFee, WeightMeter},
			construct_runtime, parameter_types,
        };
        use frame_system::EnsureRoot;
        use cumulus_primitives_core::ParaId;
		use core_primitives::{Balance, AccountId};
		use xcm_executor::XcmExecutor;
        use polkadot_runtime_parachains::{configuration, origin, shared};
        use xcm::latest::prelude::{NetworkId, MultiLocation, Here, Junction, Parachain, X1};
        use sp_core::H256;
        use sp_runtime::{testing::Header, traits::IdentityLookup, AccountId32};
        use xcm_builder::{
            AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
            AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, ChildParachainAsNative,
            ChildParachainConvertsVia, CurrencyAdapter as XcmCurrencyAdapter, FixedWeightBounds,
            IsChildSystemParachain, IsConcrete, SignedAccountId32AsNative,
            SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit, UsingComponents,
        };
		use xcm_simulator::{ProcessMessage, AggregateMessageOrigin, UmpQueueId, ProcessMessageError};
        use runtime_common::BlockHashCount;
        use runtime_common::tests::setup::relay::{SovereignAccountOf,LocalAssetTransactor,KsmLocation,KusamaNetwork};
		use runtime_common::tests::setup::relay::{OnlyParachains, LocalOriginConverter, UniversalLocation};
        // created by `decl_test_network!`
		type XcmRouter = RelayChainXcmRouter;

		impl frame_system::Config for Runtime {
			type RuntimeOrigin = RuntimeOrigin;
			type RuntimeCall = RuntimeCall;
			type Nonce = u64;
			type Block = frame_system::mocking::MockBlock<Runtime>;
			type Hash = H256;
			type Hashing = ::sp_runtime::traits::BlakeTwo256;
			type AccountId = AccountId;
			type Lookup = IdentityLookup<Self::AccountId>;
			type RuntimeEvent = RuntimeEvent;
			type BlockHashCount = BlockHashCount;
			type BlockWeights = ();
			type BlockLength = ();
			type Version = ();
			type PalletInfo = PalletInfo;
			type AccountData = pallet_balances::AccountData<Balance>;
			type OnNewAccount = ();
			type OnKilledAccount = ();
			type DbWeight = ();
			type BaseCallFilter = Everything;
			type SystemWeightInfo = ();
			type SS58Prefix = ();
			type OnSetCode = ();
			type MaxConsumers = ConstU32<16>;
		}

		impl pallet_balances::Config for Runtime {
			type MaxLocks = ConstU32<50>;
			type Balance = Balance;
			type RuntimeEvent = RuntimeEvent;
			type DustRemoval = ();
			type ExistentialDeposit = ConstU128<1>;
			type AccountStore = System;
			type WeightInfo = ();
			type MaxReserves = ();
			type ReserveIdentifier = ();
			type FreezeIdentifier = ();
			type MaxHolds = ();
			type MaxFreezes = ();
			type RuntimeHoldReason = ();
		}

		impl shared::Config for Runtime {}

		impl configuration::Config for Runtime {
			type WeightInfo = configuration::TestWeightInfo;
		}

		/// The barriers one of which must be passed for an XCM message to be executed.
		pub type Barrier = (
			// Weight that is paid for may be consumed.
			TakeWeightCredit,
			// If the message is one that immediately attemps to pay for execution, then allow it.
			AllowTopLevelPaidExecutionFrom<Everything>,
			// Messages coming from system parachains need not pay for execution.
			AllowUnpaidExecutionFrom<IsChildSystemParachain<ParaId>>,
			// Expected responses are OK.
			AllowKnownQueryResponses<XcmPallet>,
			// Subscriptions for version tracking are OK.
			AllowSubscriptionsFrom<OnlyParachains>,
		);

		parameter_types! {
			pub const MaxAssetsIntoHolding: u32 = 64;
			/// The amount of weight an XCM operation takes. This is a safe overestimate.
			pub const BaseXcmWeight: Weight = Weight::from_parts(10, 0);
			/// Maximum number of instructions in a single XCM fragment. A sanity check against weight
			/// calculations getting too crazy.
			pub const MaxInstructions: u32 = 100;
		}

		pub struct XcmConfig;
		impl xcm_executor::Config for XcmConfig {
			type RuntimeCall = RuntimeCall;
			type XcmSender = XcmRouter;
			type AssetTransactor = LocalAssetTransactor<Runtime>;
			type OriginConverter = LocalOriginConverter<RuntimeOrigin>;
			type IsReserve = ();
			type IsTeleporter = ();
			type UniversalLocation = UniversalLocation;
			type Barrier = Barrier; // This is the setting should be same from Kusama
			type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
			type Trader = UsingComponents<IdentityFee<Balance>, KsmLocation, AccountId, Balances, ()>;
			type ResponseHandler = ();
			type AssetTrap = ();
			type AssetLocker = ();
			type AssetExchanger = ();
			type AssetClaims = ();
			type SubscriptionService = XcmPallet;
			type PalletInstancesInfo = AllPalletsWithSystem;
			type MaxAssetsIntoHolding = MaxAssetsIntoHolding;
			type FeeManager = ();
			type MessageExporter = ();
			type UniversalAliases = Nothing;
			type CallDispatcher = RuntimeCall;
			type SafeCallFilter = Everything;
			type Aliasers = Nothing;
		}

		pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, KusamaNetwork>;

		#[cfg(feature = "runtime-benchmarks")]
		parameter_types! {
			pub ReachableDest: Option<MultiLocation> = Some(MultiLocation::parent());
		}

		impl pallet_xcm::Config for Runtime {
			type RuntimeEvent = RuntimeEvent;
			type SendXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
			type XcmRouter = XcmRouter;
			type ExecuteXcmOrigin =
				xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
			type XcmExecuteFilter = Everything;
			type XcmExecutor = XcmExecutor<XcmConfig>;
			type XcmTeleportFilter = Everything;
			type XcmReserveTransferFilter = Everything;
			type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
			type UniversalLocation = UniversalLocation;
			type RuntimeOrigin = RuntimeOrigin;
			type RuntimeCall = RuntimeCall;
			const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
			type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
			type Currency = Balances;
			type CurrencyMatcher = IsConcrete<KsmLocation>;
			type TrustedLockers = ();
			type SovereignAccountOf = SovereignAccountOf;
			type MaxLockers = ConstU32<8>;
			type WeightInfo = pallet_xcm::TestWeightInfo;
			#[cfg(feature = "runtime-benchmarks")]
			type ReachableDest = ReachableDest;
			type AdminOrigin = EnsureRoot<AccountId>;
			type MaxRemoteLockConsumers = ConstU32<0>;
			type RemoteLockConsumerIdentifier = ();
		}

		impl origin::Config for Runtime {}

		parameter_types! {
			/// Amount of weight that can be spent per block to service messages.
			pub MessageQueueServiceWeight: Weight = Weight::from_parts(1_000_000_000, 1_000_000);
			pub const MessageQueueHeapSize: u32 = 65_536;
			pub const MessageQueueMaxStale: u32 = 16;
		}

		/// Message processor to handle any messages that were enqueued into the `MessageQueue` pallet.
		pub struct MessageProcessor;
		impl ProcessMessage for MessageProcessor {
			type Origin = AggregateMessageOrigin;

			fn process_message(
				message: &[u8],
				origin: Self::Origin,
				meter: &mut WeightMeter,
				id: &mut [u8; 32],
			) -> Result<bool, ProcessMessageError> {
				let para = match origin {
					AggregateMessageOrigin::Ump(UmpQueueId::Para(para)) => para,
				};
				xcm_builder::ProcessXcmMessage::<
					Junction,
					xcm_executor::XcmExecutor<XcmConfig>,
					RuntimeCall,
				>::process_message(message, Junction::Parachain(para.into()), meter, id)
			}
		}

		impl pallet_message_queue::Config for Runtime {
			type RuntimeEvent = RuntimeEvent;
			type Size = u32;
			type HeapSize = MessageQueueHeapSize;
			type MaxStale = MessageQueueMaxStale;
			type ServiceWeight = MessageQueueServiceWeight;
			type MessageProcessor = MessageProcessor;
			type QueueChangeHandler = ();
			type QueuePausedQuery = ();
			type WeightInfo = ();
		}

		construct_runtime!(
			pub enum Runtime
			{
				System: frame_system,
				Balances: pallet_balances,
				ParasOrigin: origin,
				XcmPallet: pallet_xcm,
				MessageQueue: pallet_message_queue,
				Configuration: configuration,
			}
		);
	};
}
