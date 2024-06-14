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

struct Identity {
    uint32 identity_type;
    bytes value;
    uint32[] networks;
}

library IdentityTypes {
    // web2
    uint32 public constant Twitter = 0;
    uint32 public constant Discord = 1;
    uint32 public constant Github = 2;

    // web3
    uint32 public constant Substrate = 3;
    uint32 public constant Evm = 4;
    uint32 public constant Bitcoin = 5;
    uint32 public constant Solana = 6;
}

library Web3Networks {
    // substrate
    uint32 public constant Polkadot = 0;
    uint32 public constant Kusama = 1;
    uint32 public constant Litentry = 2;
    uint32 public constant Litmus = 3;
    uint32 public constant LitentryRococo = 4;
    uint32 public constant Khala = 5;
    uint32 public constant SubstrateTestnet = 6;

    // evm
    uint32 public constant Ethereum = 7;
    uint32 public constant Bsc = 8;
    uint32 public constant Polygon = 14;
    uint32 public constant Arbitrum = 15;
    uint32 public constant Solana = 16;
    uint32 public constant Combo = 17;

    // btc
    uint32 public constant BitcoinP2tr = 9;
    uint32 public constant BitcoinP2pkh = 10;
    uint32 public constant BitcoinP2sh = 11;
    uint32 public constant BitcoinP2wpkh = 12;
    uint32 public constant BitcoinP2wsh = 13;
}

library Identities {
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
        return is_of_type(identity, IdentityTypes.Twitter);
    }

    function is_discord(Identity memory identity) internal pure returns (bool) {
        return is_of_type(identity, IdentityTypes.Discord);
    }

    function is_github(Identity memory identity) internal pure returns (bool) {
        return is_of_type(identity, IdentityTypes.Github);
    }

    function is_substrate(Identity memory identity)
        internal
        pure
        returns (bool)
    {
        return is_of_type(identity, IdentityTypes.Substrate);
    }

    function is_evm(Identity memory identity) internal pure returns (bool) {
        return is_of_type(identity, IdentityTypes.Evm);
    }

    function is_bitcoin(Identity memory identity) internal pure returns (bool) {
        return is_of_type(identity, IdentityTypes.Bitcoin);
    }

    function is_solana(Identity memory identity) internal pure returns (bool) {
        return is_of_type(identity, IdentityTypes.Solana);
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
        return has_network(identity_type, Web3Networks.Polkadot);
    }

    function has_kusama_network(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return has_network(identity_type, Web3Networks.Kusama);
    }

    function has_litentry_network(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return has_network(identity_type, Web3Networks.Litentry);
    }

    function has_litmus_network(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return has_network(identity_type, Web3Networks.Litmus);
    }

    function has_litentry_rococo_network(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return has_network(identity_type, Web3Networks.LitentryRococo);
    }

    function has_khala_network(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return has_network(identity_type, Web3Networks.Khala);
    }

    function has_substrate_testnet_network(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return has_network(identity_type, Web3Networks.SubstrateTestnet);
    }

    function has_ethereum_network(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return has_network(identity_type, Web3Networks.Ethereum);
    }

    function has_bsc_network(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return has_network(identity_type, Web3Networks.Bsc);
    }

    function has_bitcoin_p2tr_network(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return has_network(identity_type, Web3Networks.BitcoinP2tr);
    }

    function has_bitcoin_p2pkh_network(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return has_network(identity_type, Web3Networks.BitcoinP2pkh);
    }

    function has_bitcoin_p2sh_network(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return has_network(identity_type, Web3Networks.BitcoinP2sh);
    }

    function has_bitcoin_p2wpkh_network(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return has_network(identity_type, Web3Networks.BitcoinP2wpkh);
    }

    function has_bitcoin_p2wsh_network(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return has_network(identity_type, Web3Networks.BitcoinP2wsh);
    }

    function has_polygon_network(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return has_network(identity_type, Web3Networks.Polygon);
    }

    function has_arbitrum_network(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return has_network(identity_type, Web3Networks.Arbitrum);
    }

    function has_solana_network(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return has_network(identity_type, Web3Networks.Solana);
    }

    function has_combo_network(Identity memory identity_type)
        internal
        pure
        returns (bool)
    {
        return has_network(identity_type, Web3Networks.Combo);
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
