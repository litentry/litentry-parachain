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

import "../../libraries/Identities.sol";
import "../Constants.sol";

library Trx {
    function getTokenRanges() internal pure returns (TokenInfoRanges memory) {
        uint256[] memory ranges = new uint256[](10);
        ranges[0] = 0;
        ranges[1] = 1;
        ranges[2] = 50;
        ranges[3] = 100;
        ranges[4] = 200;
        ranges[5] = 500;
        ranges[6] = 800;
        ranges[7] = 1200;
        ranges[8] = 1600;
        ranges[9] = 3000;

        return TokenInfoRanges(ranges, 0);
    }

    function getTokenNetworks()
        internal
        pure
        returns (TokenInfoNetwork[] memory)
    {
        TokenInfoNetwork[] memory networks = new TokenInfoNetwork[](2);
        networks[0] = TokenInfoNetwork(
            Web3Networks.Ethereum,
            "0x50327c6c5a14dcade707abad2e27eb517df87ab5",
            DataProviderTypes.NoderealClient,
            18
        );
        networks[1] = TokenInfoNetwork(
            Web3Networks.Bsc,
            "0xCE7de646e7208a4Ef112cb6ed5038FA6cC6b12e3",
            DataProviderTypes.NoderealClient,
            18
        );
        return networks;
    }
}
