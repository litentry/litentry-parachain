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
#![allow(clippy::clone_on_copy)]
#![allow(clippy::useless_conversion)]

// use frame_system::RawOrigin as SystemRawOrigin;
// use pallet_collective::RawOrigin as CollectiveRawOrigin;
use frame_support::{
	match_types, parameter_types,
	traits::{Everything, Nothing},
	weights::IdentityFee,
	PalletId,
};
use orml_traits::{location::AbsoluteReserveProvider, parameter_type_with_key};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
// Litentry: The CheckAccount implementation is forced by the bug of FungiblesAdapter.
// We should replace () regarding fake_pallet_id account after our PR passed.
use primitives::{AccountId, XcmV2Weight};
use runtime_common::xcm_impl::{
	AccountIdToMultiLocation, AssetIdMuliLocationConvert, CurrencyId,
	CurrencyIdMultiLocationConvert, FirstAssetTrader, MultiNativeAsset, NewAnchoringSelfReserve,
	OldAnchoringSelfReserve, XcmFeesToAccount,
};

use runtime_common::{EnsureRootOrTwoThirdsCouncil, FilterEnsureOrigin};
use sp_runtime::traits::AccountIdConversion;
use xcm::latest::prelude::*;
use xcm_builder::{
	AccountId32Aliases, AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom,
	ConvertedConcreteAssetId, CurrencyAdapter, EnsureXcmOrigin, FixedWeightBounds,
	FungiblesAdapter, IsConcrete, LocationInverter, ParentIsPreset, RelayChainAsNative,
	SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative,
	SignedToAccountId32, SovereignSignedViaLocation, TakeWeightCredit, UsingComponents,
};
use xcm_executor::{traits::JustTry, XcmExecutor};

#[cfg(test)]
use crate::tests::setup::ParachainXcmRouter;

use super::{
	AssetId, AssetManager, Balance, Balances, DealWithFees, ParachainInfo, PolkadotXcm, Runtime,
	RuntimeCall, RuntimeEvent, RuntimeOrigin, Tokens, Treasury,
};
#[cfg(not(test))]
use super::{ParachainSystem, XcmpQueue};

parameter_types! {
	pub const RelayLocation: MultiLocation = MultiLocation::parent();
	pub const RelayNetwork: NetworkId = NetworkId::Any;
	pub RelayChainOrigin: RuntimeOrigin = cumulus_pallet_xcm::Origin::Relay.into();
	pub Ancestry: MultiLocation = Parachain(ParachainInfo::parachain_id().into()).into();
}

/// Type for specifying how a `MultiLocation` can be converted into an `AccountId`. This is used
/// when determining ownership of accounts for asset transacting and when attempting to use XCM
/// `Transact` in order to determine the dispatch Origin.
pub type LocationToAccountId = (
	// The parent (Relay-chain) origin converts to the parent `AccountId`.
	ParentIsPreset<AccountId>,
	// Sibling parachain origins convert to AccountId via the `ParaId::into`.
	SiblingParachainConvertsVia<Sibling, AccountId>,
	// Straight up local `AccountId32` origins just alias directly to `AccountId`.
	AccountId32Aliases<RelayNetwork, AccountId>,
);

/// Means for transacting self reserve assets on this chain.
pub type LocalAssetTransactor = CurrencyAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	(IsConcrete<NewAnchoringSelfReserve<Runtime>>, IsConcrete<OldAnchoringSelfReserve<Runtime>>),
	// Do a simple punn to convert an AccountId32 MultiLocation into a native chain account ID:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports.
	(),
>;

parameter_types! {
	pub const TempPalletId: PalletId = PalletId(*b"py/tempA");
	pub TempAccount: AccountId = TempPalletId::get().into_account_truncating();
}
// The non-reserve fungible transactor type
// It will use orml_tokens, and the Id will be CurrencyId::ParachainReserve(MultiLocation)
pub type ForeignFungiblesTransactor = FungiblesAdapter<
	// Use this fungibles implementation
	Tokens,
	// Use this currency when it is a fungible asset matching the given location or name:
	ConvertedConcreteAssetId<AssetId, Balance, AssetIdMuliLocationConvert<Runtime>, JustTry>,
	// Do a simple punn to convert an AccountId32 MultiLocation into a native chain account ID:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We dont allow teleports.
	Nothing,
	// We dont track any teleports
	TempAccount,
>;

// The XCM transaction handlers for different type of assets.
pub type AssetTransactors = (
	// SelfReserve asset, both pre and post 0.9.16
	LocalAssetTransactor,
	// // Foreign assets (non native minted token crossed from remote chain)
	ForeignFungiblesTransactor,
);

/// Litentry: As our current XcmRouter (which used for receiving remote XCM message and call
/// XcmExecutor to handle) will force the origin to remoteChain sovereign account, this
/// XcmOriginToTransactDispatchOrigin implementation is not that useful. This is the type we use to
/// convert an (incoming) XCM origin into a local `Origin` instance, ready for dispatching a
/// transaction with Xcm's `Transact`. There is an `OriginKind` which can biases the kind of local
/// `Origin` it will become.
pub type XcmOriginToTransactDispatchOrigin = (
	// Sovereign account converter; this attempts to derive an `AccountId` from the origin location
	// using `LocationToAccountId` and then turn that into the usual `Signed` origin. Useful for
	// foreign chains who want to have a local sovereign account on this chain which they control.
	SovereignSignedViaLocation<LocationToAccountId, RuntimeOrigin>,
	// Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
	// recognized.
	RelayChainAsNative<RelayChainOrigin, RuntimeOrigin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognized.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, RuntimeOrigin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, RuntimeOrigin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<RuntimeOrigin>,
);

parameter_types! {
	// One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
	// How much we charge for XCM from remote chain per XCM command.
	pub UnitWeightCost: XcmV2Weight = 200_000_000u64;
	pub const MaxInstructions: u32 = 100;
}

match_types! {
	pub type ParentOrParentsExecutivePlurality: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(Plurality { id: BodyId::Executive, .. }) }
	};
}

pub type Barriers = (
	TakeWeightCredit,
	AllowTopLevelPaidExecutionFrom<Everything>,
	AllowUnpaidExecutionFrom<ParentOrParentsExecutivePlurality>,
	// ^^^ Parent and its exec plurality get free execution
);

parameter_types! {
	/// Xcm fees will go to the treasury account
	pub XcmFeesAccount: AccountId = Treasury::account_id();
}

pub type Traders = (
	UsingComponents<
		IdentityFee<Balance>,
		NewAnchoringSelfReserve<Runtime>,
		AccountId,
		Balances,
		DealWithFees<Runtime>,
	>,
	UsingComponents<
		IdentityFee<Balance>,
		OldAnchoringSelfReserve<Runtime>,
		AccountId,
		Balances,
		DealWithFees<Runtime>,
	>,
	// TODO::Implement foreign asset fee to weight rule from AssetManager Setting; Need more test
	FirstAssetTrader<
		CurrencyId<Runtime>,
		AssetManager,
		XcmFeesToAccount<
			Tokens,
			ConvertedConcreteAssetId<
				AssetId,
				Balance,
				AssetIdMuliLocationConvert<Runtime>,
				JustTry,
			>,
			AccountId,
			XcmFeesAccount,
		>,
	>,
);

/// Xcm Weigher shared between multiple Xcm-related configs.
pub type XcmWeigher = FixedWeightBounds<UnitWeightCost, RuntimeCall, MaxInstructions>;

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type RuntimeCall = RuntimeCall;
	type XcmSender = XcmRouter;
	// How to withdraw and deposit an asset.
	type AssetTransactor = AssetTransactors;
	type OriginConverter = XcmOriginToTransactDispatchOrigin;
	// Only Allow chains to handle their own reserve assets crossed on local chain whatever way they
	// want.
	type IsReserve = MultiNativeAsset;
	type IsTeleporter = (); // Teleporting is disabled.
	type LocationInverter = LocationInverter<Ancestry>;
	type Barrier = Barriers;
	type Weigher = XcmWeigher;
	// Litentry: This is the tool used for calculating that inside XcmExecutor vm, how to transfer
	// asset into weight fee. Usually this is in order to fulfull Barrier
	// AllowTopLevelPaidExecutionFrom requirement. Currently we have not implement the asset to fee
	// rule for Foreign Asset, so pure cross chain transfer from XCM parachain will be rejected no
	// matter.
	type Trader = Traders;
	type ResponseHandler = PolkadotXcm;
	type AssetTrap = PolkadotXcm;
	type AssetClaims = PolkadotXcm;
	type SubscriptionService = PolkadotXcm;
}

/// No local origins on this chain are allowed to dispatch XCM sends/executions.
pub type LocalOriginToLocation = SignedToAccountId32<RuntimeOrigin, AccountId, RelayNetwork>;

#[cfg(test)]
/// The mimic XcmRouter which only change storage locally for Xcm to digest.
/// XCM router for parachain.
pub type XcmRouter = ParachainXcmRouter<ParachainInfo>;
#[cfg(not(test))]
/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	// We use PolkadotXcm to confirm the XCM Version; Use () instead if pass anyway
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);

match_types! {
	pub type ParentOrParachains: impl Contains<MultiLocation> = {
		// Local account: Litmus
		MultiLocation { parents: 0, interior: X1(Junction::AccountId32 { .. }) } |
		// Relay-chain account: Kusama
		MultiLocation { parents: 1, interior: X1(Junction::AccountId32 { .. }) } |
		// AccountKey20 based parachain: Moonriver
		MultiLocation { parents: 1, interior: X2(Parachain( .. ), Junction::AccountKey20 { .. }) } |
		// AccountId 32 based parachain: Statemint
		MultiLocation { parents: 1, interior: X2(Parachain( .. ), Junction::AccountId32 { .. }) }
	};
}

parameter_type_with_key! {
	pub ParachainMinFee: |_location: MultiLocation| -> Option<u128> {
		// Always return `None` to disallow using fee asset and target asset with different reserve chains
		None
	};
}

parameter_types! {
	pub SelfLocation: MultiLocation = MultiLocation {
		parents:1,
		interior: Junctions::X1(
			Parachain(ParachainInfo::parachain_id().into())
		)
	};
	pub const BaseXcmWeight: XcmV2Weight = 100_000_000u64;
	pub const MaxAssetsForTransfer: usize = 3;
}

impl pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	// We allow anyone to send any XCM to anywhere
	// This is highly relied on if target chain properly filtered
	// Check their Barriers implementation
	// And for TakeWeightCredit
	// Check if their executor's ShouldExecute trait weight_credit
	type SendXcmOrigin =
		FilterEnsureOrigin<RuntimeOrigin, LocalOriginToLocation, EnsureRootOrTwoThirdsCouncil>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<RuntimeOrigin, LocalOriginToLocation>;
	type XcmExecuteFilter = Nothing;
	// ^ Disable dispatchable execute on the XCM pallet.
	// Needs to be `Everything` for local testing.
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type XcmTeleportFilter = Nothing;
	// This filter here defines what is allowed for XcmExecutor to handle with TransferReserveAsset
	// Rule.
	type XcmReserveTransferFilter = Everything;
	type Weigher = XcmWeigher;
	type LocationInverter = LocationInverter<Ancestry>;
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	// ^ Override for AdvertisedXcmVersion default
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}

impl cumulus_pallet_xcm::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl orml_xtokens::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Balance = Balance;
	type CurrencyId = CurrencyId<Runtime>;
	type AccountIdToMultiLocation = AccountIdToMultiLocation;
	type CurrencyIdConvert = CurrencyIdMultiLocationConvert<Runtime>;
	type XcmExecutor = XcmExecutor<XcmConfig>;
	type SelfLocation = SelfLocation;
	type MultiLocationsFilter = ParentOrParachains;
	type MinXcmFee = ParachainMinFee;
	type Weigher = XcmWeigher;
	type BaseXcmWeight = BaseXcmWeight;
	type LocationInverter = LocationInverter<Ancestry>;
	type MaxAssetsForTransfer = MaxAssetsForTransfer;
	type ReserveProvider = AbsoluteReserveProvider;
}
