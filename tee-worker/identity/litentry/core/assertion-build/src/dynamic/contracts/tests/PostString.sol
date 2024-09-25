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

contract PostString {
    function callPostString(
        string memory url,
        string memory jsonPointer,
        string memory payload
    ) public returns (bool, string memory) {
        HttpHeader[] memory headers = new HttpHeader[](0);
        return Http.PostString(url, jsonPointer, payload, headers);
    }

    function callPostStringTwiceAndReturnSecondResult(
        string memory firstUrl,
        string memory firstJsonPointer,
        string memory firstPayload,
        string memory secondUrl,
        string memory secondJsonPointer,
        string memory secondPayload
    ) public returns (bool, string memory) {
        HttpHeader[] memory headers = new HttpHeader[](0);
        Http.PostString(firstUrl, firstJsonPointer, firstPayload, headers);
        return
            Http.PostString(
                secondUrl,
                secondJsonPointer,
                secondPayload,
                headers
            );
    }
}
