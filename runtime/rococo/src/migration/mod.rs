// Copyright 2020-2024 Trust Computing GmbH.
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
pub mod migrate_identity;
pub use migrate_identity::ReplacePalletIdentityStorage;
pub mod migrate_multisig;
pub use migrate_multisig::ReplacePalletMultisigStorage;
pub mod migrate_proxy;
pub use migrate_proxy::ReplacePalletProxyStorage;
pub mod migrate_vesting;
pub use migrate_vesting::ReplacePalletVestingStorage;
pub mod migrate_bounty;
pub use migrate_bounty::ReplacePalletBountyStorage;
pub mod migrate_democracy;
pub use migrate_democracy::ReplaceDemocracyStorage;
pub mod migrate_preimage;
pub use migrate_preimage::ReplacePreImageStorage;
pub mod migrate_treasury;
pub use migrate_treasury::ReplaceTreasuryStorage;
pub mod fix_balances;
pub use fix_balances::ForceFixAccountFrozenStorage;
