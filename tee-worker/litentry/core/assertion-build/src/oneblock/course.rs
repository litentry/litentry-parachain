#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::{*, oneblock::{query_oneblock_status, CourseCompletionLevel}};

/// Dev Undergraduate / Outstanding Substrate Developer / Dev Beginner
// (type, description)
const COURSE_INFOS: [(&str, &str); 3] = [
	("Substrate Blockchain Development Course Completion", "Congratulations on completing the entire course jointly created by OneBlock+ and Parity:《Introduction to Substrate Blockchain Development Course》, Phase 12. We hope you will keep your enthusiasm and keep exploring in the future path."),
	("Substrate Blockchain Development Course Excellence Completion", "Congratulations on completing the entire course jointly created by OneBlock+ and Parity:《Introduction to Substrate Blockchain Development Course》, Phase 12. You stood out from all the students and were awarded the \"Outstanding Student\" title."),
	("Substrate Blockchain Development Course Participation", "Congratulations on completing the entire course jointly created by OneBlock+ and Parity:《Introduction to Substrate Blockchain Development Course》, Phase 12."),
];

pub fn build(req: &AssertionBuildRequest) -> Result<Credential> {
	if !req.who.is_substrate() {
		return Err(Error::RequestVCFailed(Assertion::Oneblock, ErrorDetail::StfError(ErrorString::truncate_from("Only substrate address supported.".into()))))
	}

	// TODO: Main Account substrate address format
	let _who = req.who.to_account_id().unwrap();
	let address = "_who".into();
    let level = query_oneblock_status(&address)?;
	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			let info = get_info(&level);
            credential_unsigned.add_subject_info(info.1, info.0);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::Oneblock, e.into_error_detail()))
		},
	}
}

fn get_info(level: &CourseCompletionLevel) -> (&str, &str) {
	match level {
		CourseCompletionLevel::Undergraduate => COURSE_INFOS[0],
		CourseCompletionLevel::Outstanding => COURSE_INFOS[1],
		CourseCompletionLevel::Beginner => COURSE_INFOS[2],
		CourseCompletionLevel::Invalid => COURSE_INFOS[0], //TODO: What will type&descrition show if in this case?
	}
}