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

pub mod setup {
	use crate::Runtime;
	#[cfg(test)]
	runtime_common::decl_test_chain!(Runtime);
}

mod base_call_filter {
	use crate::{Call, Origin, Runtime};

	runtime_common::run_call_filter_tests!();
}

mod transaction_payment {
	use crate::{Call, Origin, Runtime, TransactionByteFee};

	runtime_common::run_transaction_payment_tests!();
}

mod xcm_parachain {
	use crate::{
		tests::setup::{
			Call as RelayCall, Origin as RelayOrigin, ParaA, ParaB, Relay, RelayChainRuntime,
			TestNet,
		},
		xcm_config::{LocationToAccountId, UnitWeightCost},
		Call, Origin, Runtime,
	};

	runtime_common::run_xcm_tests!();
}
