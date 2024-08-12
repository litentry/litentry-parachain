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

contract MockHttpGetBool {
    receive() external payable {}

    fallback() external payable {
        (string memory url, string memory jsonPointer, ) = abi.decode(
            msg.data,
            (string, string, HttpHeader[])
        );

        bool success = true;
        bool value = false;

        if (
            Strings.equal(
                url,
                "https://daren.market/api/talent-asset?address=0x96aEb2216810C624131c51141da612808103d319"
            )
        ) {
            value = true;
        } else if (
            Strings.equal(
                url,
                "https://daren.market/api/talent-asset?address=success_false"
            )
        ) {
            success = false;
        }

        console.log("http_get_bool>>", url, jsonPointer, value);

        bytes memory encodedResult = abi.encode(success, value);

        assembly {
            return(add(encodedResult, 0x20), mload(encodedResult))
        }
    }
}
