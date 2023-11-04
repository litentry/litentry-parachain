/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

#[cfg(feature = "evm")]
use sp_core::{H160, U256};

#[cfg(feature = "evm")]
use std::vec::Vec;

use crate::{
	helpers::{enclave_signer_account, ensure_enclave_signer_account, ensure_self},
	trusted_call_result::*,
	Runtime, StfError, System, TrustedOperation,
};
use codec::{Decode, Encode};
use frame_support::{ensure, traits::UnfilteredDispatchable};
pub use ita_sgx_runtime::{Balance, ConvertAccountId, Index, SgxParentchainTypeConverter};
pub use itp_node_api::metadata::{
	pallet_imp::IMPCallIndexes, pallet_system::SystemSs58Prefix, pallet_teerex::TeerexCallIndexes,
	pallet_vcmp::VCMPCallIndexes, provider::AccessNodeMetadata,
};
use itp_stf_interface::ExecuteCall;
use itp_stf_primitives::types::{AccountId, KeyPair, ShardIdentifier};
pub use itp_types::{OpaqueCall, H256};
use itp_utils::stringify::account_id_to_string;
pub use litentry_primitives::{
	aes_encrypt_default, all_evm_web3networks, all_substrate_web3networks, AesOutput, Assertion,
	ErrorDetail, IMPError, Identity, LitentryMultiSignature, ParentchainAccountId,
	ParentchainBlockNumber, UserShieldingKeyNonceType, UserShieldingKeyType, VCMPError,
	ValidationData, Web3Network,
};
use log::*;
use sp_core::crypto::AccountId32;
use sp_io::hashing::blake2_256;
use sp_runtime::MultiAddress;
use std::{format, prelude::v1::*, sync::Arc};

#[cfg(feature = "evm")]
use ita_sgx_runtime::{AddressMapping, HashedAddressMapping};
use itp_node_api::metadata::NodeMetadataTrait;

#[cfg(feature = "evm")]
use crate::evm_helpers::{create_code_hash, evm_create2_address, evm_create_address};

// max number of identities in an id_graph that will be returned as the extrinsic parameter
// this has no effect on the stored id_graph, but only the returned id_graph
pub const RETURNED_IDGRAPH_MAX_LEN: usize = 20;

pub type IMTCall = ita_sgx_runtime::IdentityManagementCall<Runtime>;
pub type IMT = ita_sgx_runtime::pallet_imt::Pallet<Runtime>;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TrustedCall {
	balance_set_balance(Identity, AccountId, Balance, Balance),
	balance_transfer(Identity, AccountId, Balance),
	balance_unshield(Identity, AccountId, Balance, ShardIdentifier), // (AccountIncognito, BeneficiaryPublicAccount, Amount, Shard)
	balance_shield(Identity, AccountId, Balance),                    // (Root, AccountIncognito, Amount)
	#[cfg(feature = "evm")]
	evm_withdraw(Identity, H160, Balance),  // (Origin, Address EVM Account, Value)
	// (Origin, Source, Target, Input, Value, Gas limit, Max fee per gas, Max priority fee per gas, Nonce, Access list)
	#[cfg(feature = "evm")]
	evm_call(
		Identity,
		H160,
		H160,
		Vec<u8>,
		U256,
		u64,
		U256,
		Option<U256>,
		Option<U256>,
		Vec<(H160, Vec<H256>)>,
	),
	// (Origin, Source, Init, Value, Gas limit, Max fee per gas, Max priority fee per gas, Nonce, Access list)
	#[cfg(feature = "evm")]
	evm_create(
		Identity,
		H160,
		Vec<u8>,
		U256,
		u64,
		U256,
		Option<U256>,
		Option<U256>,
		Vec<(H160, Vec<H256>)>,
	),
	// (Origin, Source, Init, Salt, Value, Gas limit, Max fee per gas, Max priority fee per gas, Nonce, Access list)
	#[cfg(feature = "evm")]
	evm_create2(
		Identity,
		H160,
		Vec<u8>,
		H256,
		U256,
		u64,
		U256,
		Option<U256>,
		Option<U256>,
		Vec<(H160, Vec<H256>)>,
	),
	/// litentry trusted calls
	/// the calls that should deliver a result other than `Empty` will need to include the parameter: `Option<UserShieldingKeyType>`,
	/// it's a 32-byte AES key defined by the client. This key will be used to encrypt the user-sensitive result in the DI response,
	/// see `trusted_call_result.rs`.
	///
	/// It's an Option because for II call there's no need to define such a key.
	///
	/// Theoretically, this key **could** be different from what is used to encrypt the `AesRequest` payload, but in practice it's fine
	/// to simply use the same key.
	///
	/// Please note this key needs to be embeded in the trusted call itself because:
	/// - it needs to be passed around in async handling of trusted call
	/// - for multi-worker setup, the worker that processes the request can be differnet from the worker that receives the request, so
	///   we can't maintain something like a global mapping between trusted call and aes-key, which only resides in the memory of one worker.
	link_identity(
		Identity,
		Identity,
		Identity,
		ValidationData,
		Vec<Web3Network>,
		UserShieldingKeyNonceType,
		Option<UserShieldingKeyType>,
		H256,
	),
	deactivate_identity(Identity, Identity, Identity, H256),
	activate_identity(Identity, Identity, Identity, H256),
	request_vc(Identity, Identity, Assertion, Option<UserShieldingKeyType>, H256),
	set_identity_networks(Identity, Identity, Identity, Vec<Web3Network>, H256),

	// the following trusted calls should not be requested directly from external
	// they are guarded by the signature check (either root or enclave_signer_account)
	link_identity_callback(
		Identity,
		Identity,
		Identity,
		Vec<Web3Network>,
		Option<UserShieldingKeyType>,
		H256,
	),
	request_vc_callback(
		Identity,
		Identity,
		Assertion,
		H256,
		H256,
		Vec<u8>,
		Option<UserShieldingKeyType>,
		H256,
	),
	handle_imp_error(Identity, Option<Identity>, IMPError, H256),
	handle_vcmp_error(Identity, Option<Identity>, VCMPError, H256),
	send_erroneous_parentchain_call(Identity),
}

impl TrustedCall {
	pub fn sender_identity(&self) -> &Identity {
		match self {
			TrustedCall::balance_set_balance(sender_identity, ..) => sender_identity,
			TrustedCall::balance_transfer(sender_identity, ..) => sender_identity,
			TrustedCall::balance_unshield(sender_identity, ..) => sender_identity,
			TrustedCall::balance_shield(sender_identity, ..) => sender_identity,
			#[cfg(feature = "evm")]
			TrustedCall::evm_withdraw(sender_identity, ..) => sender_identity,
			#[cfg(feature = "evm")]
			TrustedCall::evm_call(sender_identity, ..) => sender_identity,
			#[cfg(feature = "evm")]
			TrustedCall::evm_create(sender_identity, ..) => sender_identity,
			#[cfg(feature = "evm")]
			TrustedCall::evm_create2(sender_identity, ..) => sender_identity,
			// litentry
			TrustedCall::link_identity(sender_identity, ..) => sender_identity,
			TrustedCall::deactivate_identity(sender_identity, ..) => sender_identity,
			TrustedCall::activate_identity(sender_identity, ..) => sender_identity,
			TrustedCall::request_vc(sender_identity, ..) => sender_identity,
			TrustedCall::set_identity_networks(sender_identity, ..) => sender_identity,
			TrustedCall::link_identity_callback(sender_identity, ..) => sender_identity,
			TrustedCall::request_vc_callback(sender_identity, ..) => sender_identity,
			TrustedCall::handle_imp_error(sender_identity, ..) => sender_identity,
			TrustedCall::handle_vcmp_error(sender_identity, ..) => sender_identity,
			TrustedCall::send_erroneous_parentchain_call(sender_identity) => sender_identity,
		}
	}

	pub fn sign(
		&self,
		pair: &KeyPair,
		nonce: Index,
		mrenclave: &[u8; 32],
		shard: &ShardIdentifier,
	) -> TrustedCallSigned {
		let mut payload = self.encode();
		payload.append(&mut nonce.encode());
		payload.append(&mut mrenclave.encode());
		payload.append(&mut shard.encode());

		TrustedCallSigned { call: self.clone(), nonce, signature: pair.sign(payload.as_slice()) }
	}
}

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
pub struct TrustedCallSigned {
	pub call: TrustedCall,
	pub nonce: Index,
	pub signature: LitentryMultiSignature,
}

impl TrustedCallSigned {
	pub fn new(call: TrustedCall, nonce: Index, signature: LitentryMultiSignature) -> Self {
		TrustedCallSigned { call, nonce, signature }
	}

	pub fn verify_signature(&self, mrenclave: &[u8; 32], shard: &ShardIdentifier) -> bool {
		let mut payload = self.call.encode();
		payload.append(&mut self.nonce.encode());
		payload.append(&mut mrenclave.encode());
		payload.append(&mut shard.encode());

		self.signature.verify(payload.as_slice(), self.call.sender_identity())
	}

	pub fn into_trusted_operation(self, direct: bool) -> TrustedOperation {
		match direct {
			true => TrustedOperation::direct_call(self),
			false => TrustedOperation::indirect_call(self),
		}
	}
}

impl<NodeMetadataRepository> ExecuteCall<NodeMetadataRepository> for TrustedCallSigned
where
	NodeMetadataRepository: AccessNodeMetadata,
	NodeMetadataRepository::MetadataType: NodeMetadataTrait,
{
	type Error = StfError;
	type Result = TrustedCallResult;

	// TODO(Kai@litentry):
	// If this function returns Err(), it will feed the executor with Ok(ExecutedOperation::failed()),
	// which will remove the failed op from its **own** top pool while preventing it from being included
	// in a sidechain block - see `execute_trusted_call_on_stf`.
	//
	// As a result, when other workers import sidechain blocks, they will treat the op as
	// "not yet executed" (before it's not recorded in the sidechain block) and try to execute it again from
	// its own top pool (if the op is added to the top pool upon e.g. parentchain block import).
	//
	// The execution will most likely fail again. However, the state could have been changed already by applying
	// the state diff from the imported sidechain block. This could cause an inconsistent/mismatching state,
	// for example, the nonce. See the nonce handling below: we increased the nonce no matter the STF is executed
	// successfully or not.
	//
	// This is probably the reason why the nonce-handling test in `demo_shielding_unshielding.sh` sometimes fails.
	//
	// Update:
	// see discussion in https://github.com/integritee-network/worker/issues/1232
	// my current thoughts are:
	// - we should return Err() if the STF execution fails, the parentchain effect will get applied regardless
	// - the failed top should be removed from the pool
	// - however, the failed top hash needs to be included in the sidechain block (still TODO)
	//
	// Almost every (Litentry) trusted call has a `H256` as parameter, this is used as the request identifier.
	// It should be generated by the client (requester), and checked against when getting the response.
	// It might seem redundant for direct invocation (DI) as the response is synchronous, however, we do need it
	// when the request is handled asynchronously interanlly, which leads to streamed responses. Without it, it's
	// impossible to pair the request and response. `top_hash` won't suffice as you can't know all hashes from
	// client side beforehand (e.g. those trusted calls signed by enclave signer).
	//
	// TODO:
	// - shall we add `req_ext_hash` in RpcReturnValue and use it to find streamed trustedCalls?
	// - show error details for "Invalid" synchronous responses
	fn execute(
		self,
		shard: &ShardIdentifier,
		top_hash: H256,
		calls: &mut Vec<OpaqueCall>,
		node_metadata_repo: Arc<NodeMetadataRepository>,
	) -> Result<Self::Result, Self::Error> {
		let sender = self.call.sender_identity().clone();
		let call_hash = blake2_256(&self.call.encode());
		let account_id: AccountId = sender.to_account_id().ok_or(Self::Error::InvalidAccount)?;
		let system_nonce = System::account_nonce(&account_id);
		ensure!(self.nonce == system_nonce, Self::Error::InvalidNonce(self.nonce, system_nonce));

		// Increment the nonce no matter if the call succeeds or fails.
		// We consider the call "valid" once it reaches here (= it entered the tx pool)
		System::inc_account_nonce(&account_id);

		// TODO: maybe we can further simplify this by effacing the duplicate code
		match self.call {
			TrustedCall::balance_set_balance(root, who, free_balance, reserved_balance) => {
				let root_account_id: AccountId =
					root.to_account_id().ok_or(Self::Error::InvalidAccount)?;
				ensure!(
					is_root::<Runtime, AccountId>(&root_account_id),
					Self::Error::MissingPrivileges(root)
				);
				debug!(
					"balance_set_balance({}, {}, {})",
					account_id_to_string(&who),
					free_balance,
					reserved_balance
				);
				ita_sgx_runtime::BalancesCall::<Runtime>::force_set_balance {
					who: MultiAddress::Id(who),
					new_free: free_balance,
				}
				.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::root())
				.map_err(|e| {
					Self::Error::Dispatch(format!("Balance Set Balance error: {:?}", e.error))
				})?;
				// This explicit Error type is somehow still needed, otherwise the compiler complains
				// 	multiple `impl`s satisfying `StfError: std::convert::From<_>`
				// 		note: and another `impl` found in the `core` crate: `impl<T> std::convert::From<T> for T;`
				// the impl From<..> for StfError conflicts with the standard convert
				//
				// Alternatively, removing the customised "impl From<..> for StfError" and use map_err directly
				// would also work
				Ok::<Self::Result, Self::Error>(TrustedCallResult::Empty)
			},
			TrustedCall::balance_transfer(from, to, value) => {
				let origin = ita_sgx_runtime::RuntimeOrigin::signed(
					from.to_account_id().ok_or(Self::Error::InvalidAccount)?,
				);
				debug!(
					"balance_transfer({}, {}, {})",
					account_id_to_string(&from),
					account_id_to_string(&to),
					value
				);
				ita_sgx_runtime::BalancesCall::<Runtime>::transfer {
					dest: MultiAddress::Id(to),
					value,
				}
				.dispatch_bypass_filter(origin)
				.map_err(|e| {
					Self::Error::Dispatch(format!("Balance Transfer error: {:?}", e.error))
				})?;
				Ok(TrustedCallResult::Empty)
			},
			TrustedCall::balance_unshield(account_incognito, beneficiary, value, shard) => {
				debug!(
					"balance_unshield({}, {}, {}, {})",
					account_id_to_string(&account_incognito),
					account_id_to_string(&beneficiary),
					value,
					shard
				);
				unshield_funds(
					account_incognito.to_account_id().ok_or(Self::Error::InvalidAccount)?,
					value,
				)?;
				calls.push(OpaqueCall::from_tuple(&(
					node_metadata_repo.get_from_metadata(|m| m.unshield_funds_call_indexes())??,
					beneficiary,
					value,
					shard,
					call_hash,
				)));
				Ok(TrustedCallResult::Empty)
			},
			TrustedCall::balance_shield(enclave_account, who, value) => {
				let account_id: AccountId32 =
					enclave_account.to_account_id().ok_or(Self::Error::InvalidAccount)?;
				ensure_enclave_signer_account(&account_id)?;
				debug!("balance_shield({}, {})", account_id_to_string(&who), value);
				shield_funds(who, value)?;

				// Send proof of execution on chain.
				calls.push(OpaqueCall::from_tuple(&(
					node_metadata_repo.get_from_metadata(|m| m.publish_hash_call_indexes())??,
					call_hash,
					Vec::<itp_types::H256>::new(),
					b"shielded some funds!".to_vec(),
				)));
				Ok(TrustedCallResult::Empty)
			},
			#[cfg(feature = "evm")]
			TrustedCall::evm_withdraw(from, address, value) => {
				debug!("evm_withdraw({}, {}, {})", account_id_to_string(&from), address, value);
				ita_sgx_runtime::EvmCall::<Runtime>::withdraw { address, value }
					.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::signed(
						from.to_account_id().ok_or(Self::Error::InvalidAccount)?,
					))
					.map_err(|e| {
						Self::Error::Dispatch(format!("Evm Withdraw error: {:?}", e.error))
					})?;
				Ok(TrustedCallResult::Empty)
			},
			#[cfg(feature = "evm")]
			TrustedCall::evm_call(
				from,
				source,
				target,
				input,
				value,
				gas_limit,
				max_fee_per_gas,
				max_priority_fee_per_gas,
				nonce,
				access_list,
			) => {
				debug!(
					"evm_call(from: {}, source: {}, target: {})",
					account_id_to_string(&from),
					source,
					target
				);
				ita_sgx_runtime::EvmCall::<Runtime>::call {
					source,
					target,
					input,
					value,
					gas_limit,
					max_fee_per_gas,
					max_priority_fee_per_gas,
					nonce,
					access_list,
				}
				.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::signed(
					from.to_account_id().ok_or(Self::Error::InvalidAccount)?,
				))
				.map_err(|e| Self::Error::Dispatch(format!("Evm Call error: {:?}", e.error)))?;
				Ok(TrustedCallResult::Empty)
			},
			#[cfg(feature = "evm")]
			TrustedCall::evm_create(
				from,
				source,
				init,
				value,
				gas_limit,
				max_fee_per_gas,
				max_priority_fee_per_gas,
				nonce,
				access_list,
			) => {
				debug!(
					"evm_create(from: {}, source: {}, value: {})",
					account_id_to_string(&from),
					source,
					value
				);
				let nonce_evm_account =
					System::account_nonce(&HashedAddressMapping::into_account_id(source));
				ita_sgx_runtime::EvmCall::<Runtime>::create {
					source,
					init,
					value,
					gas_limit,
					max_fee_per_gas,
					max_priority_fee_per_gas,
					nonce,
					access_list,
				}
				.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::signed(
					from.to_account_id().ok_or(Self::Error::InvalidAccount)?,
				))
				.map_err(|e| Self::Error::Dispatch(format!("Evm Create error: {:?}", e.error)))?;
				let contract_address = evm_create_address(source, nonce_evm_account);
				info!("Trying to create evm contract with address {:?}", contract_address);
				Ok(TrustedCallResult::Empty)
			},
			#[cfg(feature = "evm")]
			TrustedCall::evm_create2(
				from,
				source,
				init,
				salt,
				value,
				gas_limit,
				max_fee_per_gas,
				max_priority_fee_per_gas,
				nonce,
				access_list,
			) => {
				debug!(
					"evm_create2(from: {}, source: {}, value: {})",
					account_id_to_string(&from),
					source,
					value
				);
				let code_hash = create_code_hash(&init);
				ita_sgx_runtime::EvmCall::<Runtime>::create2 {
					source,
					init,
					salt,
					value,
					gas_limit,
					max_fee_per_gas,
					max_priority_fee_per_gas,
					nonce,
					access_list,
				}
				.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::signed(
					from.to_account_id().ok_or(Self::Error::InvalidAccount)?,
				))
				.map_err(|e| Self::Error::Dispatch(format!("Evm Create2 error: {:?}", e.error)))?;
				let contract_address = evm_create2_address(source, salt, code_hash);
				info!("Trying to create evm contract with address {:?}", contract_address);
				Ok(TrustedCallResult::Empty)
			},
			// Litentry trusted calls
			// the reason that most calls have an internal handling fn is that we want to capture the error and
			// handle it here to be able to send error events to the parachain
			TrustedCall::link_identity(
				signer,
				who,
				identity,
				validation_data,
				web3networks,
				nonce,
				maybe_key,
				hash,
			) => {
				debug!("link_identity, who: {}", account_id_to_string(&who));
				let account = SgxParentchainTypeConverter::convert(
					who.to_account_id().ok_or(Self::Error::InvalidAccount)?,
				);
				let verification_done = Self::link_identity_internal(
					shard,
					signer.to_account_id().ok_or(Self::Error::InvalidAccount)?,
					who.clone(),
					identity.clone(),
					validation_data,
					web3networks.clone(),
					nonce,
					top_hash,
					maybe_key,
					hash,
				)
				.map_err(|e| {
					add_call_from_imp_error(
						calls,
						node_metadata_repo.clone(),
						Some(account),
						e.to_imp_error(),
						hash,
					);
					e
				})?;

				if verification_done {
					Self::handle_link_identity_callback(
						calls,
						node_metadata_repo,
						enclave_signer_account::<AccountId>().into(),
						who,
						identity,
						web3networks,
						maybe_key,
						hash,
					)
				} else {
					Ok(TrustedCallResult::Streamed)
				}
			},
			TrustedCall::deactivate_identity(signer, who, identity, hash) => {
				debug!("deactivate_identity, who: {}", account_id_to_string(&who));
				let account = SgxParentchainTypeConverter::convert(
					who.to_account_id().ok_or(Self::Error::InvalidAccount)?,
				);
				let call_index = node_metadata_repo
					.get_from_metadata(|m| m.identity_deactivated_call_indexes())??;

				Self::deactivate_identity_internal(
					signer.to_account_id().ok_or(Self::Error::InvalidAccount)?,
					who,
					identity.clone(),
				)
				.map_err(|e| {
					debug!("pushing error event ... error: {}", e);
					add_call_from_imp_error(
						calls,
						node_metadata_repo,
						Some(account.clone()),
						e.to_imp_error(),
						hash,
					);
					e
				})?;

				debug!("pushing identity_deactivated event ...");
				calls.push(OpaqueCall::from_tuple(&(call_index, account, hash)));
				Ok(TrustedCallResult::Empty)
			},
			TrustedCall::activate_identity(signer, who, identity, hash) => {
				debug!("activate_identity, who: {}", account_id_to_string(&who));
				let account = SgxParentchainTypeConverter::convert(
					who.to_account_id().ok_or(Self::Error::InvalidAccount)?,
				);
				let call_index = node_metadata_repo
					.get_from_metadata(|m| m.identity_activated_call_indexes())??;

				Self::activate_identity_internal(
					signer.to_account_id().ok_or(Self::Error::InvalidAccount)?,
					who,
					identity.clone(),
				)
				.map_err(|e| {
					debug!("pushing error event ... error: {}", e);
					add_call_from_imp_error(
						calls,
						node_metadata_repo,
						Some(account.clone()),
						e.to_imp_error(),
						hash,
					);
					e
				})?;

				debug!("pushing identity_activated event ...");
				calls.push(OpaqueCall::from_tuple(&(call_index, account, hash)));
				Ok(TrustedCallResult::Empty)
			},
			TrustedCall::link_identity_callback(
				signer,
				who,
				identity,
				web3networks,
				maybe_key,
				hash,
			) => Self::handle_link_identity_callback(
				calls,
				node_metadata_repo,
				signer,
				who,
				identity,
				web3networks,
				maybe_key,
				hash,
			),
			TrustedCall::request_vc(signer, who, assertion, maybe_key, hash) => {
				debug!(
					"request_vc, who: {}, assertion: {:?}",
					account_id_to_string(&who),
					assertion
				);

				let account = SgxParentchainTypeConverter::convert(
					who.to_account_id().ok_or(Self::Error::InvalidAccount)?,
				);
				Self::request_vc_internal(
					signer.to_account_id().ok_or(Self::Error::InvalidAccount)?,
					who,
					assertion,
					top_hash,
					hash,
					maybe_key,
					shard,
				)
				.map_err(|e| {
					debug!("pushing error event ... error: {}", e);
					add_call_from_vcmp_error(
						calls,
						node_metadata_repo,
						Some(account),
						e.to_vcmp_error(),
						hash,
					);
					e
				})?;
				Ok(TrustedCallResult::Streamed)
			},
			TrustedCall::request_vc_callback(
				signer,
				who,
				assertion,
				vc_index,
				vc_hash,
				vc_payload,
				maybe_key,
				hash,
			) => {
				debug!(
					"request_vc_callback, who: {}, assertion: {:?}",
					account_id_to_string(&who),
					assertion
				);
				let account = SgxParentchainTypeConverter::convert(
					who.to_account_id().ok_or(Self::Error::InvalidAccount)?,
				);

				Self::request_vc_callback_internal(
					signer.to_account_id().ok_or(Self::Error::InvalidAccount)?,
					who,
					assertion.clone(),
				)
				.map_err(|e| {
					debug!("pushing error event ... error: {}", e);
					add_call_from_vcmp_error(
						calls,
						node_metadata_repo,
						Some(account.clone()),
						e.to_vcmp_error(),
						hash,
					);
					e
				})?;

				if let Some(key) = maybe_key {
					debug!("pushing vc_issued event ...");
					let call_index =
						node_metadata_repo.get_from_metadata(|m| m.vc_issued_call_indexes())??;

					calls.push(OpaqueCall::from_tuple(&(
						call_index,
						account,
						assertion,
						vc_index,
						vc_hash,
						aes_encrypt_default(&key, &vc_payload),
						hash,
					)));
					let res = RequestVCResult {
						vc_index,
						vc_hash,
						vc_payload: aes_encrypt_default(&key, &vc_payload),
					};
					Ok(TrustedCallResult::RequestVC(res))
				} else {
					Ok(TrustedCallResult::Empty)
				}
			},
			TrustedCall::set_identity_networks(signer, who, identity, web3networks, _) => {
				debug!("set_identity_networks, networks: {:?}", web3networks);
				// only support DI requests from the signer but we leave the room for changes
				ensure!(
					ensure_self(&signer, &who),
					Self::Error::Dispatch("Unauthorized signer".to_string())
				);
				IMTCall::set_identity_networks { who, identity, web3networks }
					.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::root())
					.map_err(|e| Self::Error::Dispatch(format!(" error: {:?}", e.error)))?;
				Ok(TrustedCallResult::Empty)
			},
			TrustedCall::handle_imp_error(_enclave_account, account, e, hash) => {
				// checking of `_enclave_account` is not strictly needed, as this trusted call can
				// only be constructed internally
				add_call_from_imp_error(
					calls,
					node_metadata_repo,
					account.and_then(|g| g.to_account_id()),
					e.clone(),
					hash,
				);
				Err(e.into())
			},
			TrustedCall::handle_vcmp_error(_enclave_account, account, e, hash) => {
				// checking of `_enclave_account` is not strictly needed, as this trusted call can
				// only be constructed internally
				add_call_from_vcmp_error(
					calls,
					node_metadata_repo,
					account.and_then(|g| g.to_account_id()),
					e.clone(),
					hash,
				);
				Err(e.into())
			},
			TrustedCall::send_erroneous_parentchain_call(account) => {
				// intentionally send wrong parameters, only used in testing
				calls.push(OpaqueCall::from_tuple(&(
					node_metadata_repo.get_from_metadata(|m| m.imp_some_error_call_indexes())??,
					"set_user_shielding_key".as_bytes(),
					account.encode(),
				)));
				Ok(TrustedCallResult::Empty)
			},
		}
	}

	fn get_storage_hashes_to_update(self) -> Vec<Vec<u8>> {
		let key_hashes = Vec::new();
		match self.call {
			TrustedCall::balance_set_balance(..) => debug!("No storage updates needed..."),
			TrustedCall::balance_transfer(..) => debug!("No storage updates needed..."),
			TrustedCall::balance_unshield(..) => debug!("No storage updates needed..."),
			TrustedCall::balance_shield(..) => debug!("No storage updates needed..."),
			// litentry
			TrustedCall::link_identity(..) => debug!("No storage updates needed..."),
			TrustedCall::deactivate_identity(..) => debug!("No storage updates needed..."),
			TrustedCall::activate_identity(..) => debug!("No storage updates needed..."),
			TrustedCall::request_vc(..) => debug!("No storage updates needed..."),
			TrustedCall::link_identity_callback(..) => debug!("No storage updates needed..."),
			TrustedCall::request_vc_callback(..) => debug!("No storage updates needed..."),
			TrustedCall::set_identity_networks(..) => debug!("No storage updates needed..."),
			TrustedCall::handle_imp_error(..) => debug!("No storage updates needed..."),
			TrustedCall::handle_vcmp_error(..) => debug!("No storage updates needed..."),
			TrustedCall::send_erroneous_parentchain_call(..) =>
				debug!("No storage updates needed..."),
			#[cfg(feature = "evm")]
			_ => debug!("No storage updates needed..."),
		};
		key_hashes
	}
}

fn unshield_funds(account: AccountId, amount: u128) -> Result<(), StfError> {
	let account_info = System::account(&account);
	if account_info.data.free < amount {
		return Err(StfError::MissingFunds)
	}

	ita_sgx_runtime::BalancesCall::<Runtime>::force_set_balance {
		who: MultiAddress::Id(account),
		new_free: account_info.data.free - amount,
	}
	.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::root())
	.map_err(|e| StfError::Dispatch(format!("Unshield funds error: {:?}", e.error)))?;
	Ok(())
}

fn shield_funds(account: AccountId, amount: u128) -> Result<(), StfError> {
	let account_info = System::account(&account);
	ita_sgx_runtime::BalancesCall::<Runtime>::force_set_balance {
		who: MultiAddress::Id(account),
		new_free: account_info.data.free + amount,
	}
	.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::root())
	.map_err(|e| StfError::Dispatch(format!("Shield funds error: {:?}", e.error)))?;

	Ok(())
}

pub(crate) fn is_root<Runtime, AccountId>(account: &AccountId) -> bool
where
	Runtime: frame_system::Config<AccountId = AccountId> + pallet_sudo::Config,
	AccountId: PartialEq,
{
	pallet_sudo::Pallet::<Runtime>::key().map_or(false, |k| account == &k)
}

// helper method to create and push an `OpaqueCall` from a IMPError, this function always succeeds
pub fn add_call_from_imp_error<NodeMetadataRepository>(
	calls: &mut Vec<OpaqueCall>,
	node_metadata_repo: Arc<NodeMetadataRepository>,
	account: Option<ParentchainAccountId>,
	e: IMPError,
	hash: H256,
) where
	NodeMetadataRepository: AccessNodeMetadata,
	NodeMetadataRepository::MetadataType: NodeMetadataTrait,
{
	debug!("pushing imp_some_error event ...");
	// TODO: anyway to simplify this? `and_then` won't be applicable here
	match node_metadata_repo.get_from_metadata(|m| m.imp_some_error_call_indexes()) {
		Ok(Ok(call_index)) => calls.push(OpaqueCall::from_tuple(&(call_index, account, e, hash))),
		Ok(e) => warn!("error getting IMP call indexes: {:?}", e),
		Err(e) => warn!("error getting IMP call indexes: {:?}", e),
	}
}

// helper method to create and push an `OpaqueCall` from a VCMPError, this function always succeeds
pub fn add_call_from_vcmp_error<NodeMetadataRepository>(
	calls: &mut Vec<OpaqueCall>,
	node_metadata_repo: Arc<NodeMetadataRepository>,
	account: Option<ParentchainAccountId>,
	e: VCMPError,
	hash: H256,
) where
	NodeMetadataRepository: AccessNodeMetadata,
	NodeMetadataRepository::MetadataType: NodeMetadataTrait,
{
	debug!("pushing vcmp_some_error event ...");
	match node_metadata_repo.get_from_metadata(|m| m.vcmp_some_error_call_indexes()) {
		Ok(Ok(call_index)) => calls.push(OpaqueCall::from_tuple(&(call_index, account, e, hash))),
		Ok(e) => warn!("error getting VCMP call indexes: {:?}", e),
		Err(e) => warn!("error getting VCMP call indexes: {:?}", e),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use itp_stf_primitives::types::KeyPair;
	use sp_keyring::AccountKeyring;

	#[test]
	fn verify_signature_works() {
		let nonce = 21;
		let mrenclave = [0u8; 32];
		let shard = ShardIdentifier::default();

		let call = TrustedCall::balance_set_balance(
			AccountKeyring::Alice.public().into(),
			AccountKeyring::Alice.public().into(),
			42,
			42,
		);
		let signed_call = call.sign(
			&KeyPair::Sr25519(Box::new(AccountKeyring::Alice.pair())),
			nonce,
			&mrenclave,
			&shard,
		);

		assert!(signed_call.verify_signature(&mrenclave, &shard));
	}
}
