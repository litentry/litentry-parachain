// Copyright 2020-2023 Trust Computing GmbH.
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

use crate::*;
use lc_credentials::nodereal::crypto_summary::CryptoSummaryCredentialUpdate;
use super::CryptoSummary;

pub fn build(req: &AssertionBuildRequest) -> Result<Credential> {
	let identities = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.filter(|(newtwork_type, _)| newtwork_type.is_evm())
		.flat_map(|(newtwork_type, addresses)| {
			addresses.into_iter().map(move |address| (newtwork_type, address))
		})
		.collect::<Vec<(Web3Network, String)>>();

	let mut credential_unsigned = Credential::new(&req.who, &req.shard).map_err(|e| {
		error!("Generate unsigned credential failed {:?}", e);
		Error::RequestVCFailed(Assertion::CryptoSummary, e.into_error_detail()) 
	})?;
	
	let summary = CryptoSummary::new().logic(addresses).map_err(|e| {
		Error::RequestVCFailed(Assertion::CryptoSummary, e)
	})?;
	credential_unsigned.update_crypto_summary_credential(summary.is_empty());

	Ok(credential_unsigned)
}
