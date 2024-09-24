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

contract MockHttpPostString {
    receive() external payable {}

    fallback() external payable {
        (
            string memory url,
            string memory jsonPointer,
            string memory payload,

        ) = abi.decode(msg.data, (string, string, string, HttpHeader[]));

        bool success = true;
        string memory value = "0";

        // nodereal eth
        if (Strings.equal(url, "https://eth-mainnet.nodereal.io/v1/0x12345")) {
            if (
                Strings.equal(
                    payload,
                    '{"jsonrpc": "2.0", "method": "nr_getTokenBalance20", "id": 1, "params": ["0x8D983cb9388EaC77af0474fA441C4815500Cb7BB","0xA7Ee59E733E613CC957FE203A2935E85cE39D08A", "latest"]}'
                )
            ) {
                value = "0x1";
            } else if (
                Strings.equal(
                    payload,
                    '{"jsonrpc": "2.0", "method": "nr_getTokenBalance20", "id": 1, "params": ["0x2260fac5e5542a773aa44fbcfedf7c193bc2c599","0x1C89Edd4FC080D71F92701C0794a16DbE573d4B8", "latest"]}'
                )
            ) {
                // 0.0001 * 10^18
                value = "0x5AF3107A4000";
            } else if (
                Strings.equal(
                    payload,
                    '{"jsonrpc": "2.0", "method": "nr_getTokenBalance20", "id": 1, "params": ["0x8D983cb9388EaC77af0474fA441C4815500Cb7BB","0x50BcC2FEA4A95283b196bdEF4DEa5B27AFD6323c", "latest"]}'
                )
            ) {
                // 50 * 10^6
                value = "0x2FAF080";
            }
        } else if (
            Strings.equal(url, "https://bsc-mainnet.nodereal.io/v1/0x12345")
        ) {
            if (
                Strings.equal(
                    payload,
                    '{"jsonrpc": "2.0", "method": "nr_getTokenBalance20", "id": 1, "params": ["0x0eb3a705fc54725037cc9e008bdede697f62f335","0xA7Ee59E733E613CC957FE203A2935E85cE39D08A", "latest"]}'
                )
            ) {
                // 2 * 10^18
                value = "0x1BC16D674EC80000";
            }
        } else if (
            Strings.equal(url, "https://combo-mainnet.nodereal.io/v1/0x12345")
        ) {
            if (
                Strings.equal(
                    payload,
                    '{"jsonrpc": "2.0", "method": "nr_getTokenBalance20", "id": 1, "params": ["0xba7b9936a965fac23bb7a8190364fa60622b3cff","0xa298cA90a4aa6029e26Dacc33b85c3847875615e", "latest"]}'
                )
            ) {
                // 1500 * 10^18
                value = "0x5150AE84A8CDF00000";
            } else if (
                Strings.equal(
                    payload,
                    '{"jsonrpc": "2.0", "method": "nr_getTokenBalance20", "id": 1, "params": ["0xba7b9936a965fac23bb7a8190364fa60622b3cff","0x0d4E9A8E1c26747c3d62a883b0Af5a916D6985c5", "latest"]}'
                )
            ) {
                // 60000 * 10^18
                value = "0xCB49B44BA602D800000";
            }
        }

        console.log("http_post_string>>", url, jsonPointer, value);

        bytes memory encodedResult = abi.encode(success, value);

        assembly {
            return(add(encodedResult, 0x20), mload(encodedResult))
        }
    }
}
