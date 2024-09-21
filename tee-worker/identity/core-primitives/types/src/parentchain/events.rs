use super::alloc::{format, vec::Vec};
use crate::{
	AccountId, Assertion, Balance, BlockNumber, Hash, MrEnclave, RsaRequest, ShardIdentifier,
	WorkerType,
};
use codec::{Decode, Encode};
use core::fmt::Debug;
use itp_utils::{hex::ToHexPrefixed, stringify::account_id_to_string};

use sp_core::H160;
use substrate_api_client::ac_node_api::StaticEvent;

// System pallet events
#[derive(Encode, Decode, Debug)]
pub struct ExtrinsicSuccess;

impl StaticEvent for ExtrinsicSuccess {
	const PALLET: &'static str = "System";
	const EVENT: &'static str = "ExtrinsicSuccess";
}

#[derive(Encode, Decode)]
pub struct ExtrinsicFailed;

impl StaticEvent for ExtrinsicFailed {
	const PALLET: &'static str = "System";
	const EVENT: &'static str = "ExtrinsicFailed";
}

#[derive(Encode, Decode, Debug)]
pub struct BalanceTransfer {
	pub from: AccountId,
	pub to: AccountId,
	pub amount: Balance,
}

// Balances pallet events

impl core::fmt::Display for BalanceTransfer {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let message = format!(
			"BalanceTransfer :: from: {}, to: {}, amount: {}",
			account_id_to_string::<AccountId>(&self.from),
			account_id_to_string::<AccountId>(&self.to),
			self.amount
		);
		write!(f, "{}", message)
	}
}

impl StaticEvent for BalanceTransfer {
	const PALLET: &'static str = "Balances";
	const EVENT: &'static str = "Transfer";
}

// Teebag pallet events

#[derive(Encode, Decode, Debug)]
pub struct ParentchainBlockProcessed {
	pub shard: ShardIdentifier,
	pub block_number: BlockNumber,
	pub block_hash: Hash,
	pub task_merkle_root: Hash,
}

impl core::fmt::Display for ParentchainBlockProcessed {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let message = format!(
			"ParentchainBlockProcessed :: nr {} shard: {}, merkle: {:?}, block hash {:?}",
			self.block_number, self.shard, self.task_merkle_root, self.block_hash
		);
		write!(f, "{}", message)
	}
}

impl StaticEvent for ParentchainBlockProcessed {
	const PALLET: &'static str = "Teebag";
	const EVENT: &'static str = "ParentchainBlockProcessed";
}

#[derive(Encode, Decode, Debug)]
pub struct EnclaveUnauthorized {
	pub worker_type: WorkerType,
	pub mrenclave: MrEnclave,
}

impl core::fmt::Display for EnclaveUnauthorized {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let message = format!(
			"EnclaveUnauthorized :: worker_type: {:?}, mrenclave: {}",
			self.worker_type,
			self.mrenclave.to_hex()
		);
		write!(f, "{}", message)
	}
}

impl StaticEvent for EnclaveUnauthorized {
	const PALLET: &'static str = "Teebag";
	const EVENT: &'static str = "EnclaveUnauthorized";
}

// IdentityManagement events

#[derive(Encode, Decode, Debug)]
pub struct LinkIdentityRequested {
	pub shard: ShardIdentifier,
	pub account: AccountId,
	pub encrypted_identity: Vec<u8>,
	pub encrypted_validation_data: Vec<u8>,
	pub encrypted_web3networks: Vec<u8>,
}

impl core::fmt::Display for LinkIdentityRequested {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let message = format!(
			"LinkIdentityRequested :: shard: {}, account: {}, identity: {:?}, validation_data: {:?}, web3networks: {:?}",
			self.shard,
			account_id_to_string::<AccountId>(&self.account),
			self.encrypted_identity,
			self.encrypted_validation_data,
			self.encrypted_web3networks
		);
		write!(f, "{}", message)
	}
}

impl StaticEvent for LinkIdentityRequested {
	const PALLET: &'static str = "IdentityManagement";
	const EVENT: &'static str = "LinkIdentityRequested";
}

#[derive(Encode, Decode, Debug)]
pub struct DeactivateIdentityRequested {
	pub shard: ShardIdentifier,
	pub account: AccountId,
	pub encrypted_identity: Vec<u8>,
}

impl core::fmt::Display for DeactivateIdentityRequested {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let message = format!(
			"DeactivateIdentityRequested :: shard: {}, account: {}, identity: {:?}",
			self.shard,
			account_id_to_string::<AccountId>(&self.account),
			self.encrypted_identity
		);
		write!(f, "{}", message)
	}
}

impl StaticEvent for DeactivateIdentityRequested {
	const PALLET: &'static str = "IdentityManagement";
	const EVENT: &'static str = "DeactivateIdentityRequested";
}

#[derive(Encode, Decode, Debug)]
pub struct ActivateIdentityRequested {
	pub shard: ShardIdentifier,
	pub account: AccountId,
	pub encrypted_identity: Vec<u8>,
}

impl core::fmt::Display for ActivateIdentityRequested {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let message = format!(
			"ActivateIdentityRequested :: shard: {}, account: {}, identity: {:?}",
			self.shard,
			account_id_to_string::<AccountId>(&self.account),
			self.encrypted_identity
		);
		write!(f, "{}", message)
	}
}

impl StaticEvent for ActivateIdentityRequested {
	const PALLET: &'static str = "IdentityManagement";
	const EVENT: &'static str = "ActivateIdentityRequested";
}

// VCManagement events

#[derive(Encode, Decode, Debug)]
pub struct VCRequested {
	pub shard: ShardIdentifier,
	pub account: AccountId,
	pub assertion: Assertion,
}

impl core::fmt::Display for VCRequested {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let message = format!(
			"VCRequested :: shard: {}, account: {}, assertion: {:?}",
			self.shard,
			account_id_to_string::<AccountId>(&self.account),
			self.assertion
		);
		write!(f, "{}", message)
	}
}

impl StaticEvent for VCRequested {
	const PALLET: &'static str = "VCManagement";
	const EVENT: &'static str = "VCRequested";
}

#[derive(Encode, Decode, Debug)]
pub struct OpaqueTaskPosted {
	pub request: RsaRequest,
}

impl core::fmt::Display for OpaqueTaskPosted {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let message = format!("OpaqueTaskPosted :: request: {:?}", self.request);
		write!(f, "{}", message)
	}
}

impl StaticEvent for OpaqueTaskPosted {
	const PALLET: &'static str = "Teebag";
	const EVENT: &'static str = "OpaqueTaskPosted";
}

#[derive(Encode, Decode, Debug)]
pub struct AssertionCreated {
	pub id: H160,
	pub byte_code: Vec<u8>,
	pub secrets: Vec<Vec<u8>>,
}

impl core::fmt::Display for AssertionCreated {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let message = format!(
			"{:?} :: byte_code: {:?}, secrets: {:?}",
			AssertionCreated::EVENT,
			self.byte_code,
			self.secrets
		);
		write!(f, "{}", message)
	}
}

impl StaticEvent for AssertionCreated {
	const PALLET: &'static str = "EvmAssertions";
	const EVENT: &'static str = "AssertionCreated";
}

#[derive(Encode, Decode, Debug)]
pub struct RewardDistributionStarted {
	pub total: Balance,
	pub distributed: Balance,
	pub round_index: u32,
}

impl core::fmt::Display for RewardDistributionStarted {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let message = format!(
			"{:?} :: total: {}, distributed: {}, round_index: {}",
			RewardDistributionStarted::EVENT,
			self.total,
			self.distributed,
			self.round_index
		);
		write!(f, "{}", message)
	}
}

impl StaticEvent for RewardDistributionStarted {
	const PALLET: &'static str = "ScoreStaking";
	const EVENT: &'static str = "RewardDistributionStarted";
}