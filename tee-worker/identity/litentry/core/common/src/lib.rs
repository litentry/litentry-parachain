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
// along with Litentry. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use litentry_primitives::Web3Network;

pub mod abort_strategy;

pub mod platform_user;
pub mod web3_nft;
pub mod web3_token;

pub fn web3_network_to_chain(network: &Web3Network) -> &'static str {
	match network {
		Web3Network::Polkadot => "polkadot",
		Web3Network::Kusama => "kusama",
		Web3Network::Litentry => "litentry",
		Web3Network::Litmus => "litmus",
		Web3Network::LitentryRococo => "litentry_rococo",
		Web3Network::Khala => "khala",
		Web3Network::SubstrateTestnet => "substrate_testnet",
		Web3Network::Ethereum => "ethereum",
		Web3Network::Bsc => "bsc",
		Web3Network::BitcoinP2tr => "bitcoin_p2tr",
		Web3Network::BitcoinP2pkh => "bitcoin_p2pkh",
		Web3Network::BitcoinP2sh => "bitcoin_p2sh",
		Web3Network::BitcoinP2wpkh => "bitcoin_p2wpkh",
		Web3Network::BitcoinP2wsh => "bitcoin_p2wsh",
		Web3Network::Polygon => "polygon",
		Web3Network::Arbitrum => "arbitrum",
		Web3Network::Solana => "solana",
		Web3Network::Combo => "combo",
	}
}
