use crate::{
	assertion_logic::{AssertionLogic, Op},
	Credential,
};
use litentry_primitives::OneBlockCourseType;

/// Course Completion / Course Outstanding Student / Course Participation
// (type, description)
const VC_ONEBLOCK_COURSE_INFOS: [(&str, &str); 3] = [
	("Substrate Blockchain Development Course Completion", "Congratulations on completing the entire course jointly created by OneBlock+ and Parity:《Introduction to Substrate Blockchain Development Course》, Phase 12. We hope you will keep your enthusiasm and keep exploring in the future path."),
	("Substrate Blockchain Development Course Outstanding Student", "Congratulations on completing the entire course jointly created by OneBlock+ and Parity:《Introduction to Substrate Blockchain Development Course》, Phase 12. You stood out from all the students and were awarded the [Outstanding Student] title."),
	("Substrate Blockchain Development Course Participation", "Congratulations on completing the entire course jointly created by OneBlock+ and Parity:《Introduction to Substrate Blockchain Development Course》, Phase 12."),
];

pub trait OneBlockAssertionUpdate {
	fn update_notion_assertion(&mut self, course_type: &OneBlockCourseType, value: bool);
}

impl OneBlockAssertionUpdate for Credential {
	fn update_notion_assertion(&mut self, course_type: &OneBlockCourseType, value: bool) {
		let (assertion_content, info) = match course_type {
			OneBlockCourseType::CourseCompletion =>
				("$course_completed", VC_ONEBLOCK_COURSE_INFOS[0]),
			OneBlockCourseType::CourseOutstanding =>
				("$excellent_completion", VC_ONEBLOCK_COURSE_INFOS[1]),
			OneBlockCourseType::CourseParticipation =>
				("$course_participated", VC_ONEBLOCK_COURSE_INFOS[2]),
		};

		let assertion = AssertionLogic::new_item(assertion_content, Op::Equal, "true");
		self.credential_subject.assertions.push(assertion);
		self.credential_subject.values.push(value);

		self.add_subject_info(info.1, info.0);
	}
}
