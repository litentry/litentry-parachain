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
use sp_core::{H160, H256, U256};

#[cfg(feature = "evm")]
use std::vec::Vec;

use crate::{
	helpers::{aes_encrypt_default, ensure_enclave_signer_account},
	AccountId, IdentityManagement, KeyPair, MetadataOf, Runtime, ShardIdentifier, Signature,
	StfError, System, TrustedOperation,
};
use codec::{Decode, Encode};
use frame_support::{ensure, traits::UnfilteredDispatchable};
pub use ita_sgx_runtime::{Balance, Index};
use itp_node_api::metadata::{
	pallet_imp::IMPCallIndexes, pallet_teerex::TeerexCallIndexes, provider::AccessNodeMetadata,
};
use itp_stf_interface::ExecuteCall;
use itp_types::OpaqueCall;
use itp_utils::stringify::account_id_to_string;
use litentry_primitives::{
	ChallengeCode, Identity, ParentchainBlockNumber, UserShieldingKeyType, ValidationData,
};
use log::*;
use sp_io::hashing::blake2_256;
use sp_runtime::{traits::Verify, MultiAddress};
use std::{format, prelude::v1::*, sync::Arc};

#[cfg(feature = "evm")]
use ita_sgx_runtime::{AddressMapping, HashedAddressMapping};

#[cfg(feature = "evm")]
use crate::evm_helpers::{create_code_hash, evm_create2_address, evm_create_address};

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum TrustedCall {
	balance_set_balance(AccountId, AccountId, Balance, Balance),
	balance_transfer(AccountId, AccountId, Balance),
	balance_unshield(AccountId, AccountId, Balance, ShardIdentifier), // (AccountIncognito, BeneficiaryPublicAccount, Amount, Shard)
	balance_shield(AccountId, AccountId, Balance), // (Root, AccountIncognito, Amount)
	#[cfg(feature = "evm")]
	evm_withdraw(AccountId, H160, Balance), // (Origin, Address EVM Account, Value)
	// (Origin, Source, Target, Input, Value, Gas limit, Max fee per gas, Max priority fee per gas, Nonce, Access list)
	#[cfg(feature = "evm")]
	evm_call(
		AccountId,
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
		AccountId,
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
		AccountId,
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
	// litentry
	set_user_shielding_key_preflight(AccountId, AccountId, UserShieldingKeyType), // (Root, AccountIncognito, Key) -- root as signer, only for testing
	set_user_shielding_key_runtime(AccountId, AccountId, UserShieldingKeyType), // (EnclaveSigner, AccountIncognito, Key)
	create_identity_runtime(
		AccountId,
		AccountId,
		Identity,
		Option<MetadataOf<Runtime>>,
		ParentchainBlockNumber,
	), // (EnclaveSigner, Account, identity, metadata, blocknumber)
	remove_identity_runtime(AccountId, AccountId, Identity), // (EnclaveSigner, Account, identity)
	verify_identity_preflight(
		AccountId,
		AccountId,
		Identity,
		ValidationData,
		ParentchainBlockNumber,
	), // (EnclaveSigner, Account, identity, validation, blocknumber)
	verify_identity_runtime(AccountId, AccountId, Identity, ParentchainBlockNumber), // (EnclaveSigner, Account, identity, blocknumber)
	set_challenge_code_runtime(AccountId, AccountId, Identity, ChallengeCode),       // only for testing
}

impl TrustedCall {
	pub fn sender_account(&self) -> &AccountId {
		match self {
			TrustedCall::balance_set_balance(sender_account, ..) => sender_account,
			TrustedCall::balance_transfer(sender_account, ..) => sender_account,
			TrustedCall::balance_unshield(sender_account, ..) => sender_account,
			TrustedCall::balance_shield(sender_account, ..) => sender_account,
			#[cfg(feature = "evm")]
			TrustedCall::evm_withdraw(sender_account, ..) => sender_account,
			#[cfg(feature = "evm")]
			TrustedCall::evm_call(sender_account, ..) => sender_account,
			#[cfg(feature = "evm")]
			TrustedCall::evm_create(sender_account, ..) => sender_account,
			#[cfg(feature = "evm")]
			TrustedCall::evm_create2(sender_account, ..) => sender_account,
			// litentry
			TrustedCall::set_user_shielding_key_preflight(account, _, _) => account,
			TrustedCall::set_user_shielding_key_runtime(account, _, _) => account,
			TrustedCall::create_identity_runtime(account, _, _, _, _) => account,
			TrustedCall::remove_identity_runtime(account, _, _) => account,
			TrustedCall::verify_identity_preflight(account, _, _, _, _) => account,
			TrustedCall::verify_identity_runtime(account, _, _, _) => account,
			TrustedCall::set_challenge_code_runtime(account, _, _, _) => account,
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
	pub signature: Signature,
}

impl TrustedCallSigned {
	pub fn new(call: TrustedCall, nonce: Index, signature: Signature) -> Self {
		TrustedCallSigned { call, nonce, signature }
	}

	pub fn verify_signature(&self, mrenclave: &[u8; 32], shard: &ShardIdentifier) -> bool {
		let mut payload = self.call.encode();
		payload.append(&mut self.nonce.encode());
		payload.append(&mut mrenclave.encode());
		payload.append(&mut shard.encode());
		self.signature.verify(payload.as_slice(), self.call.sender_account())
	}

	pub fn into_trusted_operation(self, direct: bool) -> TrustedOperation {
		match direct {
			true => TrustedOperation::direct_call(self),
			false => TrustedOperation::indirect_call(self),
		}
	}
}

// TODO: #91 signed return value
/*
pub struct TrustedReturnValue<T> {
	pub value: T,
	pub signer: AccountId
}

impl TrustedReturnValue
*/

impl<NodeMetadataRepository> ExecuteCall<NodeMetadataRepository> for TrustedCallSigned
where
	NodeMetadataRepository: AccessNodeMetadata,
	NodeMetadataRepository::MetadataType: TeerexCallIndexes + IMPCallIndexes,
{
	type Error = StfError;

	fn execute(
		self,
		shard: &ShardIdentifier,
		calls: &mut Vec<OpaqueCall>,
		node_metadata_repo: Arc<NodeMetadataRepository>,
	) -> Result<(), Self::Error> {
		let sender = self.call.sender_account().clone();
		let call_hash = blake2_256(&self.call.encode());
		ensure!(
			self.nonce == System::account_nonce(&sender),
			Self::Error::InvalidNonce(self.nonce)
		);
		match self.call {
			TrustedCall::balance_set_balance(root, who, free_balance, reserved_balance) => {
				ensure!(is_root::<Runtime, AccountId>(&root), Self::Error::MissingPrivileges(root));
				debug!(
					"balance_set_balance({}, {}, {})",
					account_id_to_string(&who),
					free_balance,
					reserved_balance
				);
				ita_sgx_runtime::BalancesCall::<Runtime>::set_balance {
					who: MultiAddress::Id(who),
					new_free: free_balance,
					new_reserved: reserved_balance,
				}
				.dispatch_bypass_filter(ita_sgx_runtime::Origin::root())
				.map_err(|e| {
					Self::Error::Dispatch(format!("Balance Set Balance error: {:?}", e.error))
				})?;
				Ok(())
			},
			TrustedCall::balance_transfer(from, to, value) => {
				let origin = ita_sgx_runtime::Origin::signed(from.clone());
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
				Ok(())
			},
			TrustedCall::balance_unshield(account_incognito, beneficiary, value, shard) => {
				debug!(
					"balance_unshield({}, {}, {}, {})",
					account_id_to_string(&account_incognito),
					account_id_to_string(&beneficiary),
					value,
					shard
				);
				unshield_funds(account_incognito, value)?;
				calls.push(OpaqueCall::from_tuple(&(
					node_metadata_repo.get_from_metadata(|m| m.unshield_funds_call_indexes())??,
					beneficiary,
					value,
					shard,
					call_hash,
				)));
				Ok(())
			},
			TrustedCall::balance_shield(enclave_account, who, value) => {
				ensure_enclave_signer_account(&enclave_account)?;
				debug!("balance_shield({}, {})", account_id_to_string(&who), value);
				shield_funds(who, value)?;
				Ok(())
			},
			#[cfg(feature = "evm")]
			TrustedCall::evm_withdraw(from, address, value) => {
				debug!("evm_withdraw({}, {}, {})", account_id_to_string(&from), address, value);
				ita_sgx_runtime::EvmCall::<Runtime>::withdraw { address, value }
					.dispatch_bypass_filter(ita_sgx_runtime::Origin::signed(from))
					.map_err(|e| {
						Self::Error::Dispatch(format!("Evm Withdraw error: {:?}", e.error))
					})?;
				Ok(())
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
				.dispatch_bypass_filter(ita_sgx_runtime::Origin::signed(from))
				.map_err(|e| Self::Error::Dispatch(format!("Evm Call error: {:?}", e.error)))?;
				Ok(())
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
				.dispatch_bypass_filter(ita_sgx_runtime::Origin::signed(from))
				.map_err(|e| Self::Error::Dispatch(format!("Evm Create error: {:?}", e.error)))?;
				let contract_address = evm_create_address(source, nonce_evm_account);
				info!("Trying to create evm contract with address {:?}", contract_address);
				Ok(())
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
				.dispatch_bypass_filter(ita_sgx_runtime::Origin::signed(from))
				.map_err(|e| Self::Error::Dispatch(format!("Evm Create2 error: {:?}", e.error)))?;
				let contract_address = evm_create2_address(source, salt, code_hash);
				info!("Trying to create evm contract with address {:?}", contract_address);
				Ok(())
			},
			// litentry
			TrustedCall::set_user_shielding_key_preflight(root, who, key) => {
				ensure!(is_root::<Runtime, AccountId>(&root), Self::Error::MissingPrivileges(root));
				Self::set_user_shielding_key_preflight(shard, who, key)
			},
			TrustedCall::set_user_shielding_key_runtime(enclave_account, who, key) => {
				ensure_enclave_signer_account(&enclave_account)?;
				// TODO: we only checked if the extrinsic dispatch is successful,
				//       is that enough? (i.e. is the state changed already?)
				match Self::set_user_shielding_key_runtime(who.clone(), key) {
					Ok(()) => {
						calls.push(OpaqueCall::from_tuple(&(
							node_metadata_repo
								.get_from_metadata(|m| m.user_shielding_key_set_call_indexes())??,
							aes_encrypt_default(&key, &who.encode()),
						)));
					},
					Err(err) => {
						debug!("set_user_shielding_key error: {}", err);
						calls.push(OpaqueCall::from_tuple(&(
							node_metadata_repo
								.get_from_metadata(|m| m.some_error_call_indexes())??,
							"set_user_shielding_key".as_bytes(),
							format!("{:?}", err).as_bytes(),
						)));
					},
				}
				Ok(())
			},
			TrustedCall::create_identity_runtime(enclave_account, who, identity, metadata, bn) => {
				ensure_enclave_signer_account(&enclave_account)?;
				debug!(
					"create_identity, who: {}, identity: {:?}, metadata: {:?}",
					account_id_to_string(&who),
					identity,
					metadata
				);
				match Self::create_identity_runtime(who.clone(), identity.clone(), metadata, bn) {
					Ok(code) => {
						debug!("create_identity {} OK", account_id_to_string(&who));
						if let Some(key) = IdentityManagement::user_shielding_keys(&who) {
							let id_graph =
								ita_sgx_runtime::pallet_imt::Pallet::<Runtime>::get_id_graph(&who);
							calls.push(OpaqueCall::from_tuple(&(
								node_metadata_repo
									.get_from_metadata(|m| m.identity_created_call_indexes())??,
								aes_encrypt_default(&key, &who.encode()),
								aes_encrypt_default(&key, &identity.encode()),
								aes_encrypt_default(&key, &id_graph.encode()),
							)));
							calls.push(OpaqueCall::from_tuple(&(
								node_metadata_repo.get_from_metadata(|m| {
									m.challenge_code_generated_call_indexes()
								})??,
								aes_encrypt_default(&key, &who.encode()),
								aes_encrypt_default(&key, &identity.encode()),
								aes_encrypt_default(&key, &code.encode()),
							)));
						} else {
							calls.push(OpaqueCall::from_tuple(&(
								node_metadata_repo
									.get_from_metadata(|m| m.some_error_call_indexes())??,
								"get_user_shielding_key".as_bytes(),
								"error".as_bytes(),
							)));
						}
					},
					Err(err) => {
						debug!("create_identity {} error: {}", account_id_to_string(&who), err);
						calls.push(OpaqueCall::from_tuple(&(
							node_metadata_repo
								.get_from_metadata(|m| m.some_error_call_indexes())??,
							"create_identity".as_bytes(),
							format!("{:?}", err).as_bytes(),
						)));
					},
				}
				Ok(())
			},
			TrustedCall::remove_identity_runtime(enclave_account, who, identity) => {
				ensure_enclave_signer_account(&enclave_account)?;
				debug!(
					"remove_identity, who: {}, identity: {:?}",
					account_id_to_string(&who),
					identity,
				);
				match Self::remove_identity_runtime(who.clone(), identity.clone()) {
					Ok(()) => {
						debug!("remove_identity {} OK", account_id_to_string(&who));
						if let Some(key) = IdentityManagement::user_shielding_keys(&who) {
							let id_graph =
								ita_sgx_runtime::pallet_imt::Pallet::<Runtime>::get_id_graph(&who);
							calls.push(OpaqueCall::from_tuple(&(
								node_metadata_repo
									.get_from_metadata(|m| m.identity_removed_call_indexes())??,
								aes_encrypt_default(&key, &who.encode()),
								aes_encrypt_default(&key, &identity.encode()),
								aes_encrypt_default(&key, &id_graph.encode()),
							)));
						} else {
							calls.push(OpaqueCall::from_tuple(&(
								node_metadata_repo
									.get_from_metadata(|m| m.some_error_call_indexes())??,
								"get_user_shielding_key".as_bytes(),
								"error".as_bytes(),
							)));
						}
					},
					Err(err) => {
						debug!("remove_identity {} error: {}", account_id_to_string(&who), err);
						calls.push(OpaqueCall::from_tuple(&(
							node_metadata_repo
								.get_from_metadata(|m| m.some_error_call_indexes())??,
							"remove_identity".as_bytes(),
							format!("{:?}", err).as_bytes(),
						)));
					},
				}
				Ok(())
			},
			TrustedCall::verify_identity_preflight(
				enclave_account,
				account,
				identity,
				validation_data,
				bn,
			) => {
				ensure_enclave_signer_account(&enclave_account)?;
				Self::verify_identity_preflight(shard, account, identity, validation_data, bn)
			},
			TrustedCall::verify_identity_runtime(enclave_account, who, identity, bn) => {
				ensure_enclave_signer_account(&enclave_account)?;
				debug!(
					"verify_identity, who: {}, identity: {:?}, bn: {:?}",
					account_id_to_string(&who),
					identity,
					bn
				);
				match Self::verify_identity_runtime(who.clone(), identity.clone(), bn) {
					Ok(()) => {
						debug!("verify_identity {} OK", account_id_to_string(&who));
						if let Some(key) = IdentityManagement::user_shielding_keys(&who) {
							let id_graph =
								ita_sgx_runtime::pallet_imt::Pallet::<Runtime>::get_id_graph(&who);
							calls.push(OpaqueCall::from_tuple(&(
								node_metadata_repo
									.get_from_metadata(|m| m.identity_verified_call_indexes())??,
								aes_encrypt_default(&key, &who.encode()),
								aes_encrypt_default(&key, &identity.encode()),
								aes_encrypt_default(&key, &id_graph.encode()),
							)));
						} else {
							calls.push(OpaqueCall::from_tuple(&(
								node_metadata_repo
									.get_from_metadata(|m| m.some_error_call_indexes())??,
								"get_user_shielding_key".as_bytes(),
								"error".as_bytes(),
							)));
						}
					},
					Err(err) => {
						debug!("create_identity {} error: {}", account_id_to_string(&who), err);
						calls.push(OpaqueCall::from_tuple(&(
							node_metadata_repo
								.get_from_metadata(|m| m.some_error_call_indexes())??,
							"verify_identity".as_bytes(),
							format!("{:?}", err).as_bytes(),
						)));
					},
				}
				Ok(())
			},
			TrustedCall::set_challenge_code_runtime(enclave_account, account, did, code) => {
				ensure_enclave_signer_account(&enclave_account)?;
				Self::set_challenge_code_runtime(account, did, code)
			},
		}?;
		System::inc_account_nonce(&sender);
		Ok(())
	}

	fn get_storage_hashes_to_update(&self) -> Vec<Vec<u8>> {
		let key_hashes = Vec::new();
		match self.call {
			TrustedCall::balance_set_balance(_, _, _, _) => debug!("No storage updates needed..."),
			TrustedCall::balance_transfer(_, _, _) => debug!("No storage updates needed..."),
			TrustedCall::balance_unshield(_, _, _, _) => debug!("No storage updates needed..."),
			TrustedCall::balance_shield(_, _, _) => debug!("No storage updates needed..."),
			// litentry
			TrustedCall::set_user_shielding_key_preflight(..) =>
				debug!("No storage updates needed..."),
			TrustedCall::set_user_shielding_key_runtime(..) =>
				debug!("No storage updates needed..."),
			TrustedCall::create_identity_runtime(..) => debug!("No storage updates needed..."),
			TrustedCall::remove_identity_runtime(..) => debug!("No storage updates needed..."),
			TrustedCall::verify_identity_preflight(..) => debug!("No storage updates needed..."),
			TrustedCall::verify_identity_runtime(..) => debug!("No storage updates needed..."),
			TrustedCall::set_challenge_code_runtime(..) => debug!("No storage updates needed..."),
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

	ita_sgx_runtime::BalancesCall::<Runtime>::set_balance {
		who: MultiAddress::Id(account),
		new_free: account_info.data.free - amount,
		new_reserved: account_info.data.reserved,
	}
	.dispatch_bypass_filter(ita_sgx_runtime::Origin::root())
	.map_err(|e| StfError::Dispatch(format!("Unshield funds error: {:?}", e.error)))?;
	Ok(())
}

fn shield_funds(account: AccountId, amount: u128) -> Result<(), StfError> {
	let account_info = System::account(&account);
	ita_sgx_runtime::BalancesCall::<Runtime>::set_balance {
		who: MultiAddress::Id(account),
		new_free: account_info.data.free + amount,
		new_reserved: account_info.data.reserved,
	}
	.dispatch_bypass_filter(ita_sgx_runtime::Origin::root())
	.map_err(|e| StfError::Dispatch(format!("Shield funds error: {:?}", e.error)))?;

	Ok(())
}

fn is_root<Runtime, AccountId>(account: &AccountId) -> bool
where
	Runtime: frame_system::Config<AccountId = AccountId> + pallet_sudo::Config,
	AccountId: PartialEq,
{
	pallet_sudo::Pallet::<Runtime>::key().map_or(false, |k| account == &k)
}

#[cfg(test)]
mod tests {
	use super::*;
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
