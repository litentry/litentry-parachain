// Copyright 2020-2023 Litentry Technologies GmbH.
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

use crate::{
	ensure,
	error::{from_data_provider_error, from_hex_error, Error},
	get_expected_raw_message,
};
use codec::{Decode, Encode};
use itp_sgx_crypto::ShieldingCryptoDecrypt;
use lc_data_providers::{
	discord_official::{DiscordMessage, DiscordOfficialClient},
	twitter_official::{Tweet, TwitterOfficialClient},
	UserInfo,
};
use lc_stf_task_sender::Web2IdentityVerificationRequest;
use litentry_primitives::{
	DiscordValidationData, Identity, TwitterValidationData, Web2ValidationData,
};
use std::{fmt::Debug, string::ToString, vec::Vec};

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct Web2IdentityVerification {
	pub verification_request: Web2IdentityVerificationRequest,
}

pub trait DecryptionVerificationPayload<K: ShieldingCryptoDecrypt> {
	fn decrypt_ciphertext(&self, key: K) -> Result<Vec<u8>, Error>;
}

fn payload_from_tweet(tweet: &Tweet) -> Result<Vec<u8>, Error> {
	hex::decode(tweet.text.strip_prefix("0x").unwrap_or(tweet.text.as_str()))
		.map_err(from_hex_error)
}

fn payload_from_discord(discord: &DiscordMessage) -> Result<Vec<u8>, Error> {
	let data = &discord.content;
	hex::decode(data.strip_prefix("0x").unwrap_or(data.as_str())).map_err(from_hex_error)
}

pub fn verify(request: &Web2IdentityVerificationRequest) -> Result<(), Error> {
	let (user_id, payload) = match request.validation_data {
		Web2ValidationData::Twitter(TwitterValidationData { ref tweet_id }) => {
			let mut client = TwitterOfficialClient::new();
			let tweet: Tweet =
				client.query_tweet(tweet_id.to_vec()).map_err(from_data_provider_error)?;

			let user_id = tweet.get_user_id().ok_or(Error::WrongWeb2Handle)?;

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
				.map_err(from_data_provider_error)?;

			let user = client
				.get_user_info(message.author.id.clone())
				.map_err(from_data_provider_error)?;

			let mut user_id = message.author.username.clone();
			user_id.push_str(&'#'.to_string());
			user_id.push_str(&user.discriminator);

			let payload = payload_from_discord(&message)?;
			Ok((user_id, payload))
		},
	}?;

	// the user_id must match, is it case sensitive?
	let handle = if let Identity::Web2 { ref address, .. } = request.identity {
		std::str::from_utf8(address.as_slice()).map_err(|_| Error::WrongWeb2Handle)
	} else {
		Err(Error::InvalidIdentity)
	}?;

	ensure!(user_id.eq(handle), Error::WrongWeb2Handle);
	// the payload must match
	// TODO: maybe move it to common place
	ensure!(
		payload
			== get_expected_raw_message(&request.who, &request.identity, &request.challenge_code),
		Error::UnexpectedMessage
	);
	Ok(())
}
