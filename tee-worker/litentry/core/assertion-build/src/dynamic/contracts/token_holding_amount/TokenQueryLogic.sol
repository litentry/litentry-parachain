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

import "@openzeppelin/contracts/utils/Strings.sol";
import "../libraries/Identities.sol";
import "../libraries/Utils.sol";
import { TokenHoldingAmount } from "./TokenHoldingAmount.sol";
import { NoderealClient } from "./NoderealClient.sol";
import { GeniidataClient } from "./GeniidataClient.sol";
import { BlockchainInfoClient } from "./BlockchainInfoClient.sol";
import "./MoralisClient.sol";
import "./Constants.sol";

abstract contract TokenQueryLogic is TokenHoldingAmount {
    mapping(string => TokenInfo[]) internal tokenInfo;
    uint8 tokenDecimals;

    function getTokenDecimals() internal view override returns (uint8) {
        return tokenDecimals;
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
            uint256 totalBalance = 0;

            (
                string memory tokenContractAddress,
                uint8 dataproviderType,
                uint8 decimals
            ) = getTokenInfo(tokenName, network);
            tokenDecimals = decimals;

            if (
                dataproviderType == DataProviderTypes.GeniidataClient &&
                GeniidataClient.isSupportedNetwork(network)
            ) {
                uint256 balance = GeniidataClient.getTokenBalance(
                    secrets[0],
                    identityString,
                    tokenName,
                    getTokenDecimals()
                );

                totalBalance += balance;
            } else if (
                dataproviderType == DataProviderTypes.NoderealClient &&
                NoderealClient.isSupportedNetwork(network)
            ) {
                (bool success, uint256 balance) = NoderealClient
                    .getTokenBalance(
                        network,
                        secrets[1],
                        tokenContractAddress,
                        identityString
                    );
                if (success) {
                    totalBalance += balance;
                }
            } else if (
                dataproviderType == DataProviderTypes.MoralisClient &&
                MoralisClient.isSupportedNetwork(network)
            ) {
                uint256 balance = MoralisClient.getTokenBalance(
                    network,
                    secrets[2],
                    identityString,
                    tokenContractAddress,
                    getTokenDecimals()
                );
                totalBalance += balance;
            } else if (
                dataproviderType == DataProviderTypes.BlockchainInfoClient &&
                BlockchainInfoClient.isSupportedNetwork(network)
            ) {
                string[] memory accounts = new string[](1);
                accounts[0] = identityString;
                uint256 balance = BlockchainInfoClient.getTokenBalance(
                    accounts
                );
                totalBalance += balance;
            }
            return totalBalance;
        }
        return 0;
    }

    function isSupportedNetwork(
        string memory tokenName,
        uint32 network
    ) internal view override returns (bool) {
        TokenInfo[] memory infoArray = tokenInfo[tokenName];
        for (uint32 i = 0; i < infoArray.length; i++) {
            if (network == infoArray[i].network) {
                return true;
            }
        }
        return false;
    }

    function getTokenInfo(
        string memory tokenName,
        uint32 network
    ) internal view returns (string memory, uint8, uint8) {
        string memory tokenAddress;
        uint8 dataProviderType;
        uint8 decimals;
        for (uint i = 0; i < tokenInfo[tokenName].length; i++) {
            if (tokenInfo[tokenName][i].network == network) {
                tokenAddress = tokenInfo[tokenName][i].tokenAddress;
                dataProviderType = tokenInfo[tokenName][i].dataprovierType;
                decimals = tokenInfo[tokenName][i].decimals;
                return (tokenAddress, dataProviderType, decimals);
            }
        }
        revert("TokenInfo not found");
    }
}
