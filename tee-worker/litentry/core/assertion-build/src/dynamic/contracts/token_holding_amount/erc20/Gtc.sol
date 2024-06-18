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

contract Gtc is ERC20 {
	constructor() {
		// Initialize network token addresses
		networkTokenAddresses[
			Web3Networks.Ethereum
		] = "0xde30da39c46104798bb5aa3fe8b9e0e1f348163f";
		networkTokenAddresses[
			Web3Networks.Bsc
		] = "";
		// Add more addresses as needed
	}

	function getTokenName() internal pure override returns (string memory) {
		return "gtc";
	}

	function getTokenRanges()
		internal
		pure
		override
		returns (uint256[] memory)
	{
		uint256[] memory ranges = new uint256[](10);
		ranges[0] = 0 * decimals_factor;
		ranges[1] = 1 * decimals_factor;
		ranges[2] = 50 * decimals_factor;
		ranges[3] = 100 * decimals_factor;
		ranges[4] = 200 * decimals_factor;
		ranges[5] = 500 * decimals_factor;
		ranges[6] = 800 * decimals_factor;
		ranges[7] = 1200 * decimals_factor;
		ranges[8] = 1600 * decimals_factor;
		ranges[9] = 3000 * decimals_factor;

		return ranges;
	}
}
