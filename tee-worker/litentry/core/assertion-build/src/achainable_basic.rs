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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::*;
use lc_data_providers::{
	achainable::{AchainableClient, ParamsBasicType},
	vec_to_string,
};

const VC_SUBJECT_DESCRIPTION: &str = "Class of year";
const VC_SUBJECT_TYPE: &str = "Basic Type of Assertion";

pub fn build_basic(req: &AssertionBuildRequest, param: AchainableBasic) -> Result<Credential> {
	debug!("Assertion Achainable build_basic, who: {:?}", account_id_to_string(&req.who));

	let chain = param.chain.clone();
	let chain = vec_to_string(chain.to_vec()).map_err(|_| {
		Error::RequestVCFailed(
			Assertion::Achainable(AchainableParams::Basic(param.clone())),
			ErrorDetail::ParseError,
		)
	})?;

	let name = "todo".to_string();
	let p = ParamsBasicType { name, chain };

	let mut client = AchainableClient::new();
	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	/// TODO:
	/// There are many types of names&networks here

	// let mut flag = false;
	// for address in &addresses {
	// 	if flag {
	// 		break
	// 	}

	// 	match client.class_of_year(address, p.clone()) {
	// 		Ok(b) => flag = b,
	// 		Err(e) => error!("Request class of year failed {:?}", e),
	// 	}
	// }

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_subject_info(VC_SUBJECT_DESCRIPTION, VC_SUBJECT_TYPE);
			// credential_unsigned.add_achainable(flag, date1, date2);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(
				Assertion::Achainable(AchainableParams::Basic(param)),
				e.into_error_detail(),
			))
		},
	}
}
