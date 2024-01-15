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

use core::result;

use lc_credentials::nodereal::nft_holder::weirdo_ghost_gang_holder::WeirdoGhostGangHolderAssertionUpdate;
use lc_data_providers::nodereal_jsonrpc::{
	GetTokenBalance721Param, NftApiList, NoderealChain, NoderealJsonrpcClient,
};

use crate::*;
use lc_data_providers::{DataProviderConfig, Error as DataProviderError};

const NFT_TOKEN_ADDRESS: &str = "0x9401518f4EBBA857BAA879D9f76E1Cc8b31ed197";

fn check_has_nft(
	client: &mut NoderealJsonrpcClient,
	address: &str,
) -> result::Result<bool, DataProviderError> {
	let param = GetTokenBalance721Param {
		token_address: NFT_TOKEN_ADDRESS.into(),
		account_address: address.into(),
		block_number: "latest".into(),
	};

	match client.get_token_balance_721(&param) {
		Ok(res) => {
			debug!("Get token balance 721 response: {:?}", res);
			Ok(res > 0)
		},
		Err(e) => {
			error!("Error get token balance 721 by param: {:?}, {:?}", param, e);
			Err(e)
		},
	}
}

pub fn build(
	req: &AssertionBuildRequest,
	data_provider_config: &DataProviderConfig,
) -> Result<Credential> {
	debug!("WeirdoGhostGang holder");

	let mut has_nft = false;
	let mut client = NoderealJsonrpcClient::new(NoderealChain::Eth, data_provider_config);

	let identities: Vec<(Web3Network, Vec<String>)> = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.filter(|(newtwork_type, _)| *newtwork_type == Web3Network::Ethereum)
		.flat_map(|(_, addresses)| addresses)
		.collect::<Vec<String>>();

	let mut errors: Vec<DataProviderError> = Vec::new();
	for address in addresses {
		match check_has_nft(&mut client, address.as_str()) {
			Ok(res) => {
				has_nft = res;
				break
			},
			Err(err) => {
				errors.push(err);
			},
		}
	}

	if !has_nft && !errors.is_empty() {
		return Err(Error::RequestVCFailed(
			Assertion::WeirdoGhostGangHolder,
			ErrorDetail::DataProviderError(ErrorString::truncate_from(
				errors
					.into_iter()
					.map(|e| format!("{e:?}"))
					.collect::<Vec<String>>()
					.join(", ")
					.as_bytes()
					.to_vec(),
			)),
		))
	}

	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			credential_unsigned.update_weirdo_ghost_gang_holder_assertion(has_nft);
			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::WeirdoGhostGangHolder, e.into_error_detail()))
		},
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use itp_stf_primitives::types::ShardIdentifier;
	use lc_credentials::assertion_logic::{AssertionLogic, Op};
	use lc_mock_server::run;

	fn init() -> DataProviderConfig {
		let _ = env_logger::builder().is_test(true).try_init();
		let url = run(0).unwrap() + "/nodereal_jsonrpc/";
		let mut config = DataProviderConfig::new();

		config.set_nodereal_api_key("d416f55179dbd0e45b1a8ed030e3".to_string());
		config.set_nodereal_api_chain_network_url(url);
		config
	}

	#[test]
	fn build_weirdo_ghost_gang_holder_works() {
		let config = init();
		let identities: Vec<IdentityNetworkTuple> =
			vec![(Identity::Evm([0; 20].into()), vec![Web3Network::Ethereum])];

		let req: AssertionBuildRequest = AssertionBuildRequest {
			shard: ShardIdentifier::default(),
			signer: AccountId::from([0; 32]),
			who: AccountId::from([0; 32]).into(),
			assertion: Assertion::WeirdoGhostGangHolder,
			identities,
			top_hash: Default::default(),
			parachain_block_number: 0u32,
			sidechain_block_number: 0u32,
			maybe_key: None,
			req_ext_hash: Default::default(),
		};

		match build(&req, &config) {
			Ok(credential) => {
				log::info!("build WeirdoGhostGang holder done");
				assert_eq!(
					*(credential.credential_subject.assertions.first().unwrap()),
					AssertionLogic::Item {
						src: String::from("$is_weirdo_ghost_gang_holder"),
						op: Op::Equal,
						dst: String::from("true")
					}
				);
				assert_eq!(*(credential.credential_subject.values.first().unwrap()), true);
			},
			Err(e) => {
				panic!("build WeirdoGhostGang holder failed with error {:?}", e);
			},
		}
	}
}
