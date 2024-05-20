use super::alloc::format;
use crate::{
	AccountId, Balance, BlockNumber, Hash, MrEnclave, ShardIdentifier, SidechainBlockNumber,
	WorkerType,
};
use codec::{Decode, Encode};
use core::fmt::Debug;
use itp_utils::stringify::account_id_to_string;
use litentry_primitives::{Address32, Identity};
use substrate_api_client::ac_node_api::StaticEvent;

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
pub struct ScheduledEnclaveSet {
	pub worker_type: WorkerType,
	pub sidechain_block_number: SidechainBlockNumber,
	pub mrenclave: MrEnclave,
}

impl core::fmt::Display for ScheduledEnclaveSet {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let message = format!(
			"[{}:{}] :: worker_type: {:?}, sidechain_block_number: {}, mrenclave: {:?}",
			ScheduledEnclaveSet::PALLET,
			ScheduledEnclaveSet::EVENT,
			self.worker_type,
			self.sidechain_block_number,
			self.mrenclave
		);
		write!(f, "{}", message)
	}
}

impl StaticEvent for ScheduledEnclaveSet {
	const PALLET: &'static str = "Teebag";
	const EVENT: &'static str = "ScheduledEnclaveSet";
}

#[derive(Encode, Decode, Debug)]
pub struct ScheduledEnclaveRemoved {
	pub worker_type: WorkerType,
	pub sidechain_block_number: SidechainBlockNumber,
}

impl core::fmt::Display for ScheduledEnclaveRemoved {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let message = format!(
			"ScheduledEnclaveRemoved :: worker_type: {:?}, sidechain_block_number: {}",
			self.worker_type, self.sidechain_block_number
		);
		write!(f, "{}", message)
	}
}

impl StaticEvent for ScheduledEnclaveRemoved {
	const PALLET: &'static str = "Teebag";
	const EVENT: &'static str = "ScheduledEnclaveRemoved";
}

#[derive(Encode, Decode, Debug)]
pub struct EnclaveAdded {
	pub who: Address32,
	pub worker_type: WorkerType,
	pub url: Vec<u8>,
}

impl core::fmt::Display for EnclaveAdded {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let message = format!(
			"EnclaveAdded :: who: {:?}, worker_type: {:?}, url: {:?}",
			self.who, self.worker_type, self.url
		);
		write!(f, "{}", message)
	}
}

impl StaticEvent for EnclaveAdded {
	const PALLET: &'static str = "Teebag";
	const EVENT: &'static str = "EnclaveAdded";
}

//  Bitacross pallet events

#[derive(Encode, Decode, Debug)]
pub struct RelayerAdded {
	pub who: Identity,
}

impl core::fmt::Display for RelayerAdded {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let account_id = account_id_to_string::<AccountId>(&self.who.to_account_id());
		let message = format!("RelayerAdded :: account_id: {:?}", account_id);
		write!(f, "{}", message)
	}
}

impl StaticEvent for RelayerAdded {
	const PALLET: &'static str = "Bitacross";
	const EVENT: &'static str = "RelayerAdded";
}

#[derive(Encode, Decode, Debug)]
pub struct RelayerRemoved {
	pub who: Identity,
}

impl core::fmt::Display for RelayerRemoved {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
		let account_id = account_id_to_string::<AccountId>(&self.who.to_account_id());
		let message = format!("RelayerRemoved :: account_id: {:?}", account_id);
		write!(f, "{}", message)
	}
}

impl StaticEvent for RelayerRemoved {
	const PALLET: &'static str = "Bitacross";
	const EVENT: &'static str = "RelayerRemoved";
}
