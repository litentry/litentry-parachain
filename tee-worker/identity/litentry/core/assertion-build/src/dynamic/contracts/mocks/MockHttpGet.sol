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
import { HttpHeader } from "../libraries/Http.sol";

import "hardhat/console.sol";

contract MockHttpGet {
    receive() external payable {}

    fallback() external payable {
        (string memory url, ) = abi.decode(msg.data, (string, HttpHeader[]));

        bool success = true;
        string memory value = "";

        // moralis
        if (
            Strings.equal(
                url,
                "https://deep-index.moralis.io/api/v2.2/0x50BcC2FEA4A95283b196bdEF4DEa5B27AFD6323c/erc20?chain=polygon&token_addresses[0]=0xac51C4c48Dc3116487eD4BC16542e27B5694Da1b"
            )
        ) {
            value = '[{"token_address":"0xac51C4c48Dc3116487eD4BC16542e27B5694Da1b","balance":"30"}]';
        } else if (
            Strings.equal(
                url,
                "https://deep-index.moralis.io/api/v2.2/0xbF98D4df371c2dE965a36E02b4c2E0DA89090818/erc20?chain=arbitrum&token_addresses[0]=0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1"
            )
        ) {
            value = '[{"token_address":"0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1","balance":"5"}]';
        }

        console.log("http_get>>", url, value);

        bytes memory encodedResult = abi.encode(success, value);

        assembly {
            return(add(encodedResult, 0x20), mload(encodedResult))
        }
    }
}
