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

import { BRC20 } from "../BRC20.sol";
import "../Constants.sol";

library Sats {
	function getTokenName() internal pure returns (string memory) {
		return "sats";
	}

	function getTokenRanges() internal pure returns (uint256[] memory) {
		uint256[] memory ranges = new uint256[](9);
		ranges[0] = 0 * Constants.decimals_factor;
		ranges[1] = 1 * Constants.decimals_factor;
		ranges[2] = 40000000 * Constants.decimals_factor;
		ranges[3] = 200000000 * Constants.decimals_factor;
		ranges[4] = 500000000 * Constants.decimals_factor;
		ranges[5] = 1000000000 * Constants.decimals_factor;
		ranges[6] = 2000000000 * Constants.decimals_factor;
		ranges[7] = 4000000000 * Constants.decimals_factor;
		ranges[8] = 6000000000 * Constants.decimals_factor;
		return ranges;
	}
}