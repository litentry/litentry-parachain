// Copyright 2020-2023 Litentry Technologies GmbH.
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

#![cfg(test)]

use super::{
	mock::{
		assert_events, new_test_ext, RuntimeEvent, RuntimeOrigin, Test, Whitelist, ACCOUNT_A,
		ACCOUNT_B, ACCOUNT_C,
	},
	pallet::Event as PalletEvent,
	*,
};
use crate::{EnsureWhitelist, WhitelistEnsureOriginWrapper};
use frame_support::{assert_noop, assert_ok};
use sp_std::vec;

#[test]
fn add_remove_whitelist() {
	new_test_ext().execute_with(|| {
		// Default status is false
		assert!(!Whitelist::is_whitelist(&ACCOUNT_A));
		assert!(!Whitelist::is_whitelist(&ACCOUNT_B));
		assert!(!Whitelist::is_whitelist(&ACCOUNT_C));

		// Single Add: successful
		assert_ok!(Whitelist::add_whitelist(RuntimeOrigin::root(), ACCOUNT_A));
		assert!(Whitelist::is_whitelist(&ACCOUNT_A));

		// Single Add: Already exists
		assert_noop!(
			Whitelist::add_whitelist(RuntimeOrigin::root(), ACCOUNT_A),
			Error::<Test>::WhitelistAlreadyExists
		);
		// Batch Add: Already exists
		assert_noop!(
			Whitelist::batch_add_whitelists(RuntimeOrigin::root(), vec![ACCOUNT_A, ACCOUNT_B]),
			Error::<Test>::WhitelistAlreadyExists
		);

		// Batch Add: successful
		assert_ok!(Whitelist::batch_add_whitelists(
			RuntimeOrigin::root(),
			vec![ACCOUNT_B, ACCOUNT_C]
		));
		assert!(Whitelist::is_whitelist(&ACCOUNT_B));
		assert!(Whitelist::is_whitelist(&ACCOUNT_C));

		// Single remove: successful
		assert_ok!(Whitelist::remove_whitelist(RuntimeOrigin::root(), ACCOUNT_A));
		assert!(!Whitelist::is_whitelist(&ACCOUNT_A));

		// Single remove: Already removed
		assert_noop!(
			Whitelist::remove_whitelist(RuntimeOrigin::root(), ACCOUNT_A),
			Error::<Test>::WhitelistInvalid
		);

		// Batch remove: Already removed
		assert_noop!(
			Whitelist::batch_remove_whitelists(RuntimeOrigin::root(), vec![ACCOUNT_A, ACCOUNT_B]),
			Error::<Test>::WhitelistInvalid
		);

		// Batch remove: successful
		assert_ok!(Whitelist::batch_remove_whitelists(
			RuntimeOrigin::root(),
			vec![ACCOUNT_B, ACCOUNT_C]
		));
		assert!(!Whitelist::is_whitelist(&ACCOUNT_B));
		assert!(!Whitelist::is_whitelist(&ACCOUNT_C));

		assert_events(vec![
			RuntimeEvent::Whitelist(PalletEvent::WhitelistAdded(ACCOUNT_A)),
			RuntimeEvent::Whitelist(PalletEvent::WhitelistAdded(ACCOUNT_B)),
			RuntimeEvent::Whitelist(PalletEvent::WhitelistAdded(ACCOUNT_C)),
			RuntimeEvent::Whitelist(PalletEvent::WhitelistRemoved(ACCOUNT_A)),
			RuntimeEvent::Whitelist(PalletEvent::WhitelistRemoved(ACCOUNT_B)),
			RuntimeEvent::Whitelist(PalletEvent::WhitelistRemoved(ACCOUNT_C)),
		]);
	})
}

#[test]
fn whitelist_on_off_function() {
	new_test_ext().execute_with(|| {
		// Single Add: successful
		assert_ok!(Whitelist::add_whitelist(RuntimeOrigin::root(), ACCOUNT_A));

		// Default whitelist Off
		assert!(!Whitelist::whitelist_on());
		// Not on whitelist but passed
		assert_ok!(<EnsureWhitelist<Test> as WhitelistEnsureOriginWrapper<<Test as frame_system::Config>::RuntimeOrigin, <Test as frame_system::Config>::AccountId>>::ensure_origin(RuntimeOrigin::signed(ACCOUNT_B)));

		// Switch whitelist function on
		assert_ok!(Whitelist::swtich_whitelist_on(RuntimeOrigin::root()));
		assert!(Whitelist::whitelist_on());

		// Can not pass now
		assert!(<EnsureWhitelist<Test> as WhitelistEnsureOriginWrapper<<Test as frame_system::Config>::RuntimeOrigin, <Test as frame_system::Config>::AccountId>>::ensure_origin(RuntimeOrigin::signed(ACCOUNT_B)).is_err());
	})
}
