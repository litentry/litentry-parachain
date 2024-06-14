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

contract Etc is ERC20 {
	constructor() {
		// Initialize network token addresses
		networkTokenAddresses[
			Web3Networks.Ethereum
		] = "";
		networkTokenAddresses[
			Web3Networks.Bsc
		] = "0x3d6545b08693dae087e957cb1180ee38b9e3c25e";
		// Add more addresses as needed
	}

	function getTokenName() internal pure override returns (string memory) {
		return "etc";
	}

	function getTokenRanges()
		internal
		pure
		override
		returns (uint256[] memory)
	{
		uint256[] memory ranges = new uint256[](6);
		ranges[0] = 0 * decimals_factor;
		ranges[1] = 1 * decimals_factor;
		ranges[2] = 5 * decimals_factor;
		ranges[3] = 20 * decimals_factor;
		ranges[4] = 50 * decimals_factor;
		ranges[5] = 80 * decimals_factor;

		return ranges;
	}
}
