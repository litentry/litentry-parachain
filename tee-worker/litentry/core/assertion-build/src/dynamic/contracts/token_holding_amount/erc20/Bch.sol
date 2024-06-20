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

// SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity ^0.8.8;

import { ERC20 } from "../ERC20.sol";
import "../Constants.sol";


library Bch {
	function getTokenBscAddress() internal pure returns (string memory) {
		return "0x8fF795a6F4D97E7887C79beA79aba5cc76444aDf";
	}

	function getTokenEthereumAddress() internal pure returns (string memory) {
		return "";
	}
	function getTokenName() internal pure returns (string memory) {
		return "bch";
	}

	function getTokenRanges() internal pure returns (uint256[] memory) {
		uint256[] memory ranges = new uint256[](6);

		// all ranges multiplied by decimals_factor(1000).
		// pub const BCH_AMOUNT_RANGE: [f64; 6] = [0.0, 0.1, 0.5, 2.0, 6.0, 12.0];

		ranges[0] = 0;
		ranges[1] = 100;
		ranges[2] = 500;
		ranges[3] = 2000;
		ranges[4] = 6000;
		ranges[5] = 12000;

		return ranges;
	}
}