/// Money matters.
pub mod currency {
    use crate::Balance;

    pub const UNIT: Balance = 1_000_000_000_000;
    pub const DOLLARS: Balance = UNIT;
    pub const CENTS: Balance = DOLLARS / 100;
    pub const MILLICENTS: Balance = CENTS / 1_000;

    /// Function used in some fee configurations
    pub const fn deposit(items: u32, bytes: u32) -> Balance {
        items as Balance * DOLLARS + (bytes as Balance) * 100 * MILLICENTS
    }
}