// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

interface IGuardian {
    
    /// @dev Defines GuardianVote type
    enum GuardianVote {
        Neutral,
        Aye,
        Nay,
        Specific
    }
	/// @notice Regist info hash of guardian and reserve funds, only work if not already registed
	/// @param info_hash: H256 hash of info image
    /// @custom:selector 0x3cf5464a
	/// 				 registGuardian(bytes32)
    function registGuardian(bytes32 info_hash) external;

	/// @notice Update info hash of guardian, only work if already registed
	/// @param info_hash: H256 hash of info image
    /// @custom:selector 0x2b764649
	/// 				 updateGuardian(bytes32)
    function updateGuardian(bytes32 info_hash) external;

    /// @notice Clean guardian info and return funds if not banned, otherwise no fund return
    /// @custom:selector 0xc654e77d
	/// 				 cleanGuardian()
    function cleanGuardian() external;

    /// @notice Vote guardian and express the corresponding status
    /// @custom:selector 0x55b90ea7
	/// 				 vote(bytes32,uint8,uint256)
    function vote(bytes32 guardian, GuardianVote status, uint256 potential_proposal_index) external;

    /// @notice Remove msg.sender's all existing guardian vote
    /// @custom:selector 0x3219bdc0
	/// 				 removeAllVotes()
    function removeAllVotes() external;
}