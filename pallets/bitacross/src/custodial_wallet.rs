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

use codec::{Decode, Encode};
use scale_info::TypeInfo;

pub type PubKey = [u8; 33];

/// custodial wallet that each tee worker generates and holds
#[derive(Encode, Decode, Clone, Default, Debug, PartialEq, Eq, TypeInfo)]
pub struct CustodialWallet {
	pub btc: Option<PubKey>,
	pub eth: Option<PubKey>,
}

impl CustodialWallet {
	pub fn has_btc(&self) -> bool {
		self.btc.is_some()
	}

	pub fn has_eth(&self) -> bool {
		self.eth.is_some()
	}
}
