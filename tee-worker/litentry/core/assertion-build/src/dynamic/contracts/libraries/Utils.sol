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

library Utils {
    function toHex(bytes memory bytes_value)
        internal
        returns (bool success, string memory value)
    {
        bytes memory encoded_params = abi.encode(bytes_value);
        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x041B,
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

    function identityToString(uint32 network_type, bytes memory identity_value)
        internal
        returns (bool success, string memory value)
    {
        bytes memory encoded_params = abi.encode(network_type, identity_value);
        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x041C,
                    0,
                    add(encoded_params, 0x20),
                    encoded_params_len,
                    memPtr,
                    0x1000
                )
            ) {
                revert(0, 0)
            }
            success := memPtr
            value := add(memPtr, 0x40)
            mstore(0x40, add(memPtr, 0x1000))
        }

        return (success, value);
    }

    function hexToNumber(string memory string_value)
        internal
        returns (bool success, uint256 value)
    {
        bytes memory encoded_params = abi.encode(string_value);
        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x041D,
                    0,
                    add(encoded_params, 0x20),
                    encoded_params_len,
                    memPtr,
                    0x82
                )
            ) {
                revert(0, 0)
            }
            success := mload(memPtr)
            value := mload(add(memPtr, 0x20))
            mstore(0x40, add(memPtr, 0x82))
        }

        return (success, value);
    }

    function parseDecimal(string memory string_value, uint8 decimals)
        internal
        returns (bool success, uint256 value)
    {
        bytes memory encoded_params = abi.encode(string_value, decimals);
        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x041E,
                    0,
                    add(encoded_params, 0x20),
                    encoded_params_len,
                    memPtr,
                    0x82
                )
            ) {
                revert(0, 0)
            }
            success := mload(memPtr)
            value := mload(add(memPtr, 0x20))
            mstore(0x40, add(memPtr, 0x82))
        }

        return (success, value);
    }

    function parseInt(string memory string_value)
        internal
        returns (bool success, uint256 value)
    {
        bytes memory encoded_params = abi.encode(string_value);
        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x041F,
                    0,
                    add(encoded_params, 0x20),
                    encoded_params_len,
                    memPtr,
                    0x82
                )
            ) {
                revert(0, 0)
            }
            success := mload(memPtr)
            value := mload(add(memPtr, 0x20))
            mstore(0x40, add(memPtr, 0x82))
        }

        return (success, value);
    }
    function isStringsEqual(
		string memory a,
		string memory b
	) internal pure returns (bool) {
		return keccak256(abi.encodePacked(a)) == keccak256(abi.encodePacked(b));
	}
}
