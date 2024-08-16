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
import "hardhat/console.sol";

contract MockParseDecimal {
    receive() external payable {}

    fallback() external payable {
        (string memory stringValue, uint8 decimals) = abi.decode(
            msg.data,
            (string, uint8)
        );

        bool success = true;
        uint256 value = 0;

        if (Strings.equal(stringValue, "0.1") && decimals == 18) {
            value = 1 * 10 ** 17;
        } else if (Strings.equal(stringValue, "1") && decimals == 18) {
            value = 1 * 10 ** 18;
        } else if (Strings.equal(stringValue, "1.1") && decimals == 18) {
            value = 11 * 10 ** 17;
        } else if (Strings.equal(stringValue, "5") && decimals == 18) {
            value = 5 * 10 ** 18;
        } else if (Strings.equal(stringValue, "30") && decimals == 18) {
            value = 30 * 10 ** 18;
        } else if (Strings.equal(stringValue, "600.1") && decimals == 18) {
            value = 6001 * 10 ** 17;
        } else if (Strings.equal(stringValue, "parse_decimal_fail")) {
            success = false;
        }

        console.log("parse_decimal>>", stringValue, decimals, value);

        bytes memory encodedResult = abi.encode(success, value);

        assembly {
            return(add(encodedResult, 0x20), mload(encodedResult))
        }
    }
}
