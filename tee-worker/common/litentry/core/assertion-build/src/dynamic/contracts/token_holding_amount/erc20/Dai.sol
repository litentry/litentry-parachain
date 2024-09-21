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

library Dai {
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
            "0x6b175474e89094c44da98b954eedeac495271d0f",
            DataProviderTypes.NoderealClient,
            18
        );
        networks[1] = TokenInfoNetwork(
            Web3Networks.Bsc,
            "0x1af3f329e8be154074d8769d1ffa4ee058b1dbc3",
            DataProviderTypes.NoderealClient,
            18
        );
        networks[2] = TokenInfoNetwork(
            Web3Networks.Solana,
            "EjmyN6qEC1Tf1JxiG1ae7UTJhUxSwk1TCWNWqxWV4J6o",
            DataProviderTypes.MoralisClient,
            8
        );
        networks[3] = TokenInfoNetwork(
            Web3Networks.Arbitrum,
            "0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1",
            DataProviderTypes.MoralisClient,
            18
        );
        networks[4] = TokenInfoNetwork(
            Web3Networks.Polygon,
            "0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063",
            DataProviderTypes.MoralisClient,
            18
        );

        return networks;
    }
}
