// SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity ^0.8.8;

import "../../libraries/Identities.sol";
import "../Constants.sol";

library Usdc {
    function getTokenRanges() internal pure returns (uint256[] memory) {
        uint256[] memory ranges = new uint256[](9);
        ranges[0] = 0 * Constants.decimals_factor;
        ranges[1] = 10 * Constants.decimals_factor;
        ranges[2] = 30 * Constants.decimals_factor;
        ranges[3] = 80 * Constants.decimals_factor;
        ranges[4] = 200 * Constants.decimals_factor;
        ranges[5] = 500 * Constants.decimals_factor;
        ranges[6] = 1000 * Constants.decimals_factor;
        ranges[7] = 2000 * Constants.decimals_factor;
        ranges[8] = 5000 * Constants.decimals_factor;

        return ranges;
    }

    function getTokenInfo() internal pure returns (TokenInfo[] memory) {
        TokenInfo[] memory tokenInfoList = new TokenInfo[](3);
        tokenInfoList[0] = TokenInfo(
            Web3Networks.Ethereum,
            "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48",
            DataProviderTypes.NoderealClient,
            18
        );
        tokenInfoList[1] = TokenInfo(
            Web3Networks.Bsc,
            "0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d",
            DataProviderTypes.NoderealClient,
            18
        );
        tokenInfoList[2] = TokenInfo(
            Web3Networks.Solana,
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
            DataProviderTypes.MoralisClient,
            18
        );

        return tokenInfoList;
    }
}
