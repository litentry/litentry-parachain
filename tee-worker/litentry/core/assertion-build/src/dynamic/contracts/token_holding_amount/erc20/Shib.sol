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

contract Shib is ERC20 {
	constructor() {
		// Initialize network token addresses
		networkTokenAddresses[
			Web3Networks.Ethereum
		] = "0x95ad61b0a150d79219dcf64e1e6cc01f0b64c4ce";
		networkTokenAddresses[
			Web3Networks.Bsc
		] = "";
		// Add more addresses as needed
	}

	function getTokenName() internal pure override returns (string memory) {
		return "shib";
	}

	function getTokenRanges()
		internal
		pure
		override
		returns (uint256[] memory)
	{
		uint256[] memory ranges = new uint256[](8);
		ranges[0] = 0 * decimals_factor;
		ranges[1] = 400000 * decimals_factor;
		ranges[2] = 2000000 * decimals_factor;
		ranges[3] = 10000000 * decimals_factor;
		ranges[4] = 20000000 * decimals_factor;
		ranges[5] = 40000000 * decimals_factor;
		ranges[6] = 100000000 * decimals_factor;
		ranges[7] = 200000000 * decimals_factor;

		return ranges;
	}
}
