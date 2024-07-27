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

import "hardhat/console.sol";

contract MockIdentityToString {
    receive() external payable {}

    fallback() external payable {
        (uint32 network_type, bytes memory identity_value) = abi.decode(
            msg.data,
            (uint32, bytes)
        );
        console.log(
            "identity_to_string>>",
            network_type,
            string(identity_value)
        );

        bool success = true;
        string memory value = string(identity_value);

        if (Strings.equal(value, "identity_to_string_fail")) {
            success = false;
        }

        bytes memory encodedResult = abi.encode(success, value);

        assembly {
            return(add(encodedResult, 0x20), mload(encodedResult))
        }
    }
}
