#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::PrecompileHandle;

use frame_support::dispatch::{GetDispatchInfo, PostDispatchInfo};
use pallet_collab_ai_common::GuardianVote;
use pallet_evm::AddressMapping;
use precompile_utils::prelude::*;
use sp_runtime::traits::Dispatchable;

use sp_core::{H256, U256};
use sp_std::marker::PhantomData;

pub struct GuardianPrecompile<Runtime>(PhantomData<Runtime>);

#[precompile_utils::precompile]
impl<Runtime> GuardianPrecompile<Runtime>
where
	Runtime: pallet_guardian::Config + pallet_evm::Config,
	Runtime::AccountId: From<[u8; 32]>,
	Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	Runtime::RuntimeCall: From<pallet_guardian::Call<Runtime>>,
	<Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
{
	#[precompile::public("registGuardian(bytes32)")]
	fn regist_guardian(handle: &mut impl PrecompileHandle, info_hash: H256) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let info_hash = info_hash.into();

		let call = pallet_guardian::Call::<Runtime>::regist_guardian { info_hash };
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("updateGuardian(bytes32)")]
	fn update_guardian(handle: &mut impl PrecompileHandle, info_hash: H256) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let info_hash = info_hash.into();

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
			3u8 => Ok(GuardianVote::Specific(
				potential_proposal_index
					.try_into()
					.map_err(|_| RevertReason::value_is_too_large("proposal index type"))?,
			)),
			_ => RevertReason::custom("Out of potential status result"),
		}
	}

	#[precompile::public("removeAllVotes()")]
	fn remove_all_votes(handle: &mut impl PrecompileHandle) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let call = pallet_guardian::Call::<Runtime>::remove_all_votes {};
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}
}
