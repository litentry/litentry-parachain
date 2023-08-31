#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::{*, oneblock::fetch_notion_data};

const VC_DEV_BEGINNER_SUBJECT_DESCRIPTION: &str =
	"You are an outstanding student of the OneBlock+ Substrate developer course.";
const VC_DEV_BEGINNER_SUBJECT_TYPE: &str = "OneBlock+ Outstanding Substrate Developer";

pub fn build(req: &AssertionBuildRequest) -> Result<Credential> {
    let _data = fetch_notion_data()?;
	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
            credential_unsigned.add_subject_info(VC_DEV_BEGINNER_SUBJECT_DESCRIPTION, VC_DEV_BEGINNER_SUBJECT_TYPE);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::Oneblock, e.into_error_detail()))
		},
	}
}