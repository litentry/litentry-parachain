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
use crate::types::{AccountId, Nonce};
use alloc::string::String;
use codec::{Decode, Encode};
use derive_more::Display;

pub type StfResult<T> = Result<T, StfError>;

#[derive(Debug, Display, PartialEq, Eq, Encode, Decode, Clone)]
pub enum StfError {
	#[codec(index = 0)]
	InvalidAccount,

	#[codec(index = 20)]
	#[display(fmt = "Insufficient privileges {:?}, are you sure you are root?", _0)]
	MissingPrivileges(AccountId),
	#[codec(index = 11)]
	#[display(fmt = "Valid enclave signer account is required")]
	RequireEnclaveSignerAccount,
	#[codec(index = 22)]
	#[display(fmt = "Error dispatching runtime call. {:?}", _0)]
	Dispatch(String),
	#[codec(index = 23)]
	#[display(fmt = "Not enough funds to perform operation")]
	MissingFunds,
	#[codec(index = 24)]
	#[display(fmt = "Invalid Nonce {:?} != {:?}", _0, _1)]
	InvalidNonce(Nonce, Nonce),
	#[codec(index = 25)]
	StorageHashMismatch,
	#[codec(index = 26)]
	InvalidStorageDiff,
	#[codec(index = 27)]
	InvalidMetadata,
}
