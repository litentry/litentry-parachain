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
        string[] memory,
        string memory,
        bool
    )
    {
        string
        memory description = "The user is an early bird user of the IdentityHub EVM version and has generated at least 1 credential during 2023 Aug 14th ~ Aug 21st.";
        string memory assertion_type = "IDHub EVM Version Early Bird";
        assertions.push(
            '{ "src": "$has_web2_account", "op": "==", "dst": "true" }'
        );
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
                result = GetBool(url, jsonPointer);
                if (result) {
                    break;
                }
            }
        }
        return (description, assertion_type, assertions, schema_url, result);
    }
}
