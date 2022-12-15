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

mod base_call_filter;
mod orml_xcm;

pub mod setup {
	use crate::Runtime;
	#[cfg(test)]
	runtime_common::decl_test_chain!(Runtime);
}

mod transaction_payment {
	use crate::{Runtime, RuntimeCall, RuntimeOrigin, TransactionByteFee};

	runtime_common::run_transaction_payment_tests!();
}

mod xcm_parachain {
	use crate::{
		tests::setup::{
			ParaA, ParaB, Relay, RelayChainRuntime, RuntimeCall as RelayCall,
			RuntimeOrigin as RelayOrigin, TestNet,
		},
		xcm_config::{LocationToAccountId, UnitWeightCost},
		Runtime, RuntimeCall, RuntimeOrigin,
	};

	runtime_common::run_xcm_tests!();
}
