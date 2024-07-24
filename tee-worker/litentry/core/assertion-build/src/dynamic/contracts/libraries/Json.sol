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

library Json {
    function getString(string memory json, string memory pointer)
        internal
        returns (bool, string memory)
    {
        bool success;
        string memory value;
        bytes memory encoded_params = abi.encode(json, pointer);

        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x044C,
                    0,
                    add(encoded_params, 0x20),
                    encoded_params_len,
                    memPtr,
                    0x1000
                )
            ) {
                revert(0, 0)
            }
            success := mload(memPtr)
            value := add(memPtr, 0x40)
            mstore(0x40, add(memPtr, 0x1000))
        }

        return (success, value);
    }

    function getI64(string memory json, string memory pointer)
        internal
        returns (bool, int64)
    {
        bool success;
        int64 value;

        bytes memory encoded_params = abi.encode(json, pointer);
        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x044D,
                    0,
                    add(encoded_params, 0x20),
                    encoded_params_len,
                    memPtr,
                    0x40
                )
            ) {
                revert(0, 0)
            }
            success := mload(memPtr)
            value := mload(add(memPtr, 0x20))
            mstore(0x40, add(memPtr, 0x40))
        }

        return (success, value);
    }

    function getBool(string memory json, string memory pointer)
        internal
        returns (bool, bool)
    {
        bool success;
        bool value;

        bytes memory encoded_params = abi.encode(json, pointer);
        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x044E,
                    0,
                    add(encoded_params, 0x20),
                    encoded_params_len,
                    memPtr,
                    0x40
                )
            ) {
                revert(0, 0)
            }
            success := mload(memPtr)
            value := mload(add(memPtr, 0x20))
            mstore(0x40, add(memPtr, 0x40))
        }

        return (success, value);
    }

    function getArrayLen(string memory json, string memory pointer)
        internal
        returns (bool, int64)
    {
        bool success;
        int64 value;

        bytes memory encoded_params = abi.encode(json, pointer);
        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x044F,
                    0,
                    add(encoded_params, 0x20),
                    encoded_params_len,
                    memPtr,
                    0x40
                )
            ) {
                revert(0, 0)
            }
            success := mload(memPtr)
            value := mload(add(memPtr, 0x20))
            mstore(0x40, add(memPtr, 0x40))
        }

        return (success, value);
    }
}
