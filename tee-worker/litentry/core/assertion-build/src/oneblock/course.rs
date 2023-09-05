#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use lc_credentials::oneblock::OneBlockAssertionUpdate;
use crate::{*, oneblock::query_oneblock_status};

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
			credential_unsigned.update_notion_assertion(&level);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::Oneblock, e.into_error_detail()))
		},
	}
}