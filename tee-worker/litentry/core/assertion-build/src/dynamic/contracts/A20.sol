// SPDX-License-Identifier: MIT

pragma solidity ^0.8.8;

import {DynamicAssertion, Identity} from "DynamicAssertion.sol";

contract A20 is DynamicAssertion {
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
            memory description = "The user is an early bird user of the IdentityHub EVM version and has generated at least 1 credential during 2023 Aug 14th ~ Aug 21st.";
        string memory assertion_type = "IDHub EVM Version Early Bird";
        string memory assertion = "$has_joined == true";
        bool result;

        for (uint256 i = 0; i < identities.length; i++) {
            if (is_twitter(identities[i])) {
                string
                    memory url = "http://localhost:19527/events/does-user-joined-evm-campaign?account=0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d";
                string memory jsonPointer = "/hasJoined";
                result = GetBool(url, jsonPointer);
            } else {
                string
                    memory url = "http://localhost:19527/events/does-user-joined-evm-campaign?account=test";
                string memory jsonPointer = "/hasJoined";
                result = GetBool(url, jsonPointer);
            }
            if (result) {
                break;
            }
        }
        return (description, assertion_type, assertion, result);
    }
}
