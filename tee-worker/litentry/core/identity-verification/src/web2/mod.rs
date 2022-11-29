// Copyright 2020-2022 Litentry Technologies GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

use crate::{ensure, get_expected_payload};
use codec::{Decode, Encode};
use std::{
	fmt::Debug,
	format,
	string::{String, ToString},
	vec::Vec,
};

use itp_sgx_crypto::ShieldingCryptoDecrypt;
use lc_data_providers::{
	discord_official::{DiscordMessage, DiscordOfficialClient},
	twitter_official::{Tweet, TwitterOfficialClient},
	UserInfo,
};
use lc_stf_task_sender::Web2IdentityVerificationRequest;
use litentry_primitives::{
	DiscordValidationData, IdentityHandle, TwitterValidationData, Web2ValidationData,
};

// TODO: maybe split this file into smaller mods
#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
	#[error("Request error: {0}")]
	RequestError(String),

	#[error("Other error: {0}")]
	OtherError(String),
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct Web2IdentityVerification {
	pub verification_request: Web2IdentityVerificationRequest,
}

pub trait DecryptionVerificationPayload<K: ShieldingCryptoDecrypt> {
	fn decrypt_ciphertext(&self, key: K) -> Result<Vec<u8>, Error>;
}

fn payload_from_tweet(tweet: &Tweet) -> Result<Vec<u8>, Error> {
	if tweet.text.starts_with("0x") {
		let bytes = &tweet.text.as_bytes()[b"0x".len()..];
		hex::decode(bytes).map_err(|e| Error::OtherError(format!("Hex error: {:?}", e)))
	} else {
		hex::decode(tweet.text.as_bytes())
			.map_err(|e| Error::OtherError(format!("Hex error: {:?}", e)))
	}
}

fn payload_from_discord(discord: &DiscordMessage) -> Result<Vec<u8>, Error> {
	let data = &discord.content;
	if data.starts_with("0x") {
		let bytes = &data.as_bytes()[b"0x".len()..];
		hex::decode(bytes).map_err(|e| Error::OtherError(format!("Hex error: {:?}", e)))
	} else {
		hex::decode(data.as_bytes()).map_err(|e| Error::OtherError(format!("Hex error: {:?}", e)))
	}
}

pub fn verify(request: Web2IdentityVerificationRequest) -> Result<(), Error> {
	let (user_id, payload) = match request.validation_data {
		Web2ValidationData::Twitter(TwitterValidationData { ref tweet_id }) => {
			let mut client = TwitterOfficialClient::new();
			let tweet: Tweet = client
				.query_tweet(tweet_id.to_vec())
				.map_err(|e| Error::RequestError(format!("{:?}", e)))?;

			let user_id = tweet
				.get_user_id()
				.ok_or_else(|| Error::OtherError("can not find user_id".to_string()))?;

			let payload = payload_from_tweet(&tweet)?;

			Ok((user_id, payload))
		},
		Web2ValidationData::Discord(DiscordValidationData {
			ref channel_id,
			ref message_id,
			..
		}) => {
			let mut client = DiscordOfficialClient::new();
			let message: DiscordMessage = client
				.query_message(channel_id.to_vec(), message_id.to_vec())
				.map_err(|e| Error::RequestError(format!("{:?}", e)))?;

			let user_id = message
				.get_user_id()
				.ok_or_else(|| Error::OtherError("can not find user_id".to_string()))?;

			let payload = payload_from_discord(&message)?;

			Ok((user_id, payload))
		},
	}?;

	// the user_id must match, is it case sensitive?
	match request.identity.handle {
		IdentityHandle::String(ref handle) => {
			let handle = std::str::from_utf8(handle.as_slice())
				.map_err(|_| Error::OtherError("convert IdentityHandle error".to_string()))?;
			if !user_id.eq(handle) {
				return Err(Error::OtherError("user_id not match".to_string()))
			}
		},
		_ => return Err(Error::OtherError("IdentityHandle not support".to_string())),
	}

	// the payload must match
	// TODO: maybe move it to common place
	ensure!(
		payload == get_expected_payload(&request.who, &request.identity, &request.challenge_code),
		Error::OtherError("payload not match".to_string())
	);
	Ok(())
}
