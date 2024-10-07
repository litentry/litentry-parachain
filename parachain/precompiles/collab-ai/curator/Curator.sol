// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

interface ICurator {
	/// @notice Regist info hash of curator and reserve funds, only work if not already registed
	/// @param info_hash: H256 hash of info image
    /// @custom:selector 0x8ead391c
	/// 				 registCurator(bytes32)
    function registCurator(bytes32 info_hash) external;

	/// @notice Update info hash of curator, only work if already registed
	/// @param info_hash: H256 hash of info image
    /// @custom:selector 0x457c00e6
	/// 				 updateCurator(bytes32)
    function updateCurator(bytes32 info_hash) external;

    /// @notice clean curator info and return funds if not banned, otherwise no fund return
    /// @custom:selector 0xe3b134e6
	/// 				 cleanCurator()
    function cleanCurator() external;
}