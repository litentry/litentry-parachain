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

contract Imx is ERC20 {
	constructor() {
		// Initialize network token addresses
		networkTokenAddresses[
			Web3Networks.Ethereum
		] = "0xf57e7e7c23978c3caec3c3548e3d615c346e79ff";
		networkTokenAddresses[
			Web3Networks.Bsc
		] = "";
		// Add more addresses as needed
	}

	function getTokenName() internal pure override returns (string memory) {
		return "imx";
	}

	function getTokenRanges()
		internal
		pure
		override
		returns (uint256[] memory)
	{
		uint256[] memory ranges = new uint256[](8);
		ranges[0] = 0 * decimals_factor;
		ranges[1] = 10 * decimals_factor;
		ranges[2] = 30 * decimals_factor;
		ranges[3] = 80 * decimals_factor;
		ranges[4] = 200 * decimals_factor;
		ranges[5] = 500 * decimals_factor;
		ranges[6] = 1000 * decimals_factor;
		ranges[7] = 2000 * decimals_factor;

		return ranges;
	}
}
