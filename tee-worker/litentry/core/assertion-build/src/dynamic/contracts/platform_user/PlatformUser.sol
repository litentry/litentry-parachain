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
import "../libraries/Utils.sol";
import "../DynamicAssertion.sol";
import "./DarenMarketClient.sol";

library PlatformType {
    string public constant KaratDao = "KaratDao";
    string public constant MagicCraft = "MagicCraft";
    string public constant DarenMarket = "DarenMarket";
}

contract PlatformUser is DynamicAssertion {
    function execute(
        Identity[] memory identities,
        string[] memory /*secrets*/,
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
        string memory description = "You are a user of a certain platform";
        string memory assertion_type = "Platform user";
        schema_url = "https://raw.githubusercontent.com/litentry/vc-jsonschema/main/dist/schemas/24-platform-user/1-1-2.json";

        string memory platformName = abi.decode(params, (string));

        require(isSupportedPlatform(platformName), "Platform not supported");

        bool isPlatformUser = false;
        for (uint i = 0; i < identities.length; i++) {
            Identity memory identity = identities[i];
            if (Identities.is_evm(identity)) {
                (
                    bool identityToStringSuccess,
                    string memory identityString
                ) = Utils.identityToString(
                        identity.networks[0],
                        identity.value
                    );
                if (identityToStringSuccess) {
                    isPlatformUser = checkIsPlatformUser(
                        platformName,
                        identityString
                    );
                    if (isPlatformUser) {
                        break;
                    }
                }
            }
        }

        // assemble assertions
        AssertionLogic.CompositeCondition memory cc = AssertionLogic
            .CompositeCondition(new AssertionLogic.Condition[](1), true);
        AssertionLogic.andOp(
            cc,
            0,
            "$platform",
            AssertionLogic.Op.Equal,
            platformName
        );

        string[] memory assertions = new string[](1);
        assertions[0] = AssertionLogic.toString(cc);

        return (
            description,
            assertion_type,
            assertions,
            schema_url,
            isPlatformUser
        );
    }

    function checkIsPlatformUser(
        string memory platformName,
        string memory identityString
    ) private returns (bool isPlatformUser) {
        if (Strings.equal(platformName, PlatformType.KaratDao)) {
            // TODO
        } else if (Strings.equal(platformName, PlatformType.MagicCraft)) {
            // TODO
        } else if (Strings.equal(platformName, PlatformType.DarenMarket)) {
            (bool success, bool result) = DarenMarketClient.talentAsset(
                identityString
            );
            if (success) {
                isPlatformUser = result;
            }
        }
    }

    function isSupportedPlatform(
        string memory platformName
    ) private pure returns (bool supported) {
        if (
            Strings.equal(platformName, PlatformType.KaratDao) ||
            Strings.equal(platformName, PlatformType.MagicCraft) ||
            Strings.equal(platformName, PlatformType.DarenMarket)
        ) {
            supported = true;
        }
    }
}
