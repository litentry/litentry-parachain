/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/
use crate::{
	error::{Error, Result},
	initialization::global_components::{
		EnclaveExtrinsicsFactory, EnclaveNodeMetadataRepository, EnclaveStfExecutor,
		EnclaveValidatorAccessor, IntegriteeParentchainTriggeredBlockImportDispatcher,
		TargetAParentchainTriggeredBlockImportDispatcher,
		TargetBParentchainTriggeredBlockImportDispatcher,
		GLOBAL_INTEGRITEE_PARACHAIN_HANDLER_COMPONENT,
		GLOBAL_INTEGRITEE_SOLOCHAIN_HANDLER_COMPONENT, GLOBAL_TARGET_A_PARACHAIN_HANDLER_COMPONENT,
		GLOBAL_TARGET_A_SOLOCHAIN_HANDLER_COMPONENT, GLOBAL_TARGET_B_PARACHAIN_HANDLER_COMPONENT,
		GLOBAL_TARGET_B_SOLOCHAIN_HANDLER_COMPONENT,
	},
};
use codec::{Decode, Input};
use itc_parentchain_block_import_dispatcher::BlockImportDispatcher;
use itp_component_container::ComponentGetter;
use std::{result::Result as StdResult, slice, sync::Arc};

/// Helper trait to transform the sgx-ffi pointers to any type that implements
/// `parity-scale-codec::Decode`
pub unsafe trait DecodeRaw {
	/// the type to decode into
	type Decoded: Decode;

	unsafe fn decode_raw<'a, T>(
		data: *const T,
		len: usize,
	) -> StdResult<Self::Decoded, codec::Error>
	where
		T: 'a,
		&'a [T]: Input;
}

unsafe impl<D: Decode> DecodeRaw for D {
	type Decoded = D;

	unsafe fn decode_raw<'a, T>(
		data: *const T,
		len: usize,
	) -> StdResult<Self::Decoded, codec::Error>
	where
		T: 'a,
		&'a [T]: Input,
	{
		let mut s = slice::from_raw_parts(data, len);

		Decode::decode(&mut s)
	}
}

// FIXME: When solving #1080, these helper functions should be obsolete, because no dynamic allocation
// is necessary anymore.
pub(crate) fn get_triggered_dispatcher_from_integritee_solo_or_parachain(
) -> Result<Arc<IntegriteeParentchainTriggeredBlockImportDispatcher>> {
	let dispatcher =
		if let Ok(solochain_handler) = GLOBAL_INTEGRITEE_SOLOCHAIN_HANDLER_COMPONENT.get() {
			get_triggered_dispatcher(solochain_handler.import_dispatcher.clone())?
		} else if let Ok(parachain_handler) = GLOBAL_INTEGRITEE_PARACHAIN_HANDLER_COMPONENT.get() {
			get_triggered_dispatcher(parachain_handler.import_dispatcher.clone())?
		} else {
			return Err(Error::NoLitentryParentchainAssigned)
		};
	Ok(dispatcher)
}

pub(crate) fn get_triggered_dispatcher_from_target_a_solo_or_parachain(
) -> Result<Arc<TargetAParentchainTriggeredBlockImportDispatcher>> {
	let dispatcher =
		if let Ok(solochain_handler) = GLOBAL_TARGET_A_SOLOCHAIN_HANDLER_COMPONENT.get() {
			get_triggered_dispatcher(solochain_handler.import_dispatcher.clone())?
		} else if let Ok(parachain_handler) = GLOBAL_TARGET_A_PARACHAIN_HANDLER_COMPONENT.get() {
			get_triggered_dispatcher(parachain_handler.import_dispatcher.clone())?
		} else {
			return Err(Error::NoTargetAParentchainAssigned)
		};
	Ok(dispatcher)
}

pub(crate) fn get_triggered_dispatcher_from_target_b_solo_or_parachain(
) -> Result<Arc<TargetBParentchainTriggeredBlockImportDispatcher>> {
	let dispatcher =
		if let Ok(solochain_handler) = GLOBAL_TARGET_B_SOLOCHAIN_HANDLER_COMPONENT.get() {
			get_triggered_dispatcher(solochain_handler.import_dispatcher.clone())?
		} else if let Ok(parachain_handler) = GLOBAL_TARGET_B_PARACHAIN_HANDLER_COMPONENT.get() {
			get_triggered_dispatcher(parachain_handler.import_dispatcher.clone())?
		} else {
			return Err(Error::NoTargetBParentchainAssigned)
		};
	Ok(dispatcher)
}

pub(crate) fn get_triggered_dispatcher<TriggeredDispatcher, T>(
	dispatcher: Arc<BlockImportDispatcher<TriggeredDispatcher, T>>,
) -> Result<Arc<TriggeredDispatcher>> {
	let triggered_dispatcher = dispatcher
		.triggered_dispatcher()
		.ok_or(Error::ExpectedTriggeredImportDispatcher)?;
	Ok(triggered_dispatcher)
}

pub(crate) fn get_validator_accessor_from_integritee_solo_or_parachain(
) -> Result<Arc<EnclaveValidatorAccessor>> {
	let validator_accessor =
		if let Ok(solochain_handler) = GLOBAL_INTEGRITEE_SOLOCHAIN_HANDLER_COMPONENT.get() {
			solochain_handler.validator_accessor.clone()
		} else if let Ok(parachain_handler) = GLOBAL_INTEGRITEE_PARACHAIN_HANDLER_COMPONENT.get() {
			parachain_handler.validator_accessor.clone()
		} else {
			return Err(Error::NoLitentryParentchainAssigned)
		};
	Ok(validator_accessor)
}

pub(crate) fn get_validator_accessor_from_target_a_solo_or_parachain(
) -> Result<Arc<EnclaveValidatorAccessor>> {
	let validator_accessor =
		if let Ok(solochain_handler) = GLOBAL_TARGET_A_SOLOCHAIN_HANDLER_COMPONENT.get() {
			solochain_handler.validator_accessor.clone()
		} else if let Ok(parachain_handler) = GLOBAL_TARGET_A_PARACHAIN_HANDLER_COMPONENT.get() {
			parachain_handler.validator_accessor.clone()
		} else {
			return Err(Error::NoTargetAParentchainAssigned)
		};
	Ok(validator_accessor)
}

pub(crate) fn get_validator_accessor_from_target_b_solo_or_parachain(
) -> Result<Arc<EnclaveValidatorAccessor>> {
	let validator_accessor =
		if let Ok(solochain_handler) = GLOBAL_TARGET_B_SOLOCHAIN_HANDLER_COMPONENT.get() {
			solochain_handler.validator_accessor.clone()
		} else if let Ok(parachain_handler) = GLOBAL_TARGET_B_PARACHAIN_HANDLER_COMPONENT.get() {
			parachain_handler.validator_accessor.clone()
		} else {
			return Err(Error::NoTargetBParentchainAssigned)
		};
	Ok(validator_accessor)
}

pub(crate) fn get_node_metadata_repository_from_integritee_solo_or_parachain(
) -> Result<Arc<EnclaveNodeMetadataRepository>> {
	let metadata_repository =
		if let Ok(solochain_handler) = GLOBAL_INTEGRITEE_SOLOCHAIN_HANDLER_COMPONENT.get() {
			solochain_handler.node_metadata_repository.clone()
		} else if let Ok(parachain_handler) = GLOBAL_INTEGRITEE_PARACHAIN_HANDLER_COMPONENT.get() {
			parachain_handler.node_metadata_repository.clone()
		} else {
			return Err(Error::NoLitentryParentchainAssigned)
		};
	Ok(metadata_repository)
}

pub(crate) fn get_node_metadata_repository_from_target_a_solo_or_parachain(
) -> Result<Arc<EnclaveNodeMetadataRepository>> {
	let metadata_repository =
		if let Ok(solochain_handler) = GLOBAL_TARGET_A_SOLOCHAIN_HANDLER_COMPONENT.get() {
			solochain_handler.node_metadata_repository.clone()
		} else if let Ok(parachain_handler) = GLOBAL_TARGET_A_PARACHAIN_HANDLER_COMPONENT.get() {
			parachain_handler.node_metadata_repository.clone()
		} else {
			return Err(Error::NoTargetAParentchainAssigned)
		};
	Ok(metadata_repository)
}

pub(crate) fn get_node_metadata_repository_from_target_b_solo_or_parachain(
) -> Result<Arc<EnclaveNodeMetadataRepository>> {
	let metadata_repository =
		if let Ok(solochain_handler) = GLOBAL_TARGET_B_SOLOCHAIN_HANDLER_COMPONENT.get() {
			solochain_handler.node_metadata_repository.clone()
		} else if let Ok(parachain_handler) = GLOBAL_TARGET_B_PARACHAIN_HANDLER_COMPONENT.get() {
			parachain_handler.node_metadata_repository.clone()
		} else {
			return Err(Error::NoTargetBParentchainAssigned)
		};
	Ok(metadata_repository)
}

pub(crate) fn get_extrinsic_factory_from_integritee_solo_or_parachain(
) -> Result<Arc<EnclaveExtrinsicsFactory>> {
	let extrinsics_factory =
		if let Ok(solochain_handler) = GLOBAL_INTEGRITEE_SOLOCHAIN_HANDLER_COMPONENT.get() {
			solochain_handler.extrinsics_factory.clone()
		} else if let Ok(parachain_handler) = GLOBAL_INTEGRITEE_PARACHAIN_HANDLER_COMPONENT.get() {
			parachain_handler.extrinsics_factory.clone()
		} else {
			return Err(Error::NoLitentryParentchainAssigned)
		};
	Ok(extrinsics_factory)
}

pub(crate) fn get_extrinsic_factory_from_target_a_solo_or_parachain(
) -> Result<Arc<EnclaveExtrinsicsFactory>> {
	let extrinsics_factory =
		if let Ok(solochain_handler) = GLOBAL_TARGET_A_SOLOCHAIN_HANDLER_COMPONENT.get() {
			solochain_handler.extrinsics_factory.clone()
		} else if let Ok(parachain_handler) = GLOBAL_TARGET_A_PARACHAIN_HANDLER_COMPONENT.get() {
			parachain_handler.extrinsics_factory.clone()
		} else {
			return Err(Error::NoTargetAParentchainAssigned)
		};
	Ok(extrinsics_factory)
}

pub(crate) fn get_extrinsic_factory_from_target_b_solo_or_parachain(
) -> Result<Arc<EnclaveExtrinsicsFactory>> {
	let extrinsics_factory =
		if let Ok(solochain_handler) = GLOBAL_TARGET_B_SOLOCHAIN_HANDLER_COMPONENT.get() {
			solochain_handler.extrinsics_factory.clone()
		} else if let Ok(parachain_handler) = GLOBAL_TARGET_B_PARACHAIN_HANDLER_COMPONENT.get() {
			parachain_handler.extrinsics_factory.clone()
		} else {
			return Err(Error::NoTargetBParentchainAssigned)
		};
	Ok(extrinsics_factory)
}

pub(crate) fn get_stf_executor_from_integritee_solo_or_parachain() -> Result<Arc<EnclaveStfExecutor>>
{
	let stf_executor =
		if let Ok(solochain_handler) = GLOBAL_INTEGRITEE_SOLOCHAIN_HANDLER_COMPONENT.get() {
			solochain_handler.stf_executor.clone()
		} else if let Ok(parachain_handler) = GLOBAL_INTEGRITEE_PARACHAIN_HANDLER_COMPONENT.get() {
			parachain_handler.stf_executor.clone()
		} else {
			return Err(Error::NoLitentryParentchainAssigned)
		};
	Ok(stf_executor)
}
