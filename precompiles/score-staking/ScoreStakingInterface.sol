// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

/// @author The Litentry Team
/// @title Pallet Score Staking Interface
/// @dev The interface through which solidity contracts will interact with Bridge Transfer
/// We follow this same interface including four-byte function selectors, in the precompile that
/// wraps the pallet
interface ScoreStaking {
    /// @dev Claim some balance from unpaid_reward to the caller's account
    /// @custom:selector 379607f5
    /// @param amount the amount to claim
    function claim(uint256 amount) external;

    /// @dev Claim all balance in unpaid_reward to the caller's account
    /// @custom:selector d1058e59
    function claimAll() external;   
}
