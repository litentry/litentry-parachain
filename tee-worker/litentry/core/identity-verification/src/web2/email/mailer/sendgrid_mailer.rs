#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

use super::{Mail, Mailer};
use http::header::{AUTHORIZATION, CONTENT_TYPE};
use http_req::response::Headers;
use itc_rest_client::{
	error::Error as HttpError,
	http_client::{HttpClient, SendWithCertificateVerification},
	rest_client::RestClient,
	RestGet, RestPath, RestPost,
};
use serde::{Deserialize, Serialize};
use std::{
	string::{String, ToString},
	vec,
	vec::Vec,
};
use url::Url;

#[derive(Serialize)]
struct Personalization {
	to: Vec<Email>,
}

#[derive(Serialize, Clone)]
pub(crate) struct Email {
	pub email: String,
	pub name: Option<String>,
}

#[derive(Serialize)]
struct DynamicTemplateData {
	redirect_url: String,
}

#[derive(Serialize)]
struct SendGridEmail {
	personalizations: Vec<Personalization>,
	from: Email,
	template_id: String,
	dynamic_template_data: DynamicTemplateData,
}

impl SendGridEmail {
	pub fn new(
		to: Email,
		from: Email,
		template_id: String,
		verification_code: String,
		redirect_url: String,
	) -> Self {
		Self {
			personalizations: vec![Personalization { to: vec![to] }],
			from,
			template_id,
			dynamic_template_data: DynamicTemplateData {
				redirect_url: std::format!(
					"{}?verification_code={}",
					redirect_url,
					verification_code
				),
			},
		}
	}
}

impl RestPath<String> for SendGridEmail {
	fn get_path(path: String) -> Result<String, HttpError> {
		Ok(path)
	}
}

pub(crate) struct SendGridMailer {
	api_key: String,
	client: RestClient<HttpClient<SendWithCertificateVerification>>,
	from: Email,
	template_id: String,
}

impl SendGridMailer {
	pub fn new(api_key: String, from_email: Email, template_id: String) -> Self {
		let base_url = Url::parse("https://api.sendgrid.com/v3/mail/send").unwrap();
		let authorization = std::format!("Bearer {}", api_key);

		let mut headers = Headers::new();
		headers.insert(AUTHORIZATION.as_str(), &authorization);
		headers.insert(CONTENT_TYPE.as_str(), "application/json");

		Self {
			api_key,
			client: RestClient::new(
				HttpClient::new(
					SendWithCertificateVerification::new(vec![]),
					true,
					None,
					Some(headers),
					None,
				),
				base_url,
			),
			from: from_email,
			template_id,
		}
	}
}

impl Mailer for SendGridMailer {
	fn send(&mut self, mail: Mail) -> Result<(), String> {
		let sendgrid_email = SendGridEmail::new(
			Email { email: mail.to, name: None },
			self.from.clone(),
			self.template_id.clone(),
			mail.redirect_url.unwrap_or_default(),
			mail.verification_code,
		);
		self.client
			.post(String::default(), &sendgrid_email)
			.map_err(|e| std::format!("Failed to send verification email: {:?}", e))?;

		Ok(())
	}
}
