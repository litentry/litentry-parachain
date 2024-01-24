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

use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::{vec, vec::Vec};

use crate::Web3Network;

#[derive(Encode, Decode, Clone, Debug, PartialEq, Eq, MaxEncodedLen, TypeInfo)]
pub enum Web3TokenType {
	#[codec(index = 0)]
	Bnb,
	#[codec(index = 1)]
	Eth,
	#[codec(index = 2)]
	SpaceId,
	#[codec(index = 3)]
	Lit,
	#[codec(index = 4)]
	Wbtc,
	#[codec(index = 5)]
	Usdc,
	#[codec(index = 6)]
	Usdt,
	#[codec(index = 7)]
	Crv,
	#[codec(index = 8)]
	Matic,
	#[codec(index = 9)]
	Dydx,
	#[codec(index = 10)]
	Amp,
	#[codec(index = 11)]
	Cvx,
	#[codec(index = 12)]
	Tusd,
	#[codec(index = 13)]
	Usdd,
	#[codec(index = 14)]
	Gusd,
	#[codec(index = 15)]
	Link,
	#[codec(index = 16)]
	Grt,
	#[codec(index = 17)]
	Comp,
	#[codec(index = 18)]
	People,
	#[codec(index = 19)]
	Gtc,
	#[codec(index = 20)]
	Ton,
	#[codec(index = 21)]
	Trx,
}

impl Web3TokenType {
	pub fn get_supported_networks(&self) -> Vec<Web3Network> {
		match self {
			Self::Bnb | Self::Eth | Self::SpaceId | Self::Ton | Self::Trx =>
				vec![Web3Network::Bsc, Web3Network::Ethereum],
			Self::Lit => vec![
				Web3Network::Bsc,
				Web3Network::Ethereum,
				Web3Network::Litentry,
				Web3Network::Litmus,
			],
			_ => vec![Web3Network::Ethereum],
		}
	}
}
