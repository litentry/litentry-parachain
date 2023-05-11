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

use itp_types::extrinsics::ParentchainUncheckedExtrinsicWithStatus;
use itp_types::TimestampCallFn;
use codec::{Decode, Compact};
use log::debug;
use crate::error::Result;

/// To obtain the parent chain block's timestamp, we can decode the first extrinsic of each block because the first transaction of each block is a `set extrinsic` of `pallet_timestamp`.
pub fn decode(encoded_xt_opaque: &mut &[u8]) -> Result<u64> {
	debug!(">>> Timestamp decode timestamp set ...");

	let mut timestamp = 0_u64;

	let call: Result<ParentchainUncheckedExtrinsicWithStatus<([u8; 2], Compact<u64>)>> = ParentchainUncheckedExtrinsicWithStatus::<TimestampCallFn>::decode(encoded_xt_opaque).map_err(|e| e.into());
	if let Ok(ParentchainUncheckedExtrinsicWithStatus { xt, status: _ }) = call {
		let (_, now) = &xt.function;
		timestamp = u64::from(*now);
	}
	debug!(">>> Timestamp from parent block: {:?}", timestamp);

	Ok(timestamp)
}