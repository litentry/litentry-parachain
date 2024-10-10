#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::{PrecompileFailure, PrecompileHandle};
use frame_support::{
	dispatch::{GetDispatchInfo, PostDispatchInfo},
	traits::Currency,
};
use frame_system::pallet_prelude::BlockNumberFor;
use pallet_evm::AddressMapping;
use pallet_pool_proposal::{
	AssetBalanceOf, Bond as PalletBond, PoolProposalInfo as PalletPoolProposalInfo,
};
use parity_scale_codec::MaxEncodedLen;
use precompile_utils::prelude::*;
use sp_runtime::traits::Dispatchable;

use sp_core::{Get, H256, U256};
use sp_std::marker::PhantomData;

use pallet_collab_ai_common::PoolProposalIndex;

pub struct PoolProposalPrecompile<Runtime>(PhantomData<Runtime>);

type BalanceOf<T> = <<T as pallet_pool_proposal::Config>::Currency as Currency<
	<T as frame_system::Config>::AccountId,
>>::Balance;

#[precompile_utils::precompile]
impl<Runtime> PoolProposalPrecompile<Runtime>
where
	Runtime: pallet_pool_proposal::Config + pallet_evm::Config,
	Runtime::AccountId: From<[u8; 32]> + Into<[u8; 32]>,
	Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	Runtime::RuntimeCall: From<pallet_pool_proposal::Call<Runtime>>,
	<Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
	AssetBalanceOf<Runtime>: TryFrom<U256> + Into<U256>,
	BlockNumberFor<Runtime>: TryFrom<U256> + Into<U256>,
	BalanceOf<Runtime>: TryFrom<U256> + Into<U256>,
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

		let pool_proposal_index: u128 = pool_proposal_index.try_into().map_err(|_| {
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

		let pool_proposal_index: u128 = pool_proposal_index.try_into().map_err(|_| {
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

		let pool_proposal_index: u128 = pool_proposal_index.try_into().map_err(|_| {
			Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("index type"))
		})?;

		let call = pallet_pool_proposal::Call::<Runtime>::guardian_participate_proposal {
			pool_proposal_index,
		};
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("poolProposalCount()")]
	#[precompile::view]
	fn pool_proposal_count(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
		// Storage item: PoolProposalCount ->
		// 		PoolProposalIndex (16)
		handle.record_db_read::<Runtime>(16)?;

		let next_proposal_index: U256 =
			pallet_pool_proposal::Pallet::<Runtime>::pool_proposal_count().into();
		Ok(next_proposal_index)
	}

	#[precompile::public("poolProposalDepositOf(bytes32)")]
	#[precompile::view]
	fn pool_proposal_deposit_of(
		handle: &mut impl PrecompileHandle,
		curator: H256,
	) -> EvmResult<Vec<DepositBond>> {
		// Storage item: PoolProposalDepositOf ->
		// 		OrderedSet<Bond<PoolProposalIndex, BalanceOf<T>>, T::MaximumPoolProposed>
		handle.record_db_read::<Runtime>(
			PalletBond::<PoolProposalIndex, BalanceOf<Runtime>>::max_encoded_len()
				.saturating_mul(Runtime::MaximumPoolProposed::get() as usize),
		)?;

		let curator: [u8; 32] = curator.into();
		let curator = Runtime::AccountId::from(curator);

		if let Some(result) =
			pallet_pool_proposal::Pallet::<Runtime>::pool_proposal_deposit_of(curator)
		{
			Ok(result
				.0
				.into_iter()
				.enumerate()
				.map(|(_index, bond)| DepositBond {
					owner: bond.owner.into(),
					amount: bond.amount.into(),
				})
				.collect())
		} else {
			Ok(Vec::new())
		}
	}

	#[precompile::public("pendingPoolProposalStatus()")]
	#[precompile::view]
	fn pending_pool_proposal_status(
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<Vec<PoolProposalStatus>> {
		// Storage item: PendingPoolProposalStatus ->
		// 		VecDeque<PoolProposalStatus<BlockNumberFor<T>>
		//      16 * max number
		handle.record_db_read::<Runtime>(
			16usize.saturating_mul(Runtime::MaximumPoolProposed::get() as usize),
		)?;

		let result = pallet_pool_proposal::Pallet::<Runtime>::pending_pool_proposal_status()
			.into_iter()
			.enumerate()
			.map(|(_index, status)| PoolProposalStatus {
				index: status.pool_proposal_index.into(),
				expiryTime: status.proposal_expire_time.into(),
			})
			.collect();

		Ok(result)
	}

	#[precompile::public("poolProposal(uint256)")]
	#[precompile::view]
	fn pool_proposal(
		handle: &mut impl PrecompileHandle,
		pool_proposal_index: U256,
	) -> EvmResult<(bool, PoolProposalInfo)> {
		// Storage item: PoolProposal ->
		// 		PoolProposalInfo<InfoHash, AssetBalanceOf<T>, BlockNumberFor<T>, T::AccountId>
		handle.record_db_read::<Runtime>(PalletPoolProposalInfo::<
			H256,
			AssetBalanceOf<Runtime>,
			BlockNumberFor<Runtime>,
			Runtime::AccountId,
		>::max_encoded_len())?;

		let pool_proposal_index: u128 = pool_proposal_index.try_into().map_err(|_| {
			Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("index type"))
		})?;

		if let Some(info) =
			pallet_pool_proposal::Pallet::<Runtime>::pool_proposal(pool_proposal_index)
		{
			let proposer: [u8; 32] = info.proposer.into();
			let proposer = proposer.into();

			let info = PoolProposalInfo {
				proposer,
				infoHash: info.pool_info_hash,
				maxPoolSize: info.max_pool_size.into(),
				poolStartTime: info.pool_start_time.into(),
				poolEndTime: info.pool_end_time.into(),
				estimatedEpochReward: info.estimated_epoch_reward.into(),
				proposalStatusFlags: info.proposal_status_flags.bits(),
			};

			Ok((true, info))
		} else {
			Ok((false, Default::default()))
		}
	}

	#[precompile::public("poolPreInvestings(uint256)")]
	#[precompile::view]
	fn pool_pre_investings(
		handle: &mut impl PrecompileHandle,
		pool_proposal_index: U256,
	) -> EvmResult<Vec<StakingBond>> {
		// Storage item: PoolPreInvestings ->
		// 		PoolProposalPreInvesting<T::AccountId, AssetBalanceOf<T>, BlockNumberFor<T>, T::MaximumPoolProposed>
		handle.record_db_read::<Runtime>(
			PalletBond::<Runtime::AccountId, AssetBalanceOf<Runtime>>::max_encoded_len()
				.saturating_mul(Runtime::MaximumPoolProposed::get() as usize),
		)?;

		let pool_proposal_index: u128 = pool_proposal_index.try_into().map_err(|_| {
			Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("index type"))
		})?;

		if let Some(result) =
			pallet_pool_proposal::Pallet::<Runtime>::pool_pre_investings(pool_proposal_index)
		{
			let bond_vec = result
				.pre_investings
				.into_iter()
				.enumerate()
				.map(|(index, bond)| {
					let owner: [u8; 32] = bond.owner.into();
					let owner = owner.into();
					StakingBond { owner, amount: bond.amount.into() }
				})
				.collect();

			Ok(bond_vec)
		} else {
			Ok(Default::default())
		}
	}

	#[precompile::public("poolPreInvestingsQueued(uint256)")]
	#[precompile::view]
	fn pool_pre_investings_queued(
		handle: &mut impl PrecompileHandle,
		pool_proposal_index: U256,
	) -> EvmResult<Vec<QueuedStakingBond>> {
		// Storage item: PoolPreInvestings ->
		// 		PoolProposalPreInvesting<T::AccountId, AssetBalanceOf<T>, BlockNumberFor<T>, T::MaximumPoolProposed>
		handle.record_db_read::<Runtime>(
			PalletBond::<Runtime::AccountId, AssetBalanceOf<Runtime>>::max_encoded_len()
				.saturating_mul(Runtime::MaximumPoolProposed::get() as usize),
		)?;

		let pool_proposal_index: u128 = pool_proposal_index.try_into().map_err(|_| {
			Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("index type"))
		})?;

		if let Some(result) =
			pallet_pool_proposal::Pallet::<Runtime>::pool_pre_investings(pool_proposal_index)
		{
			let bond_vec = result
				.queued_pre_investings
				.into_iter()
				.enumerate()
				.map(|(index, bond)| {
					let owner: [u8; 32] = bond.0.owner.into();
					let owner = owner.into();
					QueuedStakingBond {
						owner,
						amount: bond.0.amount.into(),
						queuedTime: bond.1.into(),
					}
				})
				.collect();

			Ok(bond_vec)
		} else {
			Ok(Default::default())
		}
	}

	#[precompile::public("poolGuardian(uint256)")]
	#[precompile::view]
	fn pool_guardian(
		handle: &mut impl PrecompileHandle,
		pool_proposal_index: U256,
	) -> EvmResult<Vec<H256>> {
		// Storage item: PoolGuardian ->
		// 		OrderedSet<T::AccountId, T::MaxGuardianPerProposal>
		handle.record_db_read::<Runtime>(
			32usize.saturating_mul(Runtime::MaxGuardianPerProposal::get() as usize),
		)?;

		let pool_proposal_index: u128 = pool_proposal_index.try_into().map_err(|_| {
			Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("index type"))
		})?;

		if let Some(result) =
			pallet_pool_proposal::Pallet::<Runtime>::pool_guardian(pool_proposal_index)
		{
			let guardian_vec = result
				.0
				.into_iter()
				.enumerate()
				.map(|(index, guardian)| {
					let guardian: [u8; 32] = guardian.into();
					guardian.into()
				})
				.collect();

			Ok(guardian_vec)
		} else {
			Ok(Default::default())
		}
	}
}

#[derive(Default, Debug, solidity::Codec)]
pub struct DepositBond {
	owner: U256,
	amount: U256,
}

#[derive(Default, Debug, solidity::Codec)]
pub struct StakingBond {
	owner: H256,
	amount: U256,
}

#[derive(Default, Debug, solidity::Codec)]
pub struct QueuedStakingBond {
	owner: H256,
	amount: U256,
	queuedTime: U256,
}

#[derive(Default, Debug, solidity::Codec)]
pub struct PoolProposalStatus {
	index: U256,
	expiryTime: U256,
}

#[derive(Default, Debug, solidity::Codec)]
struct PoolProposalInfo {
	proposer: H256,
	infoHash: H256,
	maxPoolSize: U256,
	poolStartTime: U256,
	poolEndTime: U256,
	estimatedEpochReward: U256,
	proposalStatusFlags: u8,
}
