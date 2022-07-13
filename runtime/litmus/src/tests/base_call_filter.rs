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

use crate::{Call, Origin, Runtime};
// type OpaqueCall = WrapperKeepOpaque<<Runtime as frame_system::Config>::Call>;

#[test]
fn default_mode() {
	runtime_common::tests::base_call_filter::default_mode::<Runtime>();
}

#[test]
fn multisig_enabled() {
	runtime_common::tests::base_call_filter::multisig_enabled::<Runtime, Origin, Call>();
}

#[test]
fn balance_transfer_disabled() {
	runtime_common::tests::base_call_filter::balance_transfer_disabled::<Runtime, Origin, Call>();
}

#[test]
fn balance_transfer_with_sudo_works() {
    runtime_common::tests::base_call_filter::balance_transfer_with_sudo_works::<Runtime, Origin, Call>();
}

#[test]
fn block_core_call_has_no_effect() {
    runtime_common::tests::base_call_filter::block_core_call_has_no_effect::<Runtime, Origin, Call>();
}

#[test]
fn block_non_core_call_works() {
	runtime_common::tests::base_call_filter::block_non_core_call_works::<Runtime, Origin, Call>();
}
