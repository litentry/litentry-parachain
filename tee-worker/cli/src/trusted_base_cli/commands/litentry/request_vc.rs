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

use crate::{
	get_layer_two_nonce,
	trusted_cli::TrustedCli,
	trusted_command_utils::{get_identifiers, get_pair_from_str},
	trusted_operation::perform_trusted_operation,
	Cli, CliResult, CliResultOk,
};
use ita_stf::{trusted_call_result::RequestVCResult, Index, TrustedCall, TrustedCallSigning};
use itp_stf_primitives::types::KeyPair;
use itp_utils::hex::decode_hex;
use litentry_primitives::{
	aes_decrypt, AchainableAmount, AchainableAmountHolding, AchainableAmountToken,
	AchainableAmounts, AchainableBasic, AchainableBetweenPercents, AchainableClassOfYear,
	AchainableDate, AchainableDateInterval, AchainableDatePercent, AchainableParams,
	AchainableToken, Assertion, BoundedWeb3Network, ContestType, EVMTokenType,
	GenericDiscordRoleType, Identity, OneBlockCourseType, ParameterString, RequestAesKey,
	SoraQuizType, VIP3MembershipCardLevel, Web3Network, REQUEST_AES_KEY_LEN,
};
use sp_core::Pair;
use sp_core::H160;

// usage example (you can always use --help on subcommands to see more details)
//
// a8:
// ./bin/litentry-cli trusted -d request-vc \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 a8 litentry,litmus
//
// oneblock VC:
// ./bin/litentry-cli trusted -d request-vc \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 oneblock completion
//
// achainable VC:
// ./bin/litentry-cli trusted -d request-vc \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 achainable amount-holding a litentry 1 2014-05-01
//
// ./bin/litentry-cli trusted -d request-vc \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 vip3-membership-card gold
//
// ./bin/litentry-cli trusted -d request-vc \
//   did:litentry:substrate:0x52a6c52dc82940a36fefd1474cc0778517bb1a56b7bda0e308b6c19152dd7510 achainable amount-token test-name -c=bsc,ethereum 1 token-value

pub fn to_para_str(s: &str) -> ParameterString {
	ParameterString::truncate_from(s.as_bytes().to_vec())
}

pub fn to_chains(networks: &[String]) -> BoundedWeb3Network {
	let networks: Vec<Web3Network> = networks
		.iter()
		.map(|n| n.as_str().try_into().expect("cannot convert to Web3Network"))
		.collect();

	networks.try_into().unwrap()
}

#[derive(Parser)]
pub struct RequestVcCommand {
	/// did account to whom the vc will be issued
	did: String,
	/// subcommand to define the vc type requested
	#[clap(subcommand)]
	command: Command,
}

// see `assertion.rs`
#[derive(Subcommand, Debug)]
pub enum Command {
	A1,
	A2(A2Arg),
	A3(A3Arg),
	A4(HolderArg),
	A6,
	A7(HolderArg),
	A8(A8Arg),
	A10(HolderArg),
	A11(HolderArg),
	A13(A13Arg),
	A14,
	A20,
	BnbDomainHolding,
	#[clap(subcommand)]
	Oneblock(OneblockCommand),
	#[clap(subcommand)]
	Achainable(AchainableCommand),
	#[clap(subcommand)]
	GenericDiscordRole(GenericDiscordRoleCommand),
	#[clap(subcommand)]
	VIP3MembershipCard(VIP3MembershipCardLevelCommand),
	WeirdoGhostGangHolder,
	#[clap(subcommand)]
	EVMAmountHolding(EVMAmountHoldingCommand),
	CryptoSummary,
	LITStaking,
	BRC20AmountHolder,
	Dynamic(DynamicArg),
}

#[derive(Args, Debug)]
pub struct A2Arg {
	pub guild_id: String,
}

#[derive(Args, Debug)]
pub struct DynamicArg {
	//hex encoded smart contract id
	pub smart_contract_id: String,
}

#[derive(Args, Debug)]
pub struct A3Arg {
	pub guild_id: String,
	pub channel_id: String,
	pub role_id: String,
}

// used in A4/A7/A10/A11
#[derive(Args, Debug)]
pub struct HolderArg {
	pub minimum_amount: String,
}

#[derive(Args, Debug)]
pub struct A8Arg {
	#[clap(num_args = 0.., value_delimiter = ',')]
	pub networks: Vec<String>,
}

#[derive(Args, Debug)]
pub struct A13Arg {
	pub account: String,
}

#[derive(Subcommand, Debug)]
pub enum OneblockCommand {
	Completion,
	Outstanding,
	Participation,
}

#[derive(Subcommand, Debug)]
pub enum AchainableCommand {
	AmountHolding(AmountHoldingArg),
	AmountToken(AmountTokenArg),
	Amount(AmountArg),
	Amounts(AmountsArg),
	Basic(BasicArg),
	BetweenPercents(BetweenPercentsArg),
	ClassOfYear(ClassOfYearArg),
	DateInterval(DateIntervalArg),
	DatePercent(DatePercentArg),
	Date(DateArg),
	Token(TokenArg),
}

#[derive(Subcommand, Debug)]
pub enum GenericDiscordRoleCommand {
	#[clap(subcommand)]
	Contest(ContestCommand),
	#[clap(subcommand)]
	SoraQuiz(SoraQuizCommand),
}

#[derive(Subcommand, Debug)]
pub enum VIP3MembershipCardLevelCommand {
	Gold,
	Silver,
}

#[derive(Subcommand, Debug)]
pub enum ContestCommand {
	Legend,
	Popularity,
	Participant,
}

#[derive(Subcommand, Debug)]
pub enum SoraQuizCommand {
	Attendee,
	Master,
}

#[derive(Subcommand, Debug)]
pub enum EVMAmountHoldingCommand {
	Ton,
	Trx,
}

// I haven't found a good way to use common args for subcommands
#[derive(Args, Debug)]
pub struct AmountHoldingArg {
	pub name: String,
	pub chain: String,
	pub amount: String,
	pub date: String,
	pub token: Option<String>,
}

// positional args (to vec) + required arg + optional arg is a nightmare combination for clap parser,
// additionally, only the last positional argument, or second to last positional argument may be set to `.num_args()`
//
// the best bet is to use a flag explicitly, be sure to use euqal form for `chain`, e.g.:
// -- name -c=bsc,ethereum 10
// -- name -c=bsc,ethereum 10 token
#[derive(Args, Debug)]
pub struct AmountTokenArg {
	pub name: String,
	#[clap(
		short, long,
		num_args = 1..,
		required = true,
		value_delimiter = ',',
	)]
	pub chain: Vec<String>,
	pub amount: String,
	pub token: Option<String>,
}

#[derive(Args, Debug)]
pub struct AmountArg {
	pub name: String,
	pub chain: String,
	pub amount: String,
}

#[derive(Args, Debug)]
pub struct AmountsArg {
	pub name: String,
	pub chain: String,
	pub amount1: String,
	pub amount2: String,
}

#[derive(Args, Debug)]
pub struct BasicArg {
	pub name: String,
	pub chain: String,
}

#[derive(Args, Debug)]
pub struct BetweenPercentsArg {
	pub name: String,
	pub chain: String,
	pub greater_than_or_equal_to: String,
	pub less_than_or_equal_to: String,
}

#[derive(Args, Debug)]
pub struct ClassOfYearArg {
	pub name: String,
	pub chain: String,
}

#[derive(Args, Debug)]
pub struct DateIntervalArg {
	pub name: String,
	pub chain: String,
	pub start_date: String,
	pub end_date: String,
}

#[derive(Args, Debug)]
pub struct DatePercentArg {
	pub name: String,
	pub chain: String,
	pub token: String,
	pub date: String,
	pub percent: String,
}

#[derive(Args, Debug)]
pub struct DateArg {
	pub name: String,
	pub chain: String,
	pub date: String,
}

#[derive(Args, Debug)]
pub struct TokenArg {
	pub name: String,
	pub chain: String,
	pub token: String,
}

impl RequestVcCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_cli: &TrustedCli) -> CliResult {
		let alice = get_pair_from_str(trusted_cli, "//Alice", cli);
		let id: Identity = Identity::from_did(self.did.as_str()).unwrap();
		println!(">>>id: {:?}", id);

		let (mrenclave, shard) = get_identifiers(trusted_cli, cli);
		let nonce = get_layer_two_nonce!(alice, cli, trusted_cli);
		println!(">>>nonce: {}", nonce);

		println!(">>>command: {:#?}", self.command);

		let assertion = match &self.command {
			Command::A1 => Assertion::A1,
			Command::A2(arg) => Assertion::A2(to_para_str(&arg.guild_id)),
			Command::A3(arg) => Assertion::A3(
				to_para_str(&arg.guild_id),
				to_para_str(&arg.channel_id),
				to_para_str(&arg.role_id),
			),
			Command::A4(arg) => Assertion::A4(to_para_str(&arg.minimum_amount)),
			Command::A6 => Assertion::A6,
			Command::A7(arg) => Assertion::A7(to_para_str(&arg.minimum_amount)),
			Command::A8(arg) => Assertion::A8(to_chains(&arg.networks)),
			Command::A10(arg) => Assertion::A10(to_para_str(&arg.minimum_amount)),
			Command::A11(arg) => Assertion::A11(to_para_str(&arg.minimum_amount)),
			Command::A13(arg) => {
				let raw: [u8; 32] = decode_hex(&arg.account).unwrap().try_into().unwrap();
				Assertion::A13(raw.into())
			},
			Command::A14 => Assertion::A14,
			Command::A20 => Assertion::A20,
			Command::BnbDomainHolding => Assertion::BnbDomainHolding,
			Command::Oneblock(c) => match c {
				OneblockCommand::Completion =>
					Assertion::Oneblock(OneBlockCourseType::CourseCompletion),
				OneblockCommand::Outstanding =>
					Assertion::Oneblock(OneBlockCourseType::CourseOutstanding),
				OneblockCommand::Participation =>
					Assertion::Oneblock(OneBlockCourseType::CourseParticipation),
			},
			Command::Achainable(c) => match c {
				AchainableCommand::AmountHolding(arg) => Assertion::Achainable(
					AchainableParams::AmountHolding(AchainableAmountHolding {
						name: to_para_str(&arg.name),
						chain: arg
							.chain
							.as_str()
							.try_into()
							.expect("cannot convert to Web3Network"),
						amount: to_para_str(&arg.amount),
						date: to_para_str(&arg.date),
						token: arg.token.as_ref().map(|s| to_para_str(s)),
					}),
				),
				AchainableCommand::AmountToken(arg) =>
					Assertion::Achainable(AchainableParams::AmountToken(AchainableAmountToken {
						name: to_para_str(&arg.name),
						chain: to_chains(&arg.chain),
						amount: to_para_str(&arg.amount),
						token: arg.token.as_ref().map(|s| to_para_str(s)),
					})),
				AchainableCommand::Amount(arg) =>
					Assertion::Achainable(AchainableParams::Amount(AchainableAmount {
						name: to_para_str(&arg.name),
						chain: arg
							.chain
							.as_str()
							.try_into()
							.expect("cannot convert to Web3Network"),
						amount: to_para_str(&arg.amount),
					})),
				AchainableCommand::Amounts(arg) =>
					Assertion::Achainable(AchainableParams::Amounts(AchainableAmounts {
						name: to_para_str(&arg.name),
						chain: arg
							.chain
							.as_str()
							.try_into()
							.expect("cannot convert to Web3Network"),
						amount1: to_para_str(&arg.amount1),
						amount2: to_para_str(&arg.amount2),
					})),
				AchainableCommand::Basic(arg) =>
					Assertion::Achainable(AchainableParams::Basic(AchainableBasic {
						name: to_para_str(&arg.name),
						chain: arg
							.chain
							.as_str()
							.try_into()
							.expect("cannot convert to Web3Network"),
					})),
				AchainableCommand::BetweenPercents(arg) => Assertion::Achainable(
					AchainableParams::BetweenPercents(AchainableBetweenPercents {
						name: to_para_str(&arg.name),
						chain: arg
							.chain
							.as_str()
							.try_into()
							.expect("cannot convert to Web3Network"),
						greater_than_or_equal_to: to_para_str(&arg.greater_than_or_equal_to),
						less_than_or_equal_to: to_para_str(&arg.less_than_or_equal_to),
					}),
				),
				AchainableCommand::ClassOfYear(arg) =>
					Assertion::Achainable(AchainableParams::ClassOfYear(AchainableClassOfYear {
						name: to_para_str(&arg.name),
						chain: arg
							.chain
							.as_str()
							.try_into()
							.expect("cannot convert to Web3Network"),
					})),
				AchainableCommand::DateInterval(arg) =>
					Assertion::Achainable(AchainableParams::DateInterval(AchainableDateInterval {
						name: to_para_str(&arg.name),
						chain: arg
							.chain
							.as_str()
							.try_into()
							.expect("cannot convert to Web3Network"),
						start_date: to_para_str(&arg.start_date),
						end_date: to_para_str(&arg.end_date),
					})),
				AchainableCommand::DatePercent(arg) =>
					Assertion::Achainable(AchainableParams::DatePercent(AchainableDatePercent {
						name: to_para_str(&arg.name),
						chain: arg
							.chain
							.as_str()
							.try_into()
							.expect("cannot convert to Web3Network"),
						date: to_para_str(&arg.date),
						percent: to_para_str(&arg.percent),
						token: to_para_str(&arg.token),
					})),
				AchainableCommand::Date(arg) =>
					Assertion::Achainable(AchainableParams::Date(AchainableDate {
						name: to_para_str(&arg.name),
						chain: arg
							.chain
							.as_str()
							.try_into()
							.expect("cannot convert to Web3Network"),
						date: to_para_str(&arg.date),
					})),
				AchainableCommand::Token(arg) =>
					Assertion::Achainable(AchainableParams::Token(AchainableToken {
						name: to_para_str(&arg.name),
						chain: arg
							.chain
							.as_str()
							.try_into()
							.expect("cannot convert to Web3Network"),
						token: to_para_str(&arg.token),
					})),
			},
			Command::GenericDiscordRole(c) => match c {
				GenericDiscordRoleCommand::Contest(s) => match s {
					ContestCommand::Legend => Assertion::GenericDiscordRole(
						GenericDiscordRoleType::Contest(ContestType::Legend),
					),
					ContestCommand::Popularity => Assertion::GenericDiscordRole(
						GenericDiscordRoleType::Contest(ContestType::Popularity),
					),
					ContestCommand::Participant => Assertion::GenericDiscordRole(
						GenericDiscordRoleType::Contest(ContestType::Participant),
					),
				},
				GenericDiscordRoleCommand::SoraQuiz(s) => match s {
					SoraQuizCommand::Attendee => Assertion::GenericDiscordRole(
						GenericDiscordRoleType::SoraQuiz(SoraQuizType::Attendee),
					),
					SoraQuizCommand::Master => Assertion::GenericDiscordRole(
						GenericDiscordRoleType::SoraQuiz(SoraQuizType::Master),
					),
				},
			},
			Command::VIP3MembershipCard(arg) => match arg {
				VIP3MembershipCardLevelCommand::Gold =>
					Assertion::VIP3MembershipCard(VIP3MembershipCardLevel::Gold),
				VIP3MembershipCardLevelCommand::Silver =>
					Assertion::VIP3MembershipCard(VIP3MembershipCardLevel::Silver),
			},
			Command::WeirdoGhostGangHolder => Assertion::WeirdoGhostGangHolder,
			Command::EVMAmountHolding(c) => match c {
				EVMAmountHoldingCommand::Ton => Assertion::EVMAmountHolding(EVMTokenType::Ton),
				EVMAmountHoldingCommand::Trx => Assertion::EVMAmountHolding(EVMTokenType::Trx),
			},
			Command::CryptoSummary => Assertion::CryptoSummary,
			Command::LITStaking => Assertion::LITStaking,
			Command::BRC20AmountHolder => Assertion::BRC20AmountHolder,
			Command::Dynamic(arg) => {
				let decoded_id = hex::decode(&arg.smart_contract_id.clone()).unwrap();
				let id_bytes: [u8; 20] = decoded_id.try_into().unwrap();
				Assertion::Dynamic(H160::from(id_bytes))
			},
		};

		let key = Self::random_aes_key();

		let top = TrustedCall::request_vc(
			alice.public().into(),
			id,
			assertion,
			Some(key),
			Default::default(),
		)
		.sign(&KeyPair::Sr25519(Box::new(alice)), nonce, &mrenclave, &shard)
		.into_trusted_operation(trusted_cli.direct);

		match perform_trusted_operation::<RequestVCResult>(cli, trusted_cli, &top) {
			Ok(mut vc) => {
				let decrypted = aes_decrypt(&key, &mut vc.vc_payload).unwrap();
				let credential_str = String::from_utf8(decrypted).expect("Found invalid UTF-8");
				println!("----Generated VC-----");
				println!("{}", credential_str);
			},
			Err(e) => {
				println!("{:?}", e);
			},
		}
		Ok(CliResultOk::None)
	}

	fn random_aes_key() -> RequestAesKey {
		let random: Vec<u8> = (0..REQUEST_AES_KEY_LEN).map(|_| rand::random::<u8>()).collect();
		random[0..REQUEST_AES_KEY_LEN].try_into().unwrap()
	}
}
