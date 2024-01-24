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
	Credential,
};
use litentry_primitives::{ContestType, GenericDiscordRoleType, SoraQuizType};

// Legend / Popularity / Participant
// (type, description)
const VC_LITENTRY_CONTEST_INFOS: [(&str, &str); 3] = [
	("Litentry & Contest Legend", "You got the Top Award of community contest."),
	(
		"Litentry & Popularity Award of Score Contest",
		"You got the Popularity Award of community contest.",
	),
	("Litentry & Contest Participant", "You participated in the community contest."),
];

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

pub trait GenericDiscordRoleAssertionUpdate {
	fn update_generic_discord_role_assertion(&mut self, ctype: GenericDiscordRoleType, value: bool);
}

impl GenericDiscordRoleAssertionUpdate for Credential {
	fn update_generic_discord_role_assertion(
		&mut self,
		ctype: GenericDiscordRoleType,
		value: bool,
	) {
		let assertion_content = get_generic_discord_role_assertion_content(&ctype);
		let assertion = AssertionLogic::new_item(assertion_content, Op::Equal, "true");
		self.credential_subject.assertions.push(assertion);
		self.credential_subject.values.push(value);

		let info = get_generic_discord_role_assertion_info(&ctype);
		self.add_subject_info(info.1, info.0);
	}
}

fn get_generic_discord_role_assertion_content(rtype: &GenericDiscordRoleType) -> &'static str {
	match rtype {
		GenericDiscordRoleType::Contest(ctype) => match ctype {
			ContestType::Legend => "$is_contest_legend",
			ContestType::Popularity => "$is_contest_popularity",
			ContestType::Participant => "$is_contest_participant",
		},
		GenericDiscordRoleType::SoraQuiz(qtype) => match qtype {
			SoraQuizType::Attendee => "$is_attendee",
			SoraQuizType::Master => "$is_master",
		},
	}
}

fn get_generic_discord_role_assertion_info(
	rtype: &GenericDiscordRoleType,
) -> (&'static str, &'static str) {
	match rtype {
		GenericDiscordRoleType::Contest(ctype) => match ctype {
			ContestType::Legend => VC_LITENTRY_CONTEST_INFOS[0],
			ContestType::Popularity => VC_LITENTRY_CONTEST_INFOS[1],
			ContestType::Participant => VC_LITENTRY_CONTEST_INFOS[2],
		},
		GenericDiscordRoleType::SoraQuiz(ctype) => match ctype {
			SoraQuizType::Attendee => VC_LITENTRY_SORA_QUIZ_INFOS[0],
			SoraQuizType::Master => VC_LITENTRY_SORA_QUIZ_INFOS[1],
		},
	}
}
