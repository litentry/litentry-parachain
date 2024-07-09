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

library Eth {
	function getTokenBscAddress() internal pure returns (string memory) {
		return "0x2170ed0880ac9a755fd29b2688956bd959f933f8";
	}
	function getTokenEthereumAddress() internal pure returns (string memory) {
		return "Native Token";
	}

	function getTokenName() internal pure returns (string memory) {
		return "eth";
	}

	function getTokenRanges() internal pure returns (uint256[] memory) {
		uint256[] memory ranges = new uint256[](10);

		// all ranges multiplied by decimals_factor(1000).
		// pub const ETH_AMOUNT_RANGE: [f64; 10] = [0.0, 0.01, 0.05, 0.2, 0.6, 1.2, 3.0, 8.0, 20.0, 50.0];
		ranges[0] = 0;
		ranges[1] = 10;
		ranges[2] = 50;
		ranges[3] = 200;
		ranges[4] = 600;
		ranges[5] = 1200;
		ranges[6] = 3000;
		ranges[7] = 8000;
		ranges[8] = 20000;
		ranges[9] = 50000;

		return ranges;
	}
	function getTokenNetworks() internal pure returns (uint32[] memory) {
		uint32[] memory networks = new uint32[](2);
		networks[0] = Web3Networks.Ethereum;
		networks[1] = Web3Networks.Bsc;

		return networks;
	}
}
