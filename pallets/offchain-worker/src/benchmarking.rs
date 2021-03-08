#![cfg(feature = "runtime-benchmarks")]

use super::*;

use frame_benchmarking::{benchmarks, account};
use frame_system::RawOrigin;
use sp_std::prelude::*;

benchmarks!{

    asset_claim {
        let caller = account("caller", 0, 0);
        
    }: asset_claim(RawOrigin::Signed(caller))

    submit_balance {
        let caller = account("caller", 0, 0);
        let account_id = account("Alice", 0, 0);
        <ClaimAccountIndex<T>>::insert(&account_id, Some(0_u32));
        let block_number = 1_u32;
        let data_source = urls::DataSource::EthEtherScan;
        let balance = 0_u128;
        
    }: submit_balance(RawOrigin::Signed(caller), account_id, block_number.into(), data_source.into(), balance)
}

