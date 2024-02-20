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
// along with Litentry. If not, see <https://www.gnu.org/licenses/>.

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use lc_credentials_v2::{token_holding_amount::TokenHoldingAmountAssertionUpdate, Credential};
use lc_service::web3_token::token_balance::get_token_balance;
use lc_stf_task_sender::AssertionBuildRequest;
use litentry_primitives::{Web3Network, Web3TokenType};
use log::debug;

use crate::*;

pub fn build(
	req: &AssertionBuildRequest,
	token_type: Web3TokenType,
	data_provider_config: &DataProviderConfig,
) -> Result<Credential> {
	debug!("token holding amount: {:?}", token_type);

	let identities: Vec<(Web3Network, Vec<String>)> = transpose_identity(&req.identities);
	let addresses = identities
		.into_iter()
		.flat_map(|(network_type, addresses)| {
			addresses.into_iter().map(move |address| (network_type, address))
		})
		.collect::<Vec<(Web3Network, String)>>();

	let result =
		get_token_balance(token_type.clone(), addresses, data_provider_config).map_err(|e| {
			Error::RequestVCFailed(
				Assertion::TokenHoldingAmount(token_type.clone()),
				ErrorDetail::DataProviderError(ErrorString::truncate_from(
					format!("{e:?}").as_bytes().to_vec(),
				)),
			)
		});

	match result {
		Ok(value) => match Credential::new(&req.who, &req.shard) {
			Ok(mut credential_unsigned) => {
				credential_unsigned.update_token_holding_amount_assertion(token_type, value);
				Ok(credential_unsigned)
			},
			Err(e) => {
				error!("Generate unsigned credential failed {:?}", e);
				Err(Error::RequestVCFailed(
					Assertion::TokenHoldingAmount(token_type),
					e.into_error_detail(),
				))
			},
		},
		Err(e) => Err(e),
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use itp_stf_primitives::types::ShardIdentifier;
	use itp_types::AccountId;
	use lc_common::web3_token::{TokenAddress, TokenName};
	use lc_credentials_v2::assertion_logic::{AssertionLogic, Op};
	use lc_mock_server::run;
	use litentry_hex_utils::decode_hex;
	use litentry_primitives::{Identity, IdentityNetworkTuple};

	fn crate_assertion_build_request(
		token_type: Web3TokenType,
		identities: Vec<IdentityNetworkTuple>,
	) -> AssertionBuildRequest {
		AssertionBuildRequest {
			shard: ShardIdentifier::default(),
			signer: AccountId::from([0; 32]),
			who: AccountId::from([0; 32]).into(),
			assertion: Assertion::TokenHoldingAmount(token_type),
			identities,
			top_hash: Default::default(),
			parachain_block_number: 0u32,
			sidechain_block_number: 0u32,
			maybe_key: None,
			should_create_id_graph: false,
			req_ext_hash: Default::default(),
		}
	}

	fn create_token_assertion_logic(token_type: Web3TokenType) -> Box<AssertionLogic> {
		Box::new(AssertionLogic::Item {
			src: "$token".into(),
			op: Op::Equal,
			dst: token_type.get_token_name().into(),
		})
	}

	fn create_bsc_assertion_logic() -> Box<AssertionLogic> {
		Box::new(AssertionLogic::Or {
			items: vec![
				Box::new(AssertionLogic::And {
					items: vec![Box::new(AssertionLogic::Item {
						src: "$network".into(),
						op: Op::Equal,
						dst: "bsc".into(),
					})],
				}),
				Box::new(AssertionLogic::And {
					items: vec![
						Box::new(AssertionLogic::Item {
							src: "$network".into(),
							op: Op::Equal,
							dst: "ethereum".into(),
						}),
						Box::new(AssertionLogic::Item {
							src: "$address".into(),
							op: Op::Equal,
							dst: Web3TokenType::Bnb
								.get_token_address(Web3Network::Ethereum)
								.unwrap()
								.into(),
						}),
					],
				}),
			],
		})
	}

	fn create_eth_assertion_logic() -> Box<AssertionLogic> {
		Box::new(AssertionLogic::Or {
			items: vec![
				Box::new(AssertionLogic::And {
					items: vec![
						Box::new(AssertionLogic::Item {
							src: "$network".into(),
							op: Op::Equal,
							dst: "bsc".into(),
						}),
						Box::new(AssertionLogic::Item {
							src: "$address".into(),
							op: Op::Equal,
							dst: Web3TokenType::Eth
								.get_token_address(Web3Network::Bsc)
								.unwrap()
								.into(),
						}),
					],
				}),
				Box::new(AssertionLogic::And {
					items: vec![Box::new(AssertionLogic::Item {
						src: "$network".into(),
						op: Op::Equal,
						dst: "ethereum".into(),
					})],
				}),
			],
		})
	}

	fn create_evm_assertion_logic(token_type: Web3TokenType) -> Box<AssertionLogic> {
		Box::new(AssertionLogic::Or {
			items: vec![
				Box::new(AssertionLogic::And {
					items: vec![
						Box::new(AssertionLogic::Item {
							src: "$network".into(),
							op: Op::Equal,
							dst: "bsc".into(),
						}),
						Box::new(AssertionLogic::Item {
							src: "$address".into(),
							op: Op::Equal,
							dst: token_type.get_token_address(Web3Network::Bsc).unwrap().into(),
						}),
					],
				}),
				Box::new(AssertionLogic::And {
					items: vec![
						Box::new(AssertionLogic::Item {
							src: "$network".into(),
							op: Op::Equal,
							dst: "ethereum".into(),
						}),
						Box::new(AssertionLogic::Item {
							src: "$address".into(),
							op: Op::Equal,
							dst: token_type
								.get_token_address(Web3Network::Ethereum)
								.unwrap()
								.into(),
						}),
					],
				}),
			],
		})
	}

	fn create_ethereum_assertion_logic(token_type: Web3TokenType) -> Box<AssertionLogic> {
		Box::new(AssertionLogic::Or {
			items: vec![Box::new(AssertionLogic::And {
				items: vec![
					Box::new(AssertionLogic::Item {
						src: "$network".into(),
						op: Op::Equal,
						dst: "ethereum".into(),
					}),
					Box::new(AssertionLogic::Item {
						src: "$address".into(),
						op: Op::Equal,
						dst: token_type.get_token_address(Web3Network::Ethereum).unwrap().into(),
					}),
				],
			})],
		})
	}

	fn create_lit_assertion_logic() -> Box<AssertionLogic> {
		Box::new(AssertionLogic::Or {
			items: vec![
				Box::new(AssertionLogic::And {
					items: vec![
						Box::new(AssertionLogic::Item {
							src: "$network".into(),
							op: Op::Equal,
							dst: "bsc".into(),
						}),
						Box::new(AssertionLogic::Item {
							src: "$address".into(),
							op: Op::Equal,
							dst: Web3TokenType::Lit
								.get_token_address(Web3Network::Bsc)
								.unwrap()
								.into(),
						}),
					],
				}),
				Box::new(AssertionLogic::And {
					items: vec![
						Box::new(AssertionLogic::Item {
							src: "$network".into(),
							op: Op::Equal,
							dst: "ethereum".into(),
						}),
						Box::new(AssertionLogic::Item {
							src: "$address".into(),
							op: Op::Equal,
							dst: Web3TokenType::Lit
								.get_token_address(Web3Network::Ethereum)
								.unwrap()
								.into(),
						}),
					],
				}),
				Box::new(AssertionLogic::And {
					items: vec![Box::new(AssertionLogic::Item {
						src: "$network".into(),
						op: Op::Equal,
						dst: "litentry".into(),
					})],
				}),
				Box::new(AssertionLogic::And {
					items: vec![Box::new(AssertionLogic::Item {
						src: "$network".into(),
						op: Op::Equal,
						dst: "litmus".into(),
					})],
				}),
			],
		})
	}

	fn init() -> DataProviderConfig {
		let _ = env_logger::builder().is_test(true).try_init();
		let url = run(0).unwrap();

		let mut data_provider_config = DataProviderConfig::new();

		data_provider_config.set_nodereal_api_key("d416f55179dbd0e45b1a8ed030e3".into());
		data_provider_config.set_nodereal_api_chain_network_url(url.clone() + "/nodereal_jsonrpc/");
		data_provider_config.set_achainable_url(url.clone());
		data_provider_config
	}

	#[test]
	fn build_bnb_holding_amount_works() {
		let data_provider_config = init();
		let address = decode_hex("0x45cdb67696802b9d01ed156b883269dbdb9c6239".as_bytes().to_vec())
			.unwrap()
			.as_slice()
			.try_into()
			.unwrap();
		let identities: Vec<IdentityNetworkTuple> =
			vec![(Identity::Evm(address), vec![Web3Network::Bsc, Web3Network::Ethereum])];

		let req = crate_assertion_build_request(Web3TokenType::Bnb, identities);

		match build(&req, Web3TokenType::Bnb, &data_provider_config) {
			Ok(credential) => {
				log::info!("build bnb TokenHoldingAmount done");
				assert_eq!(
					*(credential.credential_subject.assertions.first().unwrap()),
					AssertionLogic::And {
						items: vec![
							create_token_assertion_logic(Web3TokenType::Bnb),
							create_bsc_assertion_logic(),
							Box::new(AssertionLogic::Item {
								src: "$holding_amount".into(),
								op: Op::GreaterEq,
								dst: "50".into()
							}),
							Box::new(AssertionLogic::Item {
								src: "$holding_amount".into(),
								op: Op::LessThan,
								dst: "100".into()
							})
						]
					}
				);
				assert_eq!(*(credential.credential_subject.values.first().unwrap()), true);
			},
			Err(e) => {
				panic!("build bnb TokenHoldingAmount failed with error {:?}", e);
			},
		}
	}

	#[test]
	fn build_eth_holding_amount_works() {
		let data_provider_config = init();
		let identities: Vec<IdentityNetworkTuple> =
			vec![(Identity::Evm([0; 20].into()), vec![Web3Network::Ethereum])];

		let req = crate_assertion_build_request(Web3TokenType::Eth, identities);

		match build(&req, Web3TokenType::Eth, &data_provider_config) {
			Ok(credential) => {
				log::info!("build eth TokenHoldingAmount done");
				assert_eq!(
					*(credential.credential_subject.assertions.first().unwrap()),
					AssertionLogic::And {
						items: vec![
							create_token_assertion_logic(Web3TokenType::Eth),
							create_eth_assertion_logic(),
							Box::new(AssertionLogic::Item {
								src: "$holding_amount".into(),
								op: Op::GreaterEq,
								dst: "1".into()
							}),
							Box::new(AssertionLogic::Item {
								src: "$holding_amount".into(),
								op: Op::LessThan,
								dst: "50".into()
							})
						]
					}
				);
				assert_eq!(*(credential.credential_subject.values.first().unwrap()), true);
			},
			Err(e) => {
				panic!("build eth TokenHoldingAmount failed with error {:?}", e);
			},
		}
	}

	#[test]
	fn build_evm_holding_amount_works() {
		let data_provider_config = init();
		let address = decode_hex("0x75438d34c9125839c8b08d21b7f3167281659e7c".as_bytes().to_vec())
			.unwrap()
			.as_slice()
			.try_into()
			.unwrap();
		let identities: Vec<IdentityNetworkTuple> =
			vec![(Identity::Evm(address), vec![Web3Network::Bsc, Web3Network::Ethereum])];

		let req = crate_assertion_build_request(Web3TokenType::SpaceId, identities);

		match build(&req, Web3TokenType::SpaceId, &data_provider_config) {
			Ok(credential) => {
				log::info!("build evm TokenHoldingAmount done");
				assert_eq!(
					*(credential.credential_subject.assertions.first().unwrap()),
					AssertionLogic::And {
						items: vec![
							create_token_assertion_logic(Web3TokenType::SpaceId),
							create_evm_assertion_logic(Web3TokenType::SpaceId),
							Box::new(AssertionLogic::Item {
								src: "$holding_amount".into(),
								op: Op::GreaterEq,
								dst: "800".into()
							}),
							Box::new(AssertionLogic::Item {
								src: "$holding_amount".into(),
								op: Op::LessThan,
								dst: "1200".into()
							})
						]
					}
				);
				assert_eq!(*(credential.credential_subject.values.first().unwrap()), true);
			},
			Err(e) => {
				panic!("build evm TokenHoldingAmount failed with error {:?}", e);
			},
		}
	}

	#[test]
	fn build_ethereum_holding_amount_works() {
		let data_provider_config = init();
		let address = decode_hex("0x75438d34c9125839c8b08d21b7f3167281659e7c".as_bytes().to_vec())
			.unwrap()
			.as_slice()
			.try_into()
			.unwrap();
		let identities: Vec<IdentityNetworkTuple> =
			vec![(Identity::Evm(address), vec![Web3Network::Ethereum])];

		let req = crate_assertion_build_request(Web3TokenType::Amp, identities);

		match build(&req, Web3TokenType::Amp, &data_provider_config) {
			Ok(credential) => {
				log::info!("build ethereum TokenHoldingAmount done");
				assert_eq!(
					*(credential.credential_subject.assertions.first().unwrap()),
					AssertionLogic::And {
						items: vec![
							create_token_assertion_logic(Web3TokenType::Amp),
							create_ethereum_assertion_logic(Web3TokenType::Amp),
							Box::new(AssertionLogic::Item {
								src: "$holding_amount".into(),
								op: Op::GreaterEq,
								dst: "200".into()
							}),
							Box::new(AssertionLogic::Item {
								src: "$holding_amount".into(),
								op: Op::LessThan,
								dst: "500".into()
							})
						]
					}
				);
				assert_eq!(*(credential.credential_subject.values.first().unwrap()), true);
			},
			Err(e) => {
				panic!("build ethereum TokenHoldingAmount failed with error {:?}", e);
			},
		}
	}

	#[test]
	fn build_lit_holding_amount_works() {
		let data_provider_config = init();
		let address = decode_hex("0xba359c153ad11aa17c3122b05a4db8b46bb3191b".as_bytes().to_vec())
			.unwrap()
			.as_slice()
			.try_into()
			.unwrap();
		let identities: Vec<IdentityNetworkTuple> =
			vec![(Identity::Evm(address), vec![Web3Network::Ethereum, Web3Network::Litentry])];

		let req = crate_assertion_build_request(Web3TokenType::Lit, identities);

		match build(&req, Web3TokenType::Lit, &data_provider_config) {
			Ok(credential) => {
				log::info!("build lit TokenHoldingAmount done");
				assert_eq!(
					*(credential.credential_subject.assertions.first().unwrap()),
					AssertionLogic::And {
						items: vec![
							create_token_assertion_logic(Web3TokenType::Lit),
							create_lit_assertion_logic(),
							Box::new(AssertionLogic::Item {
								src: "$holding_amount".into(),
								op: Op::GreaterEq,
								dst: "1600".into()
							}),
							Box::new(AssertionLogic::Item {
								src: "$holding_amount".into(),
								op: Op::LessThan,
								dst: "3000".into()
							})
						]
					}
				);
				assert_eq!(*(credential.credential_subject.values.first().unwrap()), true);
			},
			Err(e) => {
				panic!("build lit TokenHoldingAmount failed with error {:?}", e);
			},
		}
	}
}
