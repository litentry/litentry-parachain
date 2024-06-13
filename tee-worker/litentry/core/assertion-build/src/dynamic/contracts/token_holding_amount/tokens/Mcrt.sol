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

contract Mcrt is ERC20 {
	function getTokenName() internal pure override returns (string memory) {
		return "mcrt";
	}

	function getTokenContractAddress()
		internal
		pure
		override
		returns (string[] memory)
	{
		string[] memory addresses = new string[](2);

		// [0] is bsc address, [1] is eth address
		addresses[0] = "0x4b8285aB433D8f69CB48d5Ad62b415ed1a221e4f";
		addresses[1] = "0xde16ce60804a881e9f8c4ebb3824646edecd478d";
		return addresses;
	}

	function getTokenRanges()
		internal
		pure
		override
		returns (uint256[] memory)
	{
		uint256[] memory ranges = new uint256[](6);
		ranges[0] = 0 * decimals_factor;
		ranges[1] = 2000 * decimals_factor;
		ranges[2] = 10000 * decimals_factor;
		ranges[3] = 50000 * decimals_factor;
		ranges[4] = 150000 * decimals_factor;
		ranges[5] = 500000 * decimals_factor;

		return ranges;
	}
}
