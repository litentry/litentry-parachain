// SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity ^0.8.8;

import { ERC20 } from "../ERC20.sol";
import "../../libraries/Identities.sol";

contract Usdc is ERC20 {
	constructor() {
		// Initialize network token addresses
		networkTokenAddresses[
			Web3Networks.Ethereum
		] = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
		networkTokenAddresses[
			Web3Networks.Bsc
		] = "0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d";
		// Add more addresses as needed
	}

	function getTokenName() internal pure override returns (string memory) {
		return "usdc";
	}

	function getTokenRanges()
		internal
		pure
		override
		returns (uint256[] memory)
	{
		uint256[] memory ranges = new uint256[](9);
		ranges[0] = 0 * decimals_factor;
		ranges[1] = 10 * decimals_factor;
		ranges[2] = 30 * decimals_factor;
		ranges[3] = 80 * decimals_factor;
		ranges[4] = 200 * decimals_factor;
		ranges[5] = 500 * decimals_factor;
		ranges[6] = 1000 * decimals_factor;
		ranges[7] = 2000 * decimals_factor;
		ranges[8] = 5000 * decimals_factor;

		return ranges;
	}
}
