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

import "../../libraries/Identities.sol";
import "../Constants.sol";

library Doge {
	function getTokenBscAddress() internal pure returns (string memory) {
		return "0xba2ae424d960c26247dd6c32edc70b295c744c43";
	}
	function getTokenEthereumAddress() internal pure returns (string memory) {
		return "";
	}

	function getTokenName() internal pure returns (string memory) {
		return "doge";
	}

	function getTokenRanges() internal pure returns (uint256[] memory) {
		uint256[] memory ranges = new uint256[](7);
		ranges[0] = 0 * Constants.decimals_factor;
		ranges[1] = 1000 * Constants.decimals_factor;
		ranges[2] = 5000 * Constants.decimals_factor;
		ranges[3] = 20000 * Constants.decimals_factor;
		ranges[4] = 50000 * Constants.decimals_factor;
		ranges[5] = 100000 * Constants.decimals_factor;
		ranges[6] = 300000 * Constants.decimals_factor;

		return ranges;
	}
	function getTokenNetworks() internal pure returns (uint32[] memory) {
		uint32[] memory networks = new uint32[](2);
		networks[0] = Web3Networks.Ethereum;
		networks[1] = Web3Networks.Bsc;

		return networks;
	}
}
