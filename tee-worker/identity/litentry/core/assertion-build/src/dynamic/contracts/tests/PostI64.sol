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

contract PostI64 {
    function callPostI64(
        string memory url,
        string memory jsonPointer,
        string memory payload
    ) public returns (bool, int64) {
        HttpHeader[] memory headers = new HttpHeader[](0);
        return Http.PostI64(url, jsonPointer, payload, headers);
    }

    function callPostI64TwiceAndReturnSecondResult(
        string memory firstUrl,
        string memory firstJsonPointer,
        string memory firstPayload,
        string memory secondUrl,
        string memory secondJsonPointer,
        string memory secondPayload
    ) public returns (bool, int64) {
        HttpHeader[] memory headers = new HttpHeader[](0);
        Http.PostI64(firstUrl, firstJsonPointer, firstPayload, headers);
        return
            Http.PostI64(secondUrl, secondJsonPointer, secondPayload, headers);
    }
}
