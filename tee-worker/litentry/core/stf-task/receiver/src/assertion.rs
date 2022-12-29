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

use crate::{submit_error_extrinsics, StfTaskContext, TaskHandler};
use ita_sgx_runtime::Hash;
use itp_extrinsics_factory::CreateExtrinsics;
use itp_node_api::metadata::{pallet_imp::IMPCallIndexes, provider::AccessNodeMetadata};
use itp_ocall_api::EnclaveOnChainOCallApi;
use itp_sgx_crypto::{ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::{Assertion, IMPError, IdentityWebType, Web2Network};
use log::error;
use std::{format, string::String, sync::Arc};

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::chrono::{offset::Utc as TzUtc, TimeZone};

pub(crate) struct AssertionHandler<
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	O: EnclaveOnChainOCallApi,
	C: CreateExtrinsics,
	M: AccessNodeMetadata,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
> {
	pub(crate) req: AssertionBuildRequest,
	pub(crate) context: Arc<StfTaskContext<K, O, C, M, A, S, H>>,
}

impl<K, O, C, M, A, S, H> TaskHandler for AssertionHandler<K, O, C, M, A, S, H>
where
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	O: EnclaveOnChainOCallApi,
	C: CreateExtrinsics,
	M: AccessNodeMetadata,
	M::MetadataType: IMPCallIndexes,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
	H::StateT: SgxExternalitiesTrait,
{
	type Error = IMPError;
	type Result = ();

	fn on_process(&self) -> Result<Self::Result, Self::Error> {
		match &self.req.assertion {
			Assertion::A1 => lc_assertion_build::a1::build(self.req.vec_identity.clone()),

			Assertion::A2(guild_id, handler) => {
				for identity in &self.req.vec_identity {
					if identity.web_type == IdentityWebType::Web2(Web2Network::Discord) {
						if let Err(e) =
							lc_assertion_build::a2::build(guild_id.clone(), handler.clone())
						{
							error!("error verify assertion2: {:?}", e)
						} else {
							return Ok(())
						}
					}
				}
				Err(IMPError::Assertion2Failed)
			},

			Assertion::A3(guild_id, handler) => {
				for identity in &self.req.vec_identity {
					if identity.web_type == IdentityWebType::Web2(Web2Network::Discord) {
						if let Err(e) =
							lc_assertion_build::a3::build(guild_id.clone(), handler.clone())
						{
							error!("error verify assertion3: {:?}", e)
						} else {
							return Ok(())
						}
					}
				}
				Err(IMPError::Assertion3Failed)
			},

			Assertion::A4(mini_balance, from_date) => {
				let mini_balance: f64 = (mini_balance / (10 ^ 12)) as f64;
				lc_assertion_build::a4::build(
					self.req.vec_identity.clone(),
					String::from_utf8(from_date.clone().into_inner()).unwrap(),
					mini_balance,
				)
			},

			Assertion::A5(twitter_account, original_tweet_id) => lc_assertion_build::a5::build(
				self.req.vec_identity.to_vec(),
				twitter_account.clone(),
				original_tweet_id.clone(),
			),
			Assertion::A6 => lc_assertion_build::a6::build(self.req.vec_identity.to_vec()),

			Assertion::A7(mini_balance, year) => {
				let year = year.clone();
				#[cfg(feature = "std")]
				let dt1 = TzUtc.with_ymd_and_hms(year as i32, 1, 1, 0, 0, 0);
				#[cfg(all(not(feature = "std"), feature = "sgx"))]
				let dt1 = TzUtc.ymd(year as i32, 1, 1).and_hms(0, 0, 0);
				let from_date = format!("{:?}", dt1);
				let mini_balance: f64 = (mini_balance / (10 ^ 12)) as f64;
				lc_assertion_build::a7::build(
					self.req.vec_identity.clone(),
					from_date,
					mini_balance,
				)
			},
			_ => {
				unimplemented!()
			},
		}
	}

	fn on_success(&self, _r: Self::Result) {
		// nothing
	}

	fn on_failure(&self, e: Self::Error) {
		submit_error_extrinsics(
			e,
			self.context.ocall_api.clone(),
			self.context.create_extrinsics.clone(),
			self.context.node_metadata.clone(),
		)
	}
}
