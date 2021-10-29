// Copyright 2020-2021 Litentry Technologies GmbH.
// This file is part of Litentry.
// 
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

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