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
import { GeniidataClient } from "./GeniidataClient.sol";
abstract contract TokenQueryLogic is TokenHoldingAmount {
	mapping(uint32 => string) internal networkUrls;
	mapping(uint32 => bool) private queriedNetworks;
	mapping(string => mapping(uint32 => string)) tokenAddresses;
	mapping(string => string) internal tokenBscAddress;
	mapping(string => string) internal tokenEthereumAddress;
	mapping(string => uint32[]) internal tokenNetworks;

	constructor() {
		networkUrls[Web3Networks.Bsc] = "https://bsc-mainnet.nodereal.io/v1/"; // test against mock server => "http://localhost:19530/nodereal_jsonrpc/"
		networkUrls[
			Web3Networks.Ethereum
		] = "https://eth-mainnet.nodereal.io/v1/"; // test against mock server => "http://localhost:19530/nodereal_jsonrpc/"

		networkUrls[
			Web3Networks.BitcoinP2tr
		] = "https://api.geniidata.com/api/1/brc20/balance"; //  test against mock server => "http://localhost:19529/api/1/brc20/balance"
		// Add more networks as needed
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

		if (identityToStringSuccess) {
			string memory url;
			uint32[] memory networks = tokenNetworks[tokenName];
			uint256 totalBalance = 0;

			for (uint32 i = 0; i < networks.length; i++) {
				// Check if this network has been queried
				url = networkUrls[networks[i]];

				if (!queriedNetworks[networks[i]]) {
					string memory _tokenContractAddress = tokenAddresses[
						tokenName
					][networks[i]];
					if (networks[i] == Web3Networks.BitcoinP2tr) {
						uint256 balance = GeniidataClient.getTokenBalance(
							secrets,
							url,
							identityString,
							tokenName,
							getTokenDecimals()
						);
						totalBalance += balance;
					} else if (
						networks[i] == Web3Networks.Bsc ||
						networks[i] == Web3Networks.Ethereum
					) {
						(bool success, uint256 balance) = NoderealClient
							.getTokenBalance(
								url,
								secrets,
								_tokenContractAddress,
								identityString
							);
						if (success) {
							totalBalance += balance;
						}
					}
					// Mark this network as queried
					queriedNetworks[networks[i]] = true;
				}
			}
			return totalBalance;
		}
		return 0;
	}

	function isSupportedNetwork(
		uint32 network
	) internal pure override returns (bool) {
		return
			network == Web3Networks.Bsc ||
			network == Web3Networks.Ethereum ||
			network == Web3Networks.BitcoinP2tr;
	}
}
