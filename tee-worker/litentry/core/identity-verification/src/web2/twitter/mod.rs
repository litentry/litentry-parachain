mod code_verifier_store;
pub use code_verifier_store::*;

mod helpers;
use helpers::{get_code_verifier, get_state_verifier};

use base64::{encode_config, URL_SAFE_NO_PAD};
use sp_core::hashing::sha2_256;
use std::{format, string::String};

#[derive(Debug)]
pub struct AuthorizeData {
	pub authorize_url: String,
	pub code_verifier: String,
}

pub fn get_authorize_data(client_id: &str, redirect_uri: &str) -> AuthorizeData {
	let code_verifier = get_code_verifier();
	let code_verifier_hash = sha2_256(code_verifier.as_bytes());
	let code_challenge = encode_config(code_verifier_hash, URL_SAFE_NO_PAD);
	let state = get_state_verifier();

	let authorize_url = format!(
		"https://twitter.com/i/oauth2/authorize?response_type=code&client_id={}&redirect_uri={}&scope=tweet.read%20users.read%20follows.read%20follows.write&state={}&code_challenge={}&code_challenge_method=S256",
		client_id,
		redirect_uri,
		state,
		code_challenge
	);

	AuthorizeData { authorize_url, code_verifier }
}
