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

use super::RangeCredentialDetail;
use crate::{assertion_logic::AssertionLogic, Credential};
use litentry_primitives::BnbDigitDomainType;
use std::vec::Vec;

// VC type / info
// 999ClubDomainMember / 9999ClubDomainMember
const DIGIT_DOMAIN_HOLDING_AMOUNT_INFOS: [(&str, &str); 2] = [
	(".BNB 999 club domain holding amount", "The amount of .BNB 999 club domains you are holding"),
	(".BNB 10k club domain holding amount", "The amount of .BNB 10k club domains you are holding"),
];

// [x-y)
pub const BNB_999_CLUB_MEMBER: [usize; 6] = [0, 1, 5, 10, 20, 50];
pub const BNB_10K_CLUB_MEMBER: [usize; 9] = [0, 1, 5, 10, 20, 50, 100, 200, 500];

pub struct DigitDomain {
	pub digit_domain_type: BnbDigitDomainType,
}

impl DigitDomain {
	pub fn new(digit_domain_type: BnbDigitDomainType) -> Self {
		Self { digit_domain_type }
	}
}

impl RangeCredentialDetail for DigitDomain {
	fn get_info(&self) -> (&'static str, &'static str) {
		match self.digit_domain_type {
			BnbDigitDomainType::Bnb999ClubMember => DIGIT_DOMAIN_HOLDING_AMOUNT_INFOS[0],
			BnbDigitDomainType::Bnb10kClubMember => DIGIT_DOMAIN_HOLDING_AMOUNT_INFOS[1],
		}
	}

	fn get_range(&self) -> Vec<usize> {
		match self.digit_domain_type {
			BnbDigitDomainType::Bnb999ClubMember => BNB_999_CLUB_MEMBER.to_vec(),
			BnbDigitDomainType::Bnb10kClubMember => BNB_10K_CLUB_MEMBER.to_vec(),
		}
	}

	fn get_last_value(&self) -> usize {
		match self.digit_domain_type {
			BnbDigitDomainType::Bnb999ClubMember => *BNB_999_CLUB_MEMBER.last().unwrap_or(&50),
			BnbDigitDomainType::Bnb10kClubMember => *BNB_10K_CLUB_MEMBER.last().unwrap_or(&500),
		}
	}

	fn get_breakdown(&self) -> &'static str {
		match self.digit_domain_type {
			BnbDigitDomainType::Bnb999ClubMember => "$999_club_member",
			BnbDigitDomainType::Bnb10kClubMember => "$10k_club_member",
		}
	}
}

pub trait UpdateDigitDomainClubAmountCredential {
	fn update_digit_domain_club_amount(
		&mut self,
		digit_domain_type: &BnbDigitDomainType,
		amount: usize,
	);
}

impl UpdateDigitDomainClubAmountCredential for Credential {
	fn update_digit_domain_club_amount(
		&mut self,
		digit_domain_type: &BnbDigitDomainType,
		amount: usize,
	) {
		let digit_domain = DigitDomain::new(digit_domain_type.clone());
		let items = digit_domain.get_assertion_items(amount);

		let mut assertion = AssertionLogic::new_and();
		for item in items {
			assertion = assertion.add_item(item);
		}

		self.credential_subject.assertions.push(assertion);
		// The credential value should be true if amount > ranges[0].
		self.credential_subject.values.push(amount > digit_domain.get_range()[0]);

		let info = digit_domain.get_info();
		self.add_subject_info(info.1, info.0);
	}
}
