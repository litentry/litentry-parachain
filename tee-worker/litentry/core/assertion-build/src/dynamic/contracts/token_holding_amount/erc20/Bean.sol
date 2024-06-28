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

library Bean {
	function getTokenBscAddress() internal pure returns (string memory) {
		return "0x07da81e9a684ab87fad7206b3bc8d0866f48cc7c";
	}
	function getTokenEthereumAddress() internal pure returns (string memory) {
		return "0xba7b9936a965fac23bb7a8190364fa60622b3cff";
	}
	function getTokenName() internal pure returns (string memory) {
		return "bean";
	}
	function getTokenRanges() internal pure returns (uint256[] memory) {
		uint256[] memory ranges = new uint256[](5);
		ranges[0] = 0 * Constants.decimals_factor;
		ranges[1] = 1500 * Constants.decimals_factor;
		ranges[2] = 5000 * Constants.decimals_factor;
		ranges[3] = 10000 * Constants.decimals_factor;
		ranges[4] = 50000 * Constants.decimals_factor;

		return ranges;
	}
	function getTokenNetworks() internal pure returns (uint32[] memory) {
		uint32[] memory networks = new uint32[](2);
		networks[0] = Web3Networks.Ethereum;
		networks[1] = Web3Networks.Bsc;

		return networks;
	}
}
