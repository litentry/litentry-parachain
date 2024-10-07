#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::{PrecompileFailure, PrecompileHandle};
use frame_support::dispatch::{GetDispatchInfo, PostDispatchInfo};
use frame_system::pallet_prelude::BlockNumberFor;
use pallet_evm::AddressMapping;
use pallet_pool_proposal::AssetBalanceOf;
use precompile_utils::prelude::*;
use sp_runtime::traits::Dispatchable;

use sp_core::{H256, U256};
use sp_std::marker::PhantomData;

pub struct PoolProposalPrecompile<Runtime>(PhantomData<Runtime>);

#[precompile_utils::precompile]
impl<Runtime> PoolProposalPrecompile<Runtime>
where
	Runtime: pallet_pool_proposal::Config + pallet_evm::Config,
	Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	Runtime::RuntimeCall: From<pallet_pool_proposal::Call<Runtime>>,
	<Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
	AssetBalanceOf<Runtime>: TryFrom<U256> + Into<U256>,
	BlockNumberFor<Runtime>: TryFrom<U256> + Into<U256>,
{
	#[precompile::public("proposeInvestingPool(uint256,uint256,uint256,uint256,bytes32)")]
	fn propose_investing_pool(
		handle: &mut impl PrecompileHandle,
		max_pool_size: U256,
		proposal_last_time: U256,
		pool_last_time: U256,
		estimated_epoch_reward: U256,
		pool_info_hash: H256,
	) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let max_pool_size: AssetBalanceOf<Runtime> = max_pool_size.try_into().map_err(|_| {
			Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("balance type"))
		})?;

		let proposal_last_time: BlockNumberFor<Runtime> =
			proposal_last_time.try_into().map_err(|_| {
				Into::<PrecompileFailure>::into(RevertReason::value_is_too_large(
					"block number type",
				))
			})?;

		let pool_last_time: BlockNumberFor<Runtime> = pool_last_time.try_into().map_err(|_| {
			Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("block number type"))
		})?;

		let estimated_epoch_reward: AssetBalanceOf<Runtime> =
			estimated_epoch_reward.try_into().map_err(|_| {
				Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("balance type"))
			})?;

		let call = pallet_pool_proposal::Call::<Runtime>::propose_investing_pool {
			max_pool_size,
			proposal_last_time,
			pool_last_time,
			estimated_epoch_reward,
			pool_info_hash,
		};
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("preStakeProposal(uint256,uint256)")]
	fn pre_stake_proposal(
		handle: &mut impl PrecompileHandle,
		pool_proposal_index: U256,
		amount: U256,
	) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let pool_proposal_index = pool_proposal_index.try_into().map_err(|_| {
			Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("index type"))
		})?;

		let amount: AssetBalanceOf<Runtime> = amount.try_into().map_err(|_| {
			Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("balance type"))
		})?;

		let call = pallet_pool_proposal::Call::<Runtime>::pre_stake_proposal {
			pool_proposal_index,
			amount,
		};
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("withdrawPreInvesting(uint256,uint256)")]
	fn withdraw_pre_investing(
		handle: &mut impl PrecompileHandle,
		pool_proposal_index: U256,
		amount: U256,
	) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let pool_proposal_index = pool_proposal_index.try_into().map_err(|_| {
			Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("index type"))
		})?;

		let amount: AssetBalanceOf<Runtime> = amount.try_into().map_err(|_| {
			Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("balance type"))
		})?;

		let call = pallet_pool_proposal::Call::<Runtime>::withdraw_pre_investing {
			pool_proposal_index,
			amount,
		};
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("guardianParticipateProposal(uint256)")]
	fn guardian_participate_proposal(
		handle: &mut impl PrecompileHandle,
		pool_proposal_index: U256,
	) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let pool_proposal_index = pool_proposal_index.try_into().map_err(|_| {
			Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("index type"))
		})?;

		let call = pallet_pool_proposal::Call::<Runtime>::guardian_participate_proposal {
			pool_proposal_index,
		};
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}
}
