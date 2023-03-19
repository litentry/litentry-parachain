// Copyright 2020-2023 Litentry Technologies GmbH.
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

use ethereum_types::H160;
use hex_literal::hex;

/// The ethereum contract address of vc oracle 
/// TODO: Not the actual contract address yet
pub fn vc_ethereum_contract() -> H160 {
	hex!("97Cb2a379bb6d66825a3df6a665B1FFe2392Ed35").into()
}