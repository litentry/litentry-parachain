pub mod sendgrid_mailer;

use std::string::String;

pub struct Mail {
	pub to: String,
	pub verification_code: String,
	pub redirect_url: Option<String>,
}

pub trait Mailer {
	fn send(&mut self, mail: Mail) -> Result<(), String>;
}
