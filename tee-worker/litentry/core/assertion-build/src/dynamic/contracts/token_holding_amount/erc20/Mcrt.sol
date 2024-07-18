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

library Mcrt {
	function getTokenAddress(
		uint32 network
	) internal pure returns (string memory) {
		if (network == Web3Networks.Ethereum) {
			return "0xde16ce60804a881e9f8c4ebb3824646edecd478d";
		} else if (network == Web3Networks.Bsc) {
			return "0x4b8285aB433D8f69CB48d5Ad62b415ed1a221e4f";
		} else if (network == Web3Networks.Solana) {
			return "FADm4QuSUF1K526LvTjvbJjKzeeipP6bj5bSzp3r6ipq";
		}
		revert("Unsupported network");
	}

	function getTokenRanges() internal pure returns (uint256[] memory) {
		uint256[] memory ranges = new uint256[](6);
		ranges[0] = 0 * Constants.decimals_factor;
		ranges[1] = 2000 * Constants.decimals_factor;
		ranges[2] = 10000 * Constants.decimals_factor;
		ranges[3] = 50000 * Constants.decimals_factor;
		ranges[4] = 150000 * Constants.decimals_factor;
		ranges[5] = 500000 * Constants.decimals_factor;

		return ranges;
	}
	function getTokenNetworks() internal pure returns (uint32[] memory) {
		uint32[] memory networks = new uint32[](3);
		networks[0] = Web3Networks.Ethereum;
		networks[1] = Web3Networks.Bsc;
		networks[2] = Web3Networks.Solana;

		return networks;
	}
}
