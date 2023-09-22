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

use crate::*;
use lc_credentials::achainable::amount_holding_time::AchainableAmountHoldingTimeUpdate;
use lc_data_providers::{
	achainable::{AchainableClient, AchainableHolder, ParamsBasicTypeWithAmountHolding},
	vec_to_string, DataProviderConfigReader, ReadDataProviderConfig, LIT_TOKEN_ADDRESS,
	WBTC_TOKEN_ADDRESS,
};
use litentry_primitives::AmountHoldingTimeType;
use std::string::ToString;

/// Here's an example of different assertions in this VC:
///
/// Imagine:
/// ALICE holds 100 LITs since 2018-03-02
/// BOB   holds 100 LITs since 2023-03-07
/// CAROL holds 0.1 LITs since 2020-02-22
///
/// min_amount is 1 LIT
///
/// If they all request A4, these are the received assertions:
/// ALICE:
/// [
///    from_date: < 2019-01-01
///    to_date: >= 2023-03-30 (now)
///    value: true
/// ]
///
/// BOB:
/// [
///    from_date: < 2017-01-01
///    to_date: >= 2023-03-30 (now)
///    value: false
/// ]
///
/// CAROL:
/// [
///    from_date: < 2017-01-01
///    to_date: >= 2023-03-30 (now)
///    value: false
/// ]
///
/// So just from the assertion results you can't distinguish between:
/// BOB, who just started to hold recently,
/// and CAROL, who has been holding for 3 years, but with too little amount
///
/// This is because the data provider doesn't provide more information, it only
/// takes the query with from_date and min_ammount, and returns true or false.
///
/// Please note:
/// the operators are mainly for IDHub's parsing, we will **NEVER** have:
/// - `from_date` with >= op, nor
/// - `value` is false but the `from_date` is something other than 2017-01-01.
///  

pub fn build(
	req: &AssertionBuildRequest,
	htype: AmountHoldingTimeType,
	min_balance: ParameterString,
) -> Result<Credential> {
	debug!("Assertion A4 build, who: {:?}", account_id_to_string(&req.who));

	let q_min_balance = pre_build(&htype, &min_balance)?;
	let identities = transpose_identity(&req.identities);
	let (is_hold, optimal_hold_index) = do_build(identities, &htype, &q_min_balance)
		.map_err(|e| emit_error(&htype, &min_balance, e))?;

	generate_vc(req, &htype, &q_min_balance, is_hold, optimal_hold_index)
		.map_err(|e| emit_error(&htype, &min_balance, e))
}

/// Credential Build Workflow
fn pre_build(htype: &AmountHoldingTimeType, min_balance: &ParameterString) -> Result<String> {
	vec_to_string(min_balance.to_vec())
		.map_err(|_| emit_error(htype, min_balance, ErrorDetail::ParseError))
}

// TODO:
// There's an issue for this: https://github.com/litentry/litentry-parachain/issues/1655
//
// There is a problem here, because TDF does not support mixed network types,
// It is need to request TDF 2 (substrate+evm networks) * 14 (ASSERTION_FROM_DATE) * addresses http requests.
// If TDF can handle mixed network type, and even supports from_date array,
// so that ideally, up to one http request can yield results.
fn do_build(
	identities: Vec<(Web3Network, Vec<String>)>,
	htype: &AmountHoldingTimeType,
	q_min_balance: &String,
) -> core::result::Result<(bool, usize), ErrorDetail> {
	let data_provider_config = DataProviderConfigReader::read()?;
	let mut client = AchainableClient::new(&data_provider_config);

	let mut is_hold = false;
	let mut optimal_hold_index = usize::MAX;

	// If both Substrate and Evm networks meet the conditions, take the interval with the longest holding time.
	// Here's an example:
	//
	// ALICE holds 100 LITs since 2018-03-02 on substrate network
	// ALICE holds 100 LITs since 2020-03-02 on evm network
	//
	// min_amount is 1 LIT
	//
	// the result should be
	// Alice:
	// [
	//    from_date: < 2019-01-01
	//    to_date: >= 2023-03-30 (now)
	//    value: true
	// ]
	for (network, addresses) in identities {
		// If found query result is the optimal solution, i.e optimal_hold_index = 0, (2017-01-01)
		// there is no need to query other networks.
		if optimal_hold_index == 0 {
			break
		}

		let token = match_token_address(htype, &network);
		let addresses: Vec<String> = addresses.into_iter().collect();

		for (index, date) in ASSERTION_FROM_DATE.iter().enumerate() {
			for address in &addresses {
				let holding = ParamsBasicTypeWithAmountHolding::new(
					&network,
					q_min_balance.to_string(),
					date.to_string(),
					token.clone(),
				);
				let is_amount_holder = client.is_holder(address, holding).map_err(|e| {
					error!("Assertion HoldingTime request error: {:?}", e);
					e.into_error_detail()
				})?;

				if is_amount_holder {
					if index < optimal_hold_index {
						optimal_hold_index = index;
					}

					is_hold = true;

					break
				}
			}
		}
	}

	// If is_hold is false, then the optimal_hold_index is always 0 (2017-01-01)
	if !is_hold {
		optimal_hold_index = 0;
	}

	Ok((is_hold, optimal_hold_index))
}

fn generate_vc(
	req: &AssertionBuildRequest,
	htype: &AmountHoldingTimeType,
	q_min_balance: &str,
	is_hold: bool,
	optimal_hold_index: usize,
) -> core::result::Result<Credential, ErrorDetail> {
	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.update_amount_holding_time_credential(
				htype,
				is_hold,
				q_min_balance,
				ASSERTION_FROM_DATE[optimal_hold_index],
			);

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(e.into_error_detail())
		},
	}
}

/// Utils Functions
fn emit_error(
	htype: &AmountHoldingTimeType,
	min_balance: &ParameterString,
	e: ErrorDetail,
) -> Error {
	Error::RequestVCFailed(Assertion::HoldingTime(htype.clone(), min_balance.clone()), e)
}

fn match_token_address(htype: &AmountHoldingTimeType, network: &Web3Network) -> Option<String> {
	match htype {
		AmountHoldingTimeType::WBTC => Some(WBTC_TOKEN_ADDRESS.into()),
		AmountHoldingTimeType::LIT =>
			if *network == Web3Network::Ethereum {
				Some(LIT_TOKEN_ADDRESS.into())
			} else {
				None
			},
		_ => None,
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use lc_data_providers::GLOBAL_DATA_PROVIDER_CONFIG;
	use lc_mock_server::{default_getter, run};
	use litentry_primitives::{AmountHoldingTimeType, Web3Network};
	use std::sync::Arc;

	fn init() {
		let _ = env_logger::builder().is_test(true).try_init();
		let url = run(Arc::new(default_getter), 0).unwrap();
		GLOBAL_DATA_PROVIDER_CONFIG.write().unwrap().set_achainable_url(url);
	}

	#[test]
	fn do_build_lit_works() {
		init();

		let identities = vec![
			(
				// 111111111111111111111111112 -> 2019-01-01
				// 222222222222222222222223    -> 2020-07-01
				Web3Network::Litentry,
				vec![
					"111111111111111111111111112".to_string(),
					"222222222222222222222223".to_string(),
				],
			),
			(
				// 111111111111111111111111114 -> 2018-01-01
				// 222222222222222222222225    -> 2022-07-01
				Web3Network::Litmus,
				vec![
					"111111111111111111111111114".to_string(),
					"222222222222222222222225".to_string(),
				],
			),
			(
				// 111111111111111111111111116 -> 2023-01-01
				// 222222222222222222222227    -> 2023-07-01
				Web3Network::Ethereum,
				vec![
					"111111111111111111111111116".to_string(),
					"222222222222222222222227".to_string(),
				],
			),
		];

		let htype = AmountHoldingTimeType::LIT;
		let q_min_balance = "10".to_string();

		let (is_hold, optimal_hold_index) = do_build(identities, &htype, &q_min_balance).unwrap();
		assert!(is_hold);
		assert_eq!(optimal_hold_index, 2);
	}

	#[test]
	fn do_build_dot_works() {
		init();

		let identities = vec![(
			Web3Network::Polkadot,
			vec!["11111111111111111111111111".to_string(), "22222222222222222222222".to_string()],
		)];
		let dot_type = AmountHoldingTimeType::DOT;
		let q_min_balance = "10".to_string();

		let (is_hold, optimal_hold_index) =
			do_build(identities, &dot_type, &q_min_balance).unwrap();
		assert!(is_hold);
		assert_eq!(optimal_hold_index, 0);
	}

	#[test]
	fn do_build_wbtc_works() {
		init();

		let identities = vec![(
			Web3Network::Ethereum,
			vec!["333333333333333333".to_string(), "4444444444444444444444".to_string()],
		)];
		let htype = AmountHoldingTimeType::WBTC;
		let q_min_balance = "10".to_string();

		let (is_hold, optimal_hold_index) = do_build(identities, &htype, &q_min_balance).unwrap();
		assert!(is_hold);
		assert_eq!(optimal_hold_index, 3);
	}

	#[test]
	fn do_build_non_hold_works() {
		init();

		let identities = vec![(
			// xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx -> [< 2017-01-01]
			Web3Network::Ethereum,
			vec!["xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_string()],
		)];
		let htype = AmountHoldingTimeType::LIT;
		let q_min_balance = "10".to_string();

		let (is_hold, optimal_hold_index) = do_build(identities, &htype, &q_min_balance).unwrap();
		assert!(!is_hold);
		assert_eq!(optimal_hold_index, 0);
	}

	#[test]
	fn match_token_address_works() {
		let htype = AmountHoldingTimeType::WBTC;
		let network = Web3Network::Ethereum;
		let ret = match_token_address(&htype, &network);
		assert_eq!(ret, Some(WBTC_TOKEN_ADDRESS.into()));

		let htype = AmountHoldingTimeType::LIT;
		let ret = match_token_address(&htype, &network);
		assert_eq!(ret, Some(LIT_TOKEN_ADDRESS.into()));
	}
}
