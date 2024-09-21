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
import "./Constants.sol";
import { BRC20 } from "./brc20/BRC20.sol";
library Btc {
    function getTokenRanges() internal pure returns (TokenInfoRanges memory) {
        // [0.0, 0.001, 0.1, 0.3, 0.6, 1.0, 2.0, 5.0, 10.0, 15.0, 25.0, 30.0, 40.0, 50.0];
        // all ranges multiplied by decimals_factor(1000).
        uint256[] memory ranges = new uint256[](14);
        ranges[0] = 0;
        ranges[1] = 1;
        ranges[2] = 100;
        ranges[3] = 300;
        ranges[4] = 600;
        ranges[5] = 1000;
        ranges[6] = 2000;
        ranges[7] = 5000;
        ranges[8] = 10000;
        ranges[9] = 15000;
        ranges[10] = 25000;
        ranges[11] = 30000;
        ranges[12] = 40000;
        ranges[13] = 50000;
        return TokenInfoRanges(ranges, 3);
    }

    function getTokenNetworks()
        internal
        pure
        returns (TokenInfoNetwork[] memory)
    {
        uint32[] memory defaultNetworks = BRC20.getDefaultTokenNetworks();
        TokenInfoNetwork[] memory networks = new TokenInfoNetwork[](
            defaultNetworks.length
        );
        for (uint i = 0; i < defaultNetworks.length; i++) {
            networks[i] = TokenInfoNetwork(
                defaultNetworks[i],
                "",
                DataProviderTypes.BlockchainInfoClient,
                8
            );
        }

        return networks;
    }
}
