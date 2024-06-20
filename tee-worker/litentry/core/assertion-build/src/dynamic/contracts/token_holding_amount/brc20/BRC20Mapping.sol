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

import { BRC20 } from "../BRC20.sol";

import { Btcs } from "./Btcs.sol";
import { Cats } from "./Cats.sol";
import { Long } from "./Long.sol";
import { Mmss } from "./Mmss.sol";
import { Ordi } from "./Ordi.sol";
import { Rats } from "./Rats.sol";
import { Sats } from "./Sats.sol";

contract BRC20Mapping is BRC20 {
	constructor() {
		// btcs
		tokenNames["btcs"] = Btcs.getTokenName();
		tokenRanges["btcs"] = Btcs.getTokenRanges();

		// cats
		tokenNames["cats"] = Cats.getTokenName();
		tokenRanges["cats"] = Cats.getTokenRanges();

		// long
		tokenNames["long"] = Long.getTokenName();
		tokenRanges["long"] = Long.getTokenRanges();

		// long
		tokenNames["mmss"] = Mmss.getTokenName();
		tokenRanges["mmss"] = Mmss.getTokenRanges();

		// ordi
		tokenNames["ordi"] = Ordi.getTokenName();
		tokenRanges["ordi"] = Ordi.getTokenRanges();

		// rats
		tokenNames["rats"] = Rats.getTokenName();
		tokenRanges["rats"] = Rats.getTokenRanges();

		// sats
		tokenNames["sats"] = Sats.getTokenName();
		tokenRanges["sats"] = Sats.getTokenRanges();
	}
}
