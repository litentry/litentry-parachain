# Migrate 
Under files P9190
These migration is for the follwoing task
https://github.com/litentry/litentry-parachain/releases/tag/v0.9.19
(1) token decimal change from 12 to 18
(2) New token bridge related pallet storage migration.

Rococo Only:
For rococo, the initial migration contains some error code which make account.frozen = account.reserve * 10^6
This storage error should be fixed in the following migration of rococo

And the migration of parachain_staking, pallet_balances and bridge related migration also applied in this migration for rococo.

# MigrateCollatorSelectionIntoParachainStaking
P9100.rs
https://github.com/litentry/litentry-parachain/releases/tag/v0.9.10
This migration is for the replacement of CollatorSelection with ParachainStaking

The main purpose of runtime upgrade is for make up the missing genesis build of ParachainStaking and clean the old CollatorSelection storage.

# MigrateAtStakeAutoCompound
P9130.rs
https://github.com/litentry/litentry-parachain/releases/tag/v0.9.13
This migration is for the update of AtStaked with ParachainStaking

The main purpose of runtime upgrade is for adding the autocompound staking function of ParachainStaking and need to update storage to the latest struct.