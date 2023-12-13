// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

/// @author The Litentry Team
/// @title Pallet Bridge Transfer Interface
/// @dev The interface through which solidity contracts will interact with Bridge Transfer
/// We follow this same interface including four-byte function selectors, in the precompile that
/// wraps the pallet
interface BridgeTransfer {
    /// @dev Transfers some amount of the native token to some recipient on a (whitelisted) destination chain
    /// @custom:selector c7358d27
    /// @param amount the delegator that made the delegation
    /// @param receipt the candidate for which the delegation was made
    /// @param dest_id a pending request exists for such delegation
    function transferNative(
        uint256 amount,
        bytes calldata receipt,
        uint8 dest_id
    ) external;
}
