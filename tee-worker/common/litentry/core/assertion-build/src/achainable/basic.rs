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

use crate::{
	achainable::{request_achainable, request_uniswap_v2_or_v3_user},
	*,
};
use lc_credentials::{
	achainable::{bab_holder::UpdateBABHolder, uniswap_user::UpdateUniswapUser},
	IssuerRuntimeVersion,
};
use lc_data_providers::{achainable_names::AchainableNameBasic, DataProviderConfig};

pub fn build_basic(
	req: &AssertionBuildRequest,
	param: AchainableBasic,
	data_provider_config: &DataProviderConfig,
) -> Result<Credential> {
	debug!("Assertion Achainable building Basic");

	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let achainable_param = AchainableParams::Basic(param.clone());

	let runtime_version = IssuerRuntimeVersion {
		parachain: req.parachain_runtime_version,
		sidechain: req.sidechain_runtime_version,
	};

	let mut credential = Credential::new(&req.who, &req.shard, &runtime_version).map_err(|e| {
		Error::RequestVCFailed(
			Assertion::Achainable(achainable_param.clone()),
			e.into_error_detail(),
		)
	})?;

	let basic_name = AchainableNameBasic::from(param.name).map_err(|e| {
		Error::RequestVCFailed(
			Assertion::Achainable(achainable_param.clone()),
			e.into_error_detail(),
		)
	})?;
	match basic_name {
		AchainableNameBasic::UniswapV23User => {
			let (v2_user, v3_user) =
				request_uniswap_v2_or_v3_user(addresses, achainable_param, data_provider_config)?;
			credential.update_uniswap_user(v2_user, v3_user);
		},
		AchainableNameBasic::BABHolder => {
			let is_bab_holder =
				request_achainable(addresses, achainable_param, data_provider_config)?;
			credential.update_bab_holder(is_bab_holder);
		},
	}

	Ok(credential)
}
