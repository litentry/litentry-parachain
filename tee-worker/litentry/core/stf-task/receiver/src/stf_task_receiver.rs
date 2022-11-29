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

use crate::{
	format, AuthorApi, Error, HandleState, Hash, SgxExternalitiesTrait, ShardIdentifier,
	ShieldingCryptoDecrypt, ShieldingCryptoEncrypt, StfEnclaveSigning, StfTaskContext,
};

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::chrono::{offset::Utc as TzUtc, TimeZone};

#[cfg(feature = "std")]
use chrono::{offset::Utc as TzUtc, TimeZone};

use codec::Decode;
use ita_sgx_runtime::IdentityManagement;
use lc_stf_task_sender::{stf_task_sender, RequestType};
use litentry_primitives::{Assertion, IdentityWebType, Web2Network};
use log::*;
use std::string::ToString;

const LIT_TOKEN_ADDRESS: &str = "0xb59490aB09A0f526Cc7305822aC65f2Ab12f9723";
const DOT_TOKEN_ADDRESS: &str = "0xb59490aB09A0f526Cc7305822aC65f2Ab12f9723";

// lifetime elision: StfTaskContext is guaranteed to outlive the fn
pub fn run_stf_task_receiver<K, A, S, H>(context: &StfTaskContext<K, A, S, H>) -> Result<(), Error>
where
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
	H::StateT: SgxExternalitiesTrait,
{
	let receiver = stf_task_sender::init_stf_task_sender_storage()
		.map_err(|e| Error::OtherError(format!("read storage error:{:?}", e)))?;

	// TODO: When an error occurs, send the extrinsic (error message) to the parachain
	// TODO: error handling still incomplete, we only print logs but no error handling
	// TODO: we can further simplify the handling logic
	loop {
		let request_type = receiver
			.recv()
			.map_err(|e| Error::OtherError(format!("receiver error:{:?}", e)))?;

		match request_type {
			RequestType::Web2IdentityVerification(request) =>
				match lc_identity_verification::web2::verify(request.clone()) {
					Err(e) => {
						error!("error verify web2: {:?}", e)
					},
					Ok(_) => {
						context.decode_and_submit_trusted_call(
							request.encoded_shard,
							request.encoded_callback,
						)?;
					},
				},
			RequestType::Web3IdentityVerification(request) =>
				match lc_identity_verification::web3::verify(
					request.who.clone(),
					request.identity.clone(),
					request.challenge_code,
					request.validation_data.clone(),
				) {
					Err(e) => {
						error!("error verify web3: {:?}", e)
					},
					Ok(_) => {
						context.decode_and_submit_trusted_call(
							request.encoded_shard,
							request.encoded_callback,
						)?;
					},
				},
			RequestType::AssertionVerification(request) => match request.assertion {
				Assertion::A1 => {
					if let Err(e) = lc_assertion_build::a1::build(request.vec_identity) {
						error!("error verify assertion1: {:?}", e)
					}
				},
				Assertion::A2(guild_id, handler) => {
					for identity in request.vec_identity {
						if identity.web_type == IdentityWebType::Web2(Web2Network::Discord) {
							if let Err(e) =
								lc_assertion_build::a2::build(guild_id.clone(), handler.clone())
							{
								error!("error verify assertion2: {:?}", e)
							} else {
								// When result is Ok,
								break
							}
						}
					}
				},
				Assertion::A3(guild_id, handler) => {
					for identity in request.vec_identity {
						if identity.web_type == IdentityWebType::Web2(Web2Network::Discord) {
							if let Err(e) =
								lc_assertion_build::a3::build(guild_id.clone(), handler.clone())
							{
								error!("error verify assertion3: {:?}", e)
							} else {
								// When result is Ok,
								break
							}
						}
					}
				},
				Assertion::A4 => {
					let from_date = "2022-10-16T00:00:00Z".to_string();
					#[cfg(feature = "clock")]
					let from_date = format!("{:?}", TzUtc::now());
					let token_address = LIT_TOKEN_ADDRESS.to_string();
					let mini_balance = 0f64;
					if let Err(e) = lc_assertion_build::a4_7_12::build(
						request.vec_identity,
						from_date,
						token_address,
						mini_balance,
					) {
						error!("error verify assertion4: {:?}", e)
					}
				},
				Assertion::A5(twitter_account, original_tweet_id) =>
					match lc_assertion_build::a5::build(
						request.vec_identity.to_vec(),
						twitter_account,
						original_tweet_id,
					) {
						Ok(_) => {},
						Err(e) => {
							log::error!("error verify assertion5: {:?}", e)
						},
					},
				Assertion::A6 => match lc_assertion_build::a6::build(request.vec_identity.to_vec())
				{
					Ok(_) => {},
					Err(e) => {
						log::error!("error verify assertion6: {:?}", e)
					},
				},
				Assertion::A7(mini_balance, year) => {
					#[cfg(feature = "std")]
					let dt1 = TzUtc.with_ymd_and_hms(year as i32, 1, 1, 0, 0, 0);
					#[cfg(all(not(feature = "std"), feature = "sgx"))]
					let dt1 = TzUtc.ymd(year as i32, 1, 1).and_hms(0, 0, 0);
					let from_date = format!("{:?}", dt1);
					let token_address = DOT_TOKEN_ADDRESS.to_string();
					let mini_balance: f64 = (mini_balance / (10 ^ 12)) as f64;
					if let Err(e) = lc_assertion_build::a4_7_12::build(
						request.vec_identity,
						from_date,
						token_address,
						mini_balance,
					) {
						error!("error verify assertion7: {:?}", e)
					}
				},
				Assertion::A12(mini_balance, year) => {
					#[cfg(feature = "std")]
					let dt1 = TzUtc.with_ymd_and_hms(year as i32, 1, 1, 0, 0, 0);
					#[cfg(all(not(feature = "std"), feature = "sgx"))]
					let dt1 = TzUtc.ymd(year as i32, 1, 1).and_hms(0, 0, 0);
					let from_date = format!("{:?}", dt1);
					let token_address = LIT_TOKEN_ADDRESS.to_string();
					let mini_balance: f64 = (mini_balance / (10 ^ 12)) as f64;
					if let Err(e) = lc_assertion_build::a4_7_12::build(
						request.vec_identity,
						from_date,
						token_address,
						mini_balance,
					) {
						error!("error verify assertion7: {:?}", e)
					}
				},
				_ => {
					unimplemented!()
				},
			},
			// only used for testing
			// demonstrate how to read the storage in the stf-task handling with the loaded state
			// in real cases we prefer to read the state ahead and sent the related storage as parameters in `Request`
			RequestType::SetUserShieldingKey(request) => {
				let shard = ShardIdentifier::decode(&mut request.encoded_shard.as_slice())
					.map_err(|e| {
						Error::OtherError(format!("error decoding ShardIdentifier {:?}", e))
					})?;

				let mut state = context
					.state_handler
					.load(&shard)
					.map_err(|e| Error::OtherError(format!("load state failed: {:?}", e)))?;

				let key =
					state.execute_with(|| IdentityManagement::user_shielding_keys(&request.who));

				debug!("in RequestType::SetUserShieldingKey read key is: {:?}", key);

				context.decode_and_submit_trusted_call(
					request.encoded_shard,
					request.encoded_callback,
				)?;
			},
		}
	}
}
