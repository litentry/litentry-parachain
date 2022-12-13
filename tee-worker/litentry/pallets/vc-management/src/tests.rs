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

use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use litentry_primitives::Status;
use parentchain_primitives::{SchemaContentString, SchemaIdString};

#[test]
fn add_vc_schema_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(VCMT::schema_index(), 0);
		let id: SchemaIdString = vec![1, 2, 3, 4].try_into().unwrap();
		let content: SchemaContentString = vec![5, 6, 7, 8].try_into().unwrap();
		assert_ok!(VCMT::add_schema(Origin::signed(1), 2, id, content));
		System::assert_last_event(Event::VCMT(crate::Event::SchemaAdd { who: 2, index: 0 }));
		assert_eq!(VCMT::schema_index(), 1);
		assert!(VCMT::schema_registry(0).is_some());
		let schema = VCMT::schema_registry(0).unwrap();
		assert_eq!(schema.status, Status::Active);
	});
}

#[test]
fn add_two_schema_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(VCMT::schema_index(), 0);
		let id: SchemaIdString = vec![1, 2, 3, 4].try_into().unwrap();
		let content: SchemaContentString = vec![5, 6, 7, 8].try_into().unwrap();
		assert_ok!(VCMT::add_schema(Origin::signed(1), 2, id.clone(), content.clone()));
		assert_ok!(VCMT::add_schema(Origin::signed(1), 2, id, content));
		System::assert_last_event(Event::VCMT(crate::Event::SchemaAdd { who: 2, index: 1 }));
		assert_eq!(VCMT::schema_index(), 2);
	});
}

#[test]
fn disable_schema_works() {
	new_test_ext().execute_with(|| {
		let id: SchemaIdString = vec![1, 2, 3, 4].try_into().unwrap();
		let content: SchemaContentString = vec![5, 6, 7, 8].try_into().unwrap();
		assert_ok!(VCMT::add_schema(Origin::signed(1), 2, id, content));
		System::assert_last_event(Event::VCMT(crate::Event::SchemaAdd { who: 2, index: 0 }));
		assert_ok!(VCMT::disable_schema(Origin::signed(1), 2, 0));
		assert_eq!(VCMT::schema_index(), 1);
		assert!(VCMT::schema_registry(0).is_some());
		let schema = VCMT::schema_registry(0).unwrap();
		assert_eq!(schema.status, Status::Disabled);
	});
}

#[test]
fn disable_schema_with_non_existent_fails() {
	new_test_ext().execute_with(|| {
		assert_noop!(VCMT::disable_schema(Origin::signed(1), 2, 1), Error::<Test>::SchemaNotExist);
	});
}

#[test]
fn activate_schema_works() {
	new_test_ext().execute_with(|| {
		let id: SchemaIdString = vec![1, 2, 3, 4].try_into().unwrap();
		let content: SchemaContentString = vec![5, 6, 7, 8].try_into().unwrap();
		assert_ok!(VCMT::add_schema(Origin::signed(1), 2, id, content));
		System::assert_last_event(Event::VCMT(crate::Event::SchemaAdd { who: 2, index: 0 }));
		assert_ok!(VCMT::disable_schema(Origin::signed(1), 2, 0));
		assert!(VCMT::schema_registry(0).is_some());
		// schema is activated
		assert_ok!(VCMT::activate_schema(Origin::signed(1), 2, 0));
		System::assert_last_event(Event::VCMT(crate::Event::SchemaActivated { who: 2, index: 0 }));
		assert_eq!(VCMT::schema_registry(0).unwrap().status, Status::Active);
	});
}

#[test]
fn revoke_schema_works() {
	new_test_ext().execute_with(|| {
		let id: SchemaIdString = vec![1, 2, 3, 4].try_into().unwrap();
		let content: SchemaContentString = vec![5, 6, 7, 8].try_into().unwrap();
		assert_ok!(VCMT::add_schema(Origin::signed(1), 2, id, content));
		System::assert_last_event(Event::VCMT(crate::Event::SchemaAdd { who: 2, index: 0 }));
		assert!(VCMT::schema_registry(0).is_some());
		assert_ok!(VCMT::revoke_schema(Origin::signed(1), 2, 0));
		System::assert_last_event(Event::VCMT(crate::Event::SchemaRevoked { who: 2, index: 0 }));
		// schema is deleted
		assert!(VCMT::schema_registry(0).is_none());
	});
}
