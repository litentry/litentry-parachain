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

// This file includes the predefined rulesets and the corresponding parameters
// when requesting VCs.

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use std::{
	boxed::Box,
	fmt::Debug,
	string::{String, ToString},
	vec,
	vec::Vec,
};

#[derive(Serialize, Deserialize, Encode, Decode, Debug, PartialEq, Eq, TypeInfo, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Op {
	#[codec(index = 0)]
	#[serde(rename = ">")]
	GreaterThan,
	#[codec(index = 1)]
	#[serde(rename = "<")]
	LessThan,
	#[codec(index = 2)]
	#[serde(rename = ">=")]
	GreaterEq,
	#[codec(index = 3)]
	#[serde(rename = "<=")]
	LessEq,
	#[codec(index = 4)]
	#[serde(rename = "==")]
	Equal,
	#[codec(index = 5)]
	#[serde(rename = "!=")]
	NotEq,
}

#[derive(Serialize, Deserialize, Encode, Decode, PartialEq, Eq, TypeInfo, Debug, Clone)]
#[serde(untagged)]
pub enum AssertionLogic {
	#[codec(index = 0)]
	Item { src: String, op: Op, dst: String },
	#[codec(index = 1)]
	And {
		#[serde(rename = "and")]
		items: Vec<Box<AssertionLogic>>,
	},
	#[codec(index = 2)]
	Or {
		#[serde(rename = "or")]
		items: Vec<Box<AssertionLogic>>,
	},
}

impl AssertionLogic {
	pub fn new_and() -> Self {
		Self::And { items: vec![] }
	}

	pub fn new_or() -> Self {
		Self::Or { items: vec![] }
	}

	pub fn new_item<T: ToString>(src: T, op: Op, dst: T) -> Self {
		Self::Item { src: src.to_string(), op, dst: dst.to_string() }
	}
	pub fn add_item(mut self, item: AssertionLogic) -> Self {
		match &mut self {
			Self::Item { .. } => unreachable!(),
			Self::Or { items } => items.push(Box::new(item)),
			Self::And { items } => items.push(Box::new(item)),
		}
		self
	}
}

pub trait Logic {
	fn eval(&self) -> bool;
}

impl Logic for AssertionLogic {
	fn eval(&self) -> bool {
		match self {
			Self::Item { src, op, dst } => match op {
				Op::GreaterThan => src > dst,
				Op::LessThan => src < dst,
				Op::GreaterEq => src >= dst,
				Op::LessEq => src <= dst,
				Op::Equal => src == dst,
				Op::NotEq => src != dst,
			},
			Self::And { items } => items.iter().all(|item| item.eval()),
			Self::Or { items } => items.iter().any(|item| item.eval()),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn assertion_logic_works() {
		let a1 = r#"
            {
                "or":[
                    {
                        "src":"$web2_account_cnt",
                        "op":">=",
                        "dst":"7"
                    },
                    {
                        "src":"$web3_account_cnt",
                        "op":">",
                        "dst":"3"
                    }
                ]
            }
            "#;

		let a1_from_str: AssertionLogic = serde_json::from_str(a1).unwrap();

		let web2_item = AssertionLogic::new_item("$web2_account_cnt", Op::GreaterEq, "7");
		let web3_item = AssertionLogic::new_item("$web3_account_cnt", Op::GreaterThan, "3");

		let a1_from_struct = AssertionLogic::new_or().add_item(web2_item).add_item(web3_item);

		assert_eq!(a1_from_str, a1_from_struct);
	}

	#[test]
	fn assertion_a1_eval_works() {
		let web2_item = AssertionLogic::new_item("7", Op::GreaterEq, "7");
		let web3_item = AssertionLogic::new_item("7", Op::GreaterThan, "3");

		let a1 = AssertionLogic::new_or().add_item(web2_item).add_item(web3_item);
		assert_eq!(a1.eval(), true);
	}
}
