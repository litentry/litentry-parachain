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

import "./libraries/Identities.sol";

interface IDynamicAssertion {
    function execute(
        Identity[] memory identities,
        string[] memory secrets,
        bytes memory params
    )
        external
        returns (
            string memory description,
            string memory assertionType,
            string[] memory assertions,
            string memory schemaUrl,
            bool result
        );
}

// This proxy is for test purpose.
contract ProxyDynamicAssertion {
    address private target;

    event DynamicAssertionGenerated(
        string description,
        string assertionType,
        string[] assertions,
        string schemaUrl,
        bool result
    );

    constructor(address _target) {
        target = _target;
    }

    function execute(
        Identity[] memory identities,
        string[] memory secrets,
        bytes memory params
    )
        public
        returns (
            string memory description,
            string memory assertionType,
            string[] memory assertions,
            string memory schemaUrl,
            bool result
        )
    {
        IDynamicAssertion dynamicAssertion = IDynamicAssertion(target);
        (
            description,
            assertionType,
            assertions,
            schemaUrl,
            result
        ) = dynamicAssertion.execute(identities, secrets, params);
        emit DynamicAssertionGenerated(
            description,
            assertionType,
            assertions,
            schemaUrl,
            result
        );
    }
}
