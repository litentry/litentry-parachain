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

import { Btcs } from "./Btcs.sol";
import { Cats } from "./Cats.sol";
import { Long } from "./Long.sol";
import { Mmss } from "./Mmss.sol";
import { Ordi } from "./Ordi.sol";
import { Rats } from "./Rats.sol";
import { Sats } from "./Sats.sol";

library BRC0TokenLibrary {
	function getBtcsInfo()
		public
		pure
		returns (string memory, uint256[] memory)
	{
		string memory tokenName = Btcs.getTokenName();
		uint256[] memory ranges = Btcs.getTokenRanges();

		return (tokenName, ranges);
	}
	function getCatsInfo()
		public
		pure
		returns (string memory, uint256[] memory)
	{
		string memory tokenName = Cats.getTokenName();
		uint256[] memory ranges = Cats.getTokenRanges();

		return (tokenName, ranges);
	}

	function getLongInfo()
		public
		pure
		returns (string memory, uint256[] memory)
	{
		string memory tokenName = Long.getTokenName();
		uint256[] memory ranges = Long.getTokenRanges();

		return (tokenName, ranges);
	}

	function getMmssInfo()
		public
		pure
		returns (string memory, uint256[] memory)
	{
		string memory tokenName = Mmss.getTokenName();
		uint256[] memory ranges = Mmss.getTokenRanges();

		return (tokenName, ranges);
	}
	function getOrdiInfo()
		public
		pure
		returns (string memory, uint256[] memory)
	{
		string memory tokenName = Ordi.getTokenName();
		uint256[] memory ranges = Ordi.getTokenRanges();
		return (tokenName, ranges);
	}
	function getRatsInfo()
		public
		pure
		returns (string memory, uint256[] memory)
	{
		string memory tokenName = Rats.getTokenName();
		uint256[] memory ranges = Rats.getTokenRanges();
		return (tokenName, ranges);
	}
	function getSatsInfo()
		public
		pure
		returns (string memory, uint256[] memory)
	{
		string memory tokenName = Sats.getTokenName();
		uint256[] memory ranges = Sats.getTokenRanges();
		return (tokenName, ranges);
	}
}
