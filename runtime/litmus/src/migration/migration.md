# Migrate
Under fiels P9191
The migration including the following pallets:
Minor pallet migration
Bounty, Democracy, Identity, Multisig, Preimage, Proxy, Treasury, Vesting

Big pallet migration:
Balances,
ChainBridge, BridgeTransfer => AssetsHandler

These migration is for the follwoing task
https://github.com/litentry/litentry-parachain/releases/tag/v0.9.19-02
(1) token decimal change from 12 to 18
(2) New token bridge related pallet storage migration.


# RemoveSudoAndStorage
P9115.rs
https://github.com/litentry/litentry-parachain/releases/tag/v0.9.11-1
This migration is for the remove of Sudo on Litmus

The main purpose of runtime upgrade is for removing sudo and its storage.