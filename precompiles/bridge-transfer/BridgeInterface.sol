// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

interface IBridgeTransfer {
	/// @notice Used to transfer assets through token bridge.
	/// @param amount: The amount of tokens to be transferred.
    /// @param dest_id: The destination chain id indicator
    /// @param resource_id: Resource indicator of type of assets transferred
    /// @param recipient: Recipient address, typically H160/H256
    /// @custom:selector 0x6e700f9a
	/// 				 transferAssets(uint256,uint8,bytes32,bytes)
    function transferAssets(uint256 amount, uint8 dest_id, bytes32 resource_id, bytes calldata recipient) external;
}
