// Copyright 2020-2024 Trust Computing GmbH.
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
use lc_common::abort_strategy::{loop_with_abort_strategy, AbortStrategy, LoopControls};
use lc_credentials::IssuerRuntimeVersion;
use lc_data_providers::{
	achainable::{AchainableClient, AchainableLabelQuery, LabelQueryReq, LabelQueryReqParams},
	DataProviderConfig, Error as DataProviderError,
};

const VC_A14_SUBJECT_DESCRIPTION: &str =
	"The user has participated in any Polkadot on-chain governance events";
const VC_A14_SUBJECT_TYPE: &str = "Polkadot Governance Participation Proof";

pub fn build(
	req: &AssertionBuildRequest,
	data_provider_config: &DataProviderConfig,
) -> Result<Credential> {
	debug!("Assertion A14 build, who: {:?}", account_id_to_string(&req.who));

	// achainable expects polkadot addresses (those start with 1...)
	let mut polkadot_addresses = vec![];
	for identity in &req.identities {
		if let Identity::Substrate(address) = identity.0 {
			let address = ss58_address_of(address.as_ref(), "polkadot")
				.map_err(|_| Error::RequestVCFailed(Assertion::A14, ErrorDetail::ParseError))?;
			polkadot_addresses.push(address);
		}
	}
	let mut value = false;
	let mut client = AchainableClient::new(data_provider_config);

	loop_with_abort_strategy::<fn(&_) -> bool, String, DataProviderError>(
		polkadot_addresses,
		|address| {
			let data = LabelQueryReq {
				params: LabelQueryReqParams { address: address.clone() },
				include_metadata: false,
				include_widgets: false,
			};
			let result = client.query_label("a719e99c-1f9b-432e-8f1d-cb3de0f14dde", &data)?;
			if result {
				value = result;
				Ok(LoopControls::Break)
			} else {
				Ok(LoopControls::Continue)
			}
		},
		AbortStrategy::ContinueUntilEnd::<fn(&_) -> bool>,
	)
	.map_err(|errors| {
		Error::RequestVCFailed(Assertion::A14, errors[0].clone().into_error_detail())
	})?;

	let runtime_version = IssuerRuntimeVersion {
		parachain: req.parachain_runtime_version,
		sidechain: req.sidechain_runtime_version,
	};

	match Credential::new(&req.who, &req.shard, &runtime_version) {
		Ok(mut credential_unsigned) => {
			// add subject info
			credential_unsigned.add_subject_info(VC_A14_SUBJECT_DESCRIPTION, VC_A14_SUBJECT_TYPE);

			// add assertion
			credential_unsigned.add_assertion_a14(value);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::A14, e.into_error_detail()))
		},
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use itp_stf_primitives::types::ShardIdentifier;
	use lc_credentials::assertion_logic::{AssertionLogic, Op};
	use lc_data_providers::DataProviderConfig;
	use lc_mock_server::run;
	use litentry_primitives::{Assertion, Identity, IdentityNetworkTuple};
	use log;
	use std::{vec, vec::Vec};

	fn init() -> DataProviderConfig {
		let _ = env_logger::builder().is_test(true).try_init();
		let url = run(0).unwrap();
		let mut data_provider_conifg = DataProviderConfig::new().unwrap();

		data_provider_conifg.set_achainable_url(url).unwrap();
		data_provider_conifg
	}

	#[test]
	fn build_a14_works() {
		let data_provider_config = init();

		let identities: Vec<IdentityNetworkTuple> =
			vec![(Identity::Substrate(AccountId::from([0; 32]).into()), vec![])];

		let req: AssertionBuildRequest = AssertionBuildRequest {
			shard: ShardIdentifier::default(),
			signer: AccountId::from([0; 32]),
			who: AccountId::from([0; 32]).into(),
			assertion: Assertion::A14,
			identities,
			top_hash: Default::default(),
			parachain_block_number: 0u32,
			sidechain_block_number: 0u32,
			parachain_runtime_version: 0u32,
			sidechain_runtime_version: 0u32,
			maybe_key: None,
			should_create_id_graph: false,
			req_ext_hash: Default::default(),
		};

		match build(&req, &data_provider_config) {
			Ok(credential) => {
				log::info!("build A14 done");
				assert_eq!(
					*(credential.credential_subject.assertions.first().unwrap()),
					AssertionLogic::Item {
						src: String::from("$total_governance_action"),
						op: Op::GreaterThan,
						dst: "0".into()
					}
				);
				assert_eq!(*(credential.credential_subject.values.first().unwrap()), true);
			},
			Err(e) => {
				panic!("build A14 failed with error {:?}", e);
			},
		}
	}
}
