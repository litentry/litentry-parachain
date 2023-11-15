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
use ita_stf::{trusted_call_result::RequestVCResult, Index, TrustedCall, TrustedOperation};
use itp_stf_primitives::types::KeyPair;
use itp_utils::hex::decode_hex;
use lc_credentials::Credential;
use litentry_primitives::{
	aes_decrypt, AchainableAmount, AchainableAmountHolding, AchainableAmountToken,
	AchainableAmounts, AchainableBasic, AchainableBetweenPercents, AchainableClassOfYear,
	AchainableDate, AchainableDateInterval, AchainableDatePercent, AchainableParams,
	AchainableToken, Assertion, GenericDiscordRoleType, Identity, OneBlockCourseType,
	ParameterString, RequestAesKey, SoraQuizType, Web3Network, REQUEST_AES_KEY_LEN,
};
use sp_core::Pair;

// usage example (you can always use --help on subcommands to see more details)
//
// a8:
// ./bin/litentry-cli trusted -m <mrencalve> -d request-vc \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 a8 litentry,litmus
//
// oneblock VC:
// ./bin/litentry-cli trusted -m <mrencalve> -d request-vc \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 oneblock completion
//
// achainable VC:
// ./bin/litentry-cli trusted -m <mrencalve> -d request-vc \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 achainable amount-holding a litentry 1 2014-05-01

fn to_para_str(s: &str) -> ParameterString {
	ParameterString::truncate_from(s.as_bytes().to_vec())
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
#[derive(Subcommand)]
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
	#[clap(subcommand)]
	Oneblock(OneblockCommand),
	#[clap(subcommand)]
	Achainable(AchainableCommand),
	#[clap(subcommand)]
	SoraQuiz(SoraQuizCommand),
	#[clap(subcommand)]
	GenericDiscordRole(GenericDiscordRoleCommand),
}

#[derive(Args)]
pub struct A2Arg {
	pub guild_id: String,
}

#[derive(Args)]
pub struct A3Arg {
	pub guild_id: String,
	pub channel_id: String,
	pub role_id: String,
}

// used in A4/A7/A10/A11
#[derive(Args)]
pub struct HolderArg {
	pub minimum_amount: String,
}

#[derive(Args)]
pub struct A8Arg {
	#[clap(num_args = 0.., value_delimiter = ',')]
	pub networks: Vec<String>,
}

#[derive(Args)]
pub struct A13Arg {
	pub account: String,
}

#[derive(Subcommand)]
pub enum OneblockCommand {
	Completion,
	Outstanding,
	Participation,
}

#[derive(Subcommand)]
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

#[derive(Subcommand)]
pub enum SoraQuizCommand {
	Attendee,
	Master,
}

#[derive(Subcommand)]
pub enum GenericDiscordRoleCommand {
	Legend,
	Popularity,
	Participant,
}

// I haven't found a good way to use common args for subcommands
#[derive(Args)]
pub struct AmountHoldingArg {
	pub name: String,
	pub chain: String,
	pub amount: String,
	pub date: String,
	pub token: Option<String>,
}

#[derive(Args)]
pub struct AmountTokenArg {
	pub name: String,
	pub chain: String,
	pub amount: String,
	pub token: Option<String>,
}

#[derive(Args)]
pub struct AmountArg {
	pub name: String,
	pub chain: String,
	pub amount: String,
}

#[derive(Args)]
pub struct AmountsArg {
	pub name: String,
	pub chain: String,
	pub amount1: String,
	pub amount2: String,
}

#[derive(Args)]
pub struct BasicArg {
	pub name: String,
	pub chain: String,
}

#[derive(Args)]
pub struct BetweenPercentsArg {
	pub name: String,
	pub chain: String,
	pub greater_than_or_equal_to: String,
	pub less_than_or_equal_to: String,
}

#[derive(Args)]
pub struct ClassOfYearArg {
	pub name: String,
	pub chain: String,
}

#[derive(Args)]
pub struct DateIntervalArg {
	pub name: String,
	pub chain: String,
	pub start_date: String,
	pub end_date: String,
}

#[derive(Args)]
pub struct DatePercentArg {
	pub name: String,
	pub chain: String,
	pub token: String,
	pub date: String,
	pub percent: String,
}

#[derive(Args)]
pub struct DateArg {
	pub name: String,
	pub chain: String,
	pub date: String,
}

#[derive(Args)]
pub struct TokenArg {
	pub name: String,
	pub chain: String,
	pub token: String,
}

impl RequestVcCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_cli: &TrustedCli) -> CliResult {
		let alice = get_pair_from_str(trusted_cli, "//Alice", cli);
		let id: Identity = Identity::from_did(self.did.as_str()).unwrap();

		let (mrenclave, shard) = get_identifiers(trusted_cli, cli);
		let nonce = get_layer_two_nonce!(alice, cli, trusted_cli);

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
			Command::A8(arg) => {
				let networks: Vec<Web3Network> = arg
					.networks
					.iter()
					.map(|n| n.as_str().try_into().expect("cannot convert to Web3Network"))
					.collect();
				Assertion::A8(networks.try_into().unwrap())
			},
			Command::A10(arg) => Assertion::A10(to_para_str(&arg.minimum_amount)),
			Command::A11(arg) => Assertion::A11(to_para_str(&arg.minimum_amount)),
			Command::A13(arg) => {
				let raw: [u8; 32] = decode_hex(&arg.account).unwrap().try_into().unwrap();
				Assertion::A13(raw.into())
			},
			Command::A14 => Assertion::A14,
			Command::A20 => Assertion::A20,
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
						chain: arg
							.chain
							.as_str()
							.try_into()
							.expect("cannot convert to Web3Network"),
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
			Command::SoraQuiz(c) => match c {
				SoraQuizCommand::Attendee => Assertion::SoraQuiz(SoraQuizType::Attendee),
				SoraQuizCommand::Master => Assertion::SoraQuiz(SoraQuizType::Master),
			},
			Command::GenericDiscordRole(c) => match c {
				GenericDiscordRoleCommand::Legend =>
					Assertion::GenericDiscordRole(GenericDiscordRoleType::Legend),
				GenericDiscordRoleCommand::Popularity =>
					Assertion::GenericDiscordRole(GenericDiscordRoleType::Popularity),
				GenericDiscordRoleCommand::Participant =>
					Assertion::GenericDiscordRole(GenericDiscordRoleType::Participant),
			},
		};

		let key = Self::random_aes_key();

		let top: TrustedOperation = TrustedCall::request_vc(
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
				let credential: Credential = serde_json::from_slice(&decrypted).unwrap();
				println!("----Generated VC-----");
				println!("{:?}", credential);
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
