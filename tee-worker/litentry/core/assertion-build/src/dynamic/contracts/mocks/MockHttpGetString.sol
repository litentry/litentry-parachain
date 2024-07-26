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

contract MockHttpGetString {
    receive() external payable {}

    fallback() external payable {
        (string memory url, string memory jsonPointer, ) = abi.decode(
            msg.data,
            (string, string, HttpHeader[])
        );

        bool success = true;
        string memory value = "0";

        if (
            Strings.equal(
                url,
                "https://api.geniidata.com/api/1/brc20/balance?tick=ordi&address=1DtMsV2JDUVccDU5V5zdV4mdBf5KRHaJ7Z"
            )
        ) {
            value = "0.1";
        } else if (
            Strings.equal(
                url,
                "https://api.geniidata.com/api/1/brc20/balance?tick=ordi&address=17dUgEh3jSnGrNmtMaPrVHvVgjNiqkHHhb"
            )
        ) {
            value = "1";
        } else if (
            Strings.equal(
                url,
                "https://api.geniidata.com/api/1/brc20/balance?tick=ordi&address=1BCtecRbWLi1NYzfj9CNszJhCh3c2LXGPd"
            )
        ) {
            value = "1.1";
        } else if (
            Strings.equal(
                url,
                "https://api.geniidata.com/api/1/brc20/balance?tick=ordi&address=1MWAjD8eSqHro35WVcWV3N3VGfyzCsiMVM"
            )
        ) {
            value = "600.1";
        } else if (
            Strings.equal(
                url,
                "https://api.geniidata.com/api/1/brc20/balance?tick=ordi&address=parse_decimal_fail"
            )
        ) {
            value = "parse_decimal_fail";
        } else if (
            Strings.equal(
                url,
                "https://api.geniidata.com/api/1/brc20/balance?tick=ordi&address=httt_get_string_fail"
            )
        ) {
            success = false;
        }

        console.log("http_get_string>>", url, jsonPointer, value);

        bytes memory encodedResult = abi.encode(success, value);

        assembly {
            return(add(encodedResult, 0x20), mload(encodedResult))
        }
    }
}
