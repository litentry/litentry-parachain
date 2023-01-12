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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::Result;
use lc_data_providers::graphql::{GraphQLClient, VerifiedCredentialsTotalTxs};
use litentry_primitives::Identity;
use log::debug;
use std::{str::from_utf8, string::ToString, vec, vec::Vec};

// total transactions
pub fn build(identities: Vec<Identity>) -> Result<()> {
	let mut client = GraphQLClient::new();
	let mut total_txs: u64 = 0;

	for identity in identities {
		let query = match identity {
			Identity::Substrate { network, address } =>
				from_utf8(address.as_ref()).map_or(None, |addr| {
					Some(VerifiedCredentialsTotalTxs::new(
						vec![addr.to_string()],
						vec![network.into()],
					))
				}),
			Identity::Evm { network, address } =>
				from_utf8(address.as_ref()).map_or(None, |addr| {
					Some(VerifiedCredentialsTotalTxs::new(
						vec![addr.to_string()],
						vec![network.into()],
					))
				}),
			_ => {
				debug!("ignore identity: {:?}", identity);
				None
			},
		};
		if let Some(query) = query {
			if let Ok(result) = client.query_total_transactions(query) {
				total_txs += result.iter().map(|v| v.total_transactions).sum::<u64>();
			}
		}
	}
	//TODO generate vc
	debug!("total_transactions: {}", total_txs);
	Ok(())
}
