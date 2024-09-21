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

import "@openzeppelin/contracts/utils/Strings.sol";
import "../libraries/AssertionLogic.sol";
import "../libraries/Identities.sol";
import "../DynamicAssertion.sol";
import "./Constants.sol";
import "../libraries/StringShift.sol";
import "hardhat/console.sol";

abstract contract TokenHoldingAmount is DynamicAssertion {
    mapping(string => TokenInfo) internal tokens;

    function execute(
        Identity[] memory identities,
        string[] memory secrets,
        bytes memory params
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
            memory description = "The amount of a particular token you are holding";
        string memory assertion_type = "Token Holding Amount";
        schema_url = "https://raw.githubusercontent.com/litentry/vc-jsonschema/main/dist/schemas/25-token-holding-amount/1-1-4.json";

        string memory tokenLowercaseName = abi.decode(params, (string));

        TokenInfo memory token = tokens[tokenLowercaseName];

        require(token.networks.length > 0, "Token not supported or not found");

        uint256 balance = queryTotalBalance(
            identities,
            secrets,
            tokenLowercaseName,
            token
        );

        console.log("balance>", balance);

        (uint256 index, uint256 min, int256 max) = calculateRange(
            balance,
            token
        );

        string[] memory assertions = assembleAssertions(
            min,
            max,
            balance,
            tokenLowercaseName,
            token
        );

        bool result = index > 0 || balance > 0;

        return (description, assertion_type, assertions, schema_url, result);
    }

    function queryTotalBalance(
        Identity[] memory identities,
        string[] memory secrets,
        string memory tokenName,
        TokenInfo memory token
    ) internal virtual returns (uint256) {
        uint256 total_balance = 0;
        uint256 identitiesLength = identities.length;

        for (uint256 i = 0; i < identitiesLength; i++) {
            Identity memory identity = identities[i];
            uint256 networksLength = identity.networks.length;
            for (uint32 j = 0; j < networksLength; j++) {
                uint32 network = identity.networks[j];
                if (isSupportedNetwork(token, network)) {
                    total_balance += queryBalance(
                        identity,
                        network,
                        secrets,
                        tokenName,
                        token
                    );
                }
            }
        }

        return total_balance;
    }

    function calculateRange(
        uint256 balance,
        TokenInfo memory token
    ) private pure returns (uint256, uint256, int256) {
        uint256[] memory ranges = token.ranges;
        uint256 index = ranges.length - 1;
        uint256 min = 0;
        int256 max = 0;

        for (uint32 i = 1; i < ranges.length; i++) {
            if (
                balance * 10 ** token.rangeDecimals <
                ranges[i] * 10 ** token.maxDecimals
            ) {
                index = i - 1;
                break;
            }
        }

        if (index == ranges.length - 1) {
            min = ranges[index];
            max = -1;
        } else {
            min = ranges[index];
            max = int256(ranges[index + 1]);
        }
        return (index, min, max);
    }

    function assembleAssertions(
        uint256 min,
        int256 max,
        uint256 balance,
        string memory tokenName,
        TokenInfo memory token
    ) private pure returns (string[] memory) {
        string memory variable = "$holding_amount";
        AssertionLogic.CompositeCondition memory cc = AssertionLogic
            .CompositeCondition(
                new AssertionLogic.Condition[](max > 0 && balance > 0 ? 4 : 3),
                true
            );
        AssertionLogic.andOp(
            cc,
            0,
            "$token",
            AssertionLogic.Op.Equal,
            tokenName
        );

        AssertionLogic.CompositeCondition memory networkCc = AssertionLogic
            .CompositeCondition(
                new AssertionLogic.Condition[](token.networks.length),
                false
            );
        AssertionLogic.addCompositeCondition(cc, 1, networkCc);
        for (uint256 i = 0; i < token.networks.length; i++) {
            AssertionLogic.andOp(
                networkCc,
                i,
                "$network",
                AssertionLogic.Op.Equal,
                Identities.get_network_name(token.networks[i].network)
            );
        }

        AssertionLogic.andOp(
            cc,
            2,
            variable,
            min == 0
                ? AssertionLogic.Op.GreaterThan
                : AssertionLogic.Op.GreaterEq,
            StringShift.toShiftedString(min, 10 ** token.rangeDecimals)
        );
        if (max > 0 && balance > 0) {
            AssertionLogic.andOp(
                cc,
                3,
                variable,
                AssertionLogic.Op.LessThan,
                StringShift.toShiftedString(
                    uint256(max),
                    10 ** token.rangeDecimals
                )
            );
        }

        string[] memory assertions = new string[](1);
        assertions[0] = AssertionLogic.toString(cc);

        return assertions;
    }

    function isSupportedNetwork(
        TokenInfo memory token,
        uint32 network
    ) private pure returns (bool) {
        for (uint32 i = 0; i < token.networks.length; i++) {
            if (token.networks[i].network == network) {
                return true;
            }
        }
        return false;
    }

    function queryBalance(
        Identity memory identity,
        uint32 network,
        string[] memory secrets,
        string memory tokenName,
        TokenInfo memory token
    ) internal virtual returns (uint256);
}
