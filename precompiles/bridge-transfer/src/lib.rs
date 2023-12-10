// Copyright 2020-2023 Trust Computing GmbH.
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

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use fp_evm::{PrecompileHandle, PrecompileOutput};
use frame_support::{
	dispatch::{GetDispatchInfo, PostDispatchInfo},
	sp_runtime::Percent,
	traits::{Currency, Get},
};
use pallet_evm::AddressMapping;
use precompile_utils::{
    error, revert, succeed, Address, Bytes, EvmData, EvmDataWriter, EvmResult, FunctionModifier,
    PrecompileHandleExt, RuntimeHelper,
};
use sp_core::{H160, U256};
use sp_runtime::traits::Dispatchable;
use sp_std::{marker::PhantomData, vec::Vec};

type BalanceOf<Runtime> = <<Runtime as pallet_bridge::Config>::Currency as Currency<
	<Runtime as frame_system::Config>::AccountId,
>>::Balance;

/// A precompile to wrap the functionality from bridge_transfer.
///
/// EXAMPLE USECASE:
/// A simple example usecase is a contract that allows donors to donate, and stakes all the funds
/// toward one fixed address chosen by the deployer.
/// Such a contract could be deployed by a collator candidate, and the deploy address distributed to
/// supporters who want to donate toward a perpetual nomination fund.
pub struct BridgeTransferPrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> BridgeTransferPrecompile<Runtime>
where
	Runtime: pallet_bridge::Config + pallet_bridge_transfer::Config + pallet_evm::Config,
	Runtime::AccountId: From<[u8; 32]>,
	Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
	Runtime::RuntimeCall: From<pallet_bridge_transfer::Call<Runtime>>,
	BalanceOf<Runtime>: EvmData,
{
	fn transfer_native(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(3)?;
		let amount = input.read::<BalanceOf<Runtime>>()?;
		let receipt: Vec<u8> = input.read::<Bytes>()?.into();
		let dest_id: u8 = input.read::<u8>()?;

		// Build call with origin.
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call =
			pallet_bridge_transfer::Call::<Runtime>::transfer_native { amount, receipt, dest_id };

		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}
}

#[precompile_utils::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	TransferNative = "transferNative(uint256,bytes,uint8)",
}

impl<R> Precompile for BridgeTransferPrecompile<R>
where
	R: pallet_bridge::Config + pallet_bridge_transfer::Config + pallet_evm::Config,
	R::RuntimeCall: From<pallet_bridge_transfer::Call<R>>
		+ Dispatchable<PostInfo = PostDispatchInfo>
		+ GetDispatchInfo,
	<R::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<R::AccountId>>,
	BalanceOf<R>: EvmData,
	R::AccountId: From<[u8; 32]>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		log::trace!(target: "bridge-transfer-precompile", "Execute input = {:?}", handle.input());

		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::TransferNative => FunctionModifier::NonPayable,
			_ => FunctionModifier::View,
		})?;

		match selector {
			// read storage
			// None
			// Dispatchables
			Action::TransferNative => Self::transfer_native(handle),
		}
	}
}
