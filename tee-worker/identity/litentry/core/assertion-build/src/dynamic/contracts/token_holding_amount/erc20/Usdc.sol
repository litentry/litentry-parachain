// SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity ^0.8.8;

import "../../libraries/Identities.sol";
import "../Constants.sol";

library Usdc {
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
            "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
            DataProviderTypes.NoderealClient,
            6
        );
        networks[1] = TokenInfoNetwork(
            Web3Networks.Bsc,
            "0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d",
            DataProviderTypes.NoderealClient,
            6
        );
        networks[2] = TokenInfoNetwork(
            Web3Networks.Solana,
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
            DataProviderTypes.MoralisClient,
            6
        );
        networks[3] = TokenInfoNetwork(
            Web3Networks.Arbitrum,
            "0xaf88d065e77c8cC2239327C5EDb3A432268e5831",
            DataProviderTypes.MoralisClient,
            6
        );
        networks[4] = TokenInfoNetwork(
            Web3Networks.Polygon,
            "0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359",
            DataProviderTypes.MoralisClient,
            6
        );

        return networks;
    }
}
