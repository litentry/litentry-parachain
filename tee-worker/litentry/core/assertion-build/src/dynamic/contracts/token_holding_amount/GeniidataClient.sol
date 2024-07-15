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

import "../libraries/Http.sol";
import "../libraries/Utils.sol";
library GeniidataClient {
	function getTokenBalance(
		string[] memory secrets,
		string memory url,
		string memory identityString,
		string memory tokenName,
		uint8 tokenDecimals
	) internal returns (uint256) {
		string memory encodePackedUrl = string(
			abi.encodePacked(
				url,
				"?tick=",
				tokenName,
				"&address=",
				identityString
			)
		);
		HttpHeader[] memory headers = new HttpHeader[](1);
		headers[0] = HttpHeader("api-key", secrets[1]);

		// https://geniidata.readme.io/reference/get-brc20-tick-list-copy
		(bool success, string memory value) = Http.GetString(
			encodePackedUrl,
			"/data/list/0/available_balance",
			headers
		);

		if (success) {
			(bool parseDecimalSuccess, uint256 result) = Utils.parseDecimal(
				value,
				tokenDecimals
			);
			if (parseDecimalSuccess) {
				return result;
			}
		}
		return 0;
	}
}
