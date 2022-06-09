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

use super::{
	transaction_payment::DealWithFees, AssetId, AssetManager, Balance, Balances, Call, Event,
	Origin, ParachainInfo, ParachainSystem, PolkadotXcm, Runtime, Tokens, Treasury, XcmpQueue,
};

use frame_support::{
	match_types, parameter_types,
	traits::{tokens::fungibles::Mutate, Everything, Get, Nothing, PalletInfoAccess},
	weights::{constants::WEIGHT_PER_SECOND, IdentityFee, Weight},
	PalletId,
};
use orml_traits::{location::AbsoluteReserveProvider, parameter_type_with_key};
use pallet_asset_manager::{AssetTypeGetter, UnitsToWeightRatio};
use pallet_xcm::XcmPassthrough;
use polkadot_parachain::primitives::Sibling;
use primitives::AccountId;
use sp_runtime::traits::Zero;
use sp_std::marker::PhantomData;
use xcm::latest::{prelude::*, AssetId as xcmAssetId};
use xcm_builder::{
	AccountId32Aliases, AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom,
	ConvertedConcreteAssetId, CurrencyAdapter, EnsureXcmOrigin, FixedWeightBounds,
	FungiblesAdapter, IsConcrete, LocationInverter, ParentIsPreset, RelayChainAsNative,
	SiblingParachainAsNative, SiblingParachainConvertsVia, SignedAccountId32AsNative,
	SignedToAccountId32, SovereignSignedViaLocation, TakeRevenue, TakeWeightCredit,
	UsingComponents,
};
use xcm_executor::{
	traits::{Convert as xcmConvert, FilterAssetLocation, JustTry, MatchesFungibles, WeightTrader},
	XcmExecutor,
};

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::traits::Convert as spConvert;
use sp_std::{borrow::Borrow, prelude::*};

parameter_types! {
	pub const RelayLocation: MultiLocation = MultiLocation::parent();
	pub const RelayNetwork: NetworkId = NetworkId::Any;
	pub RelayChainOrigin: Origin = cumulus_pallet_xcm::Origin::Relay.into();
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

parameter_types! {
	// The old anchoring method before 0.9.16
	// https://github.com/paritytech/polkadot/pull/4470
	pub OldAnchoringSelfReserve: MultiLocation = MultiLocation {
		parents:1,
		interior: Junctions::X2(
			Parachain(ParachainInfo::parachain_id().into()),
			PalletInstance(<Balances as PalletInfoAccess>::index() as u8)
		)
	};
	// New Self Reserve location, defines the multilocation identifiying the self-reserve currency
	// This is used to match it also against our Balances pallet when we receive such
	// a MultiLocation: (Self Balances pallet index)
	// This is the new anchoring way
	pub NewAnchoringSelfReserve: MultiLocation = MultiLocation {
		parents:0,
		interior: Junctions::X1(
			PalletInstance(<Balances as PalletInfoAccess>::index() as u8)
		)
	};
}

/// Means for transacting self reserve assets on this chain.
pub type LocalAssetTransactor = CurrencyAdapter<
	// Use this currency:
	Balances,
	// Use this currency when it is a fungible asset matching the given location or name:
	(IsConcrete<NewAnchoringSelfReserve>, IsConcrete<OldAnchoringSelfReserve>),
	// Do a simple punn to convert an AccountId32 MultiLocation into a native chain account ID:
	LocationToAccountId,
	// Our chain's account ID type (we can't get away without mentioning it explicitly):
	AccountId,
	// We don't track any teleports.
	(),
>;

// Litentry: The CheckAccount implementation is forced by the bug of FungiblesAdapter.
// We should replace () regarding fake_pallet_id account after our PR passed.
use sp_runtime::traits::AccountIdConversion;
parameter_types! {
	pub const TempPalletId: PalletId = PalletId(*b"py/tempA");
	pub TempAccount: AccountId = TempPalletId::get().into_account();
}
// The non-reserve fungible transactor type
// It will use orml_tokens, and the Id will be CurrencyId::ParachainReserve(MultiLocation)
pub type ForeignFungiblesTransactor = FungiblesAdapter<
	// Use this fungibles implementation
	Tokens,
	// Use this currency when it is a fungible asset matching the given location or name:
	ConvertedConcreteAssetId<AssetId, Balance, AssetIdMuliLocationConvert, JustTry>,
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
	SovereignSignedViaLocation<LocationToAccountId, Origin>,
	// Native converter for Relay-chain (Parent) location; will converts to a `Relay` origin when
	// recognized.
	RelayChainAsNative<RelayChainOrigin, Origin>,
	// Native converter for sibling Parachains; will convert to a `SiblingPara` origin when
	// recognized.
	SiblingParachainAsNative<cumulus_pallet_xcm::Origin, Origin>,
	// Native signed account converter; this just converts an `AccountId32` origin into a normal
	// `Origin::Signed` origin of the same 32-byte value.
	SignedAccountId32AsNative<RelayNetwork, Origin>,
	// Xcm origins can be represented natively under the Xcm pallet's Xcm origin.
	XcmPassthrough<Origin>,
);

parameter_types! {
	// One XCM operation is 1_000_000_000 weight - almost certainly a conservative estimate.
	// How much we charge for XCM from remote chain per XCM command.
	pub UnitWeightCost: Weight = 200_000_000;
	pub const MaxInstructions: u32 = 100;
}

match_types! {
	pub type ParentOrParentsExecutivePlurality: impl Contains<MultiLocation> = {
		MultiLocation { parents: 1, interior: Here } |
		MultiLocation { parents: 1, interior: X1(Plurality { id: BodyId::Executive, .. }) }
	};
}

pub trait Reserve {
	/// Returns assets reserve location.
	fn reserve(&self) -> Option<MultiLocation>;
}

// Takes the chain part of a MultiAsset
impl Reserve for MultiAsset {
	fn reserve(&self) -> Option<MultiLocation> {
		if let xcmAssetId::Concrete(location) = self.id.clone() {
			let first_interior = location.first_interior();
			let parents = location.parent_count();
			match (parents, first_interior.clone()) {
				// The only case for non-relay chain will be the chain itself.
				(0, Some(Parachain(id))) => Some(MultiLocation::new(0, X1(Parachain(id.clone())))),
				// Only Sibling parachain is recognized.
				(1, Some(Parachain(id))) => Some(MultiLocation::new(1, X1(Parachain(id.clone())))),
				// The Relay chain.
				(1, _) => Some(MultiLocation::parent()),
				// No other case is allowed for now.
				_ => None,
			}
		} else {
			None
		}
	}
}

/// A `FilterAssetLocation` implementation. Filters multi native assets whose
/// reserve is same with `origin`.
pub struct MultiNativeAsset;
impl FilterAssetLocation for MultiNativeAsset {
	fn filter_asset_location(asset: &MultiAsset, origin: &MultiLocation) -> bool {
		if let Some(ref reserve) = asset.reserve() {
			if reserve == origin {
				return true
			}
		}
		false
	}
}

pub type Barriers = (
	TakeWeightCredit,
	AllowTopLevelPaidExecutionFrom<Everything>,
	AllowUnpaidExecutionFrom<ParentOrParentsExecutivePlurality>,
	// ^^^ Parent and its exec plurality get free execution
);

// We need to know how to charge for incoming assets
// This takes the first fungible asset, and takes whatever UnitPerSecondGetter establishes
// UnitsToWeightRatio trait, which needs to be implemented by AssetIdInfoGetter
pub struct FirstAssetTrader<
	AssetType: From<MultiLocation> + Clone,
	AssetIdInfoGetter: UnitsToWeightRatio<AssetType>,
	R: TakeRevenue,
>(Weight, Option<(MultiLocation, u128, u128)>, PhantomData<(AssetType, AssetIdInfoGetter, R)>);
impl<
		AssetType: From<MultiLocation> + Clone,
		AssetIdInfoGetter: UnitsToWeightRatio<AssetType>,
		R: TakeRevenue,
	> WeightTrader for FirstAssetTrader<AssetType, AssetIdInfoGetter, R>
{
	fn new() -> Self {
		FirstAssetTrader(0, None, PhantomData)
	}
	fn buy_weight(
		&mut self,
		weight: Weight,
		payment: xcm_executor::Assets,
	) -> Result<xcm_executor::Assets, XcmError> {
		let first_asset = payment.fungible_assets_iter().next().ok_or(XcmError::TooExpensive)?;

		// We are only going to check first asset for now. This should be sufficient for simple
		// token transfers. We will see later if we change this.
		match (first_asset.id, first_asset.fun) {
			(xcmAssetId::Concrete(id), Fungibility::Fungible(_)) => {
				let asset_type: AssetType = id.clone().into();
				// Shortcut if we know the asset is not supported
				// This involves the same db read per block, mitigating any attack based on
				// non-supported assets
				if !AssetIdInfoGetter::payment_is_supported(asset_type.clone()) {
					return Err(XcmError::TooExpensive)
				}
				if let Some(units_per_second) = AssetIdInfoGetter::get_units_per_second(asset_type)
				{
					let amount = units_per_second.saturating_mul(weight as u128) /
						(WEIGHT_PER_SECOND as u128);

					// We dont need to proceed if the amount is 0
					// For cases (specially tests) where the asset is very cheap with respect
					// to the weight needed
					if amount.is_zero() {
						return Ok(payment)
					}

					let required = MultiAsset {
						fun: Fungibility::Fungible(amount),
						id: xcmAssetId::Concrete(id.clone()),
					};
					let unused =
						payment.checked_sub(required).map_err(|_| XcmError::TooExpensive)?;
					self.0 = self.0.saturating_add(weight);

					// In case the asset matches the one the trader already stored before, add
					// to later refund

					// Else we are always going to substract the weight if we can, but we latter do
					// not refund it

					// In short, we only refund on the asset the trader first succesfully was able
					// to pay for an execution
					let new_asset = match self.1.clone() {
						Some((prev_id, prev_amount, units_per_second)) =>
							if prev_id == id {
								Some((id, prev_amount.saturating_add(amount), units_per_second))
							} else {
								None
							},
						None => Some((id, amount, units_per_second)),
					};

					// Due to the trait bound, we can only refund one asset.
					if let Some(new_asset) = new_asset {
						self.0 = self.0.saturating_add(weight);
						self.1 = Some(new_asset);
					};
					Ok(unused)
				} else {
					Err(XcmError::TooExpensive)
				}
			},
			_ => Err(XcmError::TooExpensive),
		}
	}

	fn refund_weight(&mut self, weight: Weight) -> Option<MultiAsset> {
		if let Some((id, prev_amount, units_per_second)) = self.1.clone() {
			let weight = weight.min(self.0);
			self.0 -= weight;
			let amount = units_per_second * (weight as u128) / (WEIGHT_PER_SECOND as u128);
			self.1 = Some((id.clone(), prev_amount.saturating_sub(amount), units_per_second));
			Some(MultiAsset { fun: Fungibility::Fungible(amount), id: xcmAssetId::Concrete(id) })
		} else {
			None
		}
	}
}

/// Deal with spent fees, deposit them as dictated by R
impl<
		AssetType: From<MultiLocation> + Clone,
		AssetIdInfoGetter: UnitsToWeightRatio<AssetType>,
		R: TakeRevenue,
	> Drop for FirstAssetTrader<AssetType, AssetIdInfoGetter, R>
{
	fn drop(&mut self) {
		if let Some((id, amount, _)) = self.1.clone() {
			R::take_revenue((id, amount).into());
		}
	}
}

/// XCM fee depositor to which we implement the TakeRevenue trait
/// It receives a fungibles::Mutate implemented argument, a matcher to convert MultiAsset into
/// AssetId and amount, and the fee receiver account
pub struct XcmFeesToAccount<Assets, Matcher, AccountId, ReceiverAccount>(
	PhantomData<(Assets, Matcher, AccountId, ReceiverAccount)>,
);
impl<
		Assets: Mutate<AccountId>,
		Matcher: MatchesFungibles<Assets::AssetId, Assets::Balance>,
		AccountId: Clone,
		ReceiverAccount: Get<AccountId>,
	> TakeRevenue for XcmFeesToAccount<Assets, Matcher, AccountId, ReceiverAccount>
{
	fn take_revenue(revenue: MultiAsset) {
		match Matcher::matches_fungibles(&revenue) {
			Ok((asset_id, amount)) =>
				if !amount.is_zero() {
					let ok = Assets::mint_into(asset_id, &ReceiverAccount::get(), amount).is_ok();
					debug_assert!(ok, "`mint_into` cannot generally fail; qed");
				},
			Err(_) => log::debug!(
				target: "xcm",
				"take revenue failed matching fungible"
			),
		}
	}
}

parameter_types! {
	/// Xcm fees will go to the treasury account
	pub XcmFeesAccount: AccountId = Treasury::account_id();
}

pub type Traders = (
	UsingComponents<
		IdentityFee<Balance>,
		NewAnchoringSelfReserve,
		AccountId,
		Balances,
		DealWithFees<Runtime>,
	>,
	UsingComponents<
		IdentityFee<Balance>,
		OldAnchoringSelfReserve,
		AccountId,
		Balances,
		DealWithFees<Runtime>,
	>,
	// TODO::Implement foreign asset fee to weight rule from AssetManager Setting; Need more test
	FirstAssetTrader<
		CurrencyId,
		AssetManager,
		XcmFeesToAccount<
			Tokens,
			ConvertedConcreteAssetId<AssetId, Balance, AssetIdMuliLocationConvert, JustTry>,
			AccountId,
			XcmFeesAccount,
		>,
	>,
);

/// Xcm Weigher shared between multiple Xcm-related configs.
pub type XcmWeigher = FixedWeightBounds<UnitWeightCost, Call, MaxInstructions>;

pub struct XcmConfig;
impl xcm_executor::Config for XcmConfig {
	type Call = Call;
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
pub type LocalOriginToLocation = SignedToAccountId32<Origin, AccountId, RelayNetwork>;

/// The means for routing XCM messages which are not for local execution into the right message
/// queues.
pub type XcmRouter = (
	// Two routers - use UMP to communicate with the relay chain:
	// We use PolkadotXcm to confirm the XCM Version; Use () instead if pass anyway
	cumulus_primitives_utility::ParentAsUmp<ParachainSystem, PolkadotXcm>,
	// ..and XCMP to communicate with the sibling chains.
	XcmpQueue,
);

/// Instructs how to convert a 32 byte accountId into a MultiLocation
pub struct AccountIdToMultiLocation;
impl spConvert<AccountId, MultiLocation> for AccountIdToMultiLocation {
	fn convert(account: AccountId) -> MultiLocation {
		MultiLocation {
			parents: 0,
			interior: X1(Junction::AccountId32 { network: NetworkId::Any, id: account.into() }),
		}
	}
}

// Our currencyId. We distinguish for now between SelfReserve, and Others, defined by their Id.
#[derive(Clone, Eq, Debug, PartialEq, Ord, PartialOrd, Encode, Decode, TypeInfo)]
pub enum CurrencyId {
	SelfReserve, // The only parachain native token: LIT
	ParachainReserve(Box<MultiLocation>), /* Any parachain based asset, including local native
	              * minted ones. */
}
impl Default for CurrencyId {
	fn default() -> Self {
		CurrencyId::ParachainReserve(Box::new(MultiLocation::here()))
	}
}

impl From<MultiLocation> for CurrencyId {
	fn from(location: MultiLocation) -> Self {
		match location {
			a if (a == OldAnchoringSelfReserve::get()) | (a == NewAnchoringSelfReserve::get()) =>
				CurrencyId::SelfReserve,
			_ => CurrencyId::ParachainReserve(Box::new(location.into())),
		}
	}
}

impl From<Option<MultiLocation>> for CurrencyId {
	fn from(location: Option<MultiLocation>) -> Self {
		match location {
			Some(a)
				if (a == OldAnchoringSelfReserve::get()) |
					(a == NewAnchoringSelfReserve::get()) =>
				CurrencyId::SelfReserve,
			Some(multi) => CurrencyId::ParachainReserve(Box::new(multi)),
			None => CurrencyId::ParachainReserve(Box::new(MultiLocation::default())),
		}
	}
}
impl From<CurrencyId> for Option<MultiLocation> {
	fn from(currency_id: CurrencyId) -> Self {
		match currency_id {
			// For now and until Xtokens is adapted to handle 0.9.16 version we use
			// the old anchoring here
			// This is not a problem in either cases, since the view of the destination
			// chain does not change
			// TODO! change this to NewAnchoringSelfReserve once xtokens is adapted for it
			CurrencyId::SelfReserve => {
				let multi: MultiLocation = OldAnchoringSelfReserve::get();
				Some(multi)
			},
			CurrencyId::ParachainReserve(multi) => Some(*multi),
		}
	}
}

// How to convert from CurrencyId to MultiLocation: for orml convert sp_runtime Convert trait
pub struct CurrencyIdMultiLocationConvert;
impl spConvert<CurrencyId, Option<MultiLocation>> for CurrencyIdMultiLocationConvert {
	fn convert(currency: CurrencyId) -> Option<MultiLocation> {
		currency.into()
	}
}
impl spConvert<MultiLocation, Option<CurrencyId>> for CurrencyIdMultiLocationConvert {
	fn convert(multi: MultiLocation) -> Option<CurrencyId> {
		match multi {
			a if (a == OldAnchoringSelfReserve::get()) | (a == NewAnchoringSelfReserve::get()) =>
				Some(CurrencyId::SelfReserve),
			_ => Some(CurrencyId::ParachainReserve(Box::new(multi))),
		}
	}
}

/// Converter struct implementing `AssetIdConversion` converting a numeric asset ID
/// (must be `TryFrom/TryInto<u128>`) into a MultiLocation Value and Viceversa through
/// an intermediate generic type AssetType.
/// The trait bounds enforce is that the AssetTypeGetter trait is also implemented
pub struct AssetIdMuliLocationConvert;
impl xcmConvert<MultiLocation, AssetId> for AssetIdMuliLocationConvert {
	fn convert_ref(multi: impl Borrow<MultiLocation>) -> Result<AssetId, ()> {
		if let Some(currency_id) = <CurrencyIdMultiLocationConvert as spConvert<
			MultiLocation,
			Option<CurrencyId>,
		>>::convert(multi.borrow().clone().into())
		{
			if let Some(asset_id) =
				<AssetManager as AssetTypeGetter<AssetId, CurrencyId>>::get_asset_id(currency_id)
			{
				Ok(asset_id)
			} else {
				Err(())
			}
		} else {
			Err(())
		}
	}
	fn reverse_ref(asset_id: impl Borrow<AssetId>) -> Result<MultiLocation, ()> {
		if let Some(currency_id) =
			<AssetManager as AssetTypeGetter<AssetId, CurrencyId>>::get_asset_type(
				asset_id.borrow().clone().into(),
			) {
			if let Some(multi) = <CurrencyIdMultiLocationConvert as spConvert<
				CurrencyId,
				Option<MultiLocation>,
			>>::convert(currency_id)
			{
				Ok(multi)
			} else {
				Err(())
			}
		} else {
			Err(())
		}
	}
}

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

// Litentry: set this to max. The reason for doing so is to forbid using Fee Asset and Target Asset
// with different reserve chain.
parameter_type_with_key! {
	pub ParachainMinFee: |_location: MultiLocation| -> u128 {
		u128::MAX
	};
}

parameter_types! {
	pub SelfLocation: MultiLocation = MultiLocation {
		parents:1,
		interior: Junctions::X1(
			Parachain(ParachainInfo::parachain_id().into())
		)
	};
	pub const BaseXcmWeight: Weight = 100_000_000;
	pub const MaxAssetsForTransfer: usize = 3;
}

impl pallet_xcm::Config for Runtime {
	type Event = Event;
	type SendXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
	type XcmRouter = XcmRouter;
	type ExecuteXcmOrigin = EnsureXcmOrigin<Origin, LocalOriginToLocation>;
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
	type Origin = Origin;
	type Call = Call;
	const VERSION_DISCOVERY_QUEUE_SIZE: u32 = 100;
	// ^ Override for AdvertisedXcmVersion default
	type AdvertisedXcmVersion = pallet_xcm::CurrentXcmVersion;
}

impl cumulus_pallet_xcm::Config for Runtime {
	type Event = Event;
	type XcmExecutor = XcmExecutor<XcmConfig>;
}

impl orml_xtokens::Config for Runtime {
	type Event = Event;
	type Balance = Balance;
	type CurrencyId = CurrencyId;
	type AccountIdToMultiLocation = AccountIdToMultiLocation;
	type CurrencyIdConvert = CurrencyIdMultiLocationConvert;
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
