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
import "../libraries/Identities.sol";
import "../libraries/Utils.sol";
import { TokenHoldingAmount } from "./TokenHoldingAmount.sol";
import { NoderealClient } from "./NoderealClient.sol";

abstract contract ERC20 is TokenHoldingAmount {

	function getTokenDecimals() internal pure override returns (uint8) {
		return 18;
	}

	function queryBalance(
		Identity memory identity,
		uint32 network,
		string[] memory secrets
	) internal virtual override returns (uint256) {
		(bool identityToStringSuccess, string memory identityString) = Utils
			.identityToString(network, identity.value);

		if (identityToStringSuccess) {
			string[] memory _tokenContractAddresses = getTokenContractAddress();
			string memory url;

			// _tokenContractAddresses = [bsc-toen-contract-address, eth-token-contract-address]
			// We could add more dataproviders here.
			for (uint i = 0; i < _tokenContractAddresses.length; i++) {
				if (network == Web3Networks.Bsc && i == 0) {
					url = string(
						abi.encodePacked(
							"https://bsc-mainnet.nodereal.io/v1/",

                    		// 	below url is used for test against mock server
							// "http://localhost:19530/nodereal_jsonrpc/v1/",
							secrets[0]
						)
					);
				} else if (network == Web3Networks.Ethereum && i == 1) {
					url = string(
						abi.encodePacked(
							"https://eth-mainnet.nodereal.io/v1/",

                    		// 	below url is used for test against mock server
							// "http://localhost:19530/nodereal_jsonrpc/v1/",
							secrets[0]
						)
					);
				}

				(bool success, uint256 balance) = NoderealClient
					.getTokenBalance(
						url,
						_tokenContractAddresses[i],
						identityString
					);
				if (success) {
					return balance;
				}
			}
		}
		return 0;
	}
	function getTokenContractAddress()
		internal
		pure
		virtual
		returns (string[] memory);

	function isSupportedNetwork(
		uint32 network
	) internal pure override returns (bool) {
		return network == Web3Networks.Bsc || network == Web3Networks.Ethereum;
	}
}
