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

library Uni {
    function getTokenRanges() internal pure returns (TokenInfoRanges memory) {
        uint256[] memory ranges = new uint256[](9);
        ranges[0] = 0;
        ranges[1] = 10;
        ranges[2] = 30;
        ranges[3] = 80;
        ranges[4] = 200;
        ranges[5] = 500;
        ranges[6] = 1000;
        ranges[7] = 2000;
        ranges[8] = 5000;

        return TokenInfoRanges(ranges, 0);
    }

    function getTokenNetworks()
        internal
        pure
        returns (TokenInfoNetwork[] memory)
    {
        TokenInfoNetwork[] memory networks = new TokenInfoNetwork[](5);
        networks[0] = TokenInfoNetwork(
            Web3Networks.Ethereum,
            "0x1f9840a85d5af5bf1d1762f925bdaddc4201f984",
            DataProviderTypes.NoderealClient,
            18
        );
        networks[1] = TokenInfoNetwork(
            Web3Networks.Bsc,
            "0xbf5140a22578168fd562dccf235e5d43a02ce9b1",
            DataProviderTypes.NoderealClient,
            18
        );
        networks[2] = TokenInfoNetwork(
            Web3Networks.Solana,
            "8FU95xFJhUUkyyCLU13HSzDLs7oC4QZdXQHL6SCeab36",
            DataProviderTypes.MoralisClient,
            18
        );
        networks[3] = TokenInfoNetwork(
            Web3Networks.Arbitrum,
            "0xFa7F8980b0f1E64A2062791cc3b0871572f1F7f0",
            DataProviderTypes.MoralisClient,
            18
        );
        networks[4] = TokenInfoNetwork(
            Web3Networks.Polygon,
            "0xb33eaad8d922b1083446dc23f610c2567fb5180f",
            DataProviderTypes.MoralisClient,
            18
        );

        return networks;
    }
}
