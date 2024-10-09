#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::{PrecompileFailure, PrecompileHandle};
use frame_support::dispatch::{GetDispatchInfo, PostDispatchInfo};
use frame_system::pallet_prelude::BlockNumberFor;
use pallet_evm::AddressMapping;
use precompile_utils::prelude::*;
use sp_runtime::traits::Dispatchable;

use sp_core::{H256, U256};
use sp_std::marker::PhantomData;

use pallet_collab_ai_common::{CandidateStatus, GuardianVote};

pub struct GuardianPrecompile<Runtime>(PhantomData<Runtime>);

#[precompile_utils::precompile]
impl<Runtime> GuardianPrecompile<Runtime>
where
	Runtime: pallet_guardian::Config + pallet_evm::Config,
	Runtime::AccountId: From<[u8; 32]> + Into<[u8; 32]>,
	Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	Runtime::RuntimeCall: From<pallet_guardian::Call<Runtime>>,
	<Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
	BlockNumberFor<Runtime>: TryFrom<U256> + Into<U256>,
{
	#[precompile::public("registGuardian(bytes32)")]
	fn regist_guardian(handle: &mut impl PrecompileHandle, info_hash: H256) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let call = pallet_guardian::Call::<Runtime>::regist_guardian { info_hash };
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("updateGuardian(bytes32)")]
	fn update_guardian(handle: &mut impl PrecompileHandle, info_hash: H256) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let call = pallet_guardian::Call::<Runtime>::update_guardian { info_hash };
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("cleanGuardian()")]
	fn clean_guardian(handle: &mut impl PrecompileHandle) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let call = pallet_guardian::Call::<Runtime>::clean_guardian {};
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("vote(bytes32,uint8,uint256)")]
	fn vote(
		handle: &mut impl PrecompileHandle,
		guardian: H256,
		status: u8,
		potential_proposal_index: U256,
	) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let guardian: [u8; 32] = guardian.into();
		let guardian = Runtime::AccountId::from(guardian);
		let guardian_vote: GuardianVote =
			Self::to_guardian_vote(status, potential_proposal_index).in_field("guardianVote")?;
		let call = pallet_guardian::Call::<Runtime>::vote { guardian, status: Some(guardian_vote) };
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	fn to_guardian_vote(status: u8, potential_proposal_index: U256) -> MayRevert<GuardianVote> {
		match status {
			0u8 => Ok(GuardianVote::Neutral),
			1u8 => Ok(GuardianVote::Aye),
			2u8 => Ok(GuardianVote::Nay),
			3u8 => {
				Ok(GuardianVote::Specific(potential_proposal_index.try_into().map_err(|_| {
					<RevertReason as Into<Revert>>::into(RevertReason::value_is_too_large(
						"proposal index type",
					))
				})?))
			},
			_ => Err(RevertReason::custom("Out of potential status result").into()),
		}
	}

	fn guardian_vote_to(guardian_vote: Option<GuardianVote>) -> MayRevert<(u8, U256)> {
		match guardian_vote {
			None => Ok((0u8, 0.into())),
			Some(GuardianVote::Neutral) => Ok((0u8, 0.into())),
			Some(GuardianVote::Aye) => Ok((1u8, 0.into())),
			Some(GuardianVote::Nay) => Ok((2u8, 0.into())),
			Some(GuardianVote::Specific(i)) => Ok((3u8, i.into())),
		}
	}

	#[precompile::public("removeAllVotes()")]
	fn remove_all_votes(handle: &mut impl PrecompileHandle) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let call = pallet_guardian::Call::<Runtime>::remove_all_votes {};
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("PublicGuardianCount()")]
	#[precompile::view]
	fn public_guardian_count(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
		// Storage item: GuardianIndex u128:
		// 16
		handle.record_db_read::<Runtime>(16)?;

		Ok(pallet_guardian::Pallet::<Runtime>::public_guardian_count().into())
	}

	#[precompile::public("publicGuardianToIndex(bytes32)")]
	#[precompile::view]
	fn public_guardian_to_index_sub(
		handle: &mut impl PrecompileHandle,
		guardian: H256,
	) -> EvmResult<U256> {
		// Storage item: GuardianIndex u128:
		// Twox64Concat(8) + T::AccountId(32) + GuardianIndex(16)
		handle.record_db_read::<Runtime>(56)?;

		let guardian = Runtime::AccountId::from(guardian.into());

		Ok(pallet_guardian::Pallet::<Runtime>::public_guardian_to_index(guardian).into())
	}

	#[precompile::public("publicGuardianToIndex(address)")]
	#[precompile::view]
	fn public_guardian_to_index_evm(
		handle: &mut impl PrecompileHandle,
		guardian: Address,
	) -> EvmResult<U256> {
		// Storage item: GuardianIndex u128:
		// Twox64Concat(8) + T::AccountId(32) + GuardianIndex(16)
		handle.record_db_read::<Runtime>(56)?;

		let guardian = Runtime::AddressMapping::into_account_id(guardian.into());

		Ok(pallet_guardian::Pallet::<Runtime>::public_guardian_to_index(guardian).into())
	}

	fn candidate_status_to_u8(status: CandidateStatus) -> MayRevert<u8> {
		status
			.try_into()
			.map_err(|_| RevertReason::value_is_too_large("trackId type").into())
	}

	#[precompile::public("guardianIndexToInfo(uint256)")]
	#[precompile::view]
	fn guardian_index_to_info(
		handle: &mut impl PrecompileHandle,
		index: U256,
	) -> EvmResult<(H256, U256, H256, u8)> {
		// Storage item: GuardianIndex u128:
		// Twox64Concat(8) + GuardianIndex(16) + InfoHash(32) + BlockNumber(4) + T::AccountId(32) + CandidateStatus(1)
		handle.record_db_read::<Runtime>(93)?;

		let index: AssetBalanceOf<Runtime> = index.try_into().map_err(|_| {
			Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("index type"))
		})?;
		let Some((info_hash, update_block, guardian, status)) =
			pallet_guardian::Pallet::<Runtime>::guardian_index_to_info(index);

		let update_block: U256 = update_block.into();

		let guardian: [u8; 32] = guardian.into();
		let guardian: H256 = guardian.into();

		let status = Self::candidate_status_to_u8(status).in_field("candidateStatus")?;

		Ok((info_hash, update_block, guardian, status))
	}

	#[precompile::public("guardianVotes(bytes32,uint256)")]
	#[precompile::view]
	fn guardian_votes_sub(
		handle: &mut impl PrecompileHandle,
		voter: H256,
		guardian_index: U256,
	) -> EvmResult<(u8, U256)> {
		// Storage item: GuardianIndex u128:
		// 2 * Twox64Concat(8) + GuardianIndex(16) + T::AccountId(32) + GuardianVote(1) + ProposalIndex(16)
		handle.record_db_read::<Runtime>(81)?;

		let voter = Runtime::AccountId::from(voter.into());

		let guardian_index: AssetBalanceOf<Runtime> = guardian_index.try_into().map_err(|_| {
			Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("index type"))
		})?;
		let result = Self::guardian_vote_to(pallet_guardian::Pallet::<Runtime>::guardian_votes(
			voter,
			guardian_index,
		))
		.in_field("GuardianVote")?;

		Ok(result)
	}

	#[precompile::public("guardianVotes(address,uint256)")]
	#[precompile::view]
	fn guardian_votes_evm(
		handle: &mut impl PrecompileHandle,
		voter: Address,
		guardian_index: U256,
	) -> EvmResult<(u8, U256)> {
		// Storage item: GuardianIndex u128:
		// 2 * Twox64Concat(8) + GuardianIndex(16) + T::AccountId(32) + GuardianVote(1) + ProposalIndex(16)
		handle.record_db_read::<Runtime>(81)?;

		let voter = Runtime::AddressMapping::into_account_id(voter.into());
		let guardian_index: AssetBalanceOf<Runtime> = guardian_index.try_into().map_err(|_| {
			Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("index type"))
		})?;
		let result = Self::guardian_vote_to(pallet_guardian::Pallet::<Runtime>::guardian_votes(
			voter,
			guardian_index,
		))
		.in_field("GuardianVote")?;

		Ok(result)
	}
}
