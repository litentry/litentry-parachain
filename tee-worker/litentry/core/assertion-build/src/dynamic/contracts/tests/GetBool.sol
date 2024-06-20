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

import "../libraries/Http.sol";

contract GetI64 {
    function callGetBool(string memory url, string memory jsonPointer)
        public
        returns (bool, bool)
    {
        HttpHeader[] memory headers = new HttpHeader[](0);
        return Http.GetBool(url, jsonPointer, headers);
    }

    function callGetBoolTwiceAndReturnSecondResult(
        string memory firstUrl,
        string memory firstJsonPointer,
        string memory secondUrl,
        string memory secondJsonPointer
    ) public returns (bool, bool) {
        HttpHeader[] memory headers = new HttpHeader[](0);
        Http.GetBool(firstUrl, firstJsonPointer, headers);
        (firstUrl, firstJsonPointer, headers);
        return Http.GetBool(secondUrl, secondJsonPointer, headers);
    }
}
