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

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/utils/Strings.sol";
import "@openzeppelin/contracts/utils/math/Math.sol";

library StringShift {
    /**
     * @dev Converts a uint256 input to a string and shifts the decimal point to the left by the specified number of places.
     * @param value The uint256 parameter to be processed.
     * @param decimal The number of decimal places to shift.
     * @return The processed string.
     */
    function toShiftedString(
        uint256 value,
        uint256 decimal
    ) internal pure returns (string memory) {
        // Convert uint256 to string

        if (value == 0) {
            return "0";
        } else {
            string memory str = Strings.toString(value);

            // Calculate the position to insert the decimal point
            uint256 len = bytes(str).length;
            uint256 digit = Math.log10(decimal);

            if (len <= digit) {
                // If the length is less than or equal to the number of digits, pad with leading zeros and add '0.'
                string memory leadingZeros = new string(digit - len);
                for (uint256 i = 0; i < digit - len; i++) {
                    leadingZeros = string(abi.encodePacked("0", leadingZeros));
                }
                str = string(abi.encodePacked("0.", leadingZeros, str));
            } else {
                // Otherwise, insert the decimal point at the correct position
                str = string(
                    abi.encodePacked(
                        substring(str, 0, len - digit),
                        ".",
                        substring(str, len - digit, len)
                    )
                );
            }

            // Remove trailing zeros after the decimal point
            str = removeTrailingZeros(str);

            return str;
        }
    }

    /**
     * @dev Extracts a substring from a given string.
     * @param str The original string.
     * @param start The starting position of the original string.
     * @param end The ending position of the original string.
     * @return The extracted substring.
     */
    function substring(
        string memory str,
        uint256 start,
        uint256 end
    ) internal pure returns (string memory) {
        bytes memory strBytes = bytes(str);
        bytes memory result = new bytes(end - start);
        for (uint256 i = start; i < end; i++) {
            result[i - start] = strBytes[i];
        }
        return string(result);
    }

    /**
     * @dev Removes trailing zeros after the decimal point in a string.
     * @param str The input string.
     * @return The processed string with trailing zeros removed.
     */
    function removeTrailingZeros(
        string memory str
    ) internal pure returns (string memory) {
        bytes memory strBytes = bytes(str);
        uint256 len = strBytes.length;

        // Traverse from the end to find the position of the first non-zero character
        uint256 newLen = len;
        while (newLen > 0 && strBytes[newLen - 1] == "0") {
            newLen--;
        }

        // If the last character is a decimal point, remove it as well
        if (newLen > 0 && strBytes[newLen - 1] == ".") {
            newLen--;
        }

        // Create a new byte array and copy the content
        bytes memory trimmedBytes = new bytes(newLen);
        for (uint256 i = 0; i < newLen; i++) {
            trimmedBytes[i] = strBytes[i];
        }

        return string(trimmedBytes);
    }
}
