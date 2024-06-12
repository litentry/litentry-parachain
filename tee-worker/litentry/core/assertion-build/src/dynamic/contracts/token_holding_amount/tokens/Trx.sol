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

import {ERC20} from "../ERC20.sol";

contract Ton is ERC20 {
    function getTokenName() internal pure override returns (string memory) {
        return "0xf230b790E05390FC8295F4d3F60332c93BEd42e2";
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
        ranges[2] = 5;
        ranges[3] = 20;
        ranges[4] = 50;
        ranges[5] = 100;
        ranges[6] = 200;
        ranges[7] = 500;
        ranges[8] = 800;
        return ranges;
    }
}
