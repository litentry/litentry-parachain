mod email_verification_store;
pub use email_verification_store::*;

use crate::web2::helpers;
use std::string::String;

pub fn generate_verification_code() -> String {
	helpers::get_random_string(32)
}

