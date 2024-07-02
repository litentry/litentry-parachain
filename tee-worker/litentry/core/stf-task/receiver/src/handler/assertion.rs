// Copyright 2020-2024 Trust Computing GmbH.
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

#![allow(clippy::result_large_err)]

use crate::{handler::TaskHandler, EnclaveOnChainOCallApi, StfTaskContext, TrustedCall, H256};
use ita_sgx_runtime::Hash;
use ita_stf::{Getter, TrustedCallSigned};
use itp_sgx_crypto::{key_repository::AccessKey, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::ShardIdentifier;
use itp_utils::stringify::account_id_to_string;
use lc_credentials::credential_schema;
use lc_data_providers::DataProviderConfig;
use lc_dynamic_assertion::AssertionLogicRepository;
use lc_evm_dynamic_assertions::AssertionRepositoryItem;
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::{
	AmountHoldingTimeType, Assertion, ErrorDetail, ErrorString, Identity, ParameterString,
	VCMPError,
};
use log::*;
use sp_core::{Pair, H160};
use std::{
	format,
	iter::once,
	string::{String, ToString},
	sync::Arc,
	vec::Vec,
};

pub(crate) struct AssertionHandler<
	ShieldingKeyRepository,
	A: AuthorApi<Hash, Hash, TrustedCallSigned, Getter>,
	S: StfEnclaveSigning<TrustedCallSigned>,
	H: HandleState,
	O: EnclaveOnChainOCallApi,
	AR: AssertionLogicRepository<Id = H160, Item = AssertionRepositoryItem>,
> where
	ShieldingKeyRepository: AccessKey,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoEncrypt + 'static,
{
	pub(crate) req: AssertionBuildRequest,
	pub(crate) context: Arc<StfTaskContext<ShieldingKeyRepository, A, S, H, O, AR>>,
}

impl<ShieldingKeyRepository, A, S, H, O, AR> TaskHandler
	for AssertionHandler<ShieldingKeyRepository, A, S, H, O, AR>
where
	ShieldingKeyRepository: AccessKey,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoEncrypt + 'static,
	A: AuthorApi<Hash, Hash, TrustedCallSigned, Getter>,
	S: StfEnclaveSigning<TrustedCallSigned>,
	H: HandleState,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi,
	AR: AssertionLogicRepository<Id = H160, Item = AssertionRepositoryItem>,
{
	type Error = VCMPError;
	type Result = (Vec<u8>, Option<Vec<u8>>); // (vc_byte_array, optional vc_log_byte_array)

	fn on_process(&self) -> Result<Self::Result, Self::Error> {
		// create the initial credential
		// TODO: maybe we can further simplify this
		create_credential_str(&self.req, &self.context)
	}

	fn on_success(
		&self,
		result: Self::Result,
		sender: std::sync::mpsc::Sender<(ShardIdentifier, H256, TrustedCall)>,
	) {
		debug!("Assertion build OK");
		// we shouldn't have the maximum text length limit in normal RSA3072 encryption, as the payload
		// using enclave's shielding key is encrypted in chunks
		let vc_payload = result.0;
		if let Ok(enclave_signer_account) = self.context.enclave_signer.get_enclave_account() {
			let c = TrustedCall::request_vc_callback(
				enclave_signer_account.into(),
				self.req.who.clone(),
				self.req.assertion.clone(),
				vc_payload,
				self.req.maybe_key,
				self.req.should_create_id_graph,
				self.req.req_ext_hash,
			);
			if let Err(e) = sender.send((self.req.shard, self.req.top_hash, c)) {
				error!("Unable to send message to the trusted_call_receiver: {:?}", e);
			}
		} else {
			error!("can't get enclave signer");
		}
	}

	fn on_failure(
		&self,
		error: Self::Error,
		sender: std::sync::mpsc::Sender<(ShardIdentifier, H256, TrustedCall)>,
	) {
		error!("Assertion build error: {error:?}");
		if let Ok(enclave_signer_account) = self.context.enclave_signer.get_enclave_account() {
			let c = TrustedCall::handle_vcmp_error(
				enclave_signer_account.into(),
				Some(self.req.who.clone()),
				error,
				self.req.req_ext_hash,
			);
			if let Err(e) = sender.send((self.req.shard, self.req.top_hash, c)) {
				error!("Unable to send message to the trusted_call_receiver: {:?}", e);
			}
		} else {
			error!("can't get enclave signer");
		}
	}
}

fn build_holding_time(
	req: &AssertionBuildRequest,
	htype: AmountHoldingTimeType,
	min_balance: ParameterString,
	data_provider_config: &DataProviderConfig,
) -> Result<lc_credentials::Credential, VCMPError> {
	lc_assertion_build::holding_time::build(req, htype, min_balance, data_provider_config)
}

pub fn create_credential_str<
	ShieldingKeyRepository,
	A: AuthorApi<Hash, Hash, TrustedCallSigned, Getter>,
	S: StfEnclaveSigning<TrustedCallSigned>,
	H: HandleState,
	O: EnclaveOnChainOCallApi,
	AR: AssertionLogicRepository<Id = H160, Item = AssertionRepositoryItem>,
>(
	req: &AssertionBuildRequest,
	context: &Arc<StfTaskContext<ShieldingKeyRepository, A, S, H, O, AR>>,
) -> Result<(Vec<u8>, Option<Vec<u8>>), VCMPError>
where
	ShieldingKeyRepository: AccessKey,
	<ShieldingKeyRepository as AccessKey>::KeyType: ShieldingCryptoEncrypt + 'static,
{
	let mut vc_logs: Option<Vec<String>> = None;
	let mut credential = match req.assertion.clone() {
		Assertion::A1 => {
			#[cfg(test)]
			{
				std::thread::sleep(core::time::Duration::from_secs(5));
			}
			lc_assertion_build::a1::build(req)
		},
		Assertion::A2(guild_id) =>
			lc_assertion_build::a2::build(req, guild_id, &context.data_provider_config),

		Assertion::A3(guild_id, channel_id, role_id) => lc_assertion_build::a3::build(
			req,
			guild_id,
			channel_id,
			role_id,
			&context.data_provider_config,
		),

		Assertion::A4(min_balance) => build_holding_time(
			req,
			AmountHoldingTimeType::LIT,
			min_balance,
			&context.data_provider_config,
		),

		Assertion::A6 => lc_assertion_build::a6::build(req, &context.data_provider_config),

		Assertion::A7(min_balance) => build_holding_time(
			req,
			AmountHoldingTimeType::DOT,
			min_balance,
			&context.data_provider_config,
		),

		// no need to pass `networks` again because it's the same as the `get_supported_web3networks`
		Assertion::A8(_networks) =>
			lc_assertion_build::a8::build(req, &context.data_provider_config),

		Assertion::A10(min_balance) => build_holding_time(
			req,
			AmountHoldingTimeType::WBTC,
			min_balance,
			&context.data_provider_config,
		),

		Assertion::A11(min_balance) => build_holding_time(
			req,
			AmountHoldingTimeType::ETH,
			min_balance,
			&context.data_provider_config,
		),

		Assertion::A13(owner) =>
			lc_assertion_build::a13::build(req, context.ocall_api.clone(), &owner),

		Assertion::A14 => lc_assertion_build::a14::build(req, &context.data_provider_config),

		Assertion::Achainable(param) =>
			lc_assertion_build::achainable::build(req, param, &context.data_provider_config),

		Assertion::A20 => lc_assertion_build::a20::build(req, &context.data_provider_config),

		Assertion::OneBlock(course_type) => lc_assertion_build::oneblock::course::build(
			req,
			course_type,
			&context.data_provider_config,
		),

		Assertion::GenericDiscordRole(role_type) =>
			lc_assertion_build::generic_discord_role::build(
				req,
				role_type,
				&context.data_provider_config,
			),

		Assertion::BnbDomainHolding =>
			lc_assertion_build::nodereal::bnb_domain::bnb_domain_holding_amount::build(
				req,
				&context.data_provider_config,
			),

		Assertion::BnbDigitDomainClub(digit_domain_type) =>
			lc_assertion_build::nodereal::bnb_domain::bnb_digit_domain_club_amount::build(
				req,
				digit_domain_type,
				&context.data_provider_config,
			),

		Assertion::VIP3MembershipCard(level) =>
			lc_assertion_build::vip3::card::build(req, level, &context.data_provider_config),

		Assertion::WeirdoGhostGangHolder =>
			lc_assertion_build::nodereal::nft_holder::weirdo_ghost_gang_holder::build(
				req,
				&context.data_provider_config,
			),

		Assertion::LITStaking => lc_assertion_build::lit_staking::build(req),

		Assertion::EVMAmountHolding(token_type) =>
			lc_assertion_build::nodereal::amount_holding::evm_amount_holding::build(
				req,
				token_type,
				&context.data_provider_config,
			),

		Assertion::BRC20AmountHolder =>
			lc_assertion_build::brc20::amount_holder::build(req, &context.data_provider_config),

		Assertion::CryptoSummary =>
			lc_assertion_build::nodereal::crypto_summary::build(req, &context.data_provider_config),

		Assertion::TokenHoldingAmount(token_type) =>
			lc_assertion_build_v2::token_holding_amount::build(
				req,
				token_type,
				&context.data_provider_config,
			),

		Assertion::PlatformUser(platform_user_type) => lc_assertion_build_v2::platform_user::build(
			req,
			platform_user_type,
			&context.data_provider_config,
		),

		Assertion::NftHolder(nft_type) =>
			lc_assertion_build_v2::nft_holder::build(req, nft_type, &context.data_provider_config),

		Assertion::Dynamic(params) => {
			let result = lc_assertion_build::dynamic::build(
				req,
				params,
				context.assertion_repository.clone(),
			)?;
			vc_logs = Some(result.1);
			Ok(result.0)
		},
	}?;

	// post-process the credential
	let enclave_signer_account = context.enclave_signer.get_enclave_account().map_err(|e| {
		VCMPError::RequestVCFailed(
			req.assertion.clone(),
			ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
		)
	})?;

	credential.parachain_block_number = req.parachain_block_number;
	credential.sidechain_block_number = req.sidechain_block_number;

	credential.credential_subject.endpoint =
		context.data_provider_config.credential_endpoint.to_string();

	if let Some(schema) = credential_schema::get_schema_url(&req.assertion) {
		credential.credential_schema.id = schema;
	}

	let issuer_identity: Identity = context.enclave_account.public().into();
	credential.issuer.id = issuer_identity.to_did().map_err(|e| {
		VCMPError::RequestVCFailed(
			req.assertion.clone(),
			ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
		)
	})?;

	let json_string = credential
		.to_json()
		.map_err(|_| VCMPError::RequestVCFailed(req.assertion.clone(), ErrorDetail::ParseError))?;
	let payload = json_string.as_bytes();
	let sig = context.enclave_signer.sign(payload).map_err(|e| {
		VCMPError::RequestVCFailed(
			req.assertion.clone(),
			ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
		)
	})?;

	credential.add_proof(&sig, account_id_to_string(&enclave_signer_account));
	credential.validate().map_err(|e| {
		VCMPError::RequestVCFailed(
			req.assertion.clone(),
			ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
		)
	})?;

	let credential_str = credential
		.to_json()
		.map_err(|_| VCMPError::RequestVCFailed(req.assertion.clone(), ErrorDetail::ParseError))?;
	debug!("Credential: {}, length: {}", credential_str, credential_str.len());

	Ok((
		credential_str.as_bytes().to_vec(),
		vc_logs.map(|v| {
			v.iter().flat_map(|s| s.as_bytes().iter().cloned().chain(once(b'\n'))).collect()
		}),
	))
}
