// Copyright 2020-2024 Trust Computing GmbH.
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
// along with Litentry. If not, see <https://www.gnu.org/licenses/>.

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use core::result::Result;

use crate::*;

mod common;

pub fn has_nft(
	nft_type: Web3NftType,
	addresses: Vec<(Web3Network, String)>,
	data_provider_config: &DataProviderConfig,
) -> Result<bool, Error> {
	if nft_type == Web3NftType::Club3Sbt {
		common::has_nft_1155(addresses, nft_type, data_provider_config)
	} else {
		common::has_nft_721(addresses, nft_type, data_provider_config)
	}
}
