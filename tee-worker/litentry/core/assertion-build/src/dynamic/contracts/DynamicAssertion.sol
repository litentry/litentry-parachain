// SPDX-License-Identifier: MIT

pragma solidity ^0.8.8;

import {Identity} from "Identity.sol";

abstract contract DynamicAssertion {
    function execute(bytes memory input)
        public
        returns (
            string memory,
            string memory,
            string memory,
            bool
        )
    {
        Identity[] memory identities = abi.decode(input, (Identity[]));
        return doExecute(identities);
    }

    function doExecute(Identity[] memory identities)
        internal
        virtual
        returns (
            string memory,
            string memory,
            string memory,
            bool
        );

    function GetI64(string memory url, string memory jsonPointer)
        internal
        returns (int64)
    {
        int64 value;

        assembly {
            let url_len := mload(url) // Length is stored at first 32 byte word.
            let pointer_len := mload(jsonPointer) // Length is stored at first 32 byte word.
            // Free memory pointer, we will store output here
            let memPtr := mload(0x40)
            let input_len := add(url, jsonPointer)
            // call inputs are: gas, address, wei, input_start, input size, output_start, output_size
            // use pointer to url as start, we assume both values (url and pointer) are placed next to each other in the memory
            if iszero(call(not(0), 0x02, 0, url, input_len, memPtr, 0x20)) {
                revert(0, 0)
            }
            // advance free memory pointer
            mstore(0x40, add(memPtr, 0x20))
            value := mload(memPtr)
        }

        return (value);
    }
}
