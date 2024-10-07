// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

interface IPoolProposal {
	/// @notice Propose an investing pool proposal
    /// @param max_pool_size: At most this amount of raised money curator/investing pool willing to take
    /// @param proposal_last_time: How does the proposal lasts for voting/preinvesting.
    /// @param pool_last_time: How long does the investing pool last if passed
    /// @param estimated_epoch_reward: This number is only for displaying purpose without any techinical meaning
	/// @param pool_info_hash: H256 hash of pool info for including pool details
    /// @custom:selector 0x7bc55add
	/// 				 proposeInvestingPool(uint256,uint256,uint256,uint256,bytes32)
    function proposeInvestingPool(uint256 max_pool_size, uint256 proposal_last_time, uint256 pool_last_time, uint256 estimated_epoch_reward, bytes32 pool_info_hash) external;

	/// @notice Prestake the pool proposal
	/// @param pool_proposal_index: Index of pool proposal
    /// @param amount: Amount of per-staking user provides
    /// @custom:selector 0x68e3a76c
	/// 				 preStakeProposal(uint256,uint256)
    function preStakeProposal(uint256 pool_proposal_index, uint256 amount) external;

    /// @notice Withdrawal the prestaking the pool proposal
	/// @param pool_proposal_index: Index of pool proposal
    /// @param amount: Amount of per-staking user provides
    /// @custom:selector 0x389cd4af
	/// 				 withdrawPreInvesting(uint256,uint256)
    function withdrawPreInvesting(uint256 pool_proposal_index, uint256 amount) external;

    /// @notice A guardian declaring his incentive of participating pool
    /// @param pool_proposal_index: Index of pool proposal
    /// @custom:selector 0x619c08e2
	/// 				 guardianParticipateProposal(uint256)
    function guardianParticipateProposal(uint256 pool_proposal_index) external;
}