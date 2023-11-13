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
use litentry_primitives::GenericDiscordRoleType;

// ContestLegend / ContestPopularity / ContestParticipant
// (type, description)
const VC_LITENTRY_GENERIC_DISCORD_ROLE_INFOS: [(&str, &str); 3] = [
	("Litentry & Contest Legend", "You got the Top Award of community contest."),
	(
		"Litentry & Popularity Award of Score Contest",
		"You got the Popularity Award of community contest.",
	),
	("Litentry & Contest Participant", "You participated in the community contest."),
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

fn get_generic_discord_role_assertion_content(ctype: &GenericDiscordRoleType) -> &'static str {
	match ctype {
		GenericDiscordRoleType::ContestLegend => "$is_contest_legend",
		GenericDiscordRoleType::ContestPopularity => "$is_contest_popularity",
		GenericDiscordRoleType::ContestParticipant => "$is_contest_participant",
	}
}

fn get_generic_discord_role_assertion_info(
	ctype: &GenericDiscordRoleType,
) -> (&'static str, &'static str) {
	match ctype {
		GenericDiscordRoleType::ContestLegend => VC_LITENTRY_GENERIC_DISCORD_ROLE_INFOS[0],
		GenericDiscordRoleType::ContestPopularity => VC_LITENTRY_GENERIC_DISCORD_ROLE_INFOS[1],
		GenericDiscordRoleType::ContestParticipant => VC_LITENTRY_GENERIC_DISCORD_ROLE_INFOS[2],
	}
}
