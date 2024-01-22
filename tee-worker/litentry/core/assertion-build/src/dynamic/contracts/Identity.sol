// SPDX-License-Identifier: MIT

pragma solidity ^0.8.8;

struct Identity {
    uint32 identity_type;
    bytes value;
}

library IdentityUtils {
    function from(uint32 identity_type, bytes memory value)
        public
        pure
        returns (Identity memory)
    {
        return (Identity(identity_type, value));
    }

    function is_web3(Identity memory identity_type) public pure returns (bool) {
        return (is_substrate(identity_type) ||
            is_evm(identity_type) ||
            is_bitcoin(identity_type));
    }

    function is_web2(Identity memory identity_type) public pure returns (bool) {
        return (is_twitter(identity_type) ||
            is_discord(identity_type) ||
            is_github(identity_type));
    }

    function is_twitter(Identity memory identity_type)
        public
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
        public
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
        public
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
        public
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
        public
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
