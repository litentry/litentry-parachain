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

use crate::{
	assertion_logic::{AssertionLogic, Op},
	litentry_profile::{BalanceRange, BalanceRangeIndex},
	Credential,
};
use lc_data_providers::geniidata::ResponseItem;
use std::vec::Vec;

const VC_BRC20_AMOUNT_HOLDER_DESCRIPTIONS: &str =
	"The amount of a particular token you are holding";
const VC_BRC20_AMOUNT_HOLDER_TYPE: &str = "Token holding amount list";

// Keep all name in lowercase here by purpose
const BRC20_TOKENS: [&str; 7] = ["ordi", "sats", "rats", "mmss", "long", "cats", "btcs"];
const ORDI_TOKEN_BALANCE_RANGE: [f64; 8] = [0.0, 1.0, 5.0, 20.0, 50.0, 100.0, 200.0, 500.0];
const SATS_TOKEN_BALANCE_RANGE: [f64; 9] = [
	0.0,
	1.0,
	40_000_000.0,
	200_000_000.0,
	500_000_000.0,
	1_000_000_000.0,
	2_000_000_000.0,
	4_000_000_000.0,
	6_000_000_000.0,
];
const RATS_TOKEN_BALANCE_RANGE: [f64; 9] = [
	0.0,
	1.0,
	40_000.0,
	200_000.0,
	1_000_000.0,
	2_000_000.0,
	4_000_000.0,
	10_000_000.0,
	20_000_000.0,
];
const MMSS_TOKEN_BALANCE_RANGE: [f64; 9] =
	[0.0, 1.0, 20.0, 50.0, 100.0, 200.0, 500.0, 1000.0, 2000.0];
const LONG_TOKEN_BALANCE_RANGE: [f64; 9] =
	[0.0, 1.0, 20.0, 50.0, 200.0, 500.0, 1000.0, 2000.0, 3000.0];
const CATS_TOKEN_BALANCE_RANGE: [f64; 8] =
	[0.0, 1.0, 10_000.0, 50_000.0, 100_000.0, 200_000.0, 500_000.0, 800_000.0];
const BTCS_TOKEN_BALANCE_RANGE: [f64; 9] = [0.0, 1.0, 5.0, 20.0, 50.0, 100.0, 200.0, 500.0, 800.0];

#[derive(Debug, PartialEq)]
enum BRC20Token {
	Ordi,
	Sats,
	Rats,
	Mmss,
	Long,
	Cats,
	Btcs,
	Unknown,
}

struct AssertionKeys {
	token: &'static str,
	holding_amount: &'static str,
}

#[derive(Debug)]
struct Brc20TokenBalance {
	pub token: BRC20Token,
	pub balance: f64,
}

const ASSERTION_KEYS: AssertionKeys =
	AssertionKeys { token: "$token", holding_amount: "$holding_amount" };

pub trait BRC20AmountHolderCredential {
	fn update_brc20_amount_holder_credential(&mut self, response_items: &[ResponseItem]);
}

impl BRC20AmountHolderCredential for Credential {
	fn update_brc20_amount_holder_credential(&mut self, response_items: &[ResponseItem]) {
		let token_balance_pairs = collect_brc20_token_balance(response_items);
		token_balance_pairs.iter().for_each(|pair| {
			let token = &pair.token;
			let balance = pair.balance;

			update_assertion(token, balance, self);
		});

		self.add_subject_info(VC_BRC20_AMOUNT_HOLDER_DESCRIPTIONS, VC_BRC20_AMOUNT_HOLDER_TYPE);
	}
}

// There may be empty cases
// case 1 : response_items is empty.
// case 2 : response_items is not empty, but have no interaction with BRC20_TOKENS
fn collect_brc20_token_balance(response_items: &[ResponseItem]) -> Vec<Brc20TokenBalance> {
	let mut pairs = vec![];

	response_items
		.iter()
		.filter(|&item| BRC20_TOKENS.contains(&item.tick.to_lowercase().as_str()))
		.for_each(|item| {
			let token = tick_to_brctoken(item.tick.to_lowercase().as_str());
			let balance: f64 = item.overall_balance.parse().unwrap_or(0.0);

			pairs.push(Brc20TokenBalance { token, balance });
		});

	// If the pair is still empty at this point, it means that there is no suitable data.
	// In that case, set the balance of all tokens to 0.0.
	if pairs.is_empty() {
		BRC20_TOKENS.iter().for_each(|tick| {
			let token = tick_to_brctoken(tick);
			let balance = 0.0_f64;

			pairs.push(Brc20TokenBalance { token, balance });
		})
	}

	pairs
}

// TODO: the following part is exactly the same structure from 'token_balance.rs'.
// Anyway the refactor is planned later. So continue using the same mechanism.
fn update_assertion(token: &BRC20Token, balance: f64, credential: &mut Credential) {
	let mut assertion = AssertionLogic::new_and();

	assertion = assertion.add_item(AssertionLogic::new_item(
		ASSERTION_KEYS.token,
		Op::Equal,
		brctoken_to_tick(token),
	));

	let range = get_balance_range(token);
	let index = BalanceRange::index(&range, balance);
	match index {
		Some(index) => {
			let min = format!("{}", range[index]);
			let max = format!("{}", range[index + 1]);
			let min_item = AssertionLogic::new_item(
				ASSERTION_KEYS.holding_amount,
				if index == 0 { Op::GreaterThan } else { Op::GreaterEq },
				&min,
			);

			assertion = assertion.add_item(min_item);
			if balance > 0_f64 {
				let max_item =
					AssertionLogic::new_item(ASSERTION_KEYS.holding_amount, Op::LessThan, &max);
				assertion = assertion.add_item(max_item);
			}

			credential.credential_subject.values.push(index != 0 || balance > 0_f64);
		},
		None => {
			let min_item = AssertionLogic::new_item(
				ASSERTION_KEYS.holding_amount,
				Op::GreaterEq,
				&format!("{}", get_token_range_last(token)),
			);
			assertion = assertion.add_item(min_item);

			credential.credential_subject.values.push(true);
		},
	}

	credential.credential_subject.assertions.push(assertion);
}

// Keep consistent with BRC20_TOKENS, all in lowercase letters.
fn tick_to_brctoken(tick: &str) -> BRC20Token {
	match tick {
		"ordi" => BRC20Token::Ordi,
		"sats" => BRC20Token::Sats,
		"rats" => BRC20Token::Rats,
		"mmss" => BRC20Token::Mmss,
		"long" => BRC20Token::Long,
		"cats" => BRC20Token::Cats,
		"btcs" => BRC20Token::Btcs,
		_ => BRC20Token::Unknown,
	}
}

fn brctoken_to_tick(token: &BRC20Token) -> &'static str {
	match token {
		BRC20Token::Ordi => "$ordi",
		BRC20Token::Sats => "$sats",
		BRC20Token::Rats => "$rats",
		BRC20Token::Mmss => "$MMSS",
		BRC20Token::Long => "$long",
		BRC20Token::Cats => "$cats",
		BRC20Token::Btcs => "$BTCs",
		_ => "Unknown",
	}
}

fn get_balance_range(token: &BRC20Token) -> Vec<f64> {
	match token {
		BRC20Token::Ordi => ORDI_TOKEN_BALANCE_RANGE.to_vec(),
		BRC20Token::Sats => SATS_TOKEN_BALANCE_RANGE.to_vec(),
		BRC20Token::Rats => RATS_TOKEN_BALANCE_RANGE.to_vec(),
		BRC20Token::Mmss => MMSS_TOKEN_BALANCE_RANGE.to_vec(),
		BRC20Token::Long => LONG_TOKEN_BALANCE_RANGE.to_vec(),
		BRC20Token::Cats => CATS_TOKEN_BALANCE_RANGE.to_vec(),
		BRC20Token::Btcs => BTCS_TOKEN_BALANCE_RANGE.to_vec(),
		_ => {
			vec![]
		},
	}
}

fn get_token_range_last(token: &BRC20Token) -> f64 {
	match token {
		BRC20Token::Ordi => *ORDI_TOKEN_BALANCE_RANGE.last().unwrap_or(&500.0),
		BRC20Token::Sats => *SATS_TOKEN_BALANCE_RANGE.last().unwrap_or(&6_000_000_000.0),
		BRC20Token::Rats => *RATS_TOKEN_BALANCE_RANGE.last().unwrap_or(&20_000_000.0),
		BRC20Token::Mmss => *MMSS_TOKEN_BALANCE_RANGE.last().unwrap_or(&2000.0),
		BRC20Token::Long => *LONG_TOKEN_BALANCE_RANGE.last().unwrap_or(&3000.0),
		BRC20Token::Cats => *CATS_TOKEN_BALANCE_RANGE.last().unwrap_or(&800_000.0),
		BRC20Token::Btcs => *BTCS_TOKEN_BALANCE_RANGE.last().unwrap_or(&800.0),
		_ => 0.0,
	}
}

#[cfg(test)]
mod tests {
	use super::collect_brc20_token_balance;
	use crate::brc20::amount_holder::BRC20Token;
	use lc_data_providers::geniidata::ResponseItem;

	#[test]
	fn collect_brc20_token_balance_empty_items_works() {
		let response_items = vec![];
		let pairs = collect_brc20_token_balance(&response_items);
		assert_eq!(pairs.len(), 7);
		for pair in pairs {
			assert_eq!(pair.balance, 0.0_f64);
		}
	}

	#[test]
	fn collect_brc20_token_balance_do_not_have_brc20_tokens_works() {
		let response_items = vec![ResponseItem {
			tick: "no-tick".to_string(),
			address: "0x01".to_string(),
			overall_balance: "0.000000000000020000".to_string(),
			transferable_balance: "0.000000000000000000".to_string(),
			available_balance: "0.000000000000020000".to_string(),
		}];

		let pairs = collect_brc20_token_balance(&response_items);
		assert_eq!(pairs.len(), 7);
		for pair in pairs {
			assert_eq!(pair.balance, 0.0_f64);
		}
	}

	#[test]
	fn collect_brc20_token_balance_have_items_works() {
		let response_items = vec![
			ResponseItem {
				tick: "ordi".to_string(),
				address: "0x01".to_string(),
				overall_balance: "0.000000000000020000".to_string(),
				transferable_balance: "0.000000000000000000".to_string(),
				available_balance: "0.000000000000020000".to_string(),
			},
			ResponseItem {
				tick: "MMSS".to_string(),
				address: "0x01".to_string(),
				overall_balance: "0.000000000000020000".to_string(),
				transferable_balance: "0.000000000000000000".to_string(),
				available_balance: "0.000000000000020000".to_string(),
			},
		];
		let pairs = collect_brc20_token_balance(&response_items);
		assert_eq!(pairs.len(), 2);
		assert_eq!(pairs[0].token, BRC20Token::Ordi);
		assert_eq!(pairs[1].token, BRC20Token::Mmss);
	}
}
