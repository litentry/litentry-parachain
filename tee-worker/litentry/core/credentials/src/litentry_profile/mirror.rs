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
use lc_data_providers::achainable_names::AchainableNameMirror;

// Is a publication on Mirror
// Has written over quantity posts on Mirror
// (type, description)
const VC_MIRROR_INFOS: [(&str, &str); 2] = [
	("Is a publication on Mirror", "You are a publication on Mirror"),
	("Has written over quantity posts on Mirror", "You have written some posts on Mirror"),
];

pub trait MirrorInfo {
	fn update_mirror(&mut self, mtype: AchainableNameMirror, value: bool);
}

impl MirrorInfo for Credential {
	fn update_mirror(&mut self, mtype: AchainableNameMirror, value: bool) {
		let info = get_mirror_info(&mtype);
		self.add_subject_info(info.1, info.0);

		update_mirror_assertion(&mtype, value, self);
	}
}

fn update_mirror_assertion(mtype: &AchainableNameMirror, value: bool, credential: &mut Credential) {
	let content = get_mirror_content(mtype);
	let logic = AssertionLogic::new_item(content, Op::Equal, "true");
	let assertion = AssertionLogic::new_and().add_item(logic);
	credential.credential_subject.assertions.push(assertion);
	credential.credential_subject.values.push(value);
}

fn get_mirror_info(mtype: &AchainableNameMirror) -> (&'static str, &'static str) {
	match mtype {
		AchainableNameMirror::IsAPublicationOnMirror => VC_MIRROR_INFOS[0],
		AchainableNameMirror::HasWrittenOverQuantityPostsOnMirror => VC_MIRROR_INFOS[1],
	}
}

fn get_mirror_content(mtype: &AchainableNameMirror) -> &'static str {
	match mtype {
		AchainableNameMirror::IsAPublicationOnMirror => "$is_publication_on_mirror",
		AchainableNameMirror::HasWrittenOverQuantityPostsOnMirror => "$has_post_on_mirror",
	}
}
