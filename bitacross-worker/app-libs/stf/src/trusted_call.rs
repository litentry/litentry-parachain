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

#[cfg(feature = "evm")]
use crate::evm_helpers::{create_code_hash, evm_create2_address, evm_create_address};
use crate::{
	helpers::{enclave_signer_account, ensure_enclave_signer_account, get_storage_by_key_hash},
	trusted_call_result::TrustedCallResult,
	Getter,
};
use codec::{Compact, Decode, Encode};
use frame_support::{ensure, traits::UnfilteredDispatchable};
#[cfg(feature = "evm")]
use ita_sgx_runtime::{AddressMapping, HashedAddressMapping};
pub use ita_sgx_runtime::{Balance, Index, Runtime, System};
use itp_node_api::metadata::{provider::AccessNodeMetadata, NodeMetadataTrait};
use itp_node_api_metadata::{pallet_balances::BalancesCallIndexes, pallet_proxy::ProxyCallIndexes};
use itp_stf_interface::{ExecuteCall, SHARD_VAULT_KEY};
pub use itp_stf_primitives::{
	error::{StfError, StfResult},
	traits::{TrustedCallSigning, TrustedCallVerification},
	types::{AccountId, KeyPair, ShardIdentifier, TrustedOperation},
};
use itp_types::{
	parentchain::{ParentchainCall, ProxyType},
	Address,
};
pub use itp_types::{OpaqueCall, H256};
use itp_utils::stringify::account_id_to_string;
pub use litentry_primitives::{
	aes_encrypt_default, AesOutput, Identity, LitentryMultiSignature, ParentchainBlockNumber,
	RequestAesKey, RequestAesKeyNonce, ValidationData,
};
use log::*;
use sp_core::{
	crypto::{AccountId32, UncheckedFrom},
	ed25519,
};
use sp_io::hashing::blake2_256;
use sp_runtime::MultiAddress;
use std::{format, prelude::v1::*, sync::Arc};

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TrustedCall {
	// original integritee trusted calls, starting from index 50
	#[codec(index = 50)]
	noop(Identity),
	#[codec(index = 51)]
	balance_set_balance(Identity, AccountId, Balance, Balance),
	#[codec(index = 52)]
	balance_transfer(Identity, AccountId, Balance),
	#[codec(index = 53)]
	balance_unshield(Identity, AccountId, Balance, ShardIdentifier), // (AccountIncognito, BeneficiaryPublicAccount, Amount, Shard)
	#[codec(index = 54)]
	balance_shield(Identity, AccountId, Balance), // (Root, AccountIncognito, Amount)
	#[cfg(feature = "evm")]
	#[codec(index = 55)]
	evm_withdraw(Identity, H160, Balance), // (Origin, Address EVM Account, Value)
	// (Origin, Source, Target, Input, Value, Gas limit, Max fee per gas, Max priority fee per gas, Nonce, Access list)
	#[cfg(feature = "evm")]
	#[codec(index = 56)]
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
	#[codec(index = 57)]
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
	#[codec(index = 58)]
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
}

impl TrustedCall {
	pub fn sender_identity(&self) -> &Identity {
		match self {
			Self::noop(sender_identity) => sender_identity,
			Self::balance_set_balance(sender_identity, ..) => sender_identity,
			Self::balance_transfer(sender_identity, ..) => sender_identity,
			Self::balance_unshield(sender_identity, ..) => sender_identity,
			Self::balance_shield(sender_identity, ..) => sender_identity,
			#[cfg(feature = "evm")]
			Self::evm_withdraw(sender_identity, ..) => sender_identity,
			#[cfg(feature = "evm")]
			Self::evm_call(sender_identity, ..) => sender_identity,
			#[cfg(feature = "evm")]
			Self::evm_create(sender_identity, ..) => sender_identity,
			#[cfg(feature = "evm")]
			Self::evm_create2(sender_identity, ..) => sender_identity,
		}
	}
}

impl TrustedCallSigning<TrustedCallSigned> for TrustedCall {
	fn sign(
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

	pub fn into_trusted_operation(
		self,
		direct: bool,
	) -> TrustedOperation<TrustedCallSigned, Getter> {
		match direct {
			true => TrustedOperation::direct_call(self),
			false => TrustedOperation::indirect_call(self),
		}
	}
}

impl Default for TrustedCallSigned {
	fn default() -> Self {
		Self {
			call: TrustedCall::noop(AccountId32::unchecked_from([0u8; 32].into()).into()),
			nonce: 0,
			signature: LitentryMultiSignature::Ed25519(ed25519::Signature::unchecked_from(
				[0u8; 64],
			)),
		}
	}
}
impl TrustedCallVerification for TrustedCallSigned {
	fn sender_identity(&self) -> &Identity {
		self.call.sender_identity()
	}

	fn nonce(&self) -> Index {
		self.nonce
	}

	fn verify_signature(&self, mrenclave: &[u8; 32], shard: &ShardIdentifier) -> bool {
		let mut payload = self.call.encode();
		payload.append(&mut self.nonce.encode());
		payload.append(&mut mrenclave.encode());
		payload.append(&mut shard.encode());

		self.signature.verify(payload.as_slice(), self.call.sender_identity())
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
		_shard: &ShardIdentifier,
		_top_hash: H256,
		calls: &mut Vec<ParentchainCall>,
		node_metadata_repo: Arc<NodeMetadataRepository>,
	) -> Result<Self::Result, Self::Error> {
		let sender = self.call.sender_identity().clone();
		let account_id: AccountId = sender.to_account_id().ok_or(Self::Error::InvalidAccount)?;
		let system_nonce = System::account_nonce(&account_id);
		ensure!(self.nonce == system_nonce, Self::Error::InvalidNonce(self.nonce, system_nonce));

		// Increment the nonce no matter if the call succeeds or fails.
		// We consider the call "valid" once it reaches here (= it entered the tx pool)
		System::inc_account_nonce(&account_id);

		// TODO: maybe we can further simplify this by effacing the duplicate code
		match self.call {
			TrustedCall::noop(who) => {
				debug!("noop called by {}", account_id_to_string(&who),);
				Ok(TrustedCallResult::Empty)
			},
			TrustedCall::balance_set_balance(root, who, free_balance, reserved_balance) => {
				let root_account_id: AccountId =
					root.to_account_id().ok_or(Self::Error::InvalidAccount)?;
				ensure!(
					is_root::<Runtime, AccountId>(&root_account_id),
					Self::Error::MissingPrivileges(root_account_id)
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
				Ok(TrustedCallResult::Empty)
			},
			TrustedCall::balance_transfer(from, to, value) => {
				let origin = ita_sgx_runtime::RuntimeOrigin::signed(
					from.to_account_id().ok_or(Self::Error::InvalidAccount)?,
				);
				std::println!("⣿STF⣿ 🔄 balance_transfer from ⣿⣿⣿ to ⣿⣿⣿ amount ⣿⣿⣿");
				// endow fee to enclave (self)
				let fee_recipient: AccountId = enclave_signer_account();
				// fixme: apply fees through standard frame process and tune it
				let fee = crate::STF_TX_FEE;
				info!(
					"from {}, to {}, amount {}, fee {}",
					account_id_to_string(&from),
					account_id_to_string(&to),
					value,
					fee
				);
				ita_sgx_runtime::BalancesCall::<Runtime>::transfer {
					dest: MultiAddress::Id(fee_recipient),
					value: fee,
				}
				.dispatch_bypass_filter(origin.clone())
				.map_err(|e| {
					Self::Error::Dispatch(format!("Balance Transfer error: {:?}", e.error))
				})?;
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
				std::println!(
					"⣿STF⣿ 🛡👐 balance_unshield from ⣿⣿⣿ to {}, amount {}",
					account_id_to_string(&beneficiary),
					value
				);
				// endow fee to enclave (self)
				let fee_recipient: AccountId = enclave_signer_account();
				// fixme: apply fees through standard frame process and tune it. has to be at least two L1 transfer's fees
				let fee = crate::STF_TX_FEE * 3;

				info!(
					"balance_unshield(from (L2): {}, to (L1): {}, amount {} (+fee: {}), shard {})",
					account_id_to_string(&account_incognito),
					account_id_to_string(&beneficiary),
					value,
					fee,
					shard
				);

				let origin = ita_sgx_runtime::RuntimeOrigin::signed(
					account_incognito.to_account_id().ok_or(StfError::InvalidAccount)?,
				);
				ita_sgx_runtime::BalancesCall::<Runtime>::transfer {
					dest: MultiAddress::Id(fee_recipient),
					value: fee,
				}
				.dispatch_bypass_filter(origin)
				.map_err(|e| {
					Self::Error::Dispatch(format!("Balance Unshielding error: {:?}", e.error))
				})?;
				burn_funds(
					account_incognito.to_account_id().ok_or(StfError::InvalidAccount)?,
					value,
				)?;

				let vault_pubkey: [u8; 32] = get_storage_by_key_hash(SHARD_VAULT_KEY.into())
					.ok_or_else(|| {
						StfError::Dispatch("shard vault key hasn't been set".to_string())
					})?;
				let vault_address = Address::from(AccountId::from(vault_pubkey));
				let vault_transfer_call = OpaqueCall::from_tuple(&(
					node_metadata_repo
						.get_from_metadata(|m| m.transfer_keep_alive_call_indexes())
						.map_err(|_| StfError::InvalidMetadata)?
						.map_err(|_| StfError::InvalidMetadata)?,
					Address::from(beneficiary),
					Compact(value),
				));
				let proxy_call = OpaqueCall::from_tuple(&(
					node_metadata_repo
						.get_from_metadata(|m| m.proxy_call_indexes())
						.map_err(|_| StfError::InvalidMetadata)?
						.map_err(|_| StfError::InvalidMetadata)?,
					vault_address,
					None::<ProxyType>,
					vault_transfer_call,
				));
				calls.push(ParentchainCall::TargetA(proxy_call));
				Ok(TrustedCallResult::Empty)
			},
			TrustedCall::balance_shield(enclave_account, who, value) => {
				let account_id: AccountId32 =
					enclave_account.to_account_id().ok_or(Self::Error::InvalidAccount)?;
				ensure_enclave_signer_account(&account_id)?;
				debug!("balance_shield({}, {})", account_id_to_string(&who), value);
				shield_funds(who, value)?;

				// Litentry: we don't have publish_hash call in teebag
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
		}
	}

	fn get_storage_hashes_to_update(self) -> Vec<Vec<u8>> {
		debug!("No storage updates needed...");
		Vec::new()
	}
}

fn burn_funds(account: AccountId, amount: u128) -> Result<(), StfError> {
	let account_info = System::account(&account);
	if account_info.data.free < amount {
		return Err(StfError::MissingFunds)
	}

	ita_sgx_runtime::BalancesCall::<Runtime>::force_set_balance {
		who: MultiAddress::Id(account),
		new_free: account_info.data.free - amount,
	}
	.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::root())
	.map_err(|e| StfError::Dispatch(format!("Burn funds error: {:?}", e.error)))?;
	Ok(())
}

fn shield_funds(account: AccountId, amount: u128) -> Result<(), StfError> {
	//fixme: make fee configurable and send fee to vault account on L2
	let fee = amount / 571; // approx 0.175%

	// endow fee to enclave (self)
	let fee_recipient: AccountId = enclave_signer_account();

	let account_info = System::account(&fee_recipient);
	ita_sgx_runtime::BalancesCall::<Runtime>::force_set_balance {
		who: MultiAddress::Id(fee_recipient),
		new_free: account_info.data.free + fee,
	}
	.dispatch_bypass_filter(ita_sgx_runtime::RuntimeOrigin::root())
	.map_err(|e| StfError::Dispatch(format!("Shield funds error: {:?}", e.error)))?;

	// endow shieding amount - fee to beneficiary
	let account_info = System::account(&account);
	ita_sgx_runtime::BalancesCall::<Runtime>::force_set_balance {
		who: MultiAddress::Id(account),
		new_free: account_info.data.free + amount - fee,
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
