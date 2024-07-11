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
			string memory str = uintToString(value);

			// Calculate the position to insert the decimal point
			uint256 len = bytes(str).length;
			uint32 digit = log10(decimal);

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
	 * @dev Converts a uint256 to a string.
	 * @param v The uint256 value to convert.
	 * @return The converted string.
	 */
	function uintToString(uint256 v) internal pure returns (string memory) {
		uint256 maxLength = 78; // 78 is a safe upper bound for 256-bit numbers in base 10
		bytes memory reversed = new bytes(maxLength);
		uint256 length = 0;
		while (v != 0) {
			uint256 remainder = v % 10;
			v = v / 10;
			reversed[length++] = bytes1(uint8(48 + remainder));
		}
		bytes memory result = new bytes(length);
		for (uint256 i = 0; i < length; i++) {
			result[i] = reversed[length - 1 - i];
		}
		return string(result);
	}

	/**
	 * @dev Extracts a substring from a given string.
	 * @param str The original string.
	 * @param start The starting position of the substring.
	 * @param end The ending position of the substring.
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

	/**
	 * @dev Calculates the base-10 logarithm of a number (floor value).
	 * @param x The input number.
	 * @return The base-10 logarithm of the number.
	 */
	function log10(uint256 x) internal pure returns (uint32) {
		uint32 result = 0;
		while (x >= 10) {
			x /= 10;
			result++;
		}
		return result;
	}
}
