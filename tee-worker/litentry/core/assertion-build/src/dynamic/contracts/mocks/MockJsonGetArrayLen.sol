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

contract MockJsonGetArrayLen {
    receive() external payable {}

    fallback() external payable {
        (string memory json, string memory pointer) = abi.decode(
            msg.data,
            (string, string)
        );

        bool success = true;
        int64 value = 0;

        if (
            Strings.equal(
                json,
                '[{"token_address":"0xac51C4c48Dc3116487eD4BC16542e27B5694Da1b","balance":"30"}]'
            ) && Strings.equal(pointer, "")
        ) {
            value = 1;
        } else if (
            Strings.equal(
                json,
                '[{"token_address":"0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1","balance":"5"}]'
            ) && Strings.equal(pointer, "")
        ) {
            value = 1;
        }

        console.log("json_get_array_len>>", json, pointer);

        bytes memory encodedResult = abi.encode(success, value);

        assembly {
            return(add(encodedResult, 0x20), mload(encodedResult))
        }
    }
}
