// SPDX-License-Identifier: MIT

pragma solidity ^0.8.8;

import {DynamicAssertion} from "DynamicAssertion.sol";
import {Identity, IdentityUtils} from "Identity.sol";

contract IsAgeOver is DynamicAssertion {
    function doExecute(Identity[] memory identities)
        internal
        override
        returns (
            string memory,
            string memory,
            string memory,
            bool
        )
    {
        string memory url = "https://dummy.restapiexample.com/api/v1/employees";
        string memory pointer = "/data/3/employee_age";

        int64 age = GetI64(url, pointer);
        string memory description = "Is the employee over 50 years old ?";
        string memory assertion_type = "Is over 50";
        string memory assertion = "age > 50";
        bool result;

        if (IdentityUtils.is_web3(identities[0])) {
            result = true;
        } else {
            result = false;
        }

        return (description, assertion_type, assertion, result);
    }
}
