use frame_support::pallet_prelude::*;
use sp_runtime::DispatchError;

#[derive(PartialEq, Eq, Clone, Encode, Debug, Decode, TypeInfo)]
pub struct AssetInfo<AssetId, Balance> {
	pub fee: Balance,
	// None for native token
	pub asset: Option<AssetId>,
}

pub trait BridgeHandler<B, A, R, AssetId> {
	fn prepare_token_bridge_in(resource_id: R, who: A, amount: B) -> Result<B, DispatchError>;
	// Return actual amount to target chain after deduction e.g fee
	fn prepare_token_bridge_out(resource_id: R, who: A, amount: B) -> Result<B, DispatchError>;
	// Used to initialize setup for benchmark
	#[cfg(feature = "runtime-benchmarks")]
	fn setup_asset_info(
		resource_id: R,
		asset_info: AssetInfo<AssetId, B>,
	) -> Result<B, DispatchError>;
}
