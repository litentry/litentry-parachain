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
import {TokenHoldingAmount} from "./TokenHoldingAmount.sol";
import {NoderealClient} from "./NoderealClient.sol";

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
            string memory url;
            if (
            // For bip20 balance
                network == Web3Networks.Bsc) {
                url = string(
                abi.encodePacked(
                    // "https://bsc-mainnet.nodereal.io/v1/",
                    "http://localhost:19530/nodereal_jsonrpc/v1/",
                    secrets[0]
                )
            );
            } else if (

            // For erc20 balance
                network == Web3Networks.Ethereum

            ){
                
                url = string(
                abi.encodePacked(
                    // "https://ethereum-mainnet.nodereal.io/v1/",
                    "http://localhost:19530/nodereal_jsonrpc/v1/",
                    secrets[0]
                )
            );

            } else{

                revert("Unsupport network type");
            }

                string memory tokenContractAddress = getTokenName(); 
                   (bool success, uint256 balance) = NoderealClient.getErc20Balance(url, tokenContractAddress, identityString);
                if (success) {
                    return balance;
                }
        }
        return 0;
    }

    function isSupportedNetwork(uint32 network)
        internal
        pure
        override
        returns (bool)
    {
        return network == Web3Networks.Bsc || network == Web3Networks.Ethereum;
    }
}