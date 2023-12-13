// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

/// @author The Litentry Team
/// @title Pallet Parachain Staking Interface
/// @dev The interface through which solidity contracts will interact with Parachain Staking
/// We follow this same interface including four-byte function selectors, in the precompile that
/// wraps the pallet
interface ParachainStaking {
/* TODO: Only part for delagator is implemented for minimal task purpose 

    /// @dev Check whether the specified address is currently a staking delegator
    /// @custom:selector ef9bb4a8
    /// @param delegator the address that we want to confirm is a delegator
    /// @return A boolean confirming whether the address is a delegator
    function isDelegator(bytes32 delegator) external view returns (bool);

    /// @dev Check whether the specified address is currently a collator candidate
    /// @custom:selector b89694c6
    /// @param candidate the address that we want to confirm is a collator andidate
    /// @return A boolean confirming whether the address is a collator candidate
    function isCandidate(bytes32 candidate) external view returns (bool);

    /// @dev Check whether the specifies address is currently a part of the active set
    /// @custom:selector d026dbf5
    /// @param candidate the address that we want to confirm is a part of the active set
    /// @return A boolean confirming whether the address is a part of the active set
    function isSelectedCandidate(
        bytes32 candidate
    ) external view returns (bool);

    /// @dev Total points awarded to all collators in a particular round
    /// @custom:selector 63fa9a87
    /// @param round the round for which we are querying the points total
    /// @return The total points awarded to all collators in the round
    function points(uint32 round) external view returns (uint32);

    /// @dev Total points awarded to a specific collator in a particular round.
    /// A value of `0` may signify that no blocks were produced or that the storage for that round has been removed
    /// @custom:selector 7bb52d11
    /// @param round the round for which we are querying the awarded points
    /// @param candidate The candidate to whom the points are awarded
    /// @return The total points awarded to the collator for the provided round
    function awardedPoints(
        uint32 round,
        bytes32 candidate
    ) external view returns (uint32);

    /// @dev The amount delegated in support of the candidate by the delegator
    /// @custom:selector b04544c4
    /// @param delegator Who made this delegation
    /// @param candidate The candidate for which the delegation is in support of
    /// @return The amount of the delegation in support of the candidate by the delegator
    function delegationAmount(
        bytes32 delegator,
        bytes32 candidate
    ) external view returns (uint256);

    /// @dev Whether the delegation is in the top delegations
    /// @custom:selector 5cd9ac97
    /// @param delegator Who made this delegation
    /// @param candidate The candidate for which the delegation is in support of
    /// @return If delegation is in top delegations (is counted)
    function isInTopDelegations(
        bytes32 delegator,
        bytes32 candidate
    ) external view returns (bool);

    /// @dev Get the minimum delegation amount
    /// @custom:selector 02985992
    /// @return The minimum delegation amount
    function minDelegation() external view returns (uint256);

    /// @dev Get the CandidateCount weight hint
    /// @custom:selector a9a981a3
    /// @return The CandidateCount weight hint
    function candidateCount() external view returns (uint32);

    /// @dev Get the current round number
    /// @custom:selector 146ca531
    /// @return The current round number
    function round() external view returns (uint32);

    /// @dev Get the CandidateDelegationCount weight hint
    /// @custom:selector 8aee59d4
    /// @param candidate The address for which we are querying the nomination count
    /// @return The number of nominations backing the collator
    function candidateDelegationCount(
        bytes32 candidate
    ) external view returns (uint32);

    /// @dev Get the CandidateAutoCompoundingDelegationCount weight hint
    /// @custom:selector 5bbcd751
    /// @param candidate The address for which we are querying the auto compounding
    ///     delegation count
    /// @return The number of auto compounding delegations
    function candidateAutoCompoundingDelegationCount(
        bytes32 candidate
    ) external view returns (uint32);

    /// @dev Get the DelegatorDelegationCount weight hint
    /// @custom:selector 46fbaa60
    /// @param delegator The address for which we are querying the delegation count
    /// @return The number of delegations made by the delegator
    function delegatorDelegationCount(
        bytes32 delegator
    ) external view returns (uint32);

    /// @dev Get the selected candidates for the current round
    /// @custom:selector bcf868a6
    /// @return The selected candidate accounts
    function selectedCandidates() external view returns (bytes32[] memory);
TODO: Only part for delagator is implemented for minimal task purpose */

    /// @dev Whether there exists a pending request for a delegation made by a delegator
    /// @custom:selector e40d85c0
    /// @param delegator the delegator that made the delegation
    /// @param candidate the candidate for which the delegation was made
    /// @return Whether a pending request exists for such delegation
    function delegationRequestIsPending(
        bytes32 delegator,
        bytes32 candidate
    ) external view returns (bool);

/* TODO: Only part for delagator is implemented for minimal task purpose
    /// @dev Whether there exists a pending exit for candidate
    /// @custom:selector 89485bd5
    /// @param candidate the candidate for which the exit request was made
    /// @return Whether a pending request exists for such delegation
    function candidateExitIsPending(
        bytes32 candidate
    ) external view returns (bool);

    /// @dev Whether there exists a pending bond less request made by a candidate
    /// @custom:selector 30d1a8eb
    /// @param candidate the candidate which made the request
    /// @return Whether a pending bond less request was made by the candidate
    function candidateRequestIsPending(
        bytes32 candidate
    ) external view returns (bool);

    /// @dev Returns the percent value of auto-compound set for a delegation
    /// @custom:selector ac8b7e0f
    /// @param delegator the delegator that made the delegation
    /// @param candidate the candidate for which the delegation was made
    /// @return Percent of rewarded amount that is auto-compounded on each payout
    function delegationAutoCompound(
        bytes32 delegator,
        bytes32 candidate
    ) external view returns (uint8);

    /// @dev Join the set of collator candidates
    /// @custom:selector 28716aba
    /// @param amount The amount self-bonded by the caller to become a collator candidate
    function joinCandidates(uint256 amount) external;

    /// @dev Request to leave the set of collator candidates
    /// @custom:selector ad47b36c
    function scheduleLeaveCandidates() external;

    /// @dev Execute due request to leave the set of collator candidates
    /// @custom:selector f3651d1e
    /// @param candidate The candidate address for which the pending exit request will be executed
    function executeLeaveCandidates(bytes32 candidate) external;

    /// @dev Cancel request to leave the set of collator candidates
    /// @custom:selector c35da2ff
    function cancelLeaveCandidates() external;

    /// @dev Temporarily leave the set of collator candidates without unbonding
    /// @custom:selector a6485ccd
    function goOffline() external;

    /// @dev Rejoin the set of collator candidates if previously had called `goOffline`
    /// @custom:selector 6e5b676b
    function goOnline() external;

    /// @dev Request to bond more for collator candidates
    /// @custom:selector a52c8643
    /// @param more The additional amount self-bonded
    function candidateBondMore(uint256 more) external;

    /// @dev Request to bond less for collator candidates
    /// @custom:selector 60744ae0
    /// @param less The amount to be subtracted from self-bond and unreserved
    function scheduleCandidateBondLess(uint256 less) external;

    /// @dev Execute pending candidate bond request
    /// @custom:selector e0c41737
    /// @param candidate The address for the candidate for which the request will be executed
    function executeCandidateBondLess(bytes32 candidate) external;

    /// @dev Cancel pending candidate bond request
    /// @custom:selector b5ad5f07
    function cancelCandidateBondLess() external;
TODO: Only part for delagator is implemented for minimal task purpose */

    /// @notice DEPRECATED use delegateWithAutoCompound instead for lower weight and better UX
    /// @dev Make a delegation in support of a collator candidate
    /// @custom:selector 8eb7cb38
    /// @param candidate The address of the supported collator candidate
    /// @param amount The amount bonded in support of the collator candidate
    function delegate(
        bytes32 candidate,
        uint256 amount
    ) external;

    /// @dev Make a delegation in support of a collator candidate
    /// @custom:selector b7272d64
    /// @param candidate The address of the supported collator candidate
    /// @param amount The amount bonded in support of the collator candidate
    /// @param autoCompound The percent of reward that should be auto-compounded
    function delegateWithAutoCompound(
        bytes32 candidate,
        uint256 amount,
        uint8 autoCompound
    ) external;

    /// @dev Request to revoke an existing delegation
    /// @custom:selector 98803c17
    /// @param candidate The address of the collator candidate which will no longer be supported
    function scheduleRevokeDelegation(bytes32 candidate) external;

    /// @dev Bond more for delegators with respect to a specific collator candidate
    /// @custom:selector 5bd015e0
    /// @param candidate The address of the collator candidate for which delegation shall increase
    /// @param more The amount by which the delegation is increased
    function delegatorBondMore(bytes32 candidate, uint256 more) external;

    /// @dev Request to bond less for delegators with respect to a specific collator candidate
    /// @custom:selector 67faedf9
    /// @param candidate The address of the collator candidate for which delegation shall decrease
    /// @param less The amount by which the delegation is decreased (upon execution)
    function scheduleDelegatorBondLess(
        bytes32 candidate,
        uint256 less
    ) external;

    /// @dev Execute pending delegation request (if exists && is due)
    /// @custom:selector 05b46558
    /// @param delegator The address of the delegator
    /// @param candidate The address of the candidate
    function executeDelegationRequest(
        bytes32 delegator,
        bytes32 candidate
    ) external;

    /// @dev Cancel pending delegation request (already made in support of input by caller)
    /// @custom:selector 1048a9c0
    /// @param candidate The address of the candidate
    function cancelDelegationRequest(bytes32 candidate) external;

    /// @dev Sets an auto-compound value for a delegation
    /// @custom:selector ff03bb11
    /// @param candidate The address of the supported collator candidate
    /// @param value The percent of reward that should be auto-compounded
    function setAutoCompound(
        bytes32 candidate,
        uint8 value
    ) external;

/* TODO: Only part for delagator is implemented for minimal task purpose 
    /// @dev Fetch the total staked amount of a delegator, regardless of the
    /// candidate.
    /// @custom:selector 1f2d4c64
    /// @param delegator Address of the delegator.
    /// @return Total amount of stake.
    function getDelegatorTotalStaked(
        bytes32 delegator
    ) external view returns (uint256);

    /// @dev Fetch the total staked towards a candidate.
    /// @custom:selector c3c02a96
    /// @param candidate Address of the candidate.
    /// @return Total amount of stake.
    function getCandidateTotalCounted(
        bytes32 candidate
    ) external view returns (uint256);
TODO: Only part for delagator is implemented for minimal task purpose */
}
