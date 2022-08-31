// Copyright 2020-2022 Litentry Technologies GmbH.
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

//! Inspired by:
//! - Moonbeam `pallet_parachain_staking`
//! implementations.

//! traits for parachain-staking

pub trait OnCollatorPayout<AccountId, Balance> {
	fn on_collator_payout(
		for_round: crate::RoundIndex,
		collator_id: AccountId,
		amount: Balance,
	) -> frame_support::pallet_prelude::Weight;
}
impl<AccountId, Balance> OnCollatorPayout<AccountId, Balance> for () {
	fn on_collator_payout(
		_for_round: crate::RoundIndex,
		_collator_id: AccountId,
		_amount: Balance,
	) -> frame_support::pallet_prelude::Weight {
		0
	}
}

pub trait OnNewRound {
	fn on_new_round(round_index: crate::RoundIndex) -> frame_support::pallet_prelude::Weight;
}
impl OnNewRound for () {
	fn on_new_round(_round_index: crate::RoundIndex) -> frame_support::pallet_prelude::Weight {
		0
	}
}

pub trait IssuanceAdapter<Balance> {
	fn adapted_total_issuance() -> Balance;
}

use sp_runtime::traits::AtLeast32BitUnsigned;
impl<Balance: AtLeast32BitUnsigned> IssuanceAdapter<Balance> for () {
	fn adapted_total_issuance() -> Balance {
		0u32.into()
	}
}
