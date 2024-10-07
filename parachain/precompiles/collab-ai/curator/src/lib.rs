#![cfg_attr(not(feature = "std"), no_std)]

use fp_evm::PrecompileHandle;

use frame_support::dispatch::{GetDispatchInfo, PostDispatchInfo};
use pallet_evm::AddressMapping;
use precompile_utils::prelude::*;
use sp_runtime::traits::Dispatchable;

use sp_core::H256;
use sp_std::marker::PhantomData;

pub struct CuratorPrecompile<Runtime>(PhantomData<Runtime>);

#[precompile_utils::precompile]
impl<Runtime> CuratorPrecompile<Runtime>
where
	Runtime: pallet_curator::Config + pallet_evm::Config,
	Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	Runtime::RuntimeCall: From<pallet_curator::Call<Runtime>>,
	<Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
{
	#[precompile::public("registCurator(bytes32)")]
	fn regist_curator(handle: &mut impl PrecompileHandle, info_hash: H256) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let info_hash = info_hash.into();

		let call = pallet_curator::Call::<Runtime>::regist_curator { info_hash };
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(())
	}

	#[precompile::public("updateCurator(bytes32)")]
	fn update_curator(handle: &mut impl PrecompileHandle, info_hash: H256) -> EvmResult {
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);

		let info_hash = info_hash.into();

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
}
