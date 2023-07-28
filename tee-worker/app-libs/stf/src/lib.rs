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

/////////////////////////////////////////////////////////////////////////////
#![feature(structural_match)]
#![feature(rustc_attrs)]
#![feature(core_intrinsics)]
#![feature(derive_eq)]
#![cfg_attr(all(not(target_env = "sgx"), not(feature = "std")), no_std)]
#![cfg_attr(target_env = "sgx", feature(rustc_private))]

extern crate core;
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

#[cfg(feature = "sgx")]
pub use ita_sgx_runtime::{Balance, BlockNumber, Index};
#[cfg(feature = "std")]
pub use litentry_primitives::{
	ParentchainBalance as Balance, ParentchainBlockNumber as BlockNumber, ParentchainIndex as Index,
};

use codec::{Decode, Encode};
use derive_more::Display;
pub use getter::*;
use ita_sgx_runtime::{pallet_imt::UserShieldingKeys, IdentityManagement, Runtime, System};
use itp_node_api_metadata::Error as MetadataError;
use itp_node_api_metadata_provider::Error as MetadataProviderError;
use itp_stf_primitives::types::AccountId;
use litentry_primitives::{Assertion, ErrorDetail, ErrorString, IMPError, VCMPError};
use std::{format, string::String};
pub use stf_sgx_primitives::{types::*, Stf};
pub use trusted_call::*;

#[cfg(feature = "evm")]
pub mod evm_helpers;
pub mod getter;
pub mod hash;
pub mod helpers;
pub mod stf_sgx;
pub mod stf_sgx_primitives;
#[cfg(all(feature = "test", feature = "sgx"))]
pub mod stf_sgx_tests;
#[cfg(all(feature = "test", feature = "sgx"))]
pub mod test_genesis;
pub mod trusted_call;
pub mod trusted_call_litentry;
pub mod trusted_call_rpc_response;

pub(crate) const ENCLAVE_ACCOUNT_KEY: &str = "Enclave_Account_Key";

pub type StfResult<T> = Result<T, StfError>;

#[derive(Debug, Display, PartialEq, Eq, Encode, Decode, Clone)]
pub enum StfError {
	#[display(fmt = "Insufficient privileges {:?}, are you sure you are root?", _0)]
	MissingPrivileges(Identity),
	#[display(fmt = "Valid enclave signer account is required")]
	RequireEnclaveSignerAccount,
	#[display(fmt = "Error dispatching runtime call. {:?}", _0)]
	Dispatch(String),
	#[display(fmt = "Not enough funds to perform operation")]
	MissingFunds,
	#[display(fmt = "Invalid Nonce {:?} != {:?}", _0, _1)]
	InvalidNonce(Index, Index),
	StorageHashMismatch,
	InvalidStorageDiff,
	InvalidMetadata,
	// litentry
	#[display(fmt = "SetUserShieldingKeyFailed: {:?}", _0)]
	SetUserShieldingKeyFailed(ErrorDetail),
	#[display(fmt = "LinkIdentityFailed: {:?}", _0)]
	LinkIdentityFailed(ErrorDetail),
	#[display(fmt = "DeactivateIdentityFailed: {:?}", _0)]
	DeactivateIdentityFailed(ErrorDetail),
	#[display(fmt = "ActivateIdentityFailed: {:?}", _0)]
	ActivateIdentityFailed(ErrorDetail),
	#[display(fmt = "RequestVCFailed: {:?} {:?}", _0, _1)]
	RequestVCFailed(Assertion, ErrorDetail),
	SetScheduledMrEnclaveFailed,
	#[display(fmt = "SetIdentityNetworksFailed: {:?}", _0)]
	SetIdentityNetworksFailed(ErrorDetail),
	InvalidAccount,
	UnclassifiedError,
}

impl From<MetadataError> for StfError {
	fn from(_e: MetadataError) -> Self {
		StfError::InvalidMetadata
	}
}

impl From<MetadataProviderError> for StfError {
	fn from(_e: MetadataProviderError) -> Self {
		StfError::InvalidMetadata
	}
}

impl From<IMPError> for StfError {
	fn from(e: IMPError) -> Self {
		match e {
			IMPError::SetUserShieldingKeyFailed(d) => StfError::SetIdentityNetworksFailed(d),
			IMPError::LinkIdentityFailed(d) => StfError::LinkIdentityFailed(d),
			IMPError::DeactivateIdentityFailed(d) => StfError::DeactivateIdentityFailed(d),
			IMPError::ActivateIdentityFailed(d) => StfError::ActivateIdentityFailed(d),
			_ => StfError::UnclassifiedError,
		}
	}
}

impl From<VCMPError> for StfError {
	fn from(e: VCMPError) -> Self {
		match e {
			VCMPError::RequestVCFailed(a, d) => StfError::RequestVCFailed(a, d),
			_ => StfError::UnclassifiedError,
		}
	}
}

impl StfError {
	// Convert StfError to IMPError that would be sent to parentchain
	pub fn to_imp_error(&self) -> IMPError {
		match self {
			StfError::SetUserShieldingKeyFailed(d) =>
				IMPError::SetUserShieldingKeyFailed(d.clone()),
			StfError::LinkIdentityFailed(d) => IMPError::LinkIdentityFailed(d.clone()),
			StfError::DeactivateIdentityFailed(d) => IMPError::DeactivateIdentityFailed(d.clone()),
			StfError::ActivateIdentityFailed(d) => IMPError::ActivateIdentityFailed(d.clone()),
			_ => IMPError::UnclassifiedError(ErrorDetail::StfError(ErrorString::truncate_from(
				format!("{:?}", self).as_bytes().to_vec(),
			))),
		}
	}
	// Convert StfError to VCMPError that would be sent to parentchain
	pub fn to_vcmp_error(&self) -> VCMPError {
		match self {
			StfError::RequestVCFailed(a, d) => VCMPError::RequestVCFailed(a.clone(), d.clone()),
			_ => VCMPError::UnclassifiedError(ErrorDetail::StfError(ErrorString::truncate_from(
				format!("{:?}", self).as_bytes().to_vec(),
			))),
		}
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TrustedOperation {
	indirect_call(TrustedCallSigned),
	direct_call(TrustedCallSigned),
	get(Getter),
}

impl From<TrustedCallSigned> for TrustedOperation {
	fn from(item: TrustedCallSigned) -> Self {
		TrustedOperation::indirect_call(item)
	}
}

impl From<Getter> for TrustedOperation {
	fn from(item: Getter) -> Self {
		TrustedOperation::get(item)
	}
}

impl From<TrustedGetterSigned> for TrustedOperation {
	fn from(item: TrustedGetterSigned) -> Self {
		TrustedOperation::get(item.into())
	}
}

impl From<PublicGetter> for TrustedOperation {
	fn from(item: PublicGetter) -> Self {
		TrustedOperation::get(item.into())
	}
}

impl TrustedOperation {
	pub fn to_call(&self) -> Option<&TrustedCallSigned> {
		match self {
			TrustedOperation::direct_call(c) => Some(c),
			TrustedOperation::indirect_call(c) => Some(c),
			_ => None,
		}
	}

	pub fn signed_caller_account(&self) -> Option<AccountId> {
		match self {
			TrustedOperation::direct_call(c) => c.call.sender_identity().to_account_id(),
			TrustedOperation::indirect_call(c) => c.call.sender_identity().to_account_id(),
			_ => None,
		}
	}

	pub fn req_hash(&self) -> Option<&H256> {
		match self {
			TrustedOperation::direct_call(c) => c.call.req_hash(),
			TrustedOperation::indirect_call(c) => c.call.req_hash(),
			//todo: getters should also contain req_hash ?
			TrustedOperation::get(_) => None,
		}
	}
}
