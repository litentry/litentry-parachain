//!Primitives for teeracle
#![cfg_attr(not(feature = "std"), no_std)]
use common_primitives::PalletString;
use substrate_fixed::types::U32F32;

pub const MAX_ORACLE_DATA_NAME_LEN: usize = 40;

pub type ExchangeRate = U32F32;
pub type TradingPairString = PalletString;
pub type MarketDataSourceString = PalletString;
pub type OracleDataName = PalletString;
pub type DataSource = PalletString;
