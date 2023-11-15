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
use lc_credentials::{sora::SoraQuizAssertionUpdate, Credential};
use lc_data_providers::{
	discord_litentry::DiscordLitentryClient, DataProviderConfigReader, ReadDataProviderConfig,
};
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::SoraQuizType;

pub fn build(req: &AssertionBuildRequest, qtype: SoraQuizType) -> Result<Credential> {
	let role_id = get_sora_quiz_role_id(&qtype).map_err(|error_detail| {
		Error::RequestVCFailed(Assertion::SoraQuiz(qtype.clone()), error_detail)
	})?;

	let mut has_role_value = false;
	let mut client = DiscordLitentryClient::new();
	for identity in &req.identities {
		if let Identity::Discord(address) = &identity.0 {
			let resp =
				client.has_role(role_id.clone(), address.inner_ref().to_vec()).map_err(|e| {
					Error::RequestVCFailed(
						Assertion::SoraQuiz(qtype.clone()),
						e.into_error_detail(),
					)
				})?;

			debug!("Litentry & SORA Quiz has role response: {:?}", resp);

			if resp.data {
				has_role_value = true;
				break
			}
		}
	}

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.update_sora_quiz_assertion(qtype, has_role_value);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential SoraQuiz failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::SoraQuiz(qtype), e.into_error_detail()))
		},
	}
}

fn get_sora_quiz_role_id(qtype: &SoraQuizType) -> core::result::Result<String, ErrorDetail> {
	let data_provider_config = DataProviderConfigReader::read()?;
	match qtype {
		SoraQuizType::Attendee => Ok(data_provider_config.sora_quiz_attendee_id),
		SoraQuizType::Master => Ok(data_provider_config.sora_quiz_master_id),
	}
}
