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

use crate::{handler::TaskHandler, StfTaskContext};
use codec::{Decode, Encode};
use ita_sgx_runtime::Hash;
use itp_extrinsics_factory::CreateExtrinsics;
use itp_node_api::metadata::{
	pallet_imp::IMPCallIndexes, pallet_vcmp::VCMPCallIndexes, provider::AccessNodeMetadata,
};
use itp_ocall_api::EnclaveOnChainOCallApi;
use itp_sgx_crypto::{ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::{AccountId, OpaqueCall};
use lc_credentials_tee::credentials::Credential;
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::Assertion;
use log::*;
use parachain_core_primitives::VCMPError;
use std::{boxed::Box, format, string::String, sync::Arc, vec::Vec};

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::chrono::{offset::Utc as TzUtc, TimeZone};

#[cfg(feature = "std")]
use chrono::{offset::Utc as TzUtc, TimeZone};

/// Copyed from tee-worker/app-libs/stf/src/trusted_call.rs.
/// As the `credential_unsigned` needs to be unpdated from TrustedCall::build_assertion_preflight()
/// to TrustedCall::build_assertion_runtime(),
/// the TrustedCall has to be re-encoded here.
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TrustedCallFork {
	balance_set_balance(),
	balance_transfer(),
	balance_unshield(), // (AccountIncognito, BeneficiaryPublicAccount, Amount, Shard)
	balance_shield(),   // (Root, AccountIncognito, Amount)
	#[cfg(feature = "evm")]
	evm_withdraw(), // (Origin, Address EVM Account, Value)
	// (Origin, Source, Target, Input, Value, Gas limit, Max fee per gas, Max priority fee per gas, Nonce, Access list)
	#[cfg(feature = "evm")]
	evm_call(),
	// (Origin, Source, Init, Value, Gas limit, Max fee per gas, Max priority fee per gas, Nonce, Access list)
	#[cfg(feature = "evm")]
	evm_create(),
	// (Origin, Source, Init, Salt, Value, Gas limit, Max fee per gas, Max priority fee per gas, Nonce, Access list)
	#[cfg(feature = "evm")]
	evm_create2(),
	// litentry
	set_user_shielding_key_preflight(), // (Root, AccountIncognito, Key) -- root as signer, only for testing
	set_user_shielding_key_runtime(),   // (EnclaveSigner, AccountIncognito, Key)
	create_identity_runtime(),          // (EnclaveSigner, Account, identity, metadata, blocknumber)
	remove_identity_runtime(),          // (EnclaveSigner, Account, identity)
	verify_identity_preflight(),        // (EnclaveSigner, Account, identity, validation, blocknumber)
	verify_identity_runtime(),          // (EnclaveSigner, Account, identity, blocknumber)
	build_assertion_preflight(),        // (Account, Account, Assertion, Shard)
	build_assertion_runtime(AccountId, AccountId, Box<Credential>), // (Account, Account, Box<Credential>)
	set_challenge_code_runtime(),                                   // only for testing
}

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
	M::MetadataType: IMPCallIndexes + VCMPCallIndexes,
	A: AuthorApi<Hash, Hash>,
	S: StfEnclaveSigning,
	H: HandleState,
	H::StateT: SgxExternalitiesTrait,
{
	type Error = VCMPError;
	type Result = Option<(Vec<u8>, Vec<u8>)>;

	fn on_process(&self) -> Result<Self::Result, Self::Error> {
		match self.req.assertion.clone() {
			Assertion::A1 => lc_assertion_build::a1::build(
				self.req.vec_identity.clone(),
				self.req.credential.clone(),
			)
			.map(|credential| {
				let encoded_callback = TrustedCallFork::build_assertion_runtime(
					self.context.enclave_signer.get_enclave_account().unwrap(),
					self.req.who.clone(),
					Box::new(credential),
				)
				.encode();
				Some((self.req.encoded_shard.clone(), encoded_callback))
			}),

			Assertion::A2(guild_id, handler) =>
				lc_assertion_build::a2::build(self.req.vec_identity.to_vec(), guild_id, handler)
					.map(|_| None),

			Assertion::A3(guild_id, handler) =>
				lc_assertion_build::a3::build(self.req.vec_identity.to_vec(), guild_id, handler)
					.map(|_| None),

			Assertion::A4(min_balance, from_date) => {
				let min_balance: f64 = (min_balance / (10 ^ 12)) as f64;
				lc_assertion_build::a4::build(
					self.req.vec_identity.to_vec(),
					String::from_utf8(from_date.into_inner()).unwrap(),
					min_balance,
				)
				.map(|_| None)
			},

			Assertion::A5(twitter_account, original_tweet_id) => lc_assertion_build::a5::build(
				self.req.vec_identity.to_vec(),
				twitter_account,
				original_tweet_id,
			)
			.map(|_| None),

			Assertion::A6 =>
				lc_assertion_build::a6::build(self.req.vec_identity.to_vec()).map(|_| None),

			Assertion::A7(min_balance, year) => {
				let min_balance: f64 = (min_balance / (10 ^ 12)) as f64;
				lc_assertion_build::a7::build(
					self.req.vec_identity.to_vec(),
					year_to_date(year),
					min_balance,
				)
				.map(|_| None)
			},

			Assertion::A8 =>
				lc_assertion_build::a8::build(self.req.vec_identity.to_vec()).map(|_| None),

			Assertion::A10(min_balance, year) => {
				// WBTC decimals is 8.
				let min_balance: f64 = (min_balance / (10 ^ 8)) as f64;
				lc_assertion_build::a10::build(
					self.req.vec_identity.to_vec(),
					year_to_date(year),
					min_balance,
				)
				.map(|_| None)
			},
			_ => {
				unimplemented!()
			},
		}
	}

	fn on_success(&self, result: Self::Result) {
		let (shard, callback) = result.unwrap();
		match self.context.decode_and_submit_trusted_call(shard, callback) {
			Ok(_) => {},
			Err(e) => {
				error!("decode_and_submit_trusted_call failed. Due to: {:?}", e);
			},
		}
	}

	fn on_failure(&self, error: Self::Error) {
		match self
			.context
			.node_metadata
			.get_from_metadata(|m| VCMPCallIndexes::vc_some_error_call_indexes(m))
		{
			Ok(Ok(call_index)) => {
				let call = OpaqueCall::from_tuple(&(call_index, error));
				self.context.submit_to_parentchain(call)
			},
			Ok(Err(e)) => {
				error!("failed to get metadata. Due to: {:?}", e);
			},
			Err(e) => {
				error!("failed to get metadata. Due to: {:?}", e);
			},
		};
	}
}

fn year_to_date(year: u32) -> String {
	#[cfg(feature = "std")]
	let dt1 = TzUtc.with_ymd_and_hms(year as i32, 1, 1, 0, 0, 0);
	#[cfg(all(not(feature = "std"), feature = "sgx"))]
	let dt1 = TzUtc.ymd(year as i32, 1, 1).and_hms(0, 0, 0);
	format!("{:?}", dt1)
}
