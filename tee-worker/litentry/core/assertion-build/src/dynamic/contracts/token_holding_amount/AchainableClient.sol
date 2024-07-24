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
import "../libraries/Json.sol";
import "../openzeppelin/Strings.sol";


struct DisplayItem {
    string text;
    bool result;
}

struct Response {
    string name;
    bool result;
    DisplayItem[] display;
}

library AchainableClient {
    function queryHoldingAmount(
        string memory url,
        string memory key,
        string memory account,
        string memory chain
    ) internal returns (bool success, Response memory) {
        HttpHeader[] memory headers = new HttpHeader[](1);
        headers[0] = HttpHeader("authorization", key);

        string memory payload = string(
            abi.encodePacked(
                '{"name": "Balance over {amount}", "address": "',
                account,
                '", "params": {"chain": "',
                chain,
                '", "amount": "0"}, "includeMetadata": false }'
            )
        );

        (bool amountSuccess, string memory amountResponse) = Http.Post(
            url,
            payload,
            headers
        );

        if (amountSuccess) {
            (bool nameSuccess, string memory name) = Json.getString(
                amountResponse,
                "/name"
            );
            (bool resultSuccess, bool result) = Json.getBool(
                amountResponse,
                "/result"
            );

            if (!nameSuccess || !resultSuccess) {
                return (false, Response("", false, new DisplayItem[](0)));
            }

            (
                bool displaysParseSuccess,
                DisplayItem[] memory displays
            ) = parseDisplayItems(amountResponse);

            if (!displaysParseSuccess) {
                return (false, Response("", false, new DisplayItem[](0)));
            } else {
                return (true, Response(name, result, displays));
            }
        } else {
            return (false, Response("", false, new DisplayItem[](0)));
        }
    }

    function parseDisplayItems(string memory response)
        private
        returns (bool, DisplayItem[] memory)
    {
        (bool displayLenSuccess, int64 displayLen) = Json.getArrayLen(
            response,
            "/display"
        );

        if (!displayLenSuccess) {
            return (false, new DisplayItem[](0));
        }

        DisplayItem[] memory displays = new DisplayItem[](
            uint256(int256(displayLen))
        );

        for (uint256 i = 0; i < uint256(int256(displayLen)); i++) {
            (bool textSuccess, string memory text) = Json.getString(
                response,
                string(
                    abi.encodePacked("/display/", Strings.toString(i), "/text")
                )
            );

            (bool displayResultSuccess, bool displayResult) = Json.getBool(
                response,
                string(
                    abi.encodePacked(
                        "/display/",
                        Strings.toString(i),
                        "/result"
                    )
                )
            );

            if (!textSuccess || !displayResultSuccess) {
                return (false, new DisplayItem[](0));
            } else {
                displays[i] = DisplayItem(text, displayResult);
            }
        }
        return (true, displays);
    }
}
