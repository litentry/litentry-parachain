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
	mapping(uint32 => string) internal networkUrls;
	mapping(uint32 => bool) private queriedNetworks;
	mapping(uint32 => string) tokenAddresses;
	mapping(string => string) internal tokenBscAddress;
	mapping(string => string) internal tokenEthereumAddress;

	constructor() {
		networkUrls[Web3Networks.Bsc] = "https://bsc-mainnet.nodereal.io/v1/";
		networkUrls[
			Web3Networks.Ethereum
		] = "https://eth-mainnet.nodereal.io/v1/";
		// Add more networks as needed
		// below url is used for test against mock server
		// "http://localhost:19530/nodereal_jsonrpc/v1/",

		// Initialize network token addresses using Ada library

		
	}

	function getTokenDecimals() internal pure override returns (uint8) {
		return 18;
	}

	function queryBalance(
		Identity memory identity,
		uint32 network,
		string[] memory secrets,
		string memory tokenName
	) internal override returns (uint256) {
		(bool identityToStringSuccess, string memory identityString) = Utils
			.identityToString(network, identity.value);

		tokenAddresses[Web3Networks.Bsc] = tokenBscAddress[tokenName];
		tokenAddresses[Web3Networks.Ethereum] = tokenEthereumAddress[
			tokenName
		];
		if (identityToStringSuccess) {
			string memory url;
			uint32[] memory networks = getTokenNetworks();
			uint256 totalBalance = 0;

			for (uint32 i = 0; i < networks.length; i++) {
				// Check if this network has been queried
				if (!queriedNetworks[networks[i]]) {
					string memory _tokenContractAddress = tokenAddresses[
						networks[i]
					];

					url = string(
						abi.encodePacked(networkUrls[networks[i]], secrets[0])
					);

					(bool success, uint256 balance) = NoderealClient
						.getTokenBalance(
							url,
							_tokenContractAddress,
							identityString
						);

					if (success) {
						totalBalance += balance;
					}
					// Mark this network as queried
					queriedNetworks[networks[i]] = true;
				}
			}
			return totalBalance;
		}
		return 0;
	}

	function getTokenNetworks() internal pure returns (uint32[] memory) {
		uint32[] memory networks = new uint32[](2);
		networks[0] = Web3Networks.Ethereum;
		networks[1] = Web3Networks.Bsc;
		// Add more networks as needed
		return networks;
	}

	function isSupportedNetwork(
		uint32 network
	) internal pure override returns (bool) {
		return network == Web3Networks.Bsc || network == Web3Networks.Ethereum;
	}
}
