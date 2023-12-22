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

use crate::{
	assertion_logic::{AssertionLogic, Op},
	litentry_profile::{BalanceRange, BalanceRangeIndex},
	Credential,
};
use lazy_static::lazy_static;
use lc_data_providers::geniidata::ResponseItem;
use std::vec::Vec;

const VC_BRC20_AMOUNT_HOLDER_DESCRIPTIONS: &str =
	"The amount of a particular token you are holding";
const VC_BRC20_AMOUNT_HOLDER_TYPE: &str = "Token holding amount";

lazy_static! {
	static ref BRC20_TOKENS: Vec<&'static str> =
		vec!["ordi", "sats", "rats", "Mmss", "long", "cats", "BTCs",];
}

const ORDI_TOKEN_BALANCE_RANGE: [f64; 10] =
	[0.0, 1.0, 5.0, 20.0, 50.0, 100.0, 200.0, 500.0, 800.0, 1000.0];
const SATS_TOKEN_BALANCE_RANGE: [f64; 6] =
	[0.0, 40_000_000.0, 200_000_000.0, 1_000_000_000.0, 2_000_000_000.0, 4_000_000_000.0];
const RATS_TOKEN_BALANCE_RANGE: [f64; 6] =
	[0.0, 40_000_000.0, 200_000_000.0, 1_000_000_000.0, 2_000_000_000.0, 4_000_000_000.0];
const MMSS_TOKEN_BALANCE_RANGE: [f64; 10] =
	[0.0, 100.0, 200.0, 500.0, 1000.0, 2000.0, 3500.0, 5000.0, 8000.0, 12000.0];
const LONG_TOKEN_BALANCE_RANGE: [f64; 8] = [0.0, 10.0, 50.0, 200.0, 500.0, 1000.0, 2000.0, 3500.0];
const CATS_TOKEN_BALANCE_RANGE: [f64; 10] =
	[0.0, 100.0, 200.0, 500.0, 1000.0, 2000.0, 3500.0, 5000.0, 8000.0, 12000.0];
const BTCS_TOKEN_BALANCE_RANGE: [f64; 10] =
	[0.0, 100.0, 200.0, 500.0, 1000.0, 2000.0, 3500.0, 5000.0, 8000.0, 12000.0];

enum BRCToken {
	Ordi,
	Sats,
	Rats,
	Mmss,
	Long,
	Cats,
	Btcs,
	Unknown,
}

pub trait BRC20AmountHolderCredential {
	fn update_brc20_amount_holder_credential(&mut self, response_items: &[ResponseItem]);
}

impl BRC20AmountHolderCredential for Credential {
	fn update_brc20_amount_holder_credential(&mut self, response_items: &[ResponseItem]) {
		// let found_tokens: Vec<String> = items
		// 	.iter()
		// 	.filter_map(|item| {
		// 		BRC20_TOKENS
		// 			.iter()
		// 			.find(|&&name| name == item.tick)
		// 			.map(|name| name.to_string())
		// 	})
		// 	.collect();
		// let matching_items: Vec<ResponseItem> = response_items
		//     .iter()
		//     .filter(|item| BRC20_TOKENS.contains(&item.tick.as_str()))
		//     .cloned()
		//     .collect();

		for item in response_items {
			if BRC20_TOKENS.contains(&item.tick.as_str()) {
				let token = tick_to_brctoken(&item.tick);
				let balance: f64 = item.overall_balance.parse().unwrap_or(0.0);
				update_assertion(token, balance, self);
			}
		}

		self.add_subject_info(VC_BRC20_AMOUNT_HOLDER_DESCRIPTIONS, VC_BRC20_AMOUNT_HOLDER_TYPE);
	}
}

// TODO: the following part is exactly the same structure from 'token_balance.rs'.
// Anyway the refactor is planned later. So continue using the same mechanism.
fn update_assertion(token: BRCToken, balance: f64, credential: &mut Credential) {
	let mut assertion = AssertionLogic::new_and();

	let content = get_assertion_content(&token);
	let range = get_balance_range(&token);
	let index = BalanceRange::index(&range, balance);
	match index {
		Some(index) => {
			let min = format!("{}", range[index]);
			let max = format!("{}", range[index + 1]);
			let min_item = AssertionLogic::new_item(content, Op::GreaterEq, &min);
			let max_item = AssertionLogic::new_item(content, Op::LessThan, &max);

			assertion = assertion.add_item(min_item);
			assertion = assertion.add_item(max_item);

			credential.credential_subject.values.push(index != 0);
		},
		None => {
			let min_item = AssertionLogic::new_item(
				content,
				Op::GreaterEq,
				&format!("{}", get_token_range_last(&token)),
			);
			assertion = assertion.add_item(min_item);

			credential.credential_subject.values.push(true);
		},
	}

	credential.credential_subject.assertions.push(assertion);
}

fn tick_to_brctoken(tick: &str) -> BRCToken {
	match tick {
		"ordi" => BRCToken::Ordi,
		"sats" => BRCToken::Sats,
		"rats" => BRCToken::Rats,
		"Mmss" => BRCToken::Mmss,
		"long" => BRCToken::Long,
		"cats" => BRCToken::Cats,
		"BTCs" => BRCToken::Btcs,
		_ => BRCToken::Unknown,
	}
}

fn get_assertion_content(token: &BRCToken) -> &'static str {
	match token {
		BRCToken::Ordi => "$ordi_holding_amount",
		BRCToken::Sats => "$sats_holding_amount",
		BRCToken::Rats => "$rats_holding_amount",
		BRCToken::Mmss => "$MMSS_holding_amount",
		BRCToken::Long => "$long_holding_amount",
		BRCToken::Cats => "$cats_holding_amount",
		BRCToken::Btcs => "$BTCs_holding_amount",
		_ => "Unknown",
	}
}

fn get_balance_range(token: &BRCToken) -> Vec<f64> {
	match token {
		BRCToken::Ordi => ORDI_TOKEN_BALANCE_RANGE.to_vec(),
		BRCToken::Sats => SATS_TOKEN_BALANCE_RANGE.to_vec(),
		BRCToken::Rats => RATS_TOKEN_BALANCE_RANGE.to_vec(),
		BRCToken::Mmss => MMSS_TOKEN_BALANCE_RANGE.to_vec(),
		BRCToken::Long => LONG_TOKEN_BALANCE_RANGE.to_vec(),
		BRCToken::Cats => CATS_TOKEN_BALANCE_RANGE.to_vec(),
		BRCToken::Btcs => BTCS_TOKEN_BALANCE_RANGE.to_vec(),
		_ => {
			vec![]
		},
	}
}

fn get_token_range_last(token: &BRCToken) -> f64 {
	match token {
		BRCToken::Ordi => *ORDI_TOKEN_BALANCE_RANGE.last().unwrap_or(&1000.0),
		BRCToken::Sats => *SATS_TOKEN_BALANCE_RANGE.last().unwrap_or(&3000.0),
		BRCToken::Rats => *RATS_TOKEN_BALANCE_RANGE.last().unwrap_or(&1800.0),
		BRCToken::Mmss => *MMSS_TOKEN_BALANCE_RANGE.last().unwrap_or(&1800.0),
		BRCToken::Long => *LONG_TOKEN_BALANCE_RANGE.last().unwrap_or(&1200.0),
		BRCToken::Cats => *CATS_TOKEN_BALANCE_RANGE.last().unwrap_or(&1200.0),
		BRCToken::Btcs => *BTCS_TOKEN_BALANCE_RANGE.last().unwrap_or(&12000.0),
		_ => 0.0,
	}
}
