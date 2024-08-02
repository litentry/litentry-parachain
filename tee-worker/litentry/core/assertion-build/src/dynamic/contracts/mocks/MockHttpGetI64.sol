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

contract MockHttpGetI64 {
    receive() external payable {}

    fallback() external payable {
        (string memory url, string memory jsonPointer, ) = abi.decode(
            msg.data,
            (string, string, HttpHeader[])
        );

        bool success = true;
        uint256 value = 0;

        if (
            Strings.equal(
                url,
                "https://blockchain.info/multiaddr?active=bc1pg6qjsrxwg9cvqx0gxstl0t74ynhs2528t7rp0u7acl6etwn5t6vswxrzpa&n=0"
            )
        ) {
            // 0.1(decimal is 8)
            value = 10000000;
        } else if (
            Strings.equal(
                url,
                "https://blockchain.info/multiaddr?active=bc1pqdk57wus42wuh989k3v700n6w584andwg7pvxnrd69ag3rs94cfq40qx2y&n=0"
            )
        ) {
            value = 0;
        }

        console.log("http_get_i64>", url, jsonPointer, value);

        bytes memory encodedResult = abi.encode(success, value);

        assembly {
            return(add(encodedResult, 0x20), mload(encodedResult))
        }
    }
}
