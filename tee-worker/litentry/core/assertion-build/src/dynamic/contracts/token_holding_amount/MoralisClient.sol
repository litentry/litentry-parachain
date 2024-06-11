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
import "../libraries/Http.sol";
import "../libraries/Utils.sol";

library MoralisClient {
    function getSolanaNativeBalance(
        string memory url,
        string memory apiKey,
        string memory account
    ) internal returns (bool, string memory) {
        url = string(abi.encodePacked(url, "/", account, "/balance"));

        HttpHeader[] memory headers = new HttpHeader[](0);
        headers[0] = HttpHeader("X-API-Key", apiKey);
        return Http.GetString(url, "/solana", headers);
    }

    // function getSolanaTokensBalance(
    //     string memory url,
    //     string memory apiKey,
    //     string memory account
    // ) internal returns (bool, string memory) {
    //     // this return array of tokens so we need to find a way to iterate over it - currently that's not possible
    //     url = string(abi.encodePacked(url, "/", account, "/tokens"));
    //     HttpHeader[] memory headers = new HttpHeader[](0);
    //     headers[0] = HttpHeader("X-API-Key", apiKey);
    //     return Http.GetString(url, "/solana", headers);

    // }

}
