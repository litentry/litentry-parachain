#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::PrecompileHandle;

use frame_support::dispatch::{GetDispatchInfo, PostDispatchInfo};
use pallet_evm::AddressMapping;
use precompile_utils::prelude::*;
use sp_runtime::traits::Dispatchable;

use sp_core::H256;
use sp_std::marker::PhantomData;

use pallet_collab_ai_common::CandidateStatus;

pub struct CuratorPrecompile<Runtime>(PhantomData<Runtime>);

#[precompile_utils::precompile]
impl<Runtime> CuratorPrecompile<Runtime>
where
	Runtime: pallet_curator::Config + pallet_evm::Config,
	Runtime::AccountId: Into<[u8; 32]>,
	Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	Runtime::RuntimeCall: From<pallet_curator::Call<Runtime>>,
	<Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
	BlockNumberFor<Runtime>: TryFrom<U256> + Into<U256>,
{
	#[precompile::public("registCurator(bytes32)")]
	fn regist_curator(handle: &mut impl PrecompileHandle, info_hash: H256) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let call = pallet_curator::Call::<Runtime>::regist_curator { info_hash };
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("updateCurator(bytes32)")]
	fn update_curator(handle: &mut impl PrecompileHandle, info_hash: H256) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let call = pallet_curator::Call::<Runtime>::update_curator { info_hash };
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("cleanCurator()")]
	fn clean_curator(handle: &mut impl PrecompileHandle) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let call = pallet_curator::Call::<Runtime>::clean_curator {};
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("publicCuratorCount()")]
	#[precompile::view]
	fn public_curator_count(handle: &mut impl PrecompileHandle) -> EvmResult<U256> {
		// Storage item: CuratorIndex u128:
		// 16
		handle.record_db_read::<Runtime>(16)?;

		Ok(pallet_curator::Pallet::<Runtime, Instance>::public_curator_count().into())
	}

	#[precompile::public("publicCuratorToIndex(bytes32)")]
	#[precompile::view]
	fn public_curator_to_index_sub(
		handle: &mut impl PrecompileHandle,
		curator: H256,
	) -> EvmResult<U256> {
		// Storage item: CuratorIndex u128:
		// Twox64Concat(8) + T::AccountId(32) + CuratorIndex(16)
		handle.record_db_read::<Runtime>(56)?;

		let curator = Runtime::AccountId::from(curator);

		Ok(pallet_curator::Pallet::<Runtime, Instance>::public_curator_to_index(curator).into())
	}

	#[precompile::public("publicCuratorToIndex(address)")]
	#[precompile::view]
	fn public_curator_to_index_evm(
		handle: &mut impl PrecompileHandle,
		curator: Address,
	) -> EvmResult<U256> {
		// Storage item: CuratorIndex u128:
		// Twox64Concat(8) + T::AccountId(32) + CuratorIndex(16)
		handle.record_db_read::<Runtime>(56)?;

		let curator = Runtime::AddressMapping::into_account_id(curator.into());

		Ok(pallet_curator::Pallet::<Runtime, Instance>::public_curator_to_index(curator).into())
	}

	fn candidate_status_to_u8(status: CandidateStatus) -> MayRevert<u8> {
		status
			.try_into()
			.map_err(|_| RevertReason::value_is_too_large("trackId type").into())
	}

	#[precompile::public("curatorIndexToInfo(uint256)")]
	#[precompile::view]
	fn curator_index_to_info(
		handle: &mut impl PrecompileHandle,
		index: U256,
	) -> EvmResult<(H256, U256, H256, u8)> {
		// Storage item: CuratorIndex u128:
		// Twox64Concat(8) + CuratorIndex(16) + InfoHash(32) + BlockNumber(4) + T::AccountId(32) + CandidateStatus(1)
		handle.record_db_read::<Runtime>(93)?;

		let index: AssetBalanceOf<Runtime> = index.try_into().map_err(|_| {
			Into::<PrecompileFailure>::into(RevertReason::value_is_too_large("index type"))
		})?;
		let (info_hash, update_block, curator, status) =
			pallet_curator::Pallet::<Runtime, Instance>::curator_index_to_info(index);

		let update_block: U256 = update_block.into();

		let curator: [u8; 32] = curator.into();
		let curator: H256 = curator.into();

		let status = Self::candidate_status_to_u8(status).in_field("candidateStatus")?;

		Ok((info_hash, update_block, curator, status))
	}
}
