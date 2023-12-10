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

type BalanceOf<Runtime> = <<Runtime as pallet_parachain_staking::Config>::Currency as Currency<
	<Runtime as frame_system::Config>::AccountId,
>>::Balance;

/// A precompile to wrap the functionality from parachain_staking.
///
/// EXAMPLE USECASE:
/// A simple example usecase is a contract that allows donors to donate, and stakes all the funds
/// toward one fixed address chosen by the deployer.
/// Such a contract could be deployed by a collator candidate, and the deploy address distributed to
/// supporters who want to donate toward a perpetual nomination fund.
pub struct ParachainStakingPrecompile<Runtime>(PhantomData<Runtime>);

impl<Runtime> ParachainStakingPrecompile<Runtime>
where
	Runtime: pallet_parachain_staking::Config + pallet_evm::Config,
	Runtime::AccountId: From<[u8; 32]>,
	Runtime::RuntimeCall: Dispatchable<PostInfo = PostDispatchInfo> + GetDispatchInfo,
	<Runtime::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<Runtime::AccountId>>,
	Runtime::RuntimeCall: From<pallet_parachain_staking::Call<Runtime>>,
	BalanceOf<Runtime>: EvmData,
{
	/* TODO: Only part for delagator is implemented for minimal task purpose
	// Constants
	fn min_delegation(_handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let min_nomination: U256 =
			<<Runtime as pallet_parachain_staking::Config>::MinDelegation as Get<
				BalanceOf<Runtime>,
			>>::get()
			.try_into()
			.map_err(|_| revert("Amount is too large for provided balance type"))?;

			Ok(succeed(EvmDataWriter::new().write(min_nomination).build()))
	}

	// Storage Getters
	fn points(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;

		let round: u32 = input.read::<u32>()?;
		// AccountsPayable: Twox64Concat(8) + RoundIndex(4) + RewardPoint(4)
		handle.record_db_read::<Runtime>(16)?;
		let points: u32 = pallet_parachain_staking::Pallet::<Runtime>::points(round);
		Ok(succeed(EvmDataWriter::new().write(points).build()))
	}

	fn awarded_points(
		handle: &mut impl PrecompileHandle
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;

		let round: u32 = input.read::<u32>()?;

		// AccountsPayable: Twox64Concat(8) + RoundIndex(4) + Twox64Concat(8) + AccountId(32)
		// + RewardPoint(4)
		handle.record_db_read::<Runtime>(56)?;

		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);

		let points: u32 = <pallet_parachain_staking::Pallet<Runtime>>::awarded_pts(&round, &candidate);

		Ok(succeed(EvmDataWriter::new().write(points).build()))
	}

	fn candidate_count(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		// CandidatePool: UnBoundedVec(AccountId(20) + Balance(16))
		// TODO CandidatePool is unbounded, we account for a theoretical 200 pool.
		handle.record_db_read::<Runtime>(7200)?;
		// Fetch info.
		let candidate_count: u32 = <pallet_parachain_staking::Pallet<Runtime>>::candidate_pool()
			.0
			.len() as u32;

		// Build output.
		Ok(succeed(EvmDataWriter::new().write(candidate_count).build()))
	}

	fn round(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		// Round: RoundInfo(RoundIndex(4) + BlockNumber(4) + 4)
		handle.record_db_read::<Runtime>(12)?;
		let round: u32 = <pallet_parachain_staking::Pallet<Runtime>>::round().current;

		Ok(succeed(EvmDataWriter::new().write(round).build()))
	}

	fn candidate_delegation_count(
		handle: &mut impl PrecompileHandle
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);
		// CandidateInfo: Twox64Concat(8) + AccountId(32) + CandidateMetadata(105)
		handle.record_db_read::<Runtime>(145)?;
		let result = if let Some(state) =
			<pallet_parachain_staking::Pallet<Runtime>>::candidate_info(&candidate)
		{
			let candidate_delegation_count: u32 = state.delegation_count;

			log::trace!(
				target: "staking-precompile",
				"Result from pallet is {:?}",
				candidate_delegation_count
			);
			candidate_delegation_count
		} else {
			log::trace!(
				target: "staking-precompile",
				"Candidate {:?} not found, so delegation count is 0",
				candidate
			);
			0u32
		};

		Ok(succeed(EvmDataWriter::new().write(result).build()))
	}

	fn candidate_auto_compounding_delegation_count(
		handle: &mut impl PrecompileHandle
	) -> EvmResult<PrecompileOutput> {
		// AutoCompoundingDelegations:
		// Blake2128(16) + AccountId(20)
		// + BoundedVec(
		// 	AutoCompoundConfig * (MaxTopDelegationsPerCandidate + MaxBottomDelegationsPerCandidate)
		// )
		handle.record_db_read::<Runtime>(
			36 + (
				22 * (<Runtime as pallet_parachain_staking::Config>::MaxTopDelegationsPerCandidate::get()
				+ <Runtime as pallet_parachain_staking::Config>::MaxBottomDelegationsPerCandidate::get())
				as usize),
		)?;

		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);

		let count =
			<pallet_parachain_staking::Pallet<Runtime>>::auto_compounding_delegations(&candidate)
				.len() as u32;

		Ok(succeed(EvmDataWriter::new().write(count).build()))
	}

	fn delegator_delegation_count(
		handle: &mut impl PrecompileHandle
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let delegator: [u8; 32] = input.read::<H256>()?.into();
		let delegator = Runtime::AccountId::from(delegator);

		// CandidateInfo:
		// Twox64Concat(8) + AccountId(32) + Delegator(56 + MaxDelegationsPerDelegator)
		handle.record_db_read::<Runtime>(
			96 + (<Runtime as pallet_parachain_staking::Config>::MaxDelegationsPerDelegator::get()
				as usize),
		)?;
		let result = if let Some(state) =
			<pallet_parachain_staking::Pallet<Runtime>>::delegator_state(&delegator)
		{
			let delegator_delegation_count: u32 = state.delegations.0.len() as u32;

			log::trace!(
				target: "staking-precompile",
				"Result from pallet is {:?}",
				delegator_delegation_count
			);

			delegator_delegation_count
		} else {
			log::trace!(
				target: "staking-precompile",
				"Delegator {:?} not found, so delegation count is 0",
				delegator
			);
			0u32
		};

		Ok(succeed(EvmDataWriter::new().write(result).build()))
	}

	fn selected_candidates(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		// TotalSelected
		handle.record_db_read::<Runtime>(4)?;
		let total_selected = pallet_parachain_staking::Pallet::<Runtime>::total_selected();
		// SelectedCandidates: total_selected * AccountId(32)
		handle.record_db_read::<Runtime>(32 * (total_selected as usize))?;
		let selected_candidates: Vec<[u8; 32]> =
			pallet_parachain_staking::Pallet::<Runtime>::selected_candidates()
				.into_iter()
				.map(|address| address.into())
				.collect();

		Ok(succeed(EvmDataWriter::new().write(selected_candidates).build()))
	}

	fn delegation_amount(
		handle: &mut impl PrecompileHandle
	) -> EvmResult<PrecompileOutput> {
		// DelegatorState:
		// Twox64Concat(8) + AccountId(32) + Delegator(56 + MaxDelegationsPerDelegator)
		handle.record_db_read::<Runtime>(
			96 + (<Runtime as pallet_parachain_staking::Config>::MaxDelegationsPerDelegator::get()
				as usize),
		)?;

		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;
		let delegator: [u8; 32] = input.read::<H256>()?.into();
		let delegator = Runtime::AccountId::from(delegator);
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);

		let amount = pallet_parachain_staking::Pallet::<Runtime>::delegator_state(&delegator)
			.and_then(|state| {
				state
					.delegations
					.0
					.into_iter()
					.find(|b| b.owner == candidate)
			})
			.map_or(
				U256::zero(),
				|pallet_parachain_staking::Bond { amount, .. }| amount.into(),
			);

		Ok(succeed(EvmDataWriter::new().write(amount).build()))
	}

	// Role Verifiers
	fn is_in_top_delegations(
		handle: &mut impl PrecompileHandle
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;
		let delegator: [u8; 32] = input.read::<H256>()?.into();
		let delegator = Runtime::AccountId::from(delegator);
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);

		// TopDelegations:
		// Twox64Concat(8) + AccountId(32) + Balance(16)
		// + (AccountId(32) + Balance(16) * MaxTopDelegationsPerCandidate)
		handle.record_db_read::<Runtime>(
			56 + ((48
				* <Runtime as pallet_parachain_staking::Config>::MaxTopDelegationsPerCandidate::get(
				)) as usize),
		)?;
		let is_in_top_delegations = pallet_parachain_staking::Pallet::<Runtime>::top_delegations(
			&candidate,
		)
		.map_or(false, |delegations| {
			delegations
				.delegations
				.into_iter()
				.any(|b| b.owner == delegator)
		});

		Ok(succeed(EvmDataWriter::new().write(is_in_top_delegations).build()))
	}

	fn is_delegator(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let delegator: [u8; 32] = input.read::<H256>()?.into();
		let delegator = Runtime::AccountId::from(delegator);
		// DelegatorState:
		// Twox64Concat(8) + AccountId(32) + Delegator(56 + MaxDelegationsPerDelegator)
		handle.record_db_read::<Runtime>(
			96 + (<Runtime as pallet_parachain_staking::Config>::MaxDelegationsPerDelegator::get()
				as usize),
		)?;
		let is_delegator = pallet_parachain_staking::Pallet::<Runtime>::is_delegator(&delegator);

		Ok(succeed(EvmDataWriter::new().write(is_delegator).build()))
	}

	fn is_candidate(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);

		// CandidateInfo: Twox64Concat(8) + AccountId(32) + CandidateMetadata(105)
		handle.record_db_read::<Runtime>(145)?;
		let is_candidate = pallet_parachain_staking::Pallet::<Runtime>::is_candidate(&candidate);

		Ok(succeed(EvmDataWriter::new().write(is_candidate).build()))
	}

	fn is_selected_candidate(
		handle: &mut impl PrecompileHandle
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);

		// TotalSelected
		handle.record_db_read::<Runtime>(4)?;
		let total_selected = pallet_parachain_staking::Pallet::<Runtime>::total_selected();
		// SelectedCandidates: total_selected * AccountId(32)
		handle.record_db_read::<Runtime>(32 * (total_selected as usize))?;
		let is_selected =
			pallet_parachain_staking::Pallet::<Runtime>::is_selected_candidate(&candidate);

		Ok(succeed(EvmDataWriter::new().write(is_selected).build()))
	}

	TODO: Only part for delagator is implemented for minimal task purpose*/
	fn delegation_request_is_pending(
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;
		let delegator: [u8; 32] = input.read::<U256>()?.into();
		let delegator = Runtime::AccountId::from(delegator);
		let candidate: [u8; 32] = input.read::<U256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);

		// DelegationScheduledRequests:
		// Blake2128(16) + AccountId(32)
		// + Vec(
		// 	ScheduledRequest(32 + 4 + DelegationAction(18))
		// 	* (MaxTopDelegationsPerCandidate + MaxBottomDelegationsPerCandidate)
		// )
		handle.record_db_read::<Runtime>(
			48 + (
				56 * (<Runtime as pallet_parachain_staking::Config>::MaxTopDelegationsPerCandidate::get()
				+ <Runtime as pallet_parachain_staking::Config>::MaxBottomDelegationsPerCandidate::get())
				as usize),
		)?;

		// If we are not able to get delegator state, we return false
		// Users can call `is_delegator` to determine when this happens
		let pending = <pallet_parachain_staking::Pallet<Runtime>>::delegation_request_exists(
			&candidate, &delegator,
		);

		Ok(succeed(EvmDataWriter::new().write(pending).build()))
	}
	/*TODO: Only part for delagator is implemented for minimal task purpose

	fn candidate_exit_is_pending(
		handle: &mut impl PrecompileHandle
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);

		// CandidateInfo: Twox64Concat(8) + AccountId(32) + CandidateMetadata(105)
		handle.record_db_read::<Runtime>(145)?;

		// If we are not able to get delegator state, we return false
		// Users can call `is_candidate` to determine when this happens
		let pending = if let Some(state) =
			<pallet_parachain_staking::Pallet<Runtime>>::candidate_info(&candidate)
		{
			state.is_leaving()
		} else {
			log::trace!(
				target: "staking-precompile",
				"Candidate state for {:?} not found, so pending exit is false",
				candidate
			);
			false
		};

		Ok(succeed(EvmDataWriter::new().write(pending).build()))
	}

	fn candidate_request_is_pending(
		handle: &mut impl PrecompileHandle
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);

		// CandidateInfo: Twox64Concat(8) + AccountId(32) + CandidateMetadata(105)
		handle.record_db_read::<Runtime>(145)?;

		// If we are not able to get candidate metadata, we return false
		// Users can call `is_candidate` to determine when this happens
		let pending = if let Some(state) =
			<pallet_parachain_staking::Pallet<Runtime>>::candidate_info(&candidate)
		{
			state.request.is_some()
		} else {
			log::trace!(
				target: "staking-precompile",
				"Candidate metadata for {:?} not found, so pending request is false",
				candidate
			);
			false
		};

		Ok(succeed(EvmDataWriter::new().write(pending).build()))
	}

	fn delegation_auto_compound(
		handle: &mut impl PrecompileHandle
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;
		let delegator: [u8; 32] = input.read::<H256>()?.into();
		let delegator = Runtime::AccountId::from(delegator);
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);

		// AutoCompoundingDelegations:
		// Blake2128(16) + AccountId(32)
		// + BoundedVec(
		// 	AutoCompoundConfig * (MaxTopDelegationsPerCandidate + MaxBottomDelegationsPerCandidate)
		// )
		handle.record_db_read::<Runtime>(
			48 + (
				22 * (<Runtime as pallet_parachain_staking::Config>::MaxTopDelegationsPerCandidate::get()
				+ <Runtime as pallet_parachain_staking::Config>::MaxBottomDelegationsPerCandidate::get())
				as usize),
		)?;

		let value: u8 = <pallet_parachain_staking::Pallet<Runtime>>::delegation_auto_compound(
			&candidate, &delegator,
		).deconstruct();

		Ok(succeed(EvmDataWriter::new().write(value).build()))
	}

	// Runtime Methods (dispatchables)
	fn join_candidates(
		handle: &mut impl PrecompileHandle
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let amount = input.read::<BalanceOf<Runtime>>()?;

		// Build call with origin.
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = pallet_parachain_staking::Call::<Runtime>::join_candidates {
			bond: amount,
		};

		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn schedule_leave_candidates(
		handle: &mut impl PrecompileHandle
	) -> EvmResult<PrecompileOutput> {
		// Build call with origin.
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = pallet_parachain_staking::Call::<Runtime>::schedule_leave_candidates {};

		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn execute_leave_candidates(
		handle: &mut impl PrecompileHandle
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);

		// Build call with origin.
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = pallet_parachain_staking::Call::<Runtime>::execute_leave_candidates {
			candidate,
		};

		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn cancel_leave_candidates(
		handle: &mut impl PrecompileHandle
	) -> EvmResult<PrecompileOutput> {
		// Build call with origin.
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call =
			pallet_parachain_staking::Call::<Runtime>::cancel_leave_candidates {};

		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn go_offline(_: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		// go_offline should always fail when called via evm precompiles.
		Err(error("go_offline via evm precompile is not allowed"))
	}

	fn go_online(_: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		/// go_online should always fail when called via evm precompiles.
		Err(error("go_online via evm precompile is not allowed"))
	}

	fn candidate_bond_more(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let more = input.read::<BalanceOf<Runtime>>()?;

		// Build call with origin.
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = pallet_parachain_staking::Call::<Runtime>::candidate_bond_more { more };

		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn schedule_candidate_bond_less(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let less = input.read::<BalanceOf<Runtime>>()?;

		// Build call with origin.
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = pallet_parachain_staking::Call::<Runtime>::schedule_candidate_bond_less { less };

		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn execute_candidate_bond_less(
		handle: &mut impl PrecompileHandle
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);

		// Build call with origin.
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call =
			pallet_parachain_staking::Call::<Runtime>::execute_candidate_bond_less { candidate };

		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn cancel_candidate_bond_less(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		// Build call with origin.
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = pallet_parachain_staking::Call::<Runtime>::cancel_candidate_bond_less {};

		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}
	TODO: Only part for delagator is implemented for minimal task purpose*/

	fn delegate(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);
		let amount = input.read::<BalanceOf<Runtime>>()?;

		// Build call with origin.
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = pallet_parachain_staking::Call::<Runtime>::delegate { candidate, amount };

		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn delegate_with_auto_compound(
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(3)?;
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);
		let amount = input.read::<BalanceOf<Runtime>>()?;
		let auto_compound: u8 = input.read::<u8>()?;

		if auto_compound > 100 {
			return Err(RevertReason::custom("Must be an integer between 0 and 100 included")
				.in_field("auto_compound")
				.into())
		}

		// Build call with origin.
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = pallet_parachain_staking::Call::<Runtime>::delegate_with_auto_compound {
			candidate,
			amount,
			auto_compound,
		};

		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn schedule_revoke_delegation(
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);

		// Build call with origin.
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = pallet_parachain_staking::Call::<Runtime>::schedule_revoke_delegation {
			collator: candidate,
		};

		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn delegator_bond_more(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);
		let more = input.read::<BalanceOf<Runtime>>()?;

		// Build call with origin.
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call =
			pallet_parachain_staking::Call::<Runtime>::delegator_bond_more { candidate, more };

		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn schedule_delegator_bond_less(
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);
		let less = input.read::<BalanceOf<Runtime>>()?;

		// Build call with origin.
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = pallet_parachain_staking::Call::<Runtime>::schedule_delegator_bond_less {
			candidate,
			less,
		};

		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn execute_delegation_request(
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;
		let delegator: [u8; 32] = input.read::<H256>()?.into();
		let delegator = Runtime::AccountId::from(delegator);
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);

		// Build call with origin.
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call = pallet_parachain_staking::Call::<Runtime>::execute_delegation_request {
			delegator,
			candidate,
		};

		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn cancel_delegation_request(
		handle: &mut impl PrecompileHandle,
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);

		// Build call with origin.
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call =
			pallet_parachain_staking::Call::<Runtime>::cancel_delegation_request { candidate };

		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	fn set_auto_compound(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(2)?;
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);
		let value: u8 = input.read::<u8>()?;

		if value > 100 {
			return Err(RevertReason::custom("Must be an integer between 0 and 100 included")
				.in_field("value")
				.into())
		}

		let value = Percent::from_percent(value);

		// Build call with origin.
		let origin = Runtime::AddressMapping::into_account_id(handle.context().caller);
		let call =
			pallet_parachain_staking::Call::<Runtime>::set_auto_compound { candidate, value };

		// Dispatch call (if enough gas).
		RuntimeHelper::<Runtime>::try_dispatch(handle, Some(origin).into(), call)?;

		Ok(succeed(EvmDataWriter::new().write(true).build()))
	}

	/* TODO: Only part for delagator is implemented for minimal task purpose
	fn get_delegator_total_staked(
		handle: &mut impl PrecompileHandle
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let delegator: [u8; 32] = input.read::<H256>()?.into();
		let delegator = Runtime::AccountId::from(delegator);

		// DelegatorState:
		// Twox64Concat(8) + AccountId(32) + Delegator(56 + MaxDelegationsPerDelegator)
		handle.record_db_read::<Runtime>(
			96 + (<Runtime as pallet_parachain_staking::Config>::MaxDelegationsPerDelegator::get()
				as usize),
		)?;

		let amount: U256 = <pallet_parachain_staking::Pallet<Runtime>>::delegator_state(&delegator)
			.map(|state| state.total)
			.unwrap_or_default().into();

		Ok(succeed(EvmDataWriter::new().write(amount).build()))
	}

	fn get_candidate_total_counted(
		handle: &mut impl PrecompileHandle
	) -> EvmResult<PrecompileOutput> {
		let mut input = handle.read_input()?;
		input.expect_arguments(1)?;
		let candidate: [u8; 32] = input.read::<H256>()?.into();
		let candidate = Runtime::AccountId::from(candidate);

		// CandidateInfo: Twox64Concat(8) + AccountId(32) + CandidateMetadata(105)
		handle.record_db_read::<Runtime>(145)?;

		let amount: U256 = <pallet_parachain_staking::Pallet<Runtime>>::candidate_info(&candidate)
			.map(|state| state.total_counted)
			.unwrap_or_default().into();

		Ok(succeed(EvmDataWriter::new().write(amount).build()))
	}
	TODO: Only part for delagator is implemented for minimal task purpose*/
}

#[precompile_utils::generate_function_selector]
#[derive(Debug, PartialEq)]
pub enum Action {
	DelegationRequestIsPending = "delegationRequestIsPending(bytes32,bytes32)",
	Delegate = "delegate(bytes32,uint256)",
	DelegateWithAutoCompound = "delegateWithAutoCompound(bytes32,uint256,uint8)",
	ScheduleRevokeDelegation = "scheduleRevokeDelegation(bytes32)",
	DelegatorBondMore = "delegatorBondMore(bytes32,uint256)",
	ScheduleDelegatorBondLess = "scheduleDelegatorBondLess(bytes32,uint256)",
	ExecuteDelegationRequest = "executeDelegationRequest(bytes32,bytes32)",
	CancelDelegationRequest = "cancelDelegationRequest(bytes32)",
	SetAutoCompound = "setAutoCompound(bytes32,uint8)",
}

impl<R> Precompile for ParachainStakingPrecompile<R>
where
	R: pallet_parachain_staking::Config + pallet_evm::Config,
	R::RuntimeCall: From<pallet_parachain_staking::Call<R>>
		+ Dispatchable<PostInfo = PostDispatchInfo>
		+ GetDispatchInfo,
	<R::RuntimeCall as Dispatchable>::RuntimeOrigin: From<Option<R::AccountId>>,
	BalanceOf<R>: EvmData,
	R::AccountId: From<[u8; 32]>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> EvmResult<PrecompileOutput> {
		log::trace!(target: "parachain-staking-precompile", "Execute input = {:?}", handle.input());

		let selector = handle.read_selector()?;

		handle.check_function_modifier(match selector {
			Action::Delegate |
			Action::DelegateWithAutoCompound |
			Action::ScheduleRevokeDelegation |
			Action::DelegatorBondMore |
			Action::ScheduleDelegatorBondLess |
			Action::ExecuteDelegationRequest |
			Action::CancelDelegationRequest |
			Action::SetAutoCompound => FunctionModifier::NonPayable,
			_ => FunctionModifier::View,
		})?;

		match selector {
			// read storage
			Action::DelegationRequestIsPending => Self::delegation_request_is_pending(handle),
			// Dispatchables
			Action::Delegate => Self::delegate(handle),
			Action::DelegateWithAutoCompound => Self::delegate_with_auto_compound(handle),
			Action::ScheduleRevokeDelegation => Self::schedule_revoke_delegation(handle),
			Action::DelegatorBondMore => Self::delegator_bond_more(handle),
			Action::ScheduleDelegatorBondLess => Self::schedule_delegator_bond_less(handle),
			Action::ExecuteDelegationRequest => Self::execute_delegation_request(handle),
			Action::CancelDelegationRequest => Self::cancel_delegation_request(handle),
			Action::SetAutoCompound => Self::set_auto_compound(handle),
		}
	}
}
