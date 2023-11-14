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

pub mod bnb_digit_domain_club_amount;
pub mod bnb_domain_holding_amount;

use crate::*;
use lc_data_providers::{
	nodereal::{BnbDomainApiList, DomainInfo, NoderealClient},
	DataProviderConfigReader, ReadDataProviderConfig,
};
use litentry_primitives::BnbDigitDomainType;
use serde::{Deserialize, Serialize};
use std::string::ToString;

pub struct BnbDomainInfo;
impl BnbDomainInfo {
	fn get_bnb_domain_data_by_owners(
		&self,
		owners: &[String],
	) -> core::result::Result<serde_json::Value, ErrorDetail> {
		let config = DataProviderConfigReader::read()?;
		let mut client = NoderealClient::new(&config);
		client.by_owners(owners).map_err(|e| e.into_error_detail())
	}
}

pub trait BnbDomainInfoInterface {
	fn get_bnb_domain_holding_amount(&self, addresses: &[String]) -> Result<usize>;
	fn get_bnb_digit_domain_club_amount(
		&self,
		owners: &[String],
		digit_domain_type: &BnbDigitDomainType,
	) -> Result<usize>;
}

impl BnbDomainInfoInterface for BnbDomainInfo {
	fn get_bnb_domain_holding_amount(&self, owners: &[String]) -> Result<usize> {
		let response = self
			.get_bnb_domain_data_by_owners(owners)
			.map_err(|e| Error::RequestVCFailed(Assertion::BnbDomainHolding, e))?;

		let owned_domains: Domains = Domains::from_value(&response)
			.map_err(|e| Error::RequestVCFailed(Assertion::BnbDomainHolding, e))?;

		Ok(owned_domains.non_expired_domains()?.len())
	}

	fn get_bnb_digit_domain_club_amount(
		&self,
		owners: &[String],
		digit_domain_type: &BnbDigitDomainType,
	) -> Result<usize> {
		let response = self.get_bnb_domain_data_by_owners(owners).map_err(|e| {
			Error::RequestVCFailed(Assertion::BnbDigitDomainClub(digit_domain_type.clone()), e)
		})?;

		let owned_domains: Domains = Domains::from_value(&response)
			.map_err(|e| Error::RequestVCFailed(Assertion::BnbDomainHolding, e))?;

		Ok(owned_domains.digit_club_domains(digit_domain_type)?.len())
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Domain {
	owner: String,
	domain_infos: Vec<DomainInfo>,
}

impl Domain {
	pub fn new(owner: String) -> Self {
		Self { owner, domain_infos: vec![] }
	}

	pub fn add(&mut self, domain_info: DomainInfo) {
		self.domain_infos.push(domain_info);
	}

	pub fn size(&self) -> usize {
		self.domain_infos.len()
	}
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Domains {
	domains: Vec<Domain>,
}

impl Domains {
	pub fn from_value(value: &serde_json::Value) -> core::result::Result<Domains, ErrorDetail> {
		let mut domains: Vec<Domain> = vec![];

		if let Some(obj_map) = value.as_object() {
			for (key, val) in obj_map {
				let mut owned = Domain::new(key.to_string());

				if let Some(values) = val.as_array() {
					for info in values {
						let domain_info: DomainInfo = DomainInfo::from_value(info)?;
						owned.add(domain_info);
					}
				}

				domains.push(owned);
			}
		}

		Ok(Domains { domains })
	}

	pub fn size(&self) -> usize {
		self.domains.len()
	}

	pub fn non_expired_domains(&self) -> Result<Vec<&DomainInfo>> {
		let mut ts: Vec<&DomainInfo> = vec![];

		for domain in self.domains.iter() {
			for info in domain.domain_infos.iter() {
				// Filter out expired domain sizing
				let expired = info.is_expired().map_err(|e| {
					Error::RequestVCFailed(Assertion::BnbDomainHolding, e.into_error_detail())
				})?;
				if !expired {
					ts.push(info);
				}
			}
		}

		Ok(ts)
	}

	pub fn digit_club_domains(
		&self,
		digit_domain_type: &BnbDigitDomainType,
	) -> Result<Vec<&DomainInfo>> {
		let domains = self.non_expired_domains()?;

		let mut ts: Vec<&DomainInfo> = vec![];
		for domain in domains.iter() {
			if digit_domain_type.is_member(&domain.name) {
				ts.push(domain);
			}
		}

		Ok(ts)
	}
}

#[cfg(test)]
mod tests {
	use super::Domains;

	#[test]
	fn domain_from_value_works() {
		let value = r#"
		{
			"0xr4b0bf28adfcee93c5069982a895785c9231c1fe1": [
				{
					"nodeHash": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"bind": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"name": "1",
					"expires": "2024-08-24T00:36:44Z"
				},
				{
					"nodeHash": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"bind": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"name": "2",
					"expires": "2032-08-24T00:15:56Z"
				}
			],
			"0xr4b0bf28adfcee93c5069982a895785c9231c1fe2": [
				{
					"nodeHash": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"bind": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"name": "3",
					"expires": "2023-08-24T20:36:26Z"
				},
				{
					"nodeHash": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"bind": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"name": "4",
					"expires": "2023-08-24T20:35:59Z"
				},
				{
					"nodeHash": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"bind": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"name": "win",
					"expires": "2023-08-24T20:38:29Z"
				}
			],
			"0xr4b0bf28adfcee93c5069982a895785c9231c1fe3": [
				{
					"nodeHash": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"bind": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"name": "5",
					"expires": "2024-08-24T06:33:32Z"
				},
				{
					"nodeHash": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"bind": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"name": "6",
					"expires": "2024-08-24T07:57:41Z"
				},
				{
					"nodeHash": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"bind": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"name": "7",
					"expires": "2023-09-16T19:36:14Z"
				}
			],
			"0xr4b0bf28adfcee93c5069982a895785c9231c1fe4": [
				{
					"nodeHash": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"bind": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"name": "8",
					"expires": "2028-09-18T13:35:38Z"
				}
			],
			"0xr4b0bf28adfcee93c5069982a895785c9231c1fe5": [
				{
					"nodeHash": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"bind": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"name": "9",
					"expires": "2023-09-03T08:35:40Z"
				}
			],
			"0xr4b0bf28adfcee93c5069982a895785c9231c1fe6": [
				{
					"nodeHash": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"bind": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"name": "333r",
					"expires": "2024-10-30T18:40:51Z"
				},
				{
					"nodeHash": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"bind": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"name": "tttx",
					"expires": "2024-03-19T18:16:59Z"
				}
			]
		}
		"#;
		let value: serde_json::Value = serde_json::from_str(value).unwrap();
		let domains = Domains::from_value(&value).unwrap();
		assert_eq!(domains.size(), 6);
	}
}
