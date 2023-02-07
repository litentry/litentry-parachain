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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::Result;
use itp_stf_primitives::types::ShardIdentifier;
use itp_types::AccountId;
use lc_credentials::Credential;
use lc_data_providers::graphql::{GraphQLClient, VerifiedCredentialsTotalTxs};
use litentry_primitives::{Assertion, Identity, ParentchainBlockNumber};
use log::*;
use parachain_core_primitives::VCMPError;
use std::{str::from_utf8, string::ToString, vec, vec::Vec};

pub fn build(
	identities: Vec<Identity>,
	shard: &ShardIdentifier,
	who: &AccountId,
	bn: ParentchainBlockNumber,
) -> Result<Credential> {
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

	debug!("total_transactions: {}", total_txs);

	let min: u64;
	let max: u64;

	match total_txs {
		0 | 1 => {
			min = 0;
			max = 1;
		},
		2..=10 => {
			min = 1;
			max = 10;
		},
		11..=100 => {
			min = 10;
			max = 100;
		},
		101..=1000 => {
			min = 100;
			max = 1000
		},
		1001..=10000 => {
			min = 1000;
			max = 10000;
		},
		10001..=u64::MAX => {
			min = 10000;
			max = u64::MAX;
		},
	}

	match Credential::generate_unsigned_credential(&Assertion::A8, who, &shard.clone(), bn) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.add_assertion_a8(min, max);
			credential_unsigned.credential_subject.set_value(true);
			return Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
		},
	}

	Err(VCMPError::Assertion8Failed)
}
