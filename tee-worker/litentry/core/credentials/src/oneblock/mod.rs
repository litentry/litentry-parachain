use crate::{
	assertion_logic::{AssertionLogic, Op},
	Credential,
};
use serde::{Deserialize, Serialize};

/// Dev Undergraduate / Outstanding Substrate Developer / Dev Beginner
// (type, description)
const VC_ONEBLOCK_COURSE_INFOS: [(&str, &str); 3] = [
	("Substrate Blockchain Development Course Completion", "Congratulations on completing the entire course jointly created by OneBlock+ and Parity:《Introduction to Substrate Blockchain Development Course》, Phase 12. We hope you will keep your enthusiasm and keep exploring in the future path."),
	("Substrate Blockchain Development Course Excellence Completion", "Congratulations on completing the entire course jointly created by OneBlock+ and Parity:《Introduction to Substrate Blockchain Development Course》, Phase 12. You stood out from all the students and were awarded the \"Outstanding Student\" title."),
	("Substrate Blockchain Development Course Participation", "Congratulations on completing the entire course jointly created by OneBlock+ and Parity:《Introduction to Substrate Blockchain Development Course》, Phase 12."),
];

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum CourseCompletionLevel {
	Undergraduate,
	Outstanding,
	Beginner,
	Invalid,
}

pub trait OneBlockAssertionUpdate {
	fn update_notion_assertion(&mut self, level: &CourseCompletionLevel);
}

impl OneBlockAssertionUpdate for Credential {
	fn update_notion_assertion(&mut self, level: &CourseCompletionLevel) {
		let (assertion_content, info) = match level {
			CourseCompletionLevel::Undergraduate =>
				("is_undergraduate", VC_ONEBLOCK_COURSE_INFOS[0]),
			CourseCompletionLevel::Outstanding => ("is_outstanding", VC_ONEBLOCK_COURSE_INFOS[1]),
			CourseCompletionLevel::Beginner | &CourseCompletionLevel::Invalid =>
				("is_beginner", VC_ONEBLOCK_COURSE_INFOS[2]),
		};

		let assertion = AssertionLogic::new_item(assertion_content, Op::Equal, "true");
		self.credential_subject.assertions.push(assertion);

		// If and only if level == CourseCompletionLevel::Invalid, value will be FALSE
		self.credential_subject.values.push(*level != CourseCompletionLevel::Invalid);

		self.add_subject_info(info.1, info.0);
	}
}
