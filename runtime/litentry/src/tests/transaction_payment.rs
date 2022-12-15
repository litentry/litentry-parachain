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

use runtime_common::tests::transaction_payment;

use crate::{RuntimeCall, RuntimeOrigin, Runtime, TransactionByteFee};

#[test]
fn multiplier_can_grow_from_zero() {
	transaction_payment::multiplier_can_grow_from_zero::<Runtime>();
}

#[test]
fn transaction_payment_works() {
	transaction_payment::transaction_payment_works::<Runtime, TransactionByteFee, RuntimeOrigin, RuntimeCall>();
}
