#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::{benchmarks, account};
use frame_system::RawOrigin;
use sp_std::prelude::*;
const SEED: u32 = 0;

benchmarks!{
    link_eth {
        let caller = account("caller", 0, 0);
        let account_id: T::AccountId = account("Alice", 0, SEED);

        let index: u32 = 0;
        let addr_expected = [16, 146, 71, 235, 177, 95, 237, 92, 255, 45, 73, 190, 133, 132, 185, 41, 14, 77, 9, 207];
        let expiring_block_number: u32 = 10000;
        let r = [133, 13, 66, 20, 141, 102, 233, 186, 153, 38, 81, 149, 29, 16, 191, 87, 206, 103, 230, 184, 32, 165, 174, 40, 221, 54, 212, 61, 132, 38, 254, 39];
        let s = [19, 118, 77, 20, 241, 238, 52, 206, 124, 232, 254, 37, 109, 69, 191, 253, 242, 19, 48, 32, 92, 134, 123, 2, 6, 223, 233, 225, 129, 41, 235, 116];
        let v: u8 = 28_u8;
            
    }:  link_eth(RawOrigin::Signed(caller), account_id.clone(), index, addr_expected, expiring_block_number.into(), r, s, v)

    link_btc {
        let caller = account("caller", 0, 0);
        let account_id: T::AccountId = account("Alice", 0, SEED);

        let index: u32 = 0;
        let addr_expected = vec![49, 51, 121, 55, 106, 72, 52, 85, 57, 113, 68, 112, 69, 77, 77, 119, 87, 90, 117, 52, 99, 122, 52, 107, 55, 67, 81, 107, 90, 72, 100, 101, 113, 71];
        let expiring_block_number: u32 = 10000;
        let r = [250, 57, 156, 18, 181, 153, 186, 77, 81, 242, 31, 146, 82, 115, 85, 163, 136, 220, 104, 194, 98, 88, 28, 109, 163, 113, 12, 47, 193, 183, 189, 106];
        let s = [41, 163, 172, 76, 129, 83, 66, 195, 126, 213, 207, 91, 186, 70, 255, 125, 111, 38, 123, 240, 178, 101, 22, 192, 133, 22, 245, 109, 50, 175, 225, 208];
        let v: u8 = 0_u8;
            
    }:  link_btc(RawOrigin::Signed(caller), account_id.clone(), index, addr_expected, expiring_block_number.into(), r, s, v)
}
