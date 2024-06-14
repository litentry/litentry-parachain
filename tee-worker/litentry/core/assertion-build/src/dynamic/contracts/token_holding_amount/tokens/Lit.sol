// SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity ^0.8.8;

import { ERC20 } from "../ERC20.sol";
import "../../libraries/Identities.sol";

contract Lit is ERC20 {
	constructor() {
		// Initialize network token addresses
		networkTokenAddresses[
			Web3Networks.Ethereum
		] = "0xb59490ab09a0f526cc7305822ac65f2ab12f9723";
		networkTokenAddresses[
			Web3Networks.Bsc
		] = "0xb59490ab09a0f526cc7305822ac65f2ab12f9723";
		// Add more addresses as needed
	}

	function getTokenName() internal pure override returns (string memory) {
		return "lit";
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
