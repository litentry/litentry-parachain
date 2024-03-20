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

use crate::{
	get_layer_two_nonce,
	trusted_cli::TrustedCli,
	trusted_command_utils::{get_identifiers, get_pair_from_str},
	trusted_operation::send_direct_batch_vc_request,
	Cli, CliResult, CliResultOk,
};
use codec::Decode;
use ita_stf::{
	trusted_call_result::{RequestVCResult, RequestVcResultOrError},
	Index, TrustedCall, VecAssertion,
};
use itp_stf_primitives::{traits::TrustedCallSigning, types::KeyPair};
use litentry_hex_utils::decode_hex;
use litentry_primitives::{
	aes_decrypt, AchainableAmount, AchainableAmountHolding, AchainableAmountToken,
	AchainableAmounts, AchainableBasic, AchainableBetweenPercents, AchainableClassOfYear,
	AchainableDate, AchainableDateInterval, AchainableDatePercent, AchainableMirror,
	AchainableParams, AchainableToken, Assertion, BnbDigitDomainType, BoundedWeb3Network,
	ContestType, EVMTokenType, GenericDiscordRoleType, Identity, OneBlockCourseType,
	ParameterString, PlatformUserType, RequestAesKey, SoraQuizType, VIP3MembershipCardLevel,
	Web3Network, Web3NftType, Web3TokenType, REQUEST_AES_KEY_LEN,
};
use sp_core::Pair;
use sp_runtime::BoundedVec;
use std::sync::mpsc::channel;

// usage example (you can always use --help on subcommands to see more details)
//
// a8 and OneBlock VC:
// ./bin/litentry-cli trusted -d request-batch-vc \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 a a8 litentry,litmus a one-block completion
//
// OneBlock VC:
// ./bin/litentry-cli trusted -d request-batch-vc \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 a one-block completion
//
// achainable VC:
// ./bin/litentry-cli trusted -d request-batch-vc \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 a achainable amount-holding a litentry 1 2014-05-01
//
// ./bin/litentry-cli trusted -d request-batch-vc \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 a vip3-membership-card gold
//
// ./bin/litentry-cli trusted -d request-batch-vc \
//   did:litentry:substrate:0x52a6c52dc82940a36fefd1474cc0778517bb1a56b7bda0e308b6c19152dd7510 a achainable amount-token test-name -c=bsc,ethereum 1 token-value

fn to_para_str(s: &str) -> ParameterString {
	ParameterString::truncate_from(s.as_bytes().to_vec())
}

fn to_chains(networks: String) -> BoundedWeb3Network {
	let networks: Vec<String> = networks.split(',').map(|s| s.to_string()).collect();
	let networks: Vec<Web3Network> = networks
		.iter()
		.map(|n| n.as_str().try_into().expect("cannot convert to Web3Network"))
		.collect();

	networks.try_into().unwrap()
}

#[derive(Parser)]
pub struct RequestBatchVcCommand {
	/// did account to whom the vc will be issued
	did: String,
	/// vector of assertions for vc requests, separated by -a
	params: Vec<String>,
}

impl RequestBatchVcCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_cli: &TrustedCli) -> CliResult {
		let mut assertions: Vec<Assertion> = vec![];
		println!("assertions1: {:?}", assertions);

		for (command, params) in self.command_parser() {
			match Self::create_assertion(command, params) {
				Ok(assertion) => {
					assertions.push(assertion.clone());
				},
				Err(err) => {
					println!("{:?}", err);
					return Ok(CliResultOk::None)
				},
			}
		}

		println!("assertions: {:?}", assertions);

		let assertions: VecAssertion = BoundedVec::try_from(assertions).unwrap();

		let alice = get_pair_from_str(trusted_cli, "//Alice", cli);
		let id: Identity = Identity::from_did(self.did.as_str()).unwrap();
		println!(">>>id: {:?}", id);

		let (mrenclave, shard) = get_identifiers(trusted_cli, cli);
		let nonce = get_layer_two_nonce!(alice, cli, trusted_cli);
		println!(">>>nonce: {}", nonce);
		let key = Self::random_aes_key();
		let top = TrustedCall::request_batch_vc(
			alice.public().into(),
			id,
			assertions,
			Some(key),
			Default::default(),
		)
		.sign(&KeyPair::Sr25519(Box::new(alice)), nonce, &mrenclave, &shard)
		.into_trusted_operation(trusted_cli.direct);

		let (sender, receiver) = channel::<Result<RequestVcResultOrError, String>>();

		send_direct_batch_vc_request(cli, trusted_cli, &top, key, sender);

		let mut len = 0u8;
		let mut cnt = 0u8;
		loop {
			match receiver.recv() {
				Ok(res) => match res {
					Ok(response) => {
						cnt += 1;
						if len < response.len {
							len = response.len;
						}
						if response.is_error {
							println!(
								"received one error: {:?}",
								String::from_utf8(response.payload)
							);
						} else {
							let mut vc =
								RequestVCResult::decode(&mut response.payload.as_slice()).unwrap();
							let decrypted = aes_decrypt(&key, &mut vc.vc_payload).unwrap();
							let credential_str =
								String::from_utf8(decrypted).expect("Found invalid UTF-8");
							println!("----Generated VC-----");
							println!("{}", credential_str);
						}
						if cnt >= len {
							break
						}
					},
					Err(e) => {
						println!("Response error: {:?}", e);
						break
					},
				},
				Err(e) => {
					println!("channel receiver error: {:?}", e);
					break
				},
			}
		}

		Ok(CliResultOk::None)
	}

	fn random_aes_key() -> RequestAesKey {
		let random: Vec<u8> = (0..REQUEST_AES_KEY_LEN).map(|_| rand::random::<u8>()).collect();
		random[0..REQUEST_AES_KEY_LEN].try_into().unwrap()
	}

	fn command_parser(&self) -> Vec<(String, Vec<String>)> {
		let mut commands: Vec<(String, Vec<String>)> = Vec::new();
		let mut current_command: Option<String> = None;
		let mut current_params: Vec<String> = Vec::new();

		for item in &self.params {
			if item == "a" {
				if let Some(command) = current_command.take() {
					commands.push((command, current_params.clone()));
					current_params.clear();
				}
			} else if current_command.is_none() {
				current_command = Some(item.to_string());
			} else {
				current_params.push(item.to_string());
			}
		}

		if let Some(command) = current_command.take() {
			commands.push((command, current_params));
		}

		println!("Commands: {:?}", commands);
		commands
	}

	fn create_assertion(command: String, params: Vec<String>) -> Result<Assertion, String> {
		println!("command: {:?}, param: {:?}", command, params);
		match command.to_lowercase().as_str() {
			"a1" => Ok(Assertion::A1),
			"a2" => Ok(Assertion::A2(to_para_str(
				&params.get(0).ok_or("A2: Missing parameter")?.clone(),
			))),
			"a3" => Ok(Assertion::A3(
				to_para_str(&params.get(0).ok_or("A3: Missing parameter")?.clone()),
				to_para_str(&params.get(1).ok_or("A3: Missing parameter")?.clone()),
				to_para_str(&params.get(2).ok_or("A3: Missing parameter")?.clone()),
			)),
			"a4" => Ok(Assertion::A4(to_para_str(
				&params.get(0).ok_or("A4: Missing parameter")?.clone(),
			))),
			"a6" => Ok(Assertion::A6),
			"a7" => Ok(Assertion::A7(to_para_str(
				&params.get(0).ok_or("A7: Missing parameter")?.clone(),
			))),
			"a8" =>
				Ok(Assertion::A8(to_chains(params.get(0).ok_or("A8: Missing parameter")?.clone()))),
			"a10" => Ok(Assertion::A10(to_para_str(
				&params.get(0).ok_or("A10: Missing parameter")?.clone(),
			))),
			"a11" => Ok(Assertion::A11(to_para_str(
				&params.get(0).ok_or("A11: Missing parameter")?.clone(),
			))),
			"a13" => {
				let raw: [u8; 32] =
					decode_hex(&params.get(0).ok_or("A13: Missing parameter")?.clone())
						.unwrap()
						.try_into()
						.unwrap();
				Ok(Assertion::A13(raw.into()))
			},
			"a14" => Ok(Assertion::A14),
			"achainable" => {
				let param0 = params.get(0).ok_or("Achainable: Missing parameter")?.clone();
				match param0.to_lowercase().as_str() {
					"amountholding" => Ok(Assertion::Achainable(AchainableParams::AmountHolding(
						AchainableAmountHolding {
							name: to_para_str(
								&params
									.get(1)
									.ok_or("Achainable AmountHolding: Missing parameter")?
									.clone(),
							),
							chain: to_chains(
								params
									.get(2)
									.ok_or("Achainable AmountHolding: Missing parameter")?
									.clone(),
							),
							amount: to_para_str(
								&params
									.get(3)
									.ok_or("Achainable AmountHolding: Missing parameter")?
									.clone(),
							),
							date: to_para_str(
								&params
									.get(4)
									.ok_or("Achainable AmountHolding: Missing parameter")?
									.clone(),
							),
							token: params.get(5).map(|v| to_para_str(v)),
						},
					))),
					"amounttoken" => Ok(Assertion::Achainable(AchainableParams::AmountToken(
						AchainableAmountToken {
							name: to_para_str(
								&params
									.get(1)
									.ok_or("Achainable AmountToken: Missing parameter")?
									.clone(),
							),
							chain: to_chains(
								params
									.get(2)
									.ok_or("Achainable AmountToken: Missing parameter")?
									.clone(),
							),
							amount: to_para_str(
								&params
									.get(3)
									.ok_or("Achainable AmountToken: Missing parameter")?
									.clone(),
							),
							token: params.get(4).map(|v| to_para_str(v)),
						},
					))),
					"amount" =>
						Ok(Assertion::Achainable(AchainableParams::Amount(AchainableAmount {
							name: to_para_str(
								&params
									.get(1)
									.ok_or("Achainable Amount: Missing parameter")?
									.clone(),
							),
							chain: to_chains(
								params
									.get(2)
									.ok_or("Achainable Amount: Missing parameter")?
									.clone(),
							),
							amount: to_para_str(
								&params
									.get(3)
									.ok_or("Achainable Amount: Missing parameter")?
									.clone(),
							),
						}))),
					"amounts" =>
						Ok(Assertion::Achainable(AchainableParams::Amounts(AchainableAmounts {
							name: to_para_str(
								&params
									.get(1)
									.ok_or("Achainable amounts: Missing parameter")?
									.clone(),
							),
							chain: to_chains(
								params
									.get(2)
									.ok_or("Achainable amounts: Missing parameter")?
									.clone(),
							),
							amount1: to_para_str(
								&params
									.get(3)
									.ok_or("Achainable amounts: Missing parameter")?
									.clone(),
							),
							amount2: to_para_str(
								&params
									.get(4)
									.ok_or("Achainable amounts: Missing parameter")?
									.clone(),
							),
						}))),
					"basic" =>
						Ok(Assertion::Achainable(AchainableParams::Basic(AchainableBasic {
							name: to_para_str(
								&params
									.get(1)
									.ok_or("Achainable Basic: Missing parameter")?
									.clone(),
							),
							chain: to_chains(
								params.get(2).ok_or("Achainable Basic: Missing parameter")?.clone(),
							),
						}))),
					"betweenpercents" => Ok(Assertion::Achainable(
						AchainableParams::BetweenPercents(AchainableBetweenPercents {
							name: to_para_str(
								&params
									.get(1)
									.ok_or("Achainable BetweenPercents: Missing parameter")?
									.clone(),
							),
							chain: to_chains(
								params
									.get(2)
									.ok_or("Achainable BetweenPercents: Missing parameter")?
									.clone(),
							),
							greater_than_or_equal_to: to_para_str(
								&params
									.get(3)
									.ok_or("Achainable BetweenPercents: Missing parameter")?
									.clone(),
							),
							less_than_or_equal_to: to_para_str(
								&params
									.get(4)
									.ok_or("Achainable BetweenPercents: Missing parameter")?
									.clone(),
							),
						}),
					)),
					"classofyear" => Ok(Assertion::Achainable(AchainableParams::ClassOfYear(
						AchainableClassOfYear {
							name: to_para_str(
								&params
									.get(1)
									.ok_or("Achainable ClassOfYear: Missing parameter")?
									.clone(),
							),
							chain: to_chains(
								params
									.get(2)
									.ok_or("Achainable ClassOfYear: Missing parameter")?
									.clone(),
							),
						},
					))),
					"dateinterval" => Ok(Assertion::Achainable(AchainableParams::DateInterval(
						AchainableDateInterval {
							name: to_para_str(
								&params
									.get(1)
									.ok_or("Achainable DateInterval: Missing parameter")?
									.clone(),
							),
							chain: to_chains(
								params
									.get(2)
									.ok_or("Achainable DateInterval: Missing parameter")?
									.clone(),
							),
							start_date: to_para_str(
								&params
									.get(3)
									.ok_or("Achainable DateInterval: Missing parameter")?
									.clone(),
							),
							end_date: to_para_str(
								&params
									.get(4)
									.ok_or("Achainable DateInterval: Missing parameter")?
									.clone(),
							),
						},
					))),
					"datepercent" => Ok(Assertion::Achainable(AchainableParams::DatePercent(
						AchainableDatePercent {
							name: to_para_str(
								&params
									.get(1)
									.ok_or("Achainable DatePercent: Missing parameter")?
									.clone(),
							),
							chain: to_chains(
								params
									.get(2)
									.ok_or("Achainable DatePercent: Missing parameter")?
									.clone(),
							),
							date: to_para_str(
								&params
									.get(3)
									.ok_or("Achainable DatePercent: Missing parameter")?
									.clone(),
							),
							percent: to_para_str(
								&params
									.get(4)
									.ok_or("Achainable DatePercent: Missing parameter")?
									.clone(),
							),
							token: to_para_str(
								&params
									.get(5)
									.ok_or("Achainable DatePercent: Missing parameter")?
									.clone(),
							),
						},
					))),
					"date" => Ok(Assertion::Achainable(AchainableParams::Date(AchainableDate {
						name: to_para_str(
							&params.get(1).ok_or("Achainable Date: Missing parameter")?.clone(),
						),
						chain: to_chains(
							params.get(2).ok_or("Achainable Date: Missing parameter")?.clone(),
						),
						date: to_para_str(
							&params.get(3).ok_or("Achainable Date: Missing parameter")?.clone(),
						),
					}))),
					"token" =>
						Ok(Assertion::Achainable(AchainableParams::Token(AchainableToken {
							name: to_para_str(
								&params
									.get(1)
									.ok_or_else(|| {
										"Achainable Token: Missing parameter".to_string()
									})?
									.clone(),
							),
							chain: to_chains(
								params
									.get(2)
									.ok_or_else(|| {
										"Achainable Token: Missing parameter".to_string()
									})?
									.clone(),
							),
							token: to_para_str(
								&params
									.get(3)
									.ok_or_else(|| {
										"Achainable Token: Missing parameter".to_string()
									})?
									.clone(),
							),
						}))),
					"mirror" =>
						Ok(Assertion::Achainable(AchainableParams::Mirror(AchainableMirror {
							name: to_para_str(
								&params
									.get(1)
									.ok_or_else(|| {
										"Achainable Token: Missing parameter".to_string()
									})?
									.clone(),
							),
							chain: to_chains(
								params
									.get(2)
									.ok_or_else(|| {
										"Achainable Token: Missing parameter".to_string()
									})?
									.clone(),
							),
							post_quantity: params.get(3).map(|v| to_para_str(v)),
						}))),
					_ => Err("Achainable: Wrong parameter".to_string()),
				}
			},
			"a20" => Ok(Assertion::A20),
			"oneblock" => {
				let param0 = params.get(0).ok_or("OneBlock: Missing parameter")?.clone();
				match param0.to_lowercase().as_str() {
					"completion" => Ok(Assertion::OneBlock(OneBlockCourseType::CourseCompletion)),
					"outstanding" => Ok(Assertion::OneBlock(OneBlockCourseType::CourseOutstanding)),
					"participation" =>
						Ok(Assertion::OneBlock(OneBlockCourseType::CourseParticipation)),
					_ => Err("OneBlock: Wrong parameter".to_string()),
				}
			},
			"genericdiscordrole" => {
				let param0 = params.get(0).ok_or("GenericDiscordRole: Missing parameter")?.clone();
				match param0.to_lowercase().as_str() {
					"contest" => {
						let param1 =
							params.get(1).ok_or("GenericDiscordRole: Missing parameter")?.clone();
						match param1.to_lowercase().as_str() {
							"legend" => Ok(Assertion::GenericDiscordRole(
								GenericDiscordRoleType::Contest(ContestType::Legend),
							)),
							"popularity" => Ok(Assertion::GenericDiscordRole(
								GenericDiscordRoleType::Contest(ContestType::Popularity),
							)),
							"participant" => Ok(Assertion::GenericDiscordRole(
								GenericDiscordRoleType::Contest(ContestType::Participant),
							)),
							_ => Err("GenericDiscordRole: Wrong parameter".to_string()),
						}
					},
					"soraquiz" => {
						let role_type2 =
							params.get(1).ok_or("GenericDiscordRole: Missing parameter")?.clone();
						match role_type2.to_lowercase().as_str() {
							"attendee" => Ok(Assertion::GenericDiscordRole(
								GenericDiscordRoleType::SoraQuiz(SoraQuizType::Attendee),
							)),
							"master" => Ok(Assertion::GenericDiscordRole(
								GenericDiscordRoleType::SoraQuiz(SoraQuizType::Master),
							)),
							_ => Err("GenericDiscordRole: Wrong parameter".to_string()),
						}
					},
					_ => Err("GenericDiscordRole: Wrong parameter".to_string()),
				}
			},
			"bnbdomainholding" => Ok(Assertion::BnbDomainHolding),
			"bnbdigitdomainclub" => {
				let param0 = params.get(0).ok_or("BnbDigitDomainClub: Missing parameter")?.clone();
				match param0.to_lowercase().as_str() {
					"bnb999clubmember" =>
						Ok(Assertion::BnbDigitDomainClub(BnbDigitDomainType::Bnb999ClubMember)),
					"bnb10kclubmember" =>
						Ok(Assertion::BnbDigitDomainClub(BnbDigitDomainType::Bnb10kClubMember)),
					_ => Err("BnbDigitDomainClub: Wrong parameter".to_string()),
				}
			},
			"vip3membershipcard" => {
				let param0 = params.get(0).ok_or("VIP3MembershipCard: Missing parameter")?.clone();
				match param0.to_lowercase().as_str() {
					"gold" => Ok(Assertion::VIP3MembershipCard(VIP3MembershipCardLevel::Gold)),
					"silver" => Ok(Assertion::VIP3MembershipCard(VIP3MembershipCardLevel::Silver)),
					_ => Err("VIP3MembershipCard: Wrong parameter".to_string()),
				}
			},
			"weirdoghostganholder" => Ok(Assertion::WeirdoGhostGangHolder),
			"litstaking" => Ok(Assertion::LITStaking),
			"evmamountholding" => {
				let param0 = params.get(0).ok_or("EVMAmountHolding: Missing parameter")?.clone();
				match param0.to_lowercase().as_str() {
					"ton" => Ok(Assertion::EVMAmountHolding(EVMTokenType::Ton)),
					"trx" => Ok(Assertion::EVMAmountHolding(EVMTokenType::Trx)),
					_ => Err("EVMAmountHolding: Wrong parameter".to_string()),
				}
			},
			"brc20amountholder" => Ok(Assertion::BRC20AmountHolder),
			"cryptosummary" => Ok(Assertion::CryptoSummary),
			"tokenholdingamount" => {
				let param0 = params.get(0).ok_or("TokenHoldingAmount: Missing parameter")?.clone();
				match param0.to_lowercase().as_str() {
					"bnb" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Bnb)),
					"eth" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Eth)),
					"spaceid" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::SpaceId)),
					"lit" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Lit)),
					"wbtc" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Wbtc)),
					"usdc" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Usdc)),
					"usdt" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Usdt)),
					"crv" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Crv)),
					"matic" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Matic)),
					"dydx" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Dydx)),
					"amp" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Amp)),
					"cvx" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Cvx)),
					"tusd" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Tusd)),
					"usdd" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Usdd)),
					"gusd" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Gusd)),
					"link" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Link)),
					"grt" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Grt)),
					"comp" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Comp)),
					"people" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::People)),
					"gtc" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Gtc)),
					"ton" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Ton)),
					"trx" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Trx)),
					"nfp" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Nfp)),
					"sol" => Ok(Assertion::TokenHoldingAmount(Web3TokenType::Sol)),
					_ => Err("TokenHoldingAmount: Wrong parameter".to_string()),
				}
			},
			"platformuser" => {
				let param0 = params.get(0).ok_or("PlatformUser: Missing parameter")?.clone();
				match param0.to_lowercase().as_str() {
					"karatdaouser" => Ok(Assertion::PlatformUser(PlatformUserType::KaratDaoUser)),
					_ => Err("PlatformUser: Wrong parameter".to_string()),
				}
			},
			"nftholder" => {
				let param0 = params.get(0).ok_or("NFTHolder: Missing parameter")?.clone();
				match param0.to_lowercase().as_str() {
					"weirdoghostgang" => Ok(Assertion::NftHolder(Web3NftType::WeirdoGhostGang)),
					"club3sbt" => Ok(Assertion::NftHolder(Web3NftType::Club3Sbt)),
					_ => Err("NFTHolder: Wrong parameter".to_string()),
				}
			},

			_ => Err("Wrong parameter".to_string()),
		}
	}
}
