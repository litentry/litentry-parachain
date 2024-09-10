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

library Atom {
    function getTokenRanges() internal pure returns (TokenInfoRanges memory) {
        uint256[] memory ranges = new uint256[](6);
        ranges[0] = 0;
        ranges[1] = 1;
        ranges[2] = 5;
        ranges[3] = 20;
        ranges[4] = 50;
        ranges[5] = 80;

        return TokenInfoRanges(ranges, 0);
    }

    function getTokenNetworks()
        internal
        pure
        returns (TokenInfoNetwork[] memory)
    {
        TokenInfoNetwork[] memory networks = new TokenInfoNetwork[](3);
        networks[0] = TokenInfoNetwork(
            Web3Networks.Ethereum,
            "0x8D983cb9388EaC77af0474fA441C4815500Cb7BB",
            DataProviderTypes.NoderealClient,
            6
        );
        networks[1] = TokenInfoNetwork(
            Web3Networks.Bsc,
            "0x0eb3a705fc54725037cc9e008bdede697f62f335",
            DataProviderTypes.NoderealClient,
            18
        );
        networks[2] = TokenInfoNetwork(
            Web3Networks.Polygon,
            "0xac51C4c48Dc3116487eD4BC16542e27B5694Da1b",
            DataProviderTypes.MoralisClient,
            6
        );
        return networks;
    }
}
