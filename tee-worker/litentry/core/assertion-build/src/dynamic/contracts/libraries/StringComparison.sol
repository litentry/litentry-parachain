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

library StringComparison {
    /**
     * @notice Converts a given string to lowercase.
     * @dev This function iterates through each character of the input string,
     *      checks if it is an uppercase letter (A-Z), and converts it to the
     *      corresponding lowercase letter (a-z). Characters outside the range
     *      of uppercase letters remain unchanged.
     * @param str The input string to be converted to lowercase.
     * @return A new string with all uppercase letters converted to lowercase.
     */
    function toLower(string memory str) internal pure returns (string memory) {
        bytes memory bStr = bytes(str);
        bytes memory bLower = new bytes(bStr.length);

        for (uint i = 0; i < bStr.length; i++) {
            // Uppercase character range in ASCII: A-Z (65-90)
            if (bStr[i] >= 0x41 && bStr[i] <= 0x5A) {
                // Convert to lowercase by adding 32 (A -> a, B -> b, ..., Z -> z)
                bLower[i] = bytes1(uint8(bStr[i]) + 32);
            } else {
                bLower[i] = bStr[i];
            }
        }

        return string(bLower);
    }

    /**
     * @notice Compares two strings for equality, ignoring case.
     * @dev Converts both input strings to lowercase using the `toLower` function
     *      and then compares their keccak256 hashes to determine if they are equal.
     * @param str1 The first string to compare.
     * @param str2 The second string to compare.
     * @return A boolean value indicating whether the two strings are equal, ignoring case.
     */
    function compareStringsIgnoreCase(
        string memory str1,
        string memory str2
    ) internal pure returns (bool) {
        return
            keccak256(abi.encodePacked(toLower(str1))) ==
            keccak256(abi.encodePacked(toLower(str2)));
    }
}
