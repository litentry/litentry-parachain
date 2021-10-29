/// Money matters.
pub mod currency {
    use crate::Balance;

    pub const UNIT: Balance = 1_000_000_000_000;
    pub const DOLLARS: Balance = UNIT;
    pub const CENTS: Balance = DOLLARS / 100;
    pub const MILLICENTS: Balance = CENTS / 1_000;

    pub const TREASURY_PROPORTION: u32 = 40u32;
    pub const AUTHOR_PROPORTION: u32 = 0u32;
    pub const BURNED_PROPORTION: u32 = 60u32;

    // linear ratio of transaction fee distribution
    // It is recommended to set sum of ratio to 100, yet only decimal loss is concerned.
    pub struct RatioOf {
	    pub treasury: u32,
	    pub author: u32,
	    pub burned: u32,
    }

    pub const TRANSACTION_PAYMENT_RATIO: RatioOf = RatioOf {treasury: TREASURY_PROPORTION, author: AUTHOR_PROPORTION, burned: BURNED_PROPORTION};

    /// Function used in some fee configurations
    pub const fn deposit(items: u32, bytes: u32) -> Balance {
        items as Balance * DOLLARS + (bytes as Balance) * 100 * MILLICENTS
    }
}