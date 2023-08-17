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

use crate::{achainable::request_achainable, *};
use lc_data_providers::ConvertParameterString;

const CREATED_OVER_AMOUNT_CONTRACTS: &str = "Created over {amount} contracts";
const BALANCE_OVER_AMOUNT: &str = "Balance over {amount}";

/// NOTE:
/// Build Contract Creator Assertion Params
/// name: "Created over {amount} contracts"
/// chain: "ethereum",
/// amount: "0",
///
/// assertions:[
/// {
///    and:[
///        {
///            src:$is_contract_creator,
///            op:==,
///            dst:true
///        }
///        ]
/// }
///
///
/// Build ETH holder Assertion Params
/// name: Balance over {amount}
/// chain: "ethereum",
/// amount: "0",
///
/// assertions:[
/// {
///    and:[
///        {
///            src:$is_eth_holder,
///            op:==,
///            dst:true
///        }
///        ]
/// }
///
///
/// Build LIT holder Assertion Params
/// name: Balance over {amount}
/// chain: "litentry",
/// amount: "0",
///
/// assertions:[
/// {
///    and:[
///        {
///            src:$is_lit_holder,
///            op:==,
///            dst:true
///        }
///        ]
/// }
///
///
/// Build DOT holder Assertion Params
/// name: Balance over {amount}
/// chain: "polkadot",
/// amount: "0",
///
/// assertions:[
/// {
///    and:[
///        {
///            src:$is_dot_holder,
///            op:==,
///            dst:true
///        }
///        ]
/// }
///
pub fn build_amount(req: &AssertionBuildRequest, param: AchainableAmount) -> Result<Credential> {
	debug!("Assertion Achainable build_amount, who: {:?}", account_id_to_string(&req.who));
	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let achainable_param = AchainableParams::Amount(param.clone());
	let flag = request_achainable(addresses, achainable_param.clone())?;
	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			let (desc, subtype, content) =
				get_assertion_content(&achainable_param.to_string(&param.name)?, &param.chain);
			credential_unsigned.add_subject_info(desc, subtype);
			credential_unsigned.update_content(flag, content);

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

fn get_assertion_content(
	name: &String,
	chain: &Web3Network,
) -> (&'static str, &'static str, &'static str) {
	if name == CREATED_OVER_AMOUNT_CONTRACTS {
		return (
			"You are a deployer of a smart contract on these networks: Ethereum",
			"Contract Creator",
			"$is_contract_creator",
		)
	}

	if name == BALANCE_OVER_AMOUNT {
		let c = if *chain == Web3Network::Ethereum {
			"$is_eth_holder"
		} else if *chain == Web3Network::Litentry {
			"$is_lit_holder"
		} else if *chain == Web3Network::Polkadot {
			"$is_dot_holder"
		} else {
			"Unsupported"
		};

		return ("The number of a particular token you hold > 0", "Token Holder", c)
	}

	("", "", "")
}
