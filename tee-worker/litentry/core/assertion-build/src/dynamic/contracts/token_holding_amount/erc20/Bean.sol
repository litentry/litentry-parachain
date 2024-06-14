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
import "../../libraries/Identities.sol";

contract Bean is ERC20 {
	constructor() {
		// Initialize network token addresses
		networkTokenAddresses[
			Web3Networks.Ethereum
		] = "0xba7b9936a965fac23bb7a8190364fa60622b3cff";
		networkTokenAddresses[
			Web3Networks.Bsc
		] = "0x07da81e9a684ab87fad7206b3bc8d0866f48cc7c";
		// Add more addresses as needed
	}

	function getTokenName() internal pure override returns (string memory) {
		return "bean";
	}

	function getTokenRanges()
		internal
		pure
		override
		returns (uint256[] memory)
	{
		uint256[] memory ranges = new uint256[](5);
		ranges[0] = 0 * decimals_factor;
		ranges[1] = 1500 * decimals_factor;
		ranges[2] = 5000 * decimals_factor;
		ranges[3] = 10000 * decimals_factor;
		ranges[4] = 50000 * decimals_factor;

		return ranges;
	}
}
