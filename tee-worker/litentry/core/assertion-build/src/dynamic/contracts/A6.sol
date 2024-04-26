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

pragma solidity ^0.8.8;

import {DynamicAssertion, Identity, HttpHeader} from "./DynamicAssertion.sol";
import "https://github.com/OpenZeppelin/openzeppelin-contracts/blob/v4.9.0/contracts/utils/Strings.sol";

contract A6 is DynamicAssertion {
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
        memory description = "The range of the user's Twitter follower count";
        string memory assertion_type = "Twitter Follower Amount";
        schema_url = "https://raw.githubusercontent.com/litentry/vc-jsonschema/main/dist/schemas/6-twitter-follower-amount/1-0-0.json";

        bool result;

        int64 sum = 0;

        for (uint256 i = 0; i < identities.length; i++) {
            if (is_twitter(identities[i])) {
                string memory url = concatenateStrings(
                    "http://localhost:19528/2/users/by/username/",
                    string(identities[i].value)
                );
                string memory full_url = concatenateStrings(
                    url,
                    "?user.fields=public_metrics"
                );

                HttpHeader[] memory headers = new HttpHeader[](1);
                // we expect first secret to be twitter api key
                headers[0] = HttpHeader("authorization", secrets[0]);

                int64 followers_count = GetI64(
                    full_url,
                    "/data/public_metrics/followers_count",
                    headers
                );

                sum += followers_count;
            }
        }

        int64 min = 0;
        int64 max = 0;

        if (sum >= 0 && sum <= 1) {
            min = 0;
            max = 1;
        } else if (sum > 1 && sum <= 100) {
            min = 1;
            max = 100;
        } else if (sum > 100 && sum <= 1000) {
            min = 100;
            max = 1000;
        } else if (sum > 1000 && sum <= 10000) {
            min = 1000;
            max = 10000;
        } else if (sum > 10000 && sum <= 100000) {
            min = 10000;
            max = 100000;
        } else if (sum > 100000) {
            min = 100000;
            max = 9223372036854775807;
        }
        result = true;

        string memory assertion = concatenateStrings(
            '{"and": [{ "src": "$total_followers", "op": ">", "dst": "',
            Strings.toString(min)
        );
        assertion = concatenateStrings(
            assertion,
            '" }, { "src": "$has_web3_account", "op": "<=", "dst": "'
        );
        assertion = concatenateStrings(assertion, Strings.toString(max));
        assertion = concatenateStrings(assertion, '" } ] }');
        assertions.push(assertion);
        return (description, assertion_type, assertions, schema_url, result);
    }
}
