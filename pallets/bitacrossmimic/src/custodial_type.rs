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
use alloc::string::String;
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
