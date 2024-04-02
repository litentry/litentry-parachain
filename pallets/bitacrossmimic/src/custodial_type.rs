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

use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_core::{H160, H256, U256};

pub type PubKey = [u8; 33];

/// custodial bit cross record of a full single tx from BTC to ETH
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub struct BtcToEth {
	pub tx_index: u64,
	pub btc_tx_hash: H256,
	pub ethereum_receiver: H160,
	pub ethereum_tx_hash: H256,
	pub eth_tx_status: bool,
	pub symbol: String,
	pub amount: U256,
	pub tx_timestamp: u64,
}

impl BtcToEth {
	fn new(
		tx_index: u64,
		btc_tx_hash: H256,
		ethereum_receiver: H160,
		ethereum_tx_hash: H256,
		eth_tx_status: bool,
		symbol: String,
		amount: U256,
		tx_timestamp: u64,
	) -> Self {
		Self {
			tx_index,
			btc_tx_hash,
			ethereum_receiver,
			ethereum_tx_hash,
			eth_tx_status,
			symbol,
			amount,
			tx_timestamp,
		}
	}
}

/// custodial bit cross record of a full single tx from ETH to BTC
#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub struct EthToBtc {
	pub tx_index: u64,
	pub btc_tx_hash: H256,
	pub btc_receiver: PubKey,
	pub btc_receiver_length: u32,
	pub ethereum_tx_hash: H256,
	pub eth_tx_status: bool,
	pub symbol: String,
	pub amount: U256,
	pub tx_timestamp: u64,
}

impl EthToBtc {
	fn new(
		tx_index: u64,
		btc_tx_hash: H256,
		btc_receiver: PubKey,
		btc_receiver_length: u32,
		ethereum_tx_hash: H256,
		eth_tx_status: bool,
		symbol: String,
		amount: U256,
		tx_timestamp: u64,
	) -> Self {
		Self {
			tx_index,
			btc_tx_hash,
			btc_receiver,
			btc_receiver_length,
			ethereum_tx_hash,
			eth_tx_status,
			symbol,
			amount,
			tx_timestamp,
		}
	}
}
