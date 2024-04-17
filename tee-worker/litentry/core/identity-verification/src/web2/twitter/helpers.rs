#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_rand as rand;
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;
#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

use base64::engine::{general_purpose::STANDARD as BASE64_STANDARD, Engine};
use rand::{thread_rng, Rng};
use std::{format, string::String, vec::Vec};

use crate::{Error, Result};
use lc_data_providers::twitter_official::Tweet;
use litentry_primitives::{self, ErrorDetail};

pub(crate) fn get_code_verifier() -> String {
	get_random_string(128)
}

pub(crate) fn get_state_verifier() -> String {
	get_random_string(32)
}

pub(crate) fn payload_from_tweet(tweet: &Tweet) -> Result<Vec<u8>> {
	hex::decode(tweet.text.strip_prefix("0x").unwrap_or(tweet.text.as_str()))
		.map_err(|_| Error::LinkIdentityFailed(ErrorDetail::ParseError))
}

fn get_random_string(length: usize) -> String {
	let mut rng = thread_rng();
	let charset: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
	let random_string: String = (0..length)
		.map(|_| {
			#[cfg(all(not(feature = "std"), feature = "sgx"))]
			let idx = rng.gen_range(0, charset.len());
			#[cfg(all(not(feature = "sgx"), feature = "std"))]
			let idx = rng.gen_range(0..charset.len());

			charset[idx] as char
		})
		.collect();

	random_string
}

pub fn oauth2_authorization(client_id: &str, client_secret: &str) -> String {
	format!("Basic {}", BASE64_STANDARD.encode(format!("{}:{}", client_id, client_secret)))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_get_random_string() {
		let random_string = get_random_string(128);
		assert_eq!(random_string.len(), 128);
	}

	#[test]
	fn test_oauth2_authorization() {
		let client_id = "Z24wcG85SXVJUy1ldE1wdVl3MlA6MTpjaY";
		let client_secret = "lYq3l-sMbGVk94iaze3j8G4ne1MBWAQ8pH4-L58yQ7y4mHOCgp";
		let token = oauth2_authorization(client_id, client_secret);

		assert_eq!(token, "Basic WjI0d2NHODVTWFZKVXkxbGRFMXdkVmwzTWxBNk1UcGphWTpsWXEzbC1zTWJHVms5NGlhemUzajhHNG5lMU1CV0FROHBINC1MNTh5UTd5NG1IT0NncA==".to_string());
	}
}
