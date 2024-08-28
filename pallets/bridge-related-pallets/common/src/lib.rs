#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "runtime-benchmarks")]
use frame_support::pallet_prelude::DispatchResult;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::DispatchError;

#[derive(PartialEq, Eq, Clone, Encode, Debug, Decode, TypeInfo)]
pub struct AssetInfo<AssetId, Balance> {
	pub fee: Balance,
	// None for native token
	pub asset: Option<AssetId>,
}

pub trait BridgeHandler<B, A, R> {
	fn prepare_token_bridge_in(resource_id: R, who: A, amount: B) -> Result<B, DispatchError>;
	// Return actual amount to target chain after deduction e.g fee
	fn prepare_token_bridge_out(resource_id: R, who: A, amount: B) -> Result<B, DispatchError>;
	// Used to initialize setup for benchmark
	#[cfg(feature = "runtime-benchmarks")]
	fn setup_asset_info(resource_id: R, fee: B) -> DispatchResult;
}
