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

contract MockHexToNumber {
    receive() external payable {}

    fallback() external payable {
        string memory hexString = abi.decode(msg.data, (string));

        bool success = true;
        uint256 value = 0;

        if (Strings.equal(hexString, "0x1")) {
            value = 1;
        } else if (Strings.equal(hexString, "0x2FAF080")) {
            value = 50 * 10 ** 6;
        } else if (Strings.equal(hexString, "0x5AF3107A4000")) {
            value = 1 * 10 ** 14;
        } else if (Strings.equal(hexString, "0x1BC16D674EC80000")) {
            value = 2 * 10 ** 18;
        } else if (Strings.equal(hexString, "0x5150AE84A8CDF00000")) {
            value = 1500 * 10 ** 18;
        } else if (Strings.equal(hexString, "0xCB49B44BA602D800000")) {
            value = 60000 * 10 ** 18;
        }

        console.log("hex_to_number>", hexString, value);

        bytes memory encodedResult = abi.encode(success, value);

        assembly {
            return(add(encodedResult, 0x20), mload(encodedResult))
        }
    }
}
