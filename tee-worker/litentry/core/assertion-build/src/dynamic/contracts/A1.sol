// SPDX-License-Identifier: MIT

pragma solidity ^0.8.8;

import {DynamicAssertion, Identity} from "DynamicAssertion.sol";

contract A1 is DynamicAssertion {
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
        string
            memory description = "You've identified at least one account/address in both Web2 and Web3.";
        string memory assertion_type = "Basic Identity Verification";
        string
            memory assertion = "$has_web2_account == true and $has_web3_account == true";
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

        return (description, assertion_type, assertion, result);
    }
}
