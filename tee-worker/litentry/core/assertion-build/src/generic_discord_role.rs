// Copyright 2020-2023 Trust Computing GmbH.
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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::*;
use lc_credentials::{generic_discord_role::GenericDiscordRoleAssertionUpdate, Credential};
use lc_data_providers::{discord_litentry::DiscordLitentryClient, GLOBAL_DATA_PROVIDER_CONFIG};
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::{ContestType, GenericDiscordRoleType, SoraQuizType};

pub fn build(req: &AssertionBuildRequest, rtype: GenericDiscordRoleType) -> Result<Credential> {
	let role_id = get_generic_discord_role_id(&rtype).map_err(|error_detail| {
		Error::RequestVCFailed(Assertion::GenericDiscordRole(rtype.clone()), error_detail)
	})?;

	let mut has_role_value = false;
	let mut client = DiscordLitentryClient::new();
	for identity in &req.identities {
		if let Identity::Discord(address) = &identity.0 {
			let resp = client.has_role(role_id.clone(), address.to_vec()).map_err(|e| {
				Error::RequestVCFailed(
					Assertion::GenericDiscordRole(rtype.clone()),
					e.into_error_detail(),
				)
			})?;

			debug!("Litentry & Discord user has role response: {:?}", resp);

			// data is true if the user has the specified role, otherwise, it is false.
			if resp.data {
				has_role_value = true;
				break
			}
		}
	}

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.update_generic_discord_role_assertion(rtype, has_role_value);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential GenericDiscordRole failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::GenericDiscordRole(rtype), e.into_error_detail()))
		},
	}
}

fn get_generic_discord_role_id(
	rtype: &GenericDiscordRoleType,
) -> core::result::Result<String, ErrorDetail> {
	match rtype {
		GenericDiscordRoleType::Contest(ctype) => match ctype {
			ContestType::Legend => Ok(GLOBAL_DATA_PROVIDER_CONFIG
				.read()
				.unwrap()
				.contest_legend_discord_role_id
				.clone()),
			ContestType::Popularity => Ok(GLOBAL_DATA_PROVIDER_CONFIG
				.read()
				.unwrap()
				.contest_popularity_discord_role_id
				.clone()),
			ContestType::Participant => Ok(GLOBAL_DATA_PROVIDER_CONFIG
				.read()
				.unwrap()
				.contest_participant_discord_role_id
				.clone()),
		},
		GenericDiscordRoleType::SoraQuiz(qtype) => match qtype {
			SoraQuizType::Attendee =>
				Ok(GLOBAL_DATA_PROVIDER_CONFIG.read().unwrap().sora_quiz_attendee_id.clone()),
			SoraQuizType::Master =>
				Ok(GLOBAL_DATA_PROVIDER_CONFIG.read().unwrap().sora_quiz_master_id.clone()),
		},
	}
}

#[cfg(test)]
mod tests {
	use crate::{generic_discord_role::build, AccountId, Arc, AssertionBuildRequest};
	use itp_stf_primitives::types::ShardIdentifier;
	use lc_credentials::assertion_logic::{AssertionLogic, Op};
	use lc_data_providers::GLOBAL_DATA_PROVIDER_CONFIG;
	use lc_mock_server::{default_getter, run};
	use litentry_primitives::{
		Assertion, ContestType, GenericDiscordRoleType, Identity, IdentityNetworkTuple,
		IdentityString, SoraQuizType,
	};
	use log;
	use std::{vec, vec::Vec};

	fn init() {
		let _ = env_logger::builder().is_test(true).try_init();
		let url = run(Arc::new(default_getter), 0).unwrap();
		GLOBAL_DATA_PROVIDER_CONFIG.write().unwrap().set_discord_litentry_url(url);
		GLOBAL_DATA_PROVIDER_CONFIG
			.write()
			.unwrap()
			.set_contest_legend_discord_role_id("1034083718425493544".to_string());
		GLOBAL_DATA_PROVIDER_CONFIG
			.write()
			.unwrap()
			.set_sora_quiz_attendee_id("1034083718425493544".to_string());
	}

	#[test]
	fn build_contest_role_works() {
		init();

		let handler_vec: Vec<u8> = "againstwar".to_string().as_bytes().to_vec();

		let identities: Vec<IdentityNetworkTuple> =
			vec![(Identity::Discord(IdentityString::try_from(handler_vec).unwrap()), vec![])];

		let req: AssertionBuildRequest = AssertionBuildRequest {
			shard: ShardIdentifier::default(),
			signer: AccountId::from([0; 32]),
			enclave_account: AccountId::from([0; 32]),
			who: AccountId::from([0; 32]).into(),
			assertion: Assertion::GenericDiscordRole(GenericDiscordRoleType::Contest(
				ContestType::Legend,
			)),
			identities,
			hash: Default::default(),
		};

		match build(&req, GenericDiscordRoleType::Contest(ContestType::Legend)) {
			Ok(credential) => {
				log::info!("build GenericDiscordRole Contest done");
				assert_eq!(
					*(credential.credential_subject.assertions.first().unwrap()),
					AssertionLogic::Item {
						src: String::from("$is_contest_legend"),
						op: Op::Equal,
						dst: String::from("true")
					}
				);
				assert_eq!(*(credential.credential_subject.values.first().unwrap()), true);
			},
			Err(e) => {
				panic!("build GenericDiscordRole Contest failed with error {:?}", e);
			},
		}
	}

	#[test]
	fn build_sora_quiz_role_works() {
		init();

		let handler_vec: Vec<u8> = "ericzhang.eth".to_string().as_bytes().to_vec();

		let identities: Vec<IdentityNetworkTuple> =
			vec![(Identity::Discord(IdentityString::try_from(handler_vec).unwrap()), vec![])];

		let req: AssertionBuildRequest = AssertionBuildRequest {
			shard: ShardIdentifier::default(),
			signer: AccountId::from([0; 32]),
			enclave_account: AccountId::from([0; 32]),
			who: AccountId::from([0; 32]).into(),
			assertion: Assertion::GenericDiscordRole(GenericDiscordRoleType::SoraQuiz(
				SoraQuizType::Attendee,
			)),
			identities,
			hash: Default::default(),
		};

		match build(&req, GenericDiscordRoleType::SoraQuiz(SoraQuizType::Attendee)) {
			Ok(credential) => {
				log::info!("build GenericDiscordRole SoraQuiz done");
				assert_eq!(*(credential.credential_subject.values.first().unwrap()), false);
			},
			Err(e) => {
				panic!("build GenericDiscordRole SoraQuiz failed with error {:?}", e);
			},
		}
	}
}
