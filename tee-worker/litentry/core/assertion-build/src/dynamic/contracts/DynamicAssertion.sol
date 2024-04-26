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

pragma solidity ^0.8.8;

    struct Identity {
        uint32 identity_type;
        bytes value;
        uint32[] networks;
    }

    struct HttpHeader {
        string name;
        string value;
    }

abstract contract DynamicAssertion {
    string[] assertions;
    string schema_url;

    function execute(Identity[] memory identities, string[] memory secrets)
    public
    virtual
    returns (
        string memory,
        string memory,
        string[] memory,
        string memory,
        bool
    );

    function encode_params(string memory url, string memory jsonPointer)
    internal
    pure
    returns (bytes memory)
    {
        return abi.encode(url, jsonPointer);
    }

    function GetI64(
        string memory url,
        string memory jsonPointer,
        HttpHeader[] memory headers
    ) internal returns (int64) {
        int64 value;

        bytes memory encoded_params = abi.encode(url, jsonPointer, headers);
        uint256 encoded_params_len = encoded_params.length;
        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x03E8,
                    0,
                    add(encoded_params, 0x20),
                    encoded_params_len,
                    memPtr,
                    0x20
                )
            ) {
                revert(0, 0)
            }
            value := mload(memPtr)
        }

        return (value);
    }

    function GetBool(
        string memory url,
        string memory jsonPointer,
        HttpHeader[] memory headers
    ) internal returns (bool) {
        bool value;

        bytes memory encoded_params = abi.encode(url, jsonPointer, headers);
        uint256 encoded_params_len = encoded_params.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x03E9,
                    0,
                    add(encoded_params, 0x20),
                    encoded_params_len,
                    memPtr,
                    0x20
                )
            ) {
                revert(0, 0)
            }
            value := mload(memPtr)
        }

        return (value);
    }

    function concatenateStrings(string memory a, string memory b)
    internal
    pure
    returns (string memory)
    {
        bytes memory concatenatedBytes = abi.encodePacked(a, b);
        return string(concatenatedBytes);
    }

    function toHex(bytes memory bytes_value)
    internal
    returns (string memory returnVal)
    {
        bytes memory encoded = abi.encode(bytes_value);
        uint256 encoded_len = encoded.length;

        assembly {
            let memPtr := mload(0x40)
            if iszero(
                call(
                    not(0),
                    0x041B,
                    0,
                    add(encoded, 0x20),
                    encoded_len,
                    returnVal,
                    0x82
                )
            ) {
                revert(0, 0)
            }
        }
    }

    function from(
        uint32 identity_type,
        bytes memory value,
        uint32[] memory networks
    ) internal pure returns (Identity memory) {
        return (Identity(identity_type, value, networks));
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

    function is_twitter(Identity memory identity) internal pure returns (bool) {
        return is_of_type(identity, 0);
    }

    function is_discord(Identity memory identity) internal pure returns (bool) {
        return is_of_type(identity, 1);
    }

    function is_github(Identity memory identity) internal pure returns (bool) {
        return is_of_type(identity, 2);
    }

    function is_substrate(Identity memory identity)
    internal
    pure
    returns (bool)
    {
        return is_of_type(identity, 3);
    }

    function is_evm(Identity memory identity) internal pure returns (bool) {
        return is_of_type(identity, 4);
    }

    function is_bitcoin(Identity memory identity) internal pure returns (bool) {
        return is_of_type(identity, 5);
    }

    function is_solana(Identity memory identity) internal pure returns (bool) {
        return is_of_type(identity, 6);
    }

    function is_of_type(Identity memory identity, uint32 identity_type)
    internal
    pure
    returns (bool)
    {
        if (identity.identity_type == identity_type) {
            return (true);
        } else {
            return (false);
        }
    }

    function has_polkadot_network(Identity memory identity_type)
    internal
    pure
    returns (bool)
    {
        return has_network(identity_type, 0);
    }

    function has_kusama_network(Identity memory identity_type)
    internal
    pure
    returns (bool)
    {
        return has_network(identity_type, 1);
    }

    function has_litentry_network(Identity memory identity_type)
    internal
    pure
    returns (bool)
    {
        return has_network(identity_type, 2);
    }

    function has_litmus_network(Identity memory identity_type)
    internal
    pure
    returns (bool)
    {
        return has_network(identity_type, 3);
    }

    function has_litentry_rococo_network(Identity memory identity_type)
    internal
    pure
    returns (bool)
    {
        return has_network(identity_type, 4);
    }

    function has_khala_network(Identity memory identity_type)
    internal
    pure
    returns (bool)
    {
        return has_network(identity_type, 5);
    }

    function has_substrate_testnet_network(Identity memory identity_type)
    internal
    pure
    returns (bool)
    {
        return has_network(identity_type, 6);
    }

    function has_ethereum_network(Identity memory identity_type)
    internal
    pure
    returns (bool)
    {
        return has_network(identity_type, 7);
    }

    function has_bsc_network(Identity memory identity_type)
    internal
    pure
    returns (bool)
    {
        return has_network(identity_type, 8);
    }

    function has_bitcoin_p2tr_network(Identity memory identity_type)
    internal
    pure
    returns (bool)
    {
        return has_network(identity_type, 9);
    }

    function has_bitcoin_p2pkh_network(Identity memory identity_type)
    internal
    pure
    returns (bool)
    {
        return has_network(identity_type, 10);
    }

    function has_bitcoin_p2sh_network(Identity memory identity_type)
    internal
    pure
    returns (bool)
    {
        return has_network(identity_type, 11);
    }

    function has_bitcoin_p2wpkh_network(Identity memory identity_type)
    internal
    pure
    returns (bool)
    {
        return has_network(identity_type, 12);
    }

    function has_bitcoin_p2wsh_network(Identity memory identity_type)
    internal
    pure
    returns (bool)
    {
        return has_network(identity_type, 13);
    }

    function has_polygon_network(Identity memory identity_type)
    internal
    pure
    returns (bool)
    {
        return has_network(identity_type, 14);
    }

    function has_arbitrum_network(Identity memory identity_type)
    internal
    pure
    returns (bool)
    {
        return has_network(identity_type, 15);
    }

    function has_solana_network(Identity memory identity_type)
    internal
    pure
    returns (bool)
    {
        return has_network(identity_type, 16);
    }

    function has_network(Identity memory identity_type, uint32 network)
    internal
    pure
    returns (bool)
    {
        for (uint256 i = 0; i < identity_type.networks.length; i++) {
            if (identity_type.networks[i] == network) {
                return (true);
            }
        }
        return (false);
    }
}
