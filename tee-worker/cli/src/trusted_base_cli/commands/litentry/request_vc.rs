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
	trusted_operation::{perform_trusted_operation, send_direct_vc_request},
	Cli, CliResult, CliResultOk,
};
use clap::Parser;
use codec::Decode;
use ita_stf::{trusted_call_result::RequestVCResult, Index, TrustedCall};
use itp_stf_primitives::{traits::TrustedCallSigning, types::KeyPair};
use litentry_hex_utils::decode_hex;
use litentry_primitives::{
	aes_decrypt, AchainableAmount, AchainableAmountHolding, AchainableAmountToken,
	AchainableAmounts, AchainableBasic, AchainableBetweenPercents, AchainableClassOfYear,
	AchainableDate, AchainableDateInterval, AchainableDatePercent, AchainableParams,
	AchainableToken, Assertion, BnbDigitDomainType, BoundedWeb3Network, ContestType, EVMTokenType,
	GenericDiscordRoleType, Identity, OneBlockCourseType, ParameterString, PlatformUserType,
	RequestAesKey, SoraQuizType, VIP3MembershipCardLevel, Web3Network, Web3NftType, Web3TokenType,
	REQUEST_AES_KEY_LEN,
};
use sp_core::{Pair, H160};

// usage example below
//
// Basically, the assertion subcommand needs to be quoted to signal the value group for certain assertion.
// You can specifiy `-a "<value>"` multiple times to pass in a batched vc request
//
// Printing `--help` give some information but clap doesn't know anything about the value specifiction.
// However, if you put mismatched parameters for subcommands you'll get an error hint during the parsing.
// For example:
// -a "a2 p1 p2" will give you:
//   error: unexpected argument 'p2'
//   Usage: placeholder a2 <GUILD_ID>
// as a2 expects A2Arg which only has one field `guild_id`
//
// single a8:
// ./bin/litentry-cli trusted -d request-vc \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 -a "a8 litentry,litmus"
//
// single OneBlock:
// ./bin/litentry-cli trusted -d request-vc \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 -a "one-block completion"
//
// batched a1 + a2 + a3:
// ./bin/litentry-cli trusted -d request-vc \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 \
//   -a "a1" -a "a2 gid" -a "a3 gid cid rid"
//
// batched achainable + vip3:
// ./bin/litentry-cli trusted -d request-vc \
//   did:litentry:substrate:0x8eaf04151687736326c9fea17e25fc5287613693c912909cb226aa4794f26a48 \
//   -a "achainable amount-holding a -c=litentry 1 2014-05-01" \
//   -a "vip3-membership-card gold"

pub fn to_para_str<T>(s: T) -> ParameterString
where
	T: AsRef<[u8]>,
{
	ParameterString::truncate_from(s.as_ref().to_vec())
}

pub fn to_chains<T, U>(networks: U) -> BoundedWeb3Network
where
	T: AsRef<str>,
	U: IntoIterator<Item = T>,
{
	let networks: Vec<Web3Network> =
		networks.into_iter().map(|n| n.as_ref().try_into().unwrap()).collect();

	networks.try_into().unwrap()
}

#[derive(Debug, Parser)]
pub struct RequestVcCommand {
	// did account to whom the vc will be issued
	did: String,
	// mode for the request-vc
	#[clap(short, long, default_value_t = false)]
	stf: bool,
	// the assertion itself, can be specified more than once
	// the value will be passed into the parser as a whole string
	#[clap(short, long, num_args = 1..)]
	assertion: Vec<String>,
}

#[derive(Debug, Parser)]
// the wrapper to the underlying `subcommand` type
pub struct AssertionCommand {
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
	#[clap(subcommand)]
	Achainable(AchainableCommand),
	A20,
	#[clap(subcommand)]
	OneBlock(OneblockCommand),
	#[clap(subcommand)]
	GenericDiscordRole(GenericDiscordRoleCommand),
	BnbDomainHolding,
	#[clap(subcommand)]
	BnbDigitalDomainClub(BnbDigitalDomainClubCommand),
	#[clap(subcommand)]
	VIP3MembershipCard(VIP3MembershipCardLevelCommand),
	WeirdoGhostGangHolder,
	LITStaking,
	#[clap(subcommand)]
	EVMAmountHolding(EVMAmountHoldingCommand),
	BRC20AmountHolder,
	CryptoSummary,
	#[clap(subcommand)]
	TokenHoldingAmount(TokenHoldingAmountCommand),
	#[clap(subcommand)]
	PlatformUser(PlatformUserCommand),
	#[clap(subcommand)]
	NftHolder(NftHolderCommand),
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
pub enum OneblockCommand {
	Completion,
	Outstanding,
	Participation,
}

#[derive(Subcommand, Debug)]
pub enum GenericDiscordRoleCommand {
	#[clap(subcommand)]
	Contest(ContestCommand),
	#[clap(subcommand)]
	SoraQuiz(SoraQuizCommand),
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
pub enum BnbDigitalDomainClubCommand {
	Bnb999ClubMember,
	Bnb10kClubMember,
}

#[derive(Subcommand, Debug)]
pub enum VIP3MembershipCardLevelCommand {
	Gold,
	Silver,
}

#[derive(Subcommand, Debug)]
pub enum EVMAmountHoldingCommand {
	Ton,
	Trx,
}

#[derive(Subcommand, Debug)]
pub enum TokenHoldingAmountCommand {
	Bnb,
	Eth,
	SpaceId,
	Lit,
	Wbtc,
	Usdc,
	Usdt,
	Crv,
	Matic,
	Dydx,
	Amp,
	Cvx,
	Tusd,
	Usdd,
	Gusd,
	Link,
	Grt,
	Comp,
	People,
	Gtc,
	Ton,
	Trx,
	Nfp,
	Sol,
	Mcrt,
	Btc,
}

#[derive(Subcommand, Debug)]
pub enum PlatformUserCommand {
	KaratDaoUser,
	MagicCraftStakingUser,
}

#[derive(Subcommand, Debug)]
pub enum NftHolderCommand {
	WeirdoGhostGang,
	Club3Sbt,
}

// positional args (to vec) + required arg + optional arg is a nightmare combination for clap parser,
// additionally, only the last positional argument, or second to last positional argument may be set to `.num_args()`
//
// the best bet is to use a flag explicitly, be sure to use equal form for `chain`, e.g.:
// -- name -c=bsc,ethereum 10
// -- name -c=bsc,ethereum 10 token
macro_rules! AchainableCommandArgs {
	($type_name:ident, {$( $field_name:ident : $field_type:ty , )* }) => {
		#[derive(Args, Debug)]
		pub struct $type_name {
			pub name: String,
			#[clap(
				short, long,
				num_args = 1..,
				required = true,
				value_delimiter = ',',
			)]
			pub chain: Vec<String>,
			$( pub $field_name: $field_type ),*
		}
	};
}

AchainableCommandArgs!(AmountHoldingArg, {
	amount: String,
	date: String,
	token: Option<String>,
});

AchainableCommandArgs!(AmountTokenArg, {
	amount: String,
	token: Option<String>,
});

AchainableCommandArgs!(AmountArg, {
	amount: String,
});

AchainableCommandArgs!(AmountsArg, {
	amount1: String,
	amount2: String,
});

AchainableCommandArgs!(BasicArg, {});

AchainableCommandArgs!(BetweenPercentsArg, {
	greater_than_or_equal_to: String,
	less_than_or_equal_to: String,
});

AchainableCommandArgs!(ClassOfYearArg, {});

AchainableCommandArgs!(DateIntervalArg, {
	start_date: String,
	end_date: String,
});

AchainableCommandArgs!(DatePercentArg, {
	token: String,
	date: String,
	percent: String,
});

AchainableCommandArgs!(DateArg, {
	date: String,
});

AchainableCommandArgs!(TokenArg, {
	token: String,
});

impl RequestVcCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_cli: &TrustedCli) -> CliResult {
		let alice = get_pair_from_str(trusted_cli, "//Alice", cli);
		let identity = Identity::from_did(self.did.as_str()).unwrap();
		println!(">>> identity: {:?}", identity);

		let (mrenclave, shard) = get_identifiers(trusted_cli, cli);
		let mut nonce = get_layer_two_nonce!(alice, cli, trusted_cli);
		println!(">>> nonce: {}", nonce);

		let assertions: Vec<Assertion> = self
			.assertion
			.iter()
			.map(|a| {
				let mut s = vec!["placeholder"];
				s.extend(a.as_str().split(' '));
				AssertionCommand::parse_from(s).command.to_assertion()
			})
			.collect();

		println!(">>> assertions: {:?}", assertions);

		let key = Self::random_aes_key();

		if self.stf {
			assertions.into_iter().for_each(|a| {
				let top = TrustedCall::request_vc(
					alice.public().into(),
					identity.clone(),
					a,
					Some(key),
					Default::default(),
				)
				.sign(&KeyPair::Sr25519(Box::new(alice.clone())), nonce, &mrenclave, &shard)
				.into_trusted_operation(trusted_cli.direct);
				match perform_trusted_operation::<RequestVCResult>(cli, trusted_cli, &top) {
					Ok(mut vc) => {
						let decrypted = aes_decrypt(&key, &mut vc.vc_payload).unwrap();
						let credential_str =
							String::from_utf8(decrypted).expect("Found invalid UTF-8");
						println!("----Generated VC-----");
						println!("{}", credential_str);
					},
					Err(e) => {
						println!("{:?}", e);
					},
				}
				nonce += 1;
			});
		} else {
			let top = TrustedCall::request_batch_vc(
				alice.public().into(),
				identity,
				assertions.try_into().unwrap(),
				Some(key),
				Default::default(),
			)
			.sign(&KeyPair::Sr25519(Box::new(alice)), nonce, &mrenclave, &shard)
			.into_trusted_operation(trusted_cli.direct);

			match send_direct_vc_request(cli, trusted_cli, &top, key) {
				Ok(result) =>
					for res in result {
						if res.is_error {
							println!("received one error: {:?}", String::from_utf8(res.payload));
						} else {
							let mut vc =
								RequestVCResult::decode(&mut res.payload.as_slice()).unwrap();
							let decrypted = aes_decrypt(&key, &mut vc.vc_payload).unwrap();
							let credential_str =
								String::from_utf8(decrypted).expect("Found invalid UTF-8");
							println!("----Generated VC-----");
							println!("{}", credential_str);
						}
					},
				Err(e) => {
					println!("{:?}", e);
				},
			}
		};

		Ok(CliResultOk::None)
	}

	fn random_aes_key() -> RequestAesKey {
		let random: Vec<u8> = (0..REQUEST_AES_KEY_LEN).map(|_| rand::random::<u8>()).collect();
		random[0..REQUEST_AES_KEY_LEN].try_into().unwrap()
	}
}

impl Command {
	// helper fn to convert a `Command` to `Assertion`
	pub fn to_assertion(&self) -> Assertion {
		use Assertion::*;
		match self {
			Command::A1 => A1,
			Command::A2(arg) => A2(to_para_str(&arg.guild_id)),
			Command::A3(arg) => A3(
				to_para_str(&arg.guild_id),
				to_para_str(&arg.channel_id),
				to_para_str(&arg.role_id),
			),
			Command::A4(arg) => A4(to_para_str(&arg.minimum_amount)),
			Command::A6 => A6,
			Command::A7(arg) => A7(to_para_str(&arg.minimum_amount)),
			Command::A8(arg) => A8(to_chains(&arg.networks)),
			Command::A10(arg) => A10(to_para_str(&arg.minimum_amount)),
			Command::A11(arg) => A11(to_para_str(&arg.minimum_amount)),
			Command::A13(arg) => {
				let raw: [u8; 32] = decode_hex(&arg.account).unwrap().try_into().unwrap();
				A13(raw.into())
			},
			Command::A14 => A14,
			Command::Achainable(c) => match c {
				AchainableCommand::AmountHolding(arg) =>
					Achainable(AchainableParams::AmountHolding(AchainableAmountHolding {
						name: to_para_str(&arg.name),
						chain: to_chains(&arg.chain),
						amount: to_para_str(&arg.amount),
						date: to_para_str(&arg.date),
						token: arg.token.as_ref().map(to_para_str),
					})),
				AchainableCommand::AmountToken(arg) =>
					Achainable(AchainableParams::AmountToken(AchainableAmountToken {
						name: to_para_str(&arg.name),
						chain: to_chains(&arg.chain),
						amount: to_para_str(&arg.amount),
						token: arg.token.as_ref().map(to_para_str),
					})),
				AchainableCommand::Amount(arg) =>
					Achainable(AchainableParams::Amount(AchainableAmount {
						name: to_para_str(&arg.name),
						chain: to_chains(&arg.chain),
						amount: to_para_str(&arg.amount),
					})),
				AchainableCommand::Amounts(arg) =>
					Achainable(AchainableParams::Amounts(AchainableAmounts {
						name: to_para_str(&arg.name),
						chain: to_chains(&arg.chain),
						amount1: to_para_str(&arg.amount1),
						amount2: to_para_str(&arg.amount2),
					})),
				AchainableCommand::Basic(arg) =>
					Achainable(AchainableParams::Basic(AchainableBasic {
						name: to_para_str(&arg.name),
						chain: to_chains(&arg.chain),
					})),
				AchainableCommand::BetweenPercents(arg) =>
					Achainable(AchainableParams::BetweenPercents(AchainableBetweenPercents {
						name: to_para_str(&arg.name),
						chain: to_chains(&arg.chain),
						greater_than_or_equal_to: to_para_str(&arg.greater_than_or_equal_to),
						less_than_or_equal_to: to_para_str(&arg.less_than_or_equal_to),
					})),
				AchainableCommand::ClassOfYear(arg) =>
					Achainable(AchainableParams::ClassOfYear(AchainableClassOfYear {
						name: to_para_str(&arg.name),
						chain: to_chains(&arg.chain),
					})),
				AchainableCommand::DateInterval(arg) =>
					Achainable(AchainableParams::DateInterval(AchainableDateInterval {
						name: to_para_str(&arg.name),
						chain: to_chains(&arg.chain),
						start_date: to_para_str(&arg.start_date),
						end_date: to_para_str(&arg.end_date),
					})),
				AchainableCommand::DatePercent(arg) =>
					Achainable(AchainableParams::DatePercent(AchainableDatePercent {
						name: to_para_str(&arg.name),
						chain: to_chains(&arg.chain),
						date: to_para_str(&arg.date),
						percent: to_para_str(&arg.percent),
						token: to_para_str(&arg.token),
					})),
				AchainableCommand::Date(arg) =>
					Achainable(AchainableParams::Date(AchainableDate {
						name: to_para_str(&arg.name),
						chain: to_chains(&arg.chain),
						date: to_para_str(&arg.date),
					})),
				AchainableCommand::Token(arg) =>
					Achainable(AchainableParams::Token(AchainableToken {
						name: to_para_str(&arg.name),
						chain: to_chains(&arg.chain),
						token: to_para_str(&arg.token),
					})),
			},
			Command::A20 => A20,
			Command::OneBlock(c) => match c {
				OneblockCommand::Completion => OneBlock(OneBlockCourseType::CourseCompletion),
				OneblockCommand::Outstanding => OneBlock(OneBlockCourseType::CourseOutstanding),
				OneblockCommand::Participation => OneBlock(OneBlockCourseType::CourseParticipation),
			},
			Command::GenericDiscordRole(c) => match c {
				GenericDiscordRoleCommand::Contest(s) => match s {
					ContestCommand::Legend =>
						GenericDiscordRole(GenericDiscordRoleType::Contest(ContestType::Legend)),
					ContestCommand::Popularity =>
						GenericDiscordRole(GenericDiscordRoleType::Contest(ContestType::Popularity)),
					ContestCommand::Participant => GenericDiscordRole(
						GenericDiscordRoleType::Contest(ContestType::Participant),
					),
				},
				GenericDiscordRoleCommand::SoraQuiz(s) => match s {
					SoraQuizCommand::Attendee =>
						GenericDiscordRole(GenericDiscordRoleType::SoraQuiz(SoraQuizType::Attendee)),
					SoraQuizCommand::Master =>
						GenericDiscordRole(GenericDiscordRoleType::SoraQuiz(SoraQuizType::Master)),
				},
			},
			Command::BnbDomainHolding => BnbDomainHolding,
			Command::BnbDigitalDomainClub(c) => match c {
				BnbDigitalDomainClubCommand::Bnb999ClubMember =>
					BnbDigitDomainClub(BnbDigitDomainType::Bnb999ClubMember),
				BnbDigitalDomainClubCommand::Bnb10kClubMember =>
					BnbDigitDomainClub(BnbDigitDomainType::Bnb10kClubMember),
			},
			Command::VIP3MembershipCard(arg) => match arg {
				VIP3MembershipCardLevelCommand::Gold =>
					VIP3MembershipCard(VIP3MembershipCardLevel::Gold),
				VIP3MembershipCardLevelCommand::Silver =>
					VIP3MembershipCard(VIP3MembershipCardLevel::Silver),
			},
			Command::WeirdoGhostGangHolder => WeirdoGhostGangHolder,
			Command::EVMAmountHolding(c) => match c {
				EVMAmountHoldingCommand::Ton => EVMAmountHolding(EVMTokenType::Ton),
				EVMAmountHoldingCommand::Trx => EVMAmountHolding(EVMTokenType::Trx),
			},
			Command::CryptoSummary => CryptoSummary,
			Command::LITStaking => LITStaking,
			Command::BRC20AmountHolder => BRC20AmountHolder,
			Command::TokenHoldingAmount(arg) => match arg {
				TokenHoldingAmountCommand::Bnb => TokenHoldingAmount(Web3TokenType::Bnb),
				TokenHoldingAmountCommand::Eth => TokenHoldingAmount(Web3TokenType::Eth),
				TokenHoldingAmountCommand::SpaceId => TokenHoldingAmount(Web3TokenType::SpaceId),
				TokenHoldingAmountCommand::Lit => TokenHoldingAmount(Web3TokenType::Lit),
				TokenHoldingAmountCommand::Wbtc => TokenHoldingAmount(Web3TokenType::Wbtc),
				TokenHoldingAmountCommand::Usdc => TokenHoldingAmount(Web3TokenType::Usdc),
				TokenHoldingAmountCommand::Usdt => TokenHoldingAmount(Web3TokenType::Usdt),
				TokenHoldingAmountCommand::Crv => TokenHoldingAmount(Web3TokenType::Crv),
				TokenHoldingAmountCommand::Matic => TokenHoldingAmount(Web3TokenType::Matic),
				TokenHoldingAmountCommand::Dydx => TokenHoldingAmount(Web3TokenType::Dydx),
				TokenHoldingAmountCommand::Amp => TokenHoldingAmount(Web3TokenType::Amp),
				TokenHoldingAmountCommand::Cvx => TokenHoldingAmount(Web3TokenType::Cvx),
				TokenHoldingAmountCommand::Tusd => TokenHoldingAmount(Web3TokenType::Tusd),
				TokenHoldingAmountCommand::Usdd => TokenHoldingAmount(Web3TokenType::Usdd),
				TokenHoldingAmountCommand::Gusd => TokenHoldingAmount(Web3TokenType::Gusd),
				TokenHoldingAmountCommand::Link => TokenHoldingAmount(Web3TokenType::Link),
				TokenHoldingAmountCommand::Grt => TokenHoldingAmount(Web3TokenType::Grt),
				TokenHoldingAmountCommand::Comp => TokenHoldingAmount(Web3TokenType::Comp),
				TokenHoldingAmountCommand::People => TokenHoldingAmount(Web3TokenType::People),
				TokenHoldingAmountCommand::Gtc => TokenHoldingAmount(Web3TokenType::Gtc),
				TokenHoldingAmountCommand::Ton => TokenHoldingAmount(Web3TokenType::Ton),
				TokenHoldingAmountCommand::Trx => TokenHoldingAmount(Web3TokenType::Trx),
				TokenHoldingAmountCommand::Nfp => TokenHoldingAmount(Web3TokenType::Nfp),
				TokenHoldingAmountCommand::Sol => TokenHoldingAmount(Web3TokenType::Sol),
				TokenHoldingAmountCommand::Mcrt => TokenHoldingAmount(Web3TokenType::Mcrt),
				TokenHoldingAmountCommand::Btc => TokenHoldingAmount(Web3TokenType::Btc),
			},
			Command::PlatformUser(arg) => match arg {
				PlatformUserCommand::KaratDaoUser => PlatformUser(PlatformUserType::KaratDaoUser),
				PlatformUserCommand::MagicCraftStakingUser =>
					PlatformUser(PlatformUserType::MagicCraftStakingUser),
			},
			Command::NftHolder(arg) => match arg {
				NftHolderCommand::WeirdoGhostGang => NftHolder(Web3NftType::WeirdoGhostGang),
				NftHolderCommand::Club3Sbt => NftHolder(Web3NftType::Club3Sbt),
			},
			Command::Dynamic(arg) => {
				let decoded_id = hex::decode(&arg.smart_contract_id.clone()).unwrap();
				let id_bytes: [u8; 20] = decoded_id.try_into().unwrap();
				Assertion::Dynamic(H160::from(id_bytes))
			},
		}
	}
}
