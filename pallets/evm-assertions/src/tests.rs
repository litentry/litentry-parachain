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
use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use sp_core::H160;

#[test]
fn should_create_new_assertion() {
	new_test_ext().execute_with(|| {
		let assertion_id: H160 = H160::from_slice(&[1u8; 20]);
		let byte_code = [0u8; 256].to_vec();
		let secrets = vec![[2u8; 13].to_vec(), [3u8; 32].to_vec()];

		assert_ok!(EvmAssertions::create_assertion(
			RuntimeOrigin::root(),
			assertion_id,
			byte_code.clone(),
			secrets.clone()
		));
		System::assert_last_event(RuntimeEvent::EvmAssertions(crate::Event::AssertionCreated {
			id: assertion_id,
			byte_code,
			secrets,
		}));
	});
}

#[test]
fn should_not_create_new_assertion_if_exists() {
	new_test_ext().execute_with(|| {
		let assertion_id: H160 = H160::from_slice(&[1u8; 20]);
		let byte_code = [0u8; 256].to_vec();
		let secrets = vec![[2u8; 13].to_vec(), [3u8; 32].to_vec()];

		assert_ok!(EvmAssertions::create_assertion(
			RuntimeOrigin::root(),
			assertion_id,
			byte_code.clone(),
			secrets.clone()
		));

		assert_noop!(
			EvmAssertions::create_assertion(
				RuntimeOrigin::root(),
				assertion_id,
				byte_code,
				secrets
			),
			Error::<Test>::AssertionExists
		);
	});
}
