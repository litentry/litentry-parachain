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

use crate::{
	identity_context::IdentityContext, mock::*, Error, MetadataOf, ParentchainBlockNumber,
	UserShieldingKeyType,
};
use frame_support::{assert_noop, assert_ok};
use litentry_primitives::{
	Identity, IdentityHandle, IdentityString, IdentityWebType, Web2Network, USER_SHIELDING_KEY_LEN,
};

#[test]
fn add_vc_schema_works() {
	new_test_ext().execute_with(|| {
		let shielding_key: UserShieldingKeyType = [0u8; USER_SHIELDING_KEY_LEN];
		assert_eq!(IMT::user_shielding_keys(2), None);
		assert_ok!(IMT::set_user_shielding_key(Origin::signed(1), 2, shielding_key.clone()));
		assert_eq!(IMT::user_shielding_keys(2), Some(shielding_key.clone()));
		System::assert_last_event(Event::IMT(crate::Event::UserShieldingKeySet {
			who: 2,
			key: shielding_key,
		}));
	});
}
