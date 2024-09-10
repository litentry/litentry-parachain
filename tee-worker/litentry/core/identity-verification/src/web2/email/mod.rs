mod verification_code_store;
pub use verification_code_store::*;

mod mailer;
pub use mailer::*;

use crate::web2::helpers;
use std::string::String;

pub fn generate_verification_code() -> String {
	helpers::get_random_string(32)
}

pub fn send_verification_email(
	mailer: &mut impl Mailer,
	to_email: String,
	redirect_url: String,
) -> Result<(), String> {
	let mail = mailer::Mail {
		to: to_email,
		subject: String::from("Verify your email address"),
		body: template::VERYFY_EMAIL_TEMPLATE.replace("{{ redirect_url }}", &redirect_url),
	};

	mailer.send(mail)
}
