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

import "../Constants.sol";
import "../../libraries/Identities.sol";

library Long {
	function getTokenName() internal pure returns (string memory) {
		return "long";
	}

	function getTokenRanges() internal pure returns (uint256[] memory) {
		uint256[] memory ranges = new uint256[](9);
		ranges[0] = 0 * Constants.decimals_factor;
		ranges[1] = 1 * Constants.decimals_factor;
		ranges[2] = 20 * Constants.decimals_factor;
		ranges[3] = 50 * Constants.decimals_factor;
		ranges[4] = 200 * Constants.decimals_factor;
		ranges[5] = 500 * Constants.decimals_factor;
		ranges[6] = 1000 * Constants.decimals_factor;
		ranges[7] = 2000 * Constants.decimals_factor;
		ranges[8] = 3000 * Constants.decimals_factor;
		return ranges;
	}
	function getTokenNetworks() internal pure returns (uint32[] memory) {
		uint32[] memory networks = new uint32[](1);
		networks[0] = Web3Networks.BitcoinP2tr;
		return networks;
	}
}
