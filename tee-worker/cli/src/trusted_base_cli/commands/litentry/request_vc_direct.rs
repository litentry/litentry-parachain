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
	trusted_base_cli::commands::litentry::request_vc::*,
	trusted_cli::TrustedCli,
	trusted_command_utils::{get_identifiers, get_pair_from_str},
	trusted_operation::perform_direct_operation,
	Cli, CliResult, CliResultOk,
};
use ita_stf::{trusted_call_result::RequestVCResult, Index, TrustedCall, TrustedCallSigning};
use itp_stf_primitives::types::KeyPair;
use itp_utils::hex::decode_hex;
use litentry_primitives::{
	aes_decrypt, AchainableAmount, AchainableAmountHolding, AchainableAmountToken,
	AchainableAmounts, AchainableBasic, AchainableBetweenPercents, AchainableClassOfYear,
	AchainableDate, AchainableDateInterval, AchainableDatePercent, AchainableParams,
	AchainableToken, Assertion, ContestType, EVMTokenType, GenericDiscordRoleType, Identity,
	OneBlockCourseType, RequestAesKey, SoraQuizType, VIP3MembershipCardLevel, Web3Network,
};
use sp_core::Pair;

// usage example (you can always use --help on subcommands to see more details)
//
// a8:
// ./bin/litentry-cli trusted -d request-vc-direct \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 a8 litentry,litmus
//
// oneblock VC:
// ./bin/litentry-cli trusted -d request-vc-direct \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 oneblock completion
//
// achainable VC:
// ./bin/litentry-cli trusted -d request-vc-direct \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 achainable amount-holding a litentry 1 2014-05-01
//
// vip3 VC:
// ./bin/litentry-cli trusted -d request-vc-direct \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 vip3-membership-card gold

#[derive(Parser)]
pub struct RequestVcDirectCommand {
	/// did account to whom the vc will be issued
	did: String,
	/// subcommand to define the vc type requested
	#[clap(subcommand)]
	command: Command,
}

impl RequestVcDirectCommand {
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
		};

		let mut key: RequestAesKey = RequestAesKey::default();
		hex::decode_to_slice(
			"22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12",
			&mut key,
		)
		.expect("decoding shielding_key failed");

		let top = TrustedCall::request_vc(
			alice.public().into(),
			id,
			assertion,
			Some(key),
			Default::default(),
		)
		.sign(&KeyPair::Sr25519(Box::new(alice)), nonce, &mrenclave, &shard)
		.into_trusted_operation(trusted_cli.direct);

		// This should contain the AES Key for AESRequest
		match perform_direct_operation::<RequestVCResult>(cli, trusted_cli, &top, key) {
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
}
