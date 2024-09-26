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

use super::request_achainable;
use crate::*;
use lc_credentials::{litentry_profile::mirror::MirrorInfo, Credential, IssuerRuntimeVersion};
use lc_data_providers::{achainable_names::AchainableNameMirror, DataProviderConfig};
use litentry_primitives::AchainableMirror;

// Request Inputs
// {
//     "name": "Has written over quantity posts on Mirror",
//     "address": "0xCdd39B6D1cC4D0a7243b389Ed9356E23Df6240eb",
//     "params": {
//         "chain": "ethereum",
//         "postQuantity": "0"
//     },
//     "includeMetadata": true
// }

// {
//     "name": "Is a publication on Mirror",
//     "address": "0xCdd39B6D1cC4D0a7243b389Ed9356E23Df6240eb",
//     "params": {
//         "chain": "ethereum"
//     },
//     "includeMetadata": true
// }

pub fn build_on_mirror(
	req: &AssertionBuildRequest,
	param: AchainableMirror,
	data_provider_config: &DataProviderConfig,
) -> Result<Credential> {
	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let achainable_param = AchainableParams::Mirror(param);
	let mtype = AchainableNameMirror::from(achainable_param.name()).map_err(|e| {
		Error::RequestVCFailed(
			Assertion::Achainable(achainable_param.clone()),
			e.into_error_detail(),
		)
	})?;
	let value = request_achainable(addresses, achainable_param.clone(), data_provider_config)?;

	let runtime_version = IssuerRuntimeVersion {
		parachain: req.parachain_runtime_version,
		sidechain: req.sidechain_runtime_version,
	};

	match Credential::new(&req.who, &req.shard, &runtime_version) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.update_mirror(mtype, value);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(
				Assertion::Achainable(achainable_param),
				e.into_error_detail(),
			))
		},
	}
}
