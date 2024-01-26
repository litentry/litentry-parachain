// SPDX-License-Identifier: MIT

pragma solidity ^0.8.8;

struct Identity {
    uint32 identity_type;
    bytes value;
}

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

    function from(uint32 identity_type, bytes memory value)
        internal
        pure
        returns (Identity memory)
    {
        return (Identity(identity_type, value));
    }

    function is_web3(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return (is_substrate(identity_type) ||
            is_evm(identity_type) ||
            is_bitcoin(identity_type));
    }

    function is_web2(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return (is_twitter(identity_type) ||
            is_discord(identity_type) ||
            is_github(identity_type));
    }

    function is_twitter(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        if (identity_type.identity_type == 0) {
            return (true);
        } else {
            return (false);
        }
    }

    function is_discord(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        if (identity_type.identity_type == 1) {
            return (true);
        } else {
            return (false);
        }
    }

    function is_github(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        if (identity_type.identity_type == 2) {
            return (true);
        } else {
            return (false);
        }
    }

    function is_substrate(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        if (identity_type.identity_type == 3) {
            return (true);
        } else {
            return (false);
        }
    }

    function is_evm(Identity memory identity_type) public pure returns (bool) {
        if (identity_type.identity_type == 4) {
            return (true);
        } else {
            return (false);
        }
    }

    function is_bitcoin(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        if (identity_type.identity_type == 5) {
            return (true);
        } else {
            return (false);
        }
    }
}
