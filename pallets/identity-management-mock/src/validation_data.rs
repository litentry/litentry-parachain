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

#[cfg(feature = "std")]
extern crate std;

#[cfg(not(feature = "std"))]
use sp_std::prelude::*;

use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Encode, Decode, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Web2ValidationData {
	pub link: Vec<u8>,
}

#[derive(Clone, Encode, Decode, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Web3ValidationData {
	pub message: Vec<u8>,
	// should be eventually decoded to sp_runtime::MultiSignature, I can't use this type
	// directly though, as it can't be deserialized under `no_std`
	pub signature: Vec<u8>,
}

#[derive(Clone, Encode, Decode, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidationData {
	pub web2_validation_data: Option<Web2ValidationData>,
	pub web3_validation_data: Option<Web3ValidationData>,
}
