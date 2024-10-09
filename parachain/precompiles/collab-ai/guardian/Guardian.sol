// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

interface IGuardian {
    /// @dev Defines the candidate status type.
    enum CandidateStatus {
        Unverified,
	    Verified,
	    Banned
    }
    
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

    /// @notice Vote guardian and express the corresponding status, evm
    /// @custom:selector 0x55b90ea7
	/// 				 vote(bytes32,uint8,uint256)
    function vote(bytes32 guardian, GuardianVote status, uint256 potential_proposal_index) external;

    /// @notice Remove msg.sender's all existing guardian vote
    /// @custom:selector 0x3219bdc0
	/// 				 removeAllVotes()
    function removeAllVotes() external;

    /// @notice public guardian count of next guardian index will be
    /// @custom:selector 0x69d0b14e
	/// 				 publicGuardianCount()
    function publicGuardianCount() external view returns (uint256 count);

    /// @notice public guardian to index, substrate address format
    /// @param guardian: substrate format address
    /// @custom:selector 0xf46175b8
	/// 				 publicGuardianToIndex(bytes32)
    function publicGuardianToIndex(bytes32 guardian) external view returns (uint256 index);

    /// @notice public guardian to index, ethereum address format
    /// @param guardian: ethereum format address
    /// @custom:selector 0x02916580
	/// 				 publicGuardianToIndex(address)
    function publicGuardianToIndex(address guardian) external view returns (uint256 index);

    /// @notice Guardian index to guardian info
    /// @param index: Guardian index
    /// @custom:selector 0x59c95743
	/// 				 guardianIndexToInfo(address)
    function guardianIndexToInfo(uint256 index) external view returns (bytes32 info_hash, uint256 update_block, bytes32 guardian, CandidateStatus status);
    
    /// @notice Query voter's vote of one specific guardian given its guardian index, substrate
    /// @custom:selector 0xfaad0ba2
	/// 				 guardianVotes(bytes32,uint256)
    function guardianVotes(bytes32 voter, uint256 guardian_index) external view returns (GuardianVote guardian_vote, uint256 potential_proposal_index);

    /// @notice Query voter's vote of one specific guardian given its guardian index, ethereum
    /// @custom:selector 0xcbdbf0b2
	/// 				 guardianVotes(address,uint256)
    function guardianVotes(address voter, uint256 guardian_index) external view returns (GuardianVote guardian_vote, uint256 potential_proposal_index);
}