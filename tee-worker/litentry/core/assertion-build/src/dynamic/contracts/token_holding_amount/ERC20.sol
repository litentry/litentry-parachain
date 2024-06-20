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
import { ERC20TokenLibrary } from "./erc20/ERC20TokenLibrary.sol";
contract ERC20 is TokenHoldingAmount {
	mapping(uint32 => string) internal networkUrls;
	mapping(uint32 => bool) private queriedNetworks;

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
		string memory /*tokenName*/
	) internal override returns (uint256) {
		(bool identityToStringSuccess, string memory identityString) = Utils
			.identityToString(network, identity.value);

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

	function getTokenInfo(
		string memory decodedParams
	)
		internal
		pure
		override
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName;
		uint256[] memory ranges;
		string memory tokenBscAddress;
		string memory tokenEthereumAddress;

		if (Utils.isStringsEqual(decodedParams, "ada")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getAdaInfo();
		} else if (Utils.isStringsEqual(decodedParams, "amp")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getAmpInfo();
		} else if (Utils.isStringsEqual(decodedParams, "atom")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getAtomInfo();
		} else if (Utils.isStringsEqual(decodedParams, "bch")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getBchInfo();
		} else if (Utils.isStringsEqual(decodedParams, "bean")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getBeanInfo();
		} else if (Utils.isStringsEqual(decodedParams, "bnb")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getBnbInfo();
		} else if (Utils.isStringsEqual(decodedParams, "comp")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getCompInfo();
		} else if (Utils.isStringsEqual(decodedParams, "cro")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getCroInfo();
		} else if (Utils.isStringsEqual(decodedParams, "crv")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getCrvInfo();
		} else if (Utils.isStringsEqual(decodedParams, "dai")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getDaiInfo();
		} else if (Utils.isStringsEqual(decodedParams, "doge")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getDogeInfo();
		} else if (Utils.isStringsEqual(decodedParams, "dydx")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getDydxInfo();
		} else if (Utils.isStringsEqual(decodedParams, "etc")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getEtcInfo();
		} else if (Utils.isStringsEqual(decodedParams, "eth")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getEthInfo();
		} else if (Utils.isStringsEqual(decodedParams, "fil")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getFilInfo();
		} else if (Utils.isStringsEqual(decodedParams, "grt")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getGrtInfo();
		} else if (Utils.isStringsEqual(decodedParams, "gtc")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getGtcInfo();
		} else if (Utils.isStringsEqual(decodedParams, "gusd")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getGusdInfo();
		} else if (Utils.isStringsEqual(decodedParams, "imx")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getImxInfo();
		} else if (Utils.isStringsEqual(decodedParams, "inj")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getInjInfo();
		} else if (Utils.isStringsEqual(decodedParams, "leo")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getLeoInfo();
		} else if (Utils.isStringsEqual(decodedParams, "link")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getLinkInfo();
		} else if (Utils.isStringsEqual(decodedParams, "lit")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getLitInfo();
		} else if (Utils.isStringsEqual(decodedParams, "matic")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getMaticInfo();
		} else if (Utils.isStringsEqual(decodedParams, "mcrt")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getMcrtInfo();
		} else if (Utils.isStringsEqual(decodedParams, "nfp")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getNfpInfo();
		} else if (Utils.isStringsEqual(decodedParams, "people")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getPeopleInfo();
		} else if (Utils.isStringsEqual(decodedParams, "shib")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getShibInfo();
		} else if (Utils.isStringsEqual(decodedParams, "sol")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getSolInfo();
		} else if (Utils.isStringsEqual(decodedParams, "spaceid")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getSpaceIdInfo();
		} else if (Utils.isStringsEqual(decodedParams, "ton")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getTonInfo();
		} else if (Utils.isStringsEqual(decodedParams, "trx")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getTrxInfo();
		} else if (Utils.isStringsEqual(decodedParams, "tusd")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getTusdInfo();
		} else if (Utils.isStringsEqual(decodedParams, "uni")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getUniInfo();
		} else if (Utils.isStringsEqual(decodedParams, "usdc")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getUsdcInfo();
		} else if (Utils.isStringsEqual(decodedParams, "usdt")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getUsdtInfo();
		} else if (Utils.isStringsEqual(decodedParams, "wbtc")) {
			(
				tokenName,
				ranges,
				tokenBscAddress,
				tokenEthereumAddress
			) = ERC20TokenLibrary.getWbtcInfo();
		} else {
			revert("Unsupported token");
		}

		return (tokenName, ranges, tokenBscAddress, tokenEthereumAddress);
	}
}
