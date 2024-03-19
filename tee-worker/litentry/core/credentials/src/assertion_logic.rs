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
		let mut src_string = src.to_string();
		if !src_string.starts_with("$") {
			log::warn!("AssertionLogic::new_item - src missing $ prefix: {}", src_string);
			src_string.insert_str(0, "$");
		}
		Self::Item { src: src_string, op, dst: dst.to_string() }
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
	fn assertion_new_item_adds_dollar_prefix_if_missing() {
		let item = AssertionLogic::new_item("some_src", Op::GreaterEq, "7");
		assert_eq!(
			item,
			AssertionLogic::Item { src: "$some_src".into(), op: Op::GreaterEq, dst: "7".into() }
		);
	}

	#[test]
	fn assertion_new_item_preserves_dollar_prefix_if_present() {
		let item = AssertionLogic::new_item("$some_src", Op::GreaterEq, "7");
		assert_eq!(
			item,
			AssertionLogic::Item { src: "$some_src".into(), op: Op::GreaterEq, dst: "7".into() }
		);
	}
}
