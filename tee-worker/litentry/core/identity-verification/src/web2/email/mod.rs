use crate::web2::helpers;
use std::string::String;

pub fn generate_verification_code() -> String {
	helpers::get_random_string(32)
}

