#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::{PrecompileFailure, PrecompileHandle};

use frame_support::dispatch::{GetDispatchInfo, PostDispatchInfo};
use pallet_evm::AddressMapping;
use precompile_utils::prelude::*;
use sp_runtime::traits::Dispatchable;

use sp_core::{H256, U256};
use sp_std::{marker::PhantomData, vec::Vec};

use pallet_bridge_transfer::BalanceOf;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub struct BridgeTransferPrecompile<Runtime>(PhantomData<Runtime>);

#[precompile_utils::precompile]
impl<Runtime> BridgeTransferPrecompile<Runtime>
where
	Runtime: pallet_bridge_transfer::Config + pallet_evm::Config,
	Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	Runtime::RuntimeCall: From<pallet_bridge_transfer::Call<Runtime>>,
	<Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
	BalanceOf<Runtime>: TryFrom<U256> + Into<U256>,
{
	#[precompile::public("transferAssets(uint256,bytes,uint8)")]
	fn transfer_assets(
		handle: &mut impl PrecompileHandle,
		amount: U256,
		recipient: UnboundedBytes,
		dest_id: u8,
	) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let amount: BalanceOf<Runtime> = amount.try_into().map_err(|_| {
			Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("balance type"))
		})?;
		let recipient: Vec<u8> = recipient.into();

		let call =
			pallet_bridge_transfer::Call::<Runtime>::transfer_native { amount, recipient, dest_id };
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}
}
