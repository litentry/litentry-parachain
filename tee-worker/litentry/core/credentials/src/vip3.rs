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
	Credential,
};
use litentry_primitives::VIP3MembershipCardLevel;

const VC_VIP3_CARD_INFOS: [(&str, &str); 2] = [
	// Gold: Type & Info
	("VIP3_CARD_LEVEL_GOLD_TYPE_REPLACEME", "VIP3_CARD_LEVEL_GOLD_INFO_REMPLACEME"),
	// Silver: Type & Info
	("VIP3_CARD_LEVEL_SILVER_TYPE_REMPLACEME", "VIP3_CARD_LEVEL_SILVER_INFO_REMPLACEME"),
];

const VIP3_BREAKDOWN: [&str; 2] = ["$is_gold_card", "$is_silver_card"];

pub trait EnumTypeCredentialDetail {
	fn get_info(&self) -> &'static str;
	fn get_type(&self) -> &'static str;
	fn get_breakdown(&self) -> &'static str;
}

pub struct VIP3MembershipCardEntity {
	pub level: VIP3MembershipCardLevel,
}

impl VIP3MembershipCardEntity {
	pub fn new(level: VIP3MembershipCardLevel) -> VIP3MembershipCardEntity {
		VIP3MembershipCardEntity { level }
	}
}

impl EnumTypeCredentialDetail for VIP3MembershipCardEntity {
	fn get_type(&self) -> &'static str {
		match self.level {
			VIP3MembershipCardLevel::Gold => VC_VIP3_CARD_INFOS[0].0,
			VIP3MembershipCardLevel::Silver => VC_VIP3_CARD_INFOS[1].0,
		}
	}

	fn get_info(&self) -> &'static str {
		match self.level {
			VIP3MembershipCardLevel::Gold => VC_VIP3_CARD_INFOS[0].1,
			VIP3MembershipCardLevel::Silver => VC_VIP3_CARD_INFOS[1].1,
		}
	}

	fn get_breakdown(&self) -> &'static str {
		match self.level {
			VIP3MembershipCardLevel::Gold => VIP3_BREAKDOWN[0],
			VIP3MembershipCardLevel::Silver => VIP3_BREAKDOWN[1],
		}
	}
}

pub trait UpdateVIP3MembershipCardCredential {
	fn update_vip3_membership_card(&mut self, level: VIP3MembershipCardLevel, value: bool);
}

impl UpdateVIP3MembershipCardCredential for Credential {
	fn update_vip3_membership_card(&mut self, level: VIP3MembershipCardLevel, value: bool) {
		let entity = VIP3MembershipCardEntity::new(level);

		self.add_subject_info(entity.get_info(), entity.get_type());

		let assertion = AssertionLogic::new_item(entity.get_breakdown(), Op::Equal, "true");
		self.credential_subject.assertions.push(assertion);
		self.credential_subject.values.push(value);
	}
}

#[cfg(test)]
mod tests {
	use super::VIP3MembershipCardEntity;
	use crate::vip3::{EnumTypeCredentialDetail, VC_VIP3_CARD_INFOS};
	use litentry_primitives::VIP3MembershipCardLevel;

	#[test]
	fn card_entity_gold_level_works() {
		let level = VIP3MembershipCardLevel::Gold;
		let entity = VIP3MembershipCardEntity::new(level);

		assert_eq!(entity.get_type(), VC_VIP3_CARD_INFOS[0].0);
		assert_eq!(entity.get_info(), VC_VIP3_CARD_INFOS[0].1);
	}

	#[test]
	fn card_entity_silver_level_works() {
		let level = VIP3MembershipCardLevel::Silver;
		let entity = VIP3MembershipCardEntity::new(level);

		assert_eq!(entity.get_type(), VC_VIP3_CARD_INFOS[1].0);
		assert_eq!(entity.get_info(), VC_VIP3_CARD_INFOS[1].1);
	}
}
