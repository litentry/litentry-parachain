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

import {DynamicAssertion, Identity, HttpHeader} from "DynamicAssertion.sol";

contract A20 is DynamicAssertion {
    function execute(Identity[] memory identities, string[] memory secrets)
    public
    override
    returns (
        string memory,
        string memory,
        string[] memory,
        string memory,
        bool
    )
    {
        string
        memory description = "The user is an early bird user of the IdentityHub EVM version and has generated at least 1 credential during 2023 Aug 14th ~ Aug 21st.";
        string memory assertion_type = "IDHub EVM Version Early Bird";
        assertions.push('{ "src": "$has_joined", "op": "==", "dst": "true" }');
        schema_url = "https://raw.githubusercontent.com/litentry/vc-jsonschema/main/dist/schemas/12-idhub-evm-version-early-bird/1-0-0.json";
        bool result = false;

        for (uint256 i = 0; i < identities.length; i++) {
            if (is_web3(identities[i])) {
                string memory res = toHex(identities[i].value);

                string memory url = concatenateStrings(
                    "http://localhost:19527/events/does-user-joined-evm-campaign?account=",
                    res
                );
                string memory jsonPointer = "/hasJoined";
                HttpHeader[] memory headers = new HttpHeader[](0);

                result = GetBool(url, jsonPointer, headers);
                if (result) {
                    break;
                }
            }
        }
        return (description, assertion_type, assertions, schema_url, result);
    }
}
