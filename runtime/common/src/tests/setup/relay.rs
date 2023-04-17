// Copyright 2020-2023 Litentry Technologies GmbH.
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

use cumulus_primitives_core::{InteriorMultiLocation, ParaId};
use frame_support::{match_types, parameter_types};
use polkadot_runtime_parachains::origin;
use xcm::latest::prelude::{Here, MultiLocation, NetworkId, Parachain, X1};
use xcm_builder::{
	AccountId32Aliases, ChildParachainAsNative, ChildParachainConvertsVia,
	CurrencyAdapter as XcmCurrencyAdapter, IsConcrete, SignedAccountId32AsNative,
	SovereignSignedViaLocation,
};

use core_primitives::AccountId;

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
	($runtime:ident) => {
        use frame_support::{
            traits::{ConstU128, ConstU32, ConstU64, Everything, Nothing},
            weights::IdentityFee,
        };
        use cumulus_primitives_core::ParaId;
		use core_primitives::{Balance, AccountId};
        use runtime_common::BlockHashCount;
		use xcm_executor::XcmExecutor;
        use frame_system::EnsureRoot;
        use polkadot_runtime_parachains::{configuration, origin, shared, ump};
        use xcm::latest::prelude::{NetworkId, MultiLocation, Here, Parachain, X1};
        use sp_core::H256;
        use sp_runtime::{testing::Header, traits::IdentityLookup, AccountId32};

        use xcm_builder::{
            AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
            AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, ChildParachainAsNative,
            ChildParachainConvertsVia, CurrencyAdapter as XcmCurrencyAdapter, FixedWeightBounds,
            IsChildSystemParachain, IsConcrete, SignedAccountId32AsNative,
            SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit, UsingComponents,
        };
        use runtime_common::tests::setup::relay::{SovereignAccountOf,LocalAssetTransactor,KsmLocation,KusamaNetwork};

        //created by decl_test_network(macro)
		type XcmRouter = RelayChainXcmRouter;
		type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<RelayChainRuntime>;
		type Block = frame_system::mocking::MockBlock<RelayChainRuntime>;

		impl frame_system::Config for RelayChainRuntime {
			type RuntimeOrigin = RuntimeOrigin;
			type RuntimeCall = RuntimeCall;
			type Index = u64;
			type BlockNumber = u64;
			type Hash = H256;
			type Hashing = sp_runtime::traits::BlakeTwo256;
			type AccountId = AccountId;
			type Lookup = IdentityLookup<Self::AccountId>;
			type Header = Header;
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

		impl pallet_balances::Config for RelayChainRuntime {
			type MaxLocks = ConstU32<50>;
			type Balance = Balance;
			type RuntimeEvent = RuntimeEvent;
			type DustRemoval = ();
			type ExistentialDeposit = ConstU128<1>;
			type AccountStore = System;
			type WeightInfo = ();
			type MaxReserves = ConstU32<50>;
			type ReserveIdentifier = [u8; 8];
		}

		impl shared::Config for RelayChainRuntime {}

		impl configuration::Config for RelayChainRuntime {
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
			pub const BaseXcmWeight: Weight = Weight::from_ref_time(10);
			/// Maximum number of instructions in a single XCM fragment. A sanity check against weight
			/// calculations getting too crazy.
			pub const MaxInstructions: u32 = 100;
		}

		pub struct XcmConfig;
		impl xcm_executor::Config for XcmConfig {
			type RuntimeCall = RuntimeCall;
			type XcmSender = XcmRouter;
			type AssetTransactor = LocalAssetTransactor<RelayChainRuntime>;
			type OriginConverter = LocalOriginConverter<RuntimeOrigin>;
			type IsReserve = ();
			type IsTeleporter = ();
			type UniversalLocation = UniversalLocation;
			type Barrier = Barrier; // This is the setting should be same from Kusama
			type Weigher = FixedWeightBounds<BaseXcmWeight, RuntimeCall, MaxInstructions>;
			type Trader =
				UsingComponents<IdentityFee<Balance>, KsmLocation, AccountId, Balances, ()>;
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
		}

		pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, KusamaNetwork>;

		#[cfg(feature = "runtime-benchmarks")]
		parameter_types! {
			pub ReachableDest: Option<MultiLocation> = Some(Parent.into());
		}

		impl pallet_xcm::Config for RelayChainRuntime {
			type RuntimeEvent = RuntimeEvent;
			type SendXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
			type XcmRouter = XcmRouter;
			// Anyone can execute XCM messages locally...
			type ExecuteXcmOrigin = xcm_builder::EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
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
		}

		impl ump::Config for RelayChainRuntime {
			type RuntimeEvent = RuntimeEvent;
			type UmpSink = ump::XcmSink<XcmExecutor<XcmConfig>, RelayChainRuntime>;
			type FirstMessageFactorPercent = ConstU64<100>;
			type ExecuteOverweightOrigin = EnsureRoot<AccountId>;
			type WeightInfo = polkadot_runtime_parachains::ump::TestWeightInfo;
		}

		impl origin::Config for RelayChainRuntime {}

        construct_runtime!(
            pub enum RelayChainRuntime where
                Block = Block,
                NodeBlock = Block,
                UncheckedExtrinsic = UncheckedExtrinsic,
            {
                System: frame_system::{Pallet, Call, Storage, Config, Event<T>},
                Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
                ParasOrigin: origin::{Pallet, Origin},
                ParasUmp: ump::{Pallet, Call, Storage, Event},
                XcmPallet: pallet_xcm::{Pallet, Call, Storage, Event<T>, Origin},
            }
        );
	};
}
