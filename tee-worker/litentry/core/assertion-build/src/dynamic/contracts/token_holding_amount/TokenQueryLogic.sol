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
    function queryBalance(
        Identity memory identity,
        uint32 network,
        string[] memory secrets,
        string memory tokenName,
        TokenInfo memory token
    ) internal override returns (uint256) {
        (bool identityToStringSuccess, string memory identityString) = Utils
            .identityToString(network, identity.value);

        if (identityToStringSuccess) {
            uint256 totalBalance = 0;

            (
                string memory tokenContractAddress,
                uint8 dataProviderType,
                uint8 decimals
            ) = getTokenInfoNetwork(token, network);
            uint8 tokenDecimals = token.maxDecimals;
            uint8 tokenDecimalsDiff = tokenDecimals - decimals;

            if (
                dataProviderType == DataProviderTypes.GeniidataClient &&
                GeniidataClient.isSupportedNetwork(network)
            ) {
                uint256 balance = GeniidataClient.getTokenBalance(
                    secrets[0],
                    identityString,
                    tokenName,
                    tokenDecimals
                );

                totalBalance += balance;
            } else if (
                dataProviderType == DataProviderTypes.NoderealClient &&
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
                    // Nodereal returns balance without decimals, so need multiply by the diff between maxDecimals and decimals.
                    totalBalance += balance * 10 ** tokenDecimalsDiff;
                }
            } else if (
                dataProviderType == DataProviderTypes.MoralisClient &&
                MoralisClient.isSupportedNetwork(network)
            ) {
                uint256 balance = MoralisClient.getTokenBalance(
                    network,
                    secrets[2],
                    identityString,
                    tokenContractAddress,
                    tokenDecimals
                );
                totalBalance += balance;
            } else if (
                dataProviderType == DataProviderTypes.BlockchainInfoClient &&
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

    function getTokenInfoNetwork(
        TokenInfo memory token,
        uint32 network
    ) private pure returns (string memory, uint8, uint8) {
        string memory tokenAddress;
        uint8 dataProviderType;
        uint8 decimals;
        for (uint i = 0; i < token.networks.length; i++) {
            if (token.networks[i].network == network) {
                tokenAddress = token.networks[i].tokenAddress;
                dataProviderType = token.networks[i].dataProvierType;
                decimals = token.networks[i].decimals;
                return (tokenAddress, dataProviderType, decimals);
            }
        }
        revert("TokenInfoNetwork not found");
    }
}
