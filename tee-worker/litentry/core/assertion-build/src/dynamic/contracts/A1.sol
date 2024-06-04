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

import {DynamicAssertion, Identity} from "DynamicAssertion.sol";

contract A1 is DynamicAssertion {
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
        memory description = "You've identified at least one account/address in both Web2 and Web3.";
        string memory assertion_type = "Basic Identity Verification";
        assertions.push(
            '{"and": [{ "src": "$has_web2_account", "op": "==", "dst": "true" }, { "src": "$has_web3_account", "op": "==", "dst": "true" } ] }'
        );
        schema_url = "https://raw.githubusercontent.com/litentry/vc-jsonschema/main/dist/schemas/1-basic-identity-verification/1-0-0.json";

        bool result;

        bool has_web3_identity = false;
        bool has_web2_identity = false;

        for (uint256 i = 0; i < identities.length; i++) {
            if (is_web2(identities[i])) {
                has_web2_identity = true;
            } else if (is_web3(identities[i])) {
                has_web3_identity = true;
            }
        }
        result = has_web2_identity && has_web3_identity;

        return (description, assertion_type, assertions, schema_url, result);
    }
}
