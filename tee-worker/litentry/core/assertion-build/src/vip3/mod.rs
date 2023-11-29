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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

pub mod card;

use crate::*;
use lc_data_providers::{
	vip3::{VIP3Client, VIP3QuerySet},
	DataProviderConfigReader, ReadDataProviderConfig,
};
use litentry_primitives::VIP3MembershipCardLevel;

pub struct VIP3SBTInfo {
	pub client: VIP3Client,
}

impl VIP3SBTInfo {
	pub fn new() -> core::result::Result<VIP3SBTInfo, ErrorDetail> {
		let data_provider_config = DataProviderConfigReader::read()?;
		let client = VIP3Client::new(&data_provider_config);

		Ok(VIP3SBTInfo { client })
	}
}

pub trait VIP3SBTLogicInterface {
	fn has_card_level(
		&mut self,
		addresses: Vec<String>,
		level: &VIP3MembershipCardLevel,
	) -> core::result::Result<bool, ErrorDetail>;
}

impl VIP3SBTLogicInterface for VIP3SBTInfo {
	fn has_card_level(
		&mut self,
		addresses: Vec<String>,
		level: &VIP3MembershipCardLevel,
	) -> core::result::Result<bool, ErrorDetail> {
		debug!("HAS specific card level");

		for address in addresses.iter() {
			let info = self.client.sbt_info(address).map_err(|e| e.into_error_detail())?;
			if info.data.level == level.to_level() {
				return Ok(true)
			}
		}

		Ok(false)
	}
}
