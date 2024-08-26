use frame_support::pallet_prelude::DispatchError;
pub trait BridgeHandler<B, A, R> {
	fn prepare_token_bridge_in(resource_id: R, who: A, amount: B) -> Result<B, DispatchError>;
	// Return actual amount to target chain after deduction e.g fee
	fn prepare_token_bridge_out(resource_id: R, who: A, amount: B) -> Result<B, DispatchError>;
}
