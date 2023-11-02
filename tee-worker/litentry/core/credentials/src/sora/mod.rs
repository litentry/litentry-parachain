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
use litentry_primitives::SoraQuizType;

// Attendee / Master
// (type, description)
const VC_LITENTRY_SORA_QUIZ_INFOS: [(&str, &str); 2] = [
	(
        "Litentry & SORA Quiz Attendee",
        "Congratulations on your participation in our first quiz in collaboration with our partner, Sora. You have embarked on an exciting educational journey, exploring the world of DeFi & Web3 Identity, we truly appreciate your curiosity and dedication."
    ),
    (
        "Litentry & SORA Quiz Master",
        "Congratulations on winning our first quiz in collaboration with Sora! By emerging as the winner, you have shown your excellent understanding of DeFi along with Web3 identity security and privacy. You are truly a champion in this field!"
    ),
];

pub trait SoraQuizAssertionUpdate {
	fn update_sora_quiz_assertion(&mut self, qtype: SoraQuizType, value: bool);
}

impl SoraQuizAssertionUpdate for Credential {
	fn update_sora_quiz_assertion(&mut self, qtype: SoraQuizType, value: bool) {
		let assertion_content = get_sora_assertion_content(&qtype);
		let assertion = AssertionLogic::new_item(assertion_content, Op::Equal, "true");
		self.credential_subject.assertions.push(assertion);
		self.credential_subject.values.push(value);

		let info = get_sora_assertion_info(&qtype);
		self.add_subject_info(info.1, info.0);
	}
}

fn get_sora_assertion_content(qtype: &SoraQuizType) -> &'static str {
	match qtype {
		SoraQuizType::Attendee => "$is_attendee",
		SoraQuizType::Master => "$is_master",
	}
}

fn get_sora_assertion_info(qtype: &SoraQuizType) -> (&'static str, &'static str) {
	match qtype {
		SoraQuizType::Attendee => VC_LITENTRY_SORA_QUIZ_INFOS[0],
		SoraQuizType::Master => VC_LITENTRY_SORA_QUIZ_INFOS[1],
	}
}
