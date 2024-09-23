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

struct TokenInfo {
    uint256[] ranges;
    // Some ranges are decimals, need convert them to int with multiply by rangeDecimals.
    uint256 rangeDecimals;
    // Different networks for same token may have different decimals, need use maxDecimals as multiplier factor.
    uint8 maxDecimals;
    TokenInfoNetwork[] networks;
}

struct TokenInfoNetwork {
    uint32 network;
    string tokenAddress;
    uint8 dataProvierType;
    uint8 decimals;
}

struct TokenInfoRanges {
    uint256[] ranges;
    // Some ranges are decimals, need convert them to int with multiply by rangeDecimals.
    uint256 rangeDecimals;
}

library DataProviderTypes {
    uint8 public constant AchainableClient = 0;
    uint8 public constant BlockchainInfoClient = 1;
    uint8 public constant GeniidataClient = 2;
    uint8 public constant MoralisClient = 3;
    uint8 public constant NoderealClient = 4;
}
