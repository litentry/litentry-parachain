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

import {BRC20} from "../BRC20.sol";

contract Long is BRC20 {
    function getTokenName() internal pure override returns (string memory) {
        return "long";
    }

    function getTokenRanges()
        internal
        pure
        override
        returns (uint256[] memory)
    {
        uint256[] memory ranges = new uint256[](9);
        ranges[0] = 0;
        ranges[1] = 1;
        ranges[2] = 20;
        ranges[3] = 50;
        ranges[4] = 200;
        ranges[5] = 500;
        ranges[6] = 1000;
        ranges[7] = 2000;
        ranges[8] = 3000;
        return ranges;
    }
}
