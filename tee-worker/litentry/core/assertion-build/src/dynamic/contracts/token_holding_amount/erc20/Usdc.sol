// SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity ^0.8.8;

import "../../libraries/Identities.sol";
import "../Constants.sol";

library Usdc {
	function getTokenAddress(
		uint32 network
	) internal pure returns (string memory) {
		if (network == Web3Networks.Ethereum) {
			return "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48";
		} else if (network == Web3Networks.Bsc) {
			return "0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d";
		} else if (network == Web3Networks.Solana) {
			return "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
		}
	}

	function getTokenRanges() internal pure returns (uint256[] memory) {
		uint256[] memory ranges = new uint256[](9);
		ranges[0] = 0 * Constants.decimals_factor;
		ranges[1] = 10 * Constants.decimals_factor;
		ranges[2] = 30 * Constants.decimals_factor;
		ranges[3] = 80 * Constants.decimals_factor;
		ranges[4] = 200 * Constants.decimals_factor;
		ranges[5] = 500 * Constants.decimals_factor;
		ranges[6] = 1000 * Constants.decimals_factor;
		ranges[7] = 2000 * Constants.decimals_factor;
		ranges[8] = 5000 * Constants.decimals_factor;

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
