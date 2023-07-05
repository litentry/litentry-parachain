// Copyright 2020-2023 Litentry Technologies GmbH.
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

mod aes;
mod ethereum_signature;
mod identity;
mod validation_data;

pub use aes::*;
pub use ethereum_signature::*;
pub use identity::*;
pub use validation_data::*;

pub use parentchain_primitives::{
	get_all_web3networks, AccountId as ParentchainAccountId, AesOutput, Assertion,
	Balance as ParentchainBalance, BlockNumber as ParentchainBlockNumber, BoundedWeb3Network,
	ErrorDetail, ErrorString, Hash as ParentchainHash, Header as ParentchainHeader, IMPError,
	Index as ParentchainIndex, IntoErrorDetail, ParameterString, SchemaContentString,
	SchemaIdString, Signature as ParentchainSignature, UserShieldingKeyNonceType,
	UserShieldingKeyType, VCMPError, Web3Network, ASSERTION_FROM_DATE, MAX_TAG_LEN, MINUTES,
	NONCE_LEN, USER_SHIELDING_KEY_LEN,
};
