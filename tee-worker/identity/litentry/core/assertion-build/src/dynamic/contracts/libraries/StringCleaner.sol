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

/**
 * @title StringCleaner
 * @dev A library for cleaning strings by removing non-visible ASCII characters.
 */
library StringCleaner {
    /**
     * @dev Cleans the input string by removing non-visible ASCII characters.
     * It iterates through each character in the string and retains only visible characters
     * (ASCII range 0x20 to 0x7E).
     * @param str The input string to be cleaned.
     * @return The cleaned string with only visible characters.
     */
    function cleanString(
        string memory str
    ) internal pure returns (string memory) {
        bytes memory b = bytes(str);
        bytes memory cleaned = new bytes(b.length);
        uint256 cleanedIndex = 0;

        for (uint256 i = 0; i < b.length; i++) {
            bytes1 char = b[i];
            if (isVisibleChar(char)) {
                cleaned[cleanedIndex] = char;
                cleanedIndex++;
            }
        }

        // Create a new bytes array of the correct length and copy cleaned characters into it
        bytes memory trimmedCleaned = new bytes(cleanedIndex);
        for (uint256 j = 0; j < cleanedIndex; j++) {
            trimmedCleaned[j] = cleaned[j];
        }

        return string(trimmedCleaned);
    }

    /**
     * @dev Checks if a given character is a visible ASCII character.
     * This includes characters in the ASCII range from 0x20 (space) to 0x7E (~).
     * @param char The character to be checked.
     * @return True if the character is visible, false otherwise.
     */
    function isVisibleChar(bytes1 char) internal pure returns (bool) {
        return (char >= 0x20 && char <= 0x7E);
    }
}
