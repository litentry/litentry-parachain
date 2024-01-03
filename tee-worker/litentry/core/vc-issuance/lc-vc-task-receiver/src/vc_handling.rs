#![allow(clippy::result_large_err)]

use crate::{Getter, TrustedCallSigned};
use ita_sgx_runtime::Hash;
pub use ita_stf::aes_encrypt_default;
use itp_ocall_api::EnclaveOnChainOCallApi;
use itp_sgx_crypto::{ShieldingCryptoDecrypt, ShieldingCryptoEncrypt};
use itp_sgx_externalities::SgxExternalitiesTrait;
use itp_stf_executor::traits::StfEnclaveSigning;
use itp_stf_state_handler::handle_state::HandleState;
use itp_top_pool_author::traits::AuthorApi;
use itp_types::H256;
use lc_data_providers::{DataProviderConfigReader, ReadDataProviderConfig};
use lc_stf_task_receiver::StfTaskContext;
use lc_stf_task_sender::AssertionBuildRequest;
use lc_vc_task_sender::VCResponse;
use litentry_primitives::{
	AmountHoldingTimeType, Assertion, ErrorDetail, ErrorString, Identity, ParameterString,
	VCMPError,
};
use sp_core::hashing::blake2_256;
use std::{format, sync::Arc};

pub(crate) struct VCRequestHandler<
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	A: AuthorApi<Hash, Hash, TrustedCallSigned, Getter>,
	S: StfEnclaveSigning<TrustedCallSigned>,
	H: HandleState,
	O: EnclaveOnChainOCallApi,
> {
	pub(crate) req: AssertionBuildRequest,
	pub(crate) context: Arc<StfTaskContext<K, A, S, H, O>>,
}

impl<K, A, S, H, O> VCRequestHandler<K, A, S, H, O>
where
	K: ShieldingCryptoDecrypt + ShieldingCryptoEncrypt + Clone,
	A: AuthorApi<Hash, Hash, TrustedCallSigned, Getter>,
	S: StfEnclaveSigning<TrustedCallSigned>,
	H: HandleState,
	H::StateT: SgxExternalitiesTrait,
	O: EnclaveOnChainOCallApi,
{
	pub fn process(self) -> Result<VCResponse, VCMPError> {
		let mut credential = match self.req.assertion.clone() {
			Assertion::A1 => lc_assertion_build::a1::build(&self.req),

			Assertion::A2(guild_id) => lc_assertion_build::a2::build(&self.req, guild_id),

			Assertion::A3(guild_id, channel_id, role_id) =>
				lc_assertion_build::a3::build(&self.req, guild_id, channel_id, role_id),

			Assertion::A4(min_balance) =>
				build_holding_time(&self.req, AmountHoldingTimeType::LIT, min_balance),

			Assertion::A6 => lc_assertion_build::a6::build(&self.req),

			Assertion::A7(min_balance) =>
				build_holding_time(&self.req, AmountHoldingTimeType::DOT, min_balance),

			// no need to pass `networks` again because it's the same as the `get_supported_web3networks`
			Assertion::A8(_networks) => lc_assertion_build::a8::build(&self.req),

			Assertion::A10(min_balance) =>
				build_holding_time(&self.req, AmountHoldingTimeType::WBTC, min_balance),

			Assertion::A11(min_balance) =>
				build_holding_time(&self.req, AmountHoldingTimeType::ETH, min_balance),

			Assertion::A13(owner) =>
				lc_assertion_build::a13::build(&self.req, self.context.ocall_api.clone(), &owner),

			Assertion::A14 => lc_assertion_build::a14::build(&self.req),

			Assertion::Achainable(param) => lc_assertion_build::achainable::build(&self.req, param),

			Assertion::A20 => lc_assertion_build::a20::build(&self.req),

			Assertion::Oneblock(course_type) =>
				lc_assertion_build::oneblock::course::build(&self.req, course_type),

			Assertion::GenericDiscordRole(role_type) =>
				lc_assertion_build::generic_discord_role::build(&self.req, role_type),

			Assertion::BnbDomainHolding =>
				lc_assertion_build::nodereal::bnb_domain::bnb_domain_holding_amount::build(
					&self.req,
				),

			Assertion::BnbDigitDomainClub(digit_domain_type) =>
				lc_assertion_build::nodereal::bnb_domain::bnb_digit_domain_club_amount::build(
					&self.req,
					digit_domain_type,
				),

			Assertion::VIP3MembershipCard(level) =>
				lc_assertion_build::vip3::card::build(&self.req, level),

			Assertion::WeirdoGhostGangHolder =>
				lc_assertion_build::nodereal::nft_holder::weirdo_ghost_gang_holder::build(&self.req),

			Assertion::LITStaking => lc_assertion_build::lit_staking::build(&self.req),

			Assertion::EVMAmountHolding(token_type) =>
				lc_assertion_build::nodereal::amount_holding::evm_amount_holding::build(
					&self.req, token_type,
				),

			Assertion::BRC20AmountHolder =>
				lc_assertion_build::brc20::amount_holder::build(&self.req),
		}?;

		// post-process the credential
		let signer = self.context.enclave_signer.as_ref();
		let enclave_account = signer.get_enclave_account().map_err(|e| {
			VCMPError::RequestVCFailed(
				self.req.assertion.clone(),
				ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
			)
		})?;

		let data_provider_config = DataProviderConfigReader::read()
			.map_err(|e| VCMPError::RequestVCFailed(self.req.assertion.clone(), e))?;
		credential
			.credential_subject
			.set_endpoint(data_provider_config.credential_endpoint);

		credential.issuer.id =
			Identity::Substrate(enclave_account.into()).to_did().map_err(|e| {
				VCMPError::RequestVCFailed(
					self.req.assertion.clone(),
					ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
				)
			})?;
		let payload = credential.issuer.mrenclave.clone();
		let (enclave_account, sig) = signer.sign_vc_with_self(payload.as_bytes()).map_err(|e| {
			VCMPError::RequestVCFailed(
				self.req.assertion.clone(),
				ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
			)
		})?;

		credential.add_proof(&sig, &enclave_account);
		credential.validate().map_err(|e| {
			VCMPError::RequestVCFailed(
				self.req.assertion.clone(),
				ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
			)
		})?;

		let vc_index: H256 = credential
			.get_index()
			.map_err(|e| {
				VCMPError::RequestVCFailed(
					self.req.assertion.clone(),
					ErrorDetail::StfError(ErrorString::truncate_from(format!("{e:?}").into())),
				)
			})?
			.into();
		let credential_str = credential.to_json().map_err(|_| {
			VCMPError::RequestVCFailed(self.req.assertion.clone(), ErrorDetail::ParseError)
		})?;
		let vc_hash: H256 = blake2_256(credential_str.as_bytes()).into();

		let vc_response = VCResponse {
			assertion_request: self.req.clone(),
			vc_hash,
			vc_payload: credential_str.as_bytes().to_vec(),
			vc_index,
		};

		Ok(vc_response)
	}
}

fn build_holding_time(
	req: &AssertionBuildRequest,
	htype: AmountHoldingTimeType,
	min_balance: ParameterString,
) -> Result<lc_credentials::Credential, VCMPError> {
	lc_assertion_build::holding_time::build(req, htype, min_balance)
}
