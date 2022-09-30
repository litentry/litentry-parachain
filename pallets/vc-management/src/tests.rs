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

use crate::{mock::*, ShardIdentifier};
use frame_support::assert_ok;
use sp_core::H256;

const TEST_MRENCLAVE: [u8; 32] = [2u8; 32];

#[test]
fn generate_vc_works() {
	new_test_ext().execute_with(|| {
		let shard: ShardIdentifier = H256::from_slice(&TEST_MRENCLAVE);
		assert_ok!(VCManagement::generate_vc(Origin::signed(1), shard, 1));
		System::assert_last_event(Event::VCManagement(crate::Event::VCGenerationRequested {
			shard,
			ruleset_id: 1,
		}));
	});
}
