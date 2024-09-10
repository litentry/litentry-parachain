pub mod sendgrid_mailer;
pub(crate) mod template;

use std::string::String;

pub struct Mail {
	pub to: String,
	pub subject: String,
	pub body: String,
}

pub trait Mailer {
	fn send(&mut self, mail: Mail) -> Result<(), String>;
}
