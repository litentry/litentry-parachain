// Copyright 2020-2022 Litentry Technologies GmbH.
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

#![feature(trait_alias)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use thiserror_sgx as thiserror;
	pub use url_sgx as url;
}

use itp_types::AccountId;
pub mod error;
pub mod stf_task_sender;
pub use error::Result;

use sp_runtime::{traits::ConstU32, BoundedVec};
use sp_std::vec::Vec;

use codec::{Decode, Encode};
use litentry_primitives::{
	Assertion, ChallengeCode, Identity, Web2ValidationData, Web3ValidationData,
};

/// Here a few Request structs are defined for asynchronously stf-tasks handling.
/// A `callback` exists for some request types to submit a callback TrustedCall to top pool.
/// We use the encoded version just to avoid cyclic dependency, otherwise we have
/// ita-stf -> lc-stf-task-sender -> ita-stf
///
/// In this way we make sure the state is processed "chronologically" by the StfExecutor.
/// We can't write any state in this state, otherwise we can be trapped into a situation
/// where the state doesn't match the apriori state that is recorded before executing any
/// trusted calls in block production (InvalidAprioriHash error).
///
/// Reading state is not a problem. However, we prefer to read the required storage before
/// sending the stf-task and pass it as parameters in `Request`, e.g. `challenge_code` below.
/// The reason is we actually want the "snapshot" state when the preflight TrustedCall gets
/// executed instead of the "live" state.
///
/// The callback TrustedCall will be appended to the end of top pool but we don't see a
/// problem. In case some preflight TrustedCall and callback TrustedCall are going to change
/// the same storage, we should implement them carefully and always treat it as if both
/// TrustedCalls can get executed in any order.
///
/// For more information, please see:
/// https://github.com/litentry/tee-worker/issues/110
/// https://www.notion.so/web3builders/Sidechain-block-importer-and-block-production-28292233b4c74f4ab8110a0014f8d9df

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct Web2IdentityVerificationRequest {
	pub encoded_shard: Vec<u8>,
	pub who: AccountId,
	pub identity: Identity,
	pub challenge_code: ChallengeCode,
	pub validation_data: Web2ValidationData,
	pub bn: litentry_primitives::ParentchainBlockNumber, //Parentchain BlockNumber
	pub encoded_callback: Vec<u8>,
}

/// TODO: adapt Web3 struct fields later
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct Web3IdentityVerificationRequest {
	pub encoded_shard: Vec<u8>,
	pub who: AccountId,
	pub identity: Identity,
	pub challenge_code: ChallengeCode,
	pub validation_data: Web3ValidationData,
	pub bn: litentry_primitives::ParentchainBlockNumber, //Parentchain BlockNumber
	pub encoded_callback: Vec<u8>,
}

pub type MaxIdentityLength = ConstU32<64>;
/// TODO: adapt struct fields later
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct AssertionBuildRequest {
	pub encoded_shard: Vec<u8>,
	pub who: AccountId,
	pub assertion: Assertion,
	pub vec_identity: BoundedVec<Identity, MaxIdentityLength>,
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct SetUserShieldingKeyRequest {
	pub encoded_shard: Vec<u8>,
	pub who: AccountId,
	pub encoded_callback: Vec<u8>,
}

pub enum RequestType {
	Web2IdentityVerification(Web2IdentityVerificationRequest),
	Web3IdentityVerification(Web3IdentityVerificationRequest),
	AssertionVerification(AssertionBuildRequest),
	// set the user shielding key async - just to showcase how to
	// async process the request in stf-task-receiver
	// In real scenario it should be done synchronously
	SetUserShieldingKey(SetUserShieldingKeyRequest),
}

impl From<Web2IdentityVerificationRequest> for RequestType {
	fn from(r: Web2IdentityVerificationRequest) -> Self {
		RequestType::Web2IdentityVerification(r)
	}
}

impl From<Web3IdentityVerificationRequest> for RequestType {
	fn from(r: Web3IdentityVerificationRequest) -> Self {
		RequestType::Web3IdentityVerification(r)
	}
}

impl From<AssertionBuildRequest> for RequestType {
	fn from(r: AssertionBuildRequest) -> Self {
		RequestType::AssertionVerification(r)
	}
}

impl From<SetUserShieldingKeyRequest> for RequestType {
	fn from(r: SetUserShieldingKeyRequest) -> Self {
		RequestType::SetUserShieldingKey(r)
	}
}
