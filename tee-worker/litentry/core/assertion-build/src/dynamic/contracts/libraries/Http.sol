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

struct HttpHeader {
    string name;
    string value;
}

library Http {
    function GetI64(
        string memory url,
        string memory jsonPointer,
        HttpHeader[] memory headers
    ) internal returns (bool, int64) {
        bool success;
        int64 value;

        bytes memory encoded_params = abi.encode(url, jsonPointer, headers);
        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x03E8,
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

    function GetBool(
        string memory url,
        string memory jsonPointer,
        HttpHeader[] memory headers
    ) internal returns (bool, bool) {
        bool success;
        bool value;

        bytes memory encoded_params = abi.encode(url, jsonPointer, headers);
        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x03E9,
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

    function GetString(
        string memory url,
        string memory jsonPointer,
        HttpHeader[] memory headers
    ) internal returns (bool, string memory) {
        bool success;
        string memory value;

        bytes memory encoded_params = abi.encode(url, jsonPointer, headers);
        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x03EA,
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

    function Get(string memory url, HttpHeader[] memory headers)
        internal
        returns (bool, string memory)
    {
        bool success;
        string memory value;

        bytes memory encoded_params = abi.encode(url, headers);
        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x03EE,
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

    function PostI64(
        string memory url,
        string memory jsonPointer,
        string memory payload,
        HttpHeader[] memory headers
    ) internal returns (bool, int64) {
        bool success;
        int64 value;

        bytes memory encoded_params = abi.encode(
            url,
            jsonPointer,
            payload,
            headers
        );
        uint256 encoded_params_len = encoded_params.length;
        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x03EB,
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

    function PostBool(
        string memory url,
        string memory jsonPointer,
        string memory payload,
        HttpHeader[] memory headers
    ) internal returns (bool, bool) {
        bool success;
        bool value;

        bytes memory encoded_params = abi.encode(
            url,
            jsonPointer,
            payload,
            headers
        );
        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x03EC,
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

    function PostString(
        string memory url,
        string memory jsonPointer,
        string memory payload,
        HttpHeader[] memory headers
    ) internal returns (bool, string memory) {
        bool success;
        string memory value;

        bytes memory encoded_params = abi.encode(
            url,
            jsonPointer,
            payload,
            headers
        );
        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x03ED,
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

    function Post(
        string memory url,
        string memory payload,
        HttpHeader[] memory headers
    ) internal returns (bool, string memory) {
        bool success;
        string memory value;

        bytes memory encoded_params = abi.encode(url, payload, headers);
        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x03EF,
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
}
