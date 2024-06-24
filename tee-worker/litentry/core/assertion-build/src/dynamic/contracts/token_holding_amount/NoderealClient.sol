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
library NoderealClient {
	function getTokenBalance(
		string memory url,
		string[] memory secrets,
		string memory tokenContractAddress,
		string memory account
	) internal returns (bool, uint256) {
		HttpHeader[] memory headers = new HttpHeader[](0);
		string memory request;

		string memory encodePackedUrl = string(
			abi.encodePacked(url, secrets[0])
		);
		if (
			keccak256(bytes(tokenContractAddress)) == keccak256("Native Token")
		) {
			// Use eth_getBalance method
			request = string(
				abi.encodePacked(
					'{"jsonrpc": "2.0", "method": "eth_getBalance", "id": 1, "params": ["',
					account,
					'", "latest"]}'
				)
			);
		} else if (bytes(tokenContractAddress).length == 42) {
			// Use nr_getTokenBalance20 method
			request = string(
				abi.encodePacked(
					'{"jsonrpc": "2.0", "method": "nr_getTokenBalance20", "id": 1, "params": ["',
					tokenContractAddress,
					'","',
					account,
					'", "latest"]}'
				)
			);
		} else {
			return (false, 0);
		}
		(bool result, string memory balance) = Http.PostString(
			encodePackedUrl,
			"/result",
			request,
			headers
		);
		if (result) {
			return Utils.hexToNumber(balance);
		} else {
			return (false, 0);
		}
	}
}
