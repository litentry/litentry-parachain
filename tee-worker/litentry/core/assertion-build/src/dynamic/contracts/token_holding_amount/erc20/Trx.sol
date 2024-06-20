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
import "../Constants.sol";


library Trx {
	function getTokenBscAddress() internal pure returns (string memory) {
		return "0xCE7de646e7208a4Ef112cb6ed5038FA6cC6b12e3";
	}
	function getTokenEthereumAddress() internal pure returns (string memory) {
		return "0x50327c6c5a14dcade707abad2e27eb517df87ab5";
	}

	function getTokenName() internal pure returns (string memory) {
		return "trx";
	}

	function getTokenRanges() internal pure returns (uint256[] memory) {
		uint256[] memory ranges = new uint256[](10);
		ranges[0] = 0 * Constants.decimals_factor;
		ranges[1] = 1 * Constants.decimals_factor;
		ranges[2] = 50 * Constants.decimals_factor;
		ranges[3] = 100 * Constants.decimals_factor;
		ranges[4] = 200 * Constants.decimals_factor;
		ranges[5] = 500 * Constants.decimals_factor;
		ranges[6] = 800 * Constants.decimals_factor;
		ranges[7] = 1200 * Constants.decimals_factor;
		ranges[8] = 1600 * Constants.decimals_factor;
		ranges[9] = 3000 * Constants.decimals_factor;

		return ranges;
	}
}