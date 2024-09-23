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
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use super::{BnbDomainInfo, BnbDomainInfoInterface};
use crate::*;
use lc_credentials::{
	nodereal::bnb_domain::bnb_digit_domain_club_amount::UpdateDigitDomainClubAmountCredential,
	IssuerRuntimeVersion,
};
use lc_data_providers::DataProviderConfig;
use litentry_primitives::BnbDigitDomainType;

pub fn build(
	req: &AssertionBuildRequest,
	digit_domain_type: BnbDigitDomainType,
	data_provider_config: &DataProviderConfig,
) -> Result<Credential> {
	debug!("building digit_domain credential: {:?}", digit_domain_type);

	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let amount = BnbDomainInfo.get_bnb_digit_domain_club_amount(
		&addresses,
		&digit_domain_type,
		data_provider_config,
	)?;

	let runtime_version = IssuerRuntimeVersion {
		parachain: req.parachain_runtime_version,
		sidechain: req.sidechain_runtime_version,
	};

	match Credential::new(&req.who, &req.shard, &runtime_version) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.update_digit_domain_club_amount(&digit_domain_type, amount);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(
				Assertion::BnbDigitDomainClub(digit_domain_type),
				e.into_error_detail(),
			))
		},
	}
}
