pub mod migrate_proxy;
pub use migrate_proxy::ReplacePalletProxyStorage;
pub mod migrate_vesting;
pub use migrate_vesting::ReplacePalletVestingStorage;
pub mod parachain_staking;
pub use parachain_staking::ReplaceParachainStakingStorage;
pub mod balances_transaction_payment;
pub use balances_transaction_payment::ReplaceBalancesRelatedStorage;
