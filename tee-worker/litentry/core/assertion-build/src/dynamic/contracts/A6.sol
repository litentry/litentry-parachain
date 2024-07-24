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

import "./openzeppelin/Strings.sol";
import "./libraries/AssertionLogic.sol";
import "./libraries/Http.sol";
import "./libraries/Identities.sol";
import "./DynamicAssertion.sol";

contract A6 is DynamicAssertion {
    function execute(
        Identity[] memory identities,
        string[] memory secrets,
        bytes memory /*params*/
    )
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
        schema_url = "https://raw.githubusercontent.com/litentry/vc-jsonschema/main/dist/schemas/6-twitter-follower-amount/1-1-1.json";

        bool result;

        int64 sum = 0;

        for (uint256 i = 0; i < identities.length; i++) {
            if (Identities.is_twitter(identities[i])) {
                string memory url = string(
                    abi.encodePacked(
                        "https://api.twitter.com/2/users/by/username/",
                        // below url is used for test against mock server
                        // "http://localhost:19528/2/users/by/username/",
                        string(identities[i].value),
                        "?user.fields=public_metrics"
                    )
                );

                HttpHeader[] memory headers = prepareHeaders(secrets[0]);

                (bool get_success, int64 followers_count) = Http.GetI64(
                    url,
                    "/data/public_metrics/followers_count",
                    headers
                );

                if (get_success) {
                    sum += followers_count;
                }
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
        result = min != 0;

        string memory variable = "$total_followers";
        AssertionLogic.CompositeCondition memory cc = AssertionLogic
            .CompositeCondition(new AssertionLogic.Condition[](2), true);
        AssertionLogic.andOp(
            cc,
            0,
            variable,
            AssertionLogic.Op.GreaterThan,
            Strings.toString(min)
        );
        AssertionLogic.andOp(
            cc,
            1,
            variable,
            AssertionLogic.Op.LessEq,
            Strings.toString(max)
        );

        string[] memory assertions = new string[](1);
        assertions[0] = AssertionLogic.toString(cc);

        return (description, assertion_type, assertions, schema_url, result);
    }

    function prepareHeaders(string memory apiKey)
        private
        pure
        returns (HttpHeader[] memory)
    {
        HttpHeader[] memory headers = new HttpHeader[](1);
        // we expect first secret to be twitter api key
        headers[0] = HttpHeader("authorization", prepareAuthHeader(apiKey));
        return headers;
    }

    function prepareAuthHeader(string memory apiKey)
        private
        pure
        returns (string memory)
    {
        return string(abi.encodePacked("Bearer ", apiKey));
    }
}
