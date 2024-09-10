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

import "@openzeppelin/contracts/utils/Strings.sol";
import "../libraries/Http.sol";
import "../libraries/Json.sol";
import "../libraries/Identities.sol";
import "../libraries/StringComparison.sol";
import "../libraries/Utils.sol";
struct SolanaTokenBalance {
    string mint;
    string amount;
}

struct EvmTokenBalance {
    string tokenAddress;
    string balance;
}

library MoralisClient {
    function getSolanaNativeBalance(
        uint32 network,
        string memory apiKey,
        string memory account
    ) internal returns (bool, string memory) {
        string memory url = string(
            abi.encodePacked(getNetworkUrl(network), "/", account, "/balance")
        );

        HttpHeader[] memory headers = new HttpHeader[](1);
        headers[0] = HttpHeader("X-API-Key", apiKey);
        (bool balanceSuccess, string memory balanceResponse) = Http.Get(
            url,
            headers
        );
        if (balanceSuccess) {
            (bool solanaSuccess, string memory solana) = Json.getString(
                balanceResponse,
                "/solana"
            );
            if (solanaSuccess) {
                return (solanaSuccess, solana);
            } else {
                return (false, "");
            }
        } else {
            return (false, "");
        }
    }

    function getSolanaTokensBalance(
        uint32 network,
        string memory apiKey,
        string memory account
    ) internal returns (bool, SolanaTokenBalance[] memory) {
        string memory url = string(
            abi.encodePacked(getNetworkUrl(network), "/", account, "/tokens")
        );
        HttpHeader[] memory headers = new HttpHeader[](1);
        headers[0] = HttpHeader("X-API-Key", apiKey);
        (bool tokensSuccess, string memory tokensResponse) = Http.Get(
            url,
            headers
        );
        if (tokensSuccess) {
            (bool arrayLenSuccess, int64 arrayLen) = Json.getArrayLen(
                tokensResponse,
                ""
            );
            if (arrayLenSuccess) {
                SolanaTokenBalance[] memory balances = new SolanaTokenBalance[](
                    uint256(int256(arrayLen))
                );
                for (uint256 i = 0; i < uint256(int256(arrayLen)); i++) {
                    (bool mintSuccess, string memory mint) = Json.getString(
                        tokensResponse,
                        string(
                            abi.encodePacked("/", Strings.toString(i), "/mint")
                        )
                    );
                    (bool amountSuccess, string memory amount) = Json.getString(
                        tokensResponse,
                        string(
                            abi.encodePacked(
                                "/",
                                Strings.toString(i),
                                "/amount"
                            )
                        )
                    );
                    if (!mintSuccess || !amountSuccess) {
                        return (false, new SolanaTokenBalance[](0));
                    } else {
                        balances[i] = SolanaTokenBalance(mint, amount);
                    }
                }
                return (true, balances);
            } else {
                return (false, new SolanaTokenBalance[](0));
            }
        } else {
            return (false, new SolanaTokenBalance[](0));
        }
    }

    function getErcTokensBalance(
        uint32 network,
        string memory apiKey,
        string memory account,
        string memory chain,
        string[] memory tokenAddresses
    ) internal returns (bool, EvmTokenBalance[] memory) {
        string memory url = string(
            abi.encodePacked(
                getNetworkUrl(network),
                "/",
                account,
                "/erc20",
                "?chain=",
                chain
            )
        );
        HttpHeader[] memory headers = new HttpHeader[](1);
        headers[0] = HttpHeader("X-API-Key", apiKey);
        if (tokenAddresses.length > 0) {
            url = string(abi.encodePacked(url, "&"));
        }
        for (uint256 i = 0; i < tokenAddresses.length; i++) {
            url = string(
                abi.encodePacked(
                    url,
                    "token_addresses[",
                    Strings.toString(i),
                    "]=",
                    tokenAddresses[i]
                )
            );
            if (i != tokenAddresses.length - 1) {
                url = string(abi.encodePacked(url, "&"));
            }
        }
        (bool tokensSuccess, string memory tokensResponse) = Http.Get(
            url,
            headers
        );
        if (tokensSuccess) {
            (bool arrayLenSuccess, int64 arrayLen) = Json.getArrayLen(
                tokensResponse,
                ""
            );
            if (arrayLenSuccess) {
                EvmTokenBalance[] memory balances = new EvmTokenBalance[](
                    uint256(int256(arrayLen))
                );
                for (uint256 i = 0; i < uint256(int256(arrayLen)); i++) {
                    (
                        bool tokenAddressSuccess,
                        string memory tokenAddress
                    ) = Json.getString(
                            tokensResponse,
                            string(
                                abi.encodePacked(
                                    "/",
                                    Strings.toString(i),
                                    "/token_address"
                                )
                            )
                        );
                    (bool balanceSuccess, string memory balance) = Json
                        .getString(
                            tokensResponse,
                            string(
                                abi.encodePacked(
                                    "/",
                                    Strings.toString(i),
                                    "/balance"
                                )
                            )
                        );
                    if (!tokenAddressSuccess || !balanceSuccess) {
                        return (false, new EvmTokenBalance[](0));
                    } else {
                        balances[i] = EvmTokenBalance(tokenAddress, balance);
                    }
                }
                return (true, balances);
            } else {
                return (false, new EvmTokenBalance[](0));
            }
        } else {
            return (false, new EvmTokenBalance[](0));
        }
    }
    function isSupportedNetwork(uint32 network) internal pure returns (bool) {
        return
            network == Web3Networks.Solana ||
            network == Web3Networks.Polygon ||
            network == Web3Networks.Arbitrum;
    }

    function getNetworkUrl(
        uint32 network
    ) internal pure returns (string memory url) {
        if (network == Web3Networks.Solana) {
            url = "https://solana-gateway.moralis.io/account/mainnet";
        } else if (
            network == Web3Networks.Ethereum ||
            network == Web3Networks.Polygon ||
            network == Web3Networks.Arbitrum
        ) {
            url = "https://deep-index.moralis.io/api/v2.2";
        }
    }

    function getTokenBalance(
        uint32 network,
        string memory apiKey,
        string memory account,
        string memory tokenContractAddress,
        uint8 tokenDecimals
    ) internal returns (uint256) {
        if (network == Web3Networks.Solana) {
            if (Strings.equal(tokenContractAddress, "Native Token")) {
                (bool success, string memory solanaTokenBalance) = MoralisClient
                    .getSolanaNativeBalance(network, apiKey, account);

                if (success) {
                    (bool parsedStatus, uint256 parsedAmount) = Utils
                        .parseDecimal(solanaTokenBalance, tokenDecimals);
                    if (parsedStatus) {
                        return parsedAmount;
                    }
                    return 0;
                }
            } else {
                (
                    bool success,
                    SolanaTokenBalance[] memory solanaTokenBalance
                ) = MoralisClient.getSolanaTokensBalance(
                        network,
                        apiKey,
                        account
                    );

                if (success) {
                    for (uint i = 0; i < solanaTokenBalance.length; i++) {
                        if (
                            Strings.equal(
                                solanaTokenBalance[i].mint,
                                tokenContractAddress
                            )
                        ) {
                            (bool parsedStatus, uint256 parsedAmount) = Utils
                                .parseDecimal(
                                    solanaTokenBalance[i].amount,
                                    tokenDecimals
                                );
                            if (parsedStatus) {
                                return parsedAmount;
                            }
                            return 0;
                        }
                    }
                }
            }
        } else {
            string[] memory tokenAddresses = new string[](1);
            tokenAddresses[0] = tokenContractAddress;
            (
                bool success,
                EvmTokenBalance[] memory evmTokenBalance
            ) = MoralisClient.getErcTokensBalance(
                    network,
                    apiKey,
                    account,
                    Identities.web3_network_to_chain(network),
                    tokenAddresses
                );

            if (
                success &&
                evmTokenBalance.length > 0 &&
                StringComparison.compareStringsIgnoreCase(
                    evmTokenBalance[0].tokenAddress,
                    tokenContractAddress
                )
            ) {
                (bool parsedStatus, uint256 parsedAmount) = Utils.parseDecimal(
                    evmTokenBalance[0].balance,
                    tokenDecimals
                );
                if (parsedStatus) {
                    return parsedAmount;
                }
                return 0;
            }
        }
        return 0;
    }
}
