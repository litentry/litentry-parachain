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

enum LoggingLevel {
    Debug,
    Info,
    Warn,
    Err,
    Fatal
}

library Logging {
    function debug(string memory message) internal {
        log(LoggingLevel.Debug, message);
    }

    function info(string memory message) internal {
        log(LoggingLevel.Info, message);
    }

    function warn(string memory message) internal {
        log(LoggingLevel.Warn, message);
    }

    function fatal(string memory message) internal {
        log(LoggingLevel.Fatal, message);
    }

    function error(string memory message) internal {
        log(LoggingLevel.Err, message);
    }

    function log(LoggingLevel level, string memory message) internal {
        bytes memory encoded_params = abi.encode(level, message);
        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x041A,
                    0,
                    add(encoded_params, 0x20),
                    encoded_params_len,
                    memPtr,
                    0x40
                )
            ) {
                revert(0, 0)
            }
        }
    }
}
