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
//
// TEE Implementation of Verifiable Credentials Data Model v2.0
// W3C Editor's Draft 07 January 2023
// https://w3c.github.io/vc-data-model

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "sgx"))]
#[macro_use]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use chrono_sgx as chrono;
	pub use serde_json_sgx as serde_json;
	pub use thiserror_sgx as thiserror;
}

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

use codec::{Decode, Encode};
use itp_stf_primitives::types::ShardIdentifier;
use itp_types::AccountId;
use itp_utils::stringify::account_id_to_string;
use litentry_primitives::{
	format_assertion_to_date, Assertion, Balance, ParentchainBlockNumber, ASSERTION_FROM_DATE,
};
use log::*;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_core::{hashing::blake2_256, hexdisplay::HexDisplay};
use std::{
	fmt::Debug,
	string::{String, ToString},
	vec::Vec,
};

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate rust_base58_sgx as rust_base58;

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate hex_sgx as hex;

use rust_base58::ToBase58;

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate rand_sgx as rand;

use rand::Rng;

pub mod error;
pub use error::Error;
pub mod schema;

pub mod assertion_logic;
use assertion_logic::{AssertionLogic, Op};

pub const LITENTRY_ISSUER_NAME: &str = "Litentry TEE Worker";
pub const PROOF_PURPOSE: &str = "assertionMethod";
pub const MAX_CREDENTIAL_SIZE: usize = 2048;

/// Ed25519 Signature 2018, W3C, 23 July 2021, https://w3c-ccg.github.io/lds-ed25519-2018
/// May be registered in Linked Data Cryptographic Suite Registry, W3C, 29 December 2020
/// https://w3c-ccg.github.io/ld-cryptosuite-registry
#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub enum ProofType {
	Ed25519Signature2020,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
pub enum CredentialType {
	VerifiableCredential,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct DataSource {
	/// ID of the data provider
	pub data_provider_id: u32,
	/// Endpoint of the data provider
	pub data_provider: String,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct Issuer {
	/// ID of the TEE Worker
	pub id: String,
	pub name: String,
	pub shard: String,
}

impl Issuer {
	pub fn is_empty(&self) -> bool {
		self.shard.is_empty() || self.shard.is_empty()
	}

	pub fn set_id(&mut self, id: &AccountId) {
		self.id = account_id_to_string(id);
	}
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct CredentialSubject {
	/// Identifier for the only entity that the credential was issued
	pub id: String,
	pub description: String,
	#[serde(rename = "type")]
	pub types: String,
	/// (Optional) Some externally provided identifiers
	pub tag: Vec<String>,
	/// (Optional) Data source definitions for trusted data providers
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_source: Option<Vec<DataSource>>,
	/// Several sets of assertions.
	/// Each assertion contains multiple steps to describe how to fetch data and calculate the value
	pub assertions: Vec<AssertionLogic>,
	/// Results of each set of assertions
	pub values: Vec<bool>,
	/// The extrinsic on Parentchain for credential verification purpose
	pub endpoint: String,
}

impl CredentialSubject {
	pub fn is_empty(&self) -> bool {
		self.id.is_empty()
	}
}

/// Verifiable Credentials JSON Schema 2022, W3C, 8 November 2022
/// https://w3c-ccg.github.io/vc-json-schemas/
#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct CredentialSchema {
	/// Schema ID that is maintained by Parentchain VCMP
	pub id: String,
	/// The schema type, generally it is
	#[serde(rename = "type")]
	pub types: String,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct Proof {
	/// The block number when the signature was created
	pub created_block_number: ParentchainBlockNumber,
	/// The cryptographic signature suite that used to generate signature
	#[serde(rename = "type")]
	pub proof_type: ProofType,
	/// Purpose of this proof, generally it is expected as a fixed value, such as 'assertionMethod'
	pub proof_purpose: String,
	/// The digital signature value
	pub proof_value: String,
	/// The public key from Issuer
	pub verification_method: String,
}

impl Proof {
	pub fn new(bn: ParentchainBlockNumber, sig: &Vec<u8>, issuer: &AccountId) -> Self {
		Proof {
			created_block_number: bn,
			proof_type: ProofType::Ed25519Signature2020,
			proof_purpose: PROOF_PURPOSE.to_string(),
			proof_value: format!("{}", HexDisplay::from(sig)),
			verification_method: account_id_to_string(issuer),
		}
	}

	pub fn is_empty(&self) -> bool {
		self.proof_value.is_empty()
	}
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct Credential {
	/// Contexts defines the structure and data types of the credential
	#[serde(rename = "@context")]
	pub context: Vec<String>,
	/// The specific UUID of the credential, it is used for onchain verification
	pub id: String,
	/// Uniquely identifier of the type of the credential
	#[serde(rename = "type")]
	pub types: Vec<CredentialType>,
	/// Assertions claimed about the subjects of the credential
	pub credential_subject: CredentialSubject,
	/// The TEE enclave who issued the credential
	pub issuer: Issuer,
	pub issuance_block_number: ParentchainBlockNumber,
	/// (Optional)
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expiration_block_number: Option<ParentchainBlockNumber>,
	/// Digital proof with the signature of Issuer
	#[serde(skip_deserializing)]
	pub proof: Option<Proof>,
	#[serde(skip_deserializing)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub credential_schema: Option<CredentialSchema>,
}

impl Credential {
	pub fn from_template(
		s: &str,
		who: &AccountId,
		shard: &ShardIdentifier,
		bn: ParentchainBlockNumber,
	) -> Result<Self, Error> {
		let mut vc: Self =
			serde_json::from_str(s).map_err(|err| Error::ParseError(format!("{}", err)))?;
		vc.issuer.shard = shard.encode().to_base58();
		vc.issuer.name = LITENTRY_ISSUER_NAME.to_string();
		vc.credential_subject.id = account_id_to_string(who);
		vc.issuance_block_number = bn;
		vc.expiration_block_number = None;
		vc.credential_schema = None;
		vc.generate_id();
		vc.validate_unsigned()?;
		Ok(vc)
	}

	fn generate_id(&mut self) {
		let seed = rand::thread_rng().gen::<[u8; 32]>();
		let mut ext_hash = blake2_256(self.credential_subject.id.as_bytes()).to_vec();
		ext_hash.append(&mut seed.to_vec());
		let vc_id = blake2_256(ext_hash.as_slice());
		self.id = "0x".to_string();
		self.id.push_str(&(format!("{}", HexDisplay::from(&vc_id.to_vec()))));
	}

	pub fn add_proof(&mut self, sig: &Vec<u8>, bn: ParentchainBlockNumber, issuer: &AccountId) {
		self.proof = Some(Proof::new(bn, sig, issuer));
	}

	pub fn to_json(&self) -> Result<String, Error> {
		let json_str =
			serde_json::to_string(&self).map_err(|err| Error::ParseError(format!("{}", err)))?;
		Ok(json_str)
	}

	pub fn get_index(&self) -> Result<[u8; 32], Error> {
		let bytes = &self.id.as_bytes()[b"0x".len()..];
		let index = hex::decode(bytes).map_err(|err| Error::ParseError(format!("{}", err)))?;
		let vi: [u8; 32] = index.try_into().unwrap();
		Ok(vi)
	}

	pub fn validate_unsigned(&self) -> Result<(), Error> {
		if !self.types.contains(&CredentialType::VerifiableCredential) {
			return Err(Error::EmptyCredentialType)
		}

		if self.credential_subject.id.is_empty() {
			return Err(Error::EmptyCredentialSubject)
		}

		if self.issuance_block_number == 0 {
			return Err(Error::EmptyIssuanceBlockNumber)
		}

		if self.id.is_empty() {
			return Err(Error::InvalidCredential)
		}

		Ok(())
	}

	pub fn validate(&self) -> Result<(), Error> {
		let vc = self.clone();

		vc.validate_unsigned()?;

		if vc.credential_subject.is_empty() {
			return Err(Error::EmptyCredentialSubject)
		}

		// ToDo: validate issuer
		if vc.issuer.is_empty() {
			return Err(Error::EmptyCredentialIssuer)
		}

		let exported = vc.to_json()?;
		if exported.len() > MAX_CREDENTIAL_SIZE {
			return Err(Error::CredentialIsTooLong)
		}

		if vc.proof.is_none() {
			return Err(Error::InvalidProof)
		} else {
			let proof = vc.proof.unwrap();
			if proof.created_block_number == 0 {
				return Err(Error::EmptyProofBlockNumber)
			}

			//ToDo: validate proof signature
		}

		Ok(())
	}

	pub fn validate_schema(&self) -> Result<(), Error> {
		//ToDo: fetch schema from Parentchain and check its status
		Ok(())
	}

	pub fn generate_unsigned_credential(
		assertion: &Assertion,
		who: &AccountId,
		shard: &ShardIdentifier,
		bn: ParentchainBlockNumber,
	) -> Result<Credential, Error> {
		debug!("generate unsigned credential {:?}", assertion);
		match assertion {
			Assertion::A1 => {
				let raw = include_str!("templates/a1.json");
				let mut credential: Credential = Credential::from_template(raw, who, shard, bn)?;
				credential.credential_subject.values.clear();
				Ok(credential)
			},
			Assertion::A2(_) => {
				let raw = include_str!("templates/a2.json");
				let mut credential: Credential = Credential::from_template(raw, who, shard, bn)?;
				credential.credential_subject.assertions.clear();
				credential.credential_subject.values.clear();
				Ok(credential)
			},
			Assertion::A3(_, _, _) => {
				let raw = include_str!("templates/a3.json");
				let mut credential: Credential = Credential::from_template(raw, who, shard, bn)?;
				credential.credential_subject.assertions.clear();
				credential.credential_subject.values.clear();
				Ok(credential)
			},
			Assertion::A4(_minimum_amount) => {
				let raw = include_str!("templates/a4.json");
				let mut credential: Credential = Credential::from_template(raw, who, shard, bn)?;
				// remove default assertions from template
				credential.credential_subject.assertions.clear();
				credential.credential_subject.values.clear();
				Ok(credential)
			},
			Assertion::A6 => {
				let raw = include_str!("templates/a6.json");
				let mut credential: Credential = Credential::from_template(raw, who, shard, bn)?;
				credential.credential_subject.assertions.clear();
				credential.credential_subject.values.clear();
				Ok(credential)
			},
			Assertion::A7(_minimum_amount) => {
				let raw = include_str!("templates/a7.json");
				let mut credential: Credential = Credential::from_template(raw, who, shard, bn)?;
				credential.credential_subject.assertions.clear();
				credential.credential_subject.values.clear();
				Ok(credential)
			},
			Assertion::A8 => {
				let raw = include_str!("templates/a8.json");
				let mut credential: Credential = Credential::from_template(raw, who, shard, bn)?;
				credential.credential_subject.assertions.clear();
				credential.credential_subject.values.clear();
				Ok(credential)
			},
			Assertion::A10(_minimum_amount) => {
				let raw = include_str!("templates/a10.json");
				let mut credential: Credential = Credential::from_template(raw, who, shard, bn)?;
				credential.credential_subject.assertions.clear();
				credential.credential_subject.values.clear();
				Ok(credential)
			},
			Assertion::A11(_minimum_amount) => {
				let raw = include_str!("templates/a11.json");
				let mut credential: Credential = Credential::from_template(raw, who, shard, bn)?;
				credential.credential_subject.assertions.clear();
				credential.credential_subject.values.clear();
				Ok(credential)
			},
			_ => Err(Error::UnsupportedAssertion),
		}
	}

	// Including assertion 4/7/10/11
	pub fn update_holder(&mut self, index: usize, minimum_amount: Balance) {
		let minimum_amount = format!("{}", minimum_amount);
		let to_date = format_assertion_to_date();

		let minimum_amount =
			AssertionLogic::new_item("$minimum_amount", Op::GreaterEq, &minimum_amount);
		let to_date = AssertionLogic::new_item("$to_date", Op::GreaterEq, &to_date);

		if index == 0 {
			let from_date = ASSERTION_FROM_DATE[0];
			let from_date = AssertionLogic::new_item("$from_date", Op::LessThan, from_date);
			let assertion = AssertionLogic::new_and()
				.add_item(minimum_amount)
				.add_item(from_date)
				.add_item(to_date);
			self.credential_subject.assertions.push(assertion);
			self.credential_subject.values.push(false);
		} else if (1..7).contains(&index) {
			let from_date = ASSERTION_FROM_DATE[index];
			let from_date = AssertionLogic::new_item("$from_date", Op::LessThan, from_date);
			let assertion = AssertionLogic::new_and()
				.add_item(minimum_amount)
				.add_item(from_date)
				.add_item(to_date);
			self.credential_subject.assertions.push(assertion);
			self.credential_subject.values.push(true);
		} else {
			let from_date = ASSERTION_FROM_DATE[index - 1];
			let from_date = AssertionLogic::new_item("$from_date", Op::GreaterEq, from_date);
			let assertion = AssertionLogic::new_and()
				.add_item(minimum_amount)
				.add_item(from_date)
				.add_item(to_date);
			self.credential_subject.assertions.push(assertion);
			self.credential_subject.values.push(true);
		}
	}

	pub fn add_assertion_a2(&mut self, guild_id: String) {
		let verified = AssertionLogic::new_item("$verified_discord_account", Op::GreaterThan, "0");
		let has_joined = AssertionLogic::new_item("$has_joined", Op::Equal, "true");
		let guild = AssertionLogic::new_item("$discord_guild_id", Op::Equal, guild_id.as_str());

		let assertion = AssertionLogic::new_and()
			.add_item(verified)
			.add_item(has_joined)
			.add_item(guild);
		self.credential_subject.assertions.push(assertion);
	}

	pub fn add_assertion_a3(&mut self, guild_id: String, channel_id: String, role_id: String) {
		let has_role = AssertionLogic::new_item("$has_role", Op::Equal, "true");
		let has_commented = AssertionLogic::new_item("$has_commented", Op::Equal, "true");
		let guild = AssertionLogic::new_item("$discord_guild_id", Op::Equal, guild_id.as_str());
		let channel =
			AssertionLogic::new_item("$discord_channel_id", Op::Equal, channel_id.as_str());
		let role = AssertionLogic::new_item("$discord_role_id", Op::Equal, role_id.as_str());

		let assertion = AssertionLogic::new_and()
			.add_item(has_role)
			.add_item(has_commented)
			.add_item(guild)
			.add_item(channel)
			.add_item(role);
		self.credential_subject.assertions.push(assertion);
	}

	pub fn add_assertion_a6(&mut self, min: u64, max: u64) {
		let min = format!("{}", min);
		let max = format!("{}", max);

		let follower_min = AssertionLogic::new_item("$total_followers", Op::GreaterThan, &min);
		let follower_max = AssertionLogic::new_item("$total_followers", Op::LessEq, &max);

		let assertion = AssertionLogic::new_and().add_item(follower_min).add_item(follower_max);
		self.credential_subject.assertions.push(assertion);
	}

	pub fn add_assertion_a8(&mut self, min: u64, max: u64) {
		let min = format!("{}", min);
		let max = format!("{}", max);

		let min_item = AssertionLogic::new_item("$total_txs", Op::GreaterThan, &min);
		let max_item = AssertionLogic::new_item("$total_txs", Op::LessEq, &max);

		let assertion = AssertionLogic::new_and().add_item(min_item).add_item(max_item);
		self.credential_subject.assertions.push(assertion);
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn eval_simple_success() {
		let who = AccountId::from([0; 32]);
		let data = include_str!("templates/a1.json");
		let shard = ShardIdentifier::default();

		let vc = Credential::from_template(data, &who, &shard, 1u32).unwrap();
		assert!(vc.validate_unsigned().is_ok());
		let id: String = vc.credential_subject.id.clone();
		assert_eq!(id, account_id_to_string(&who));
	}

	#[test]
	fn update_holder_works() {
		let who = AccountId::from([0; 32]);
		let shard = ShardIdentifier::default();
		let min_balance = 1;
		let to_date = format_assertion_to_date();

		{
			// case 1: from_date_index = 0
			let from_date_index = 0_usize;
			let from_date = AssertionLogic::new_item("$from_date", Op::LessThan, "2017-01-01");

			let a11 = Assertion::A11(min_balance);
			let mut credential_unsigned =
				Credential::generate_unsigned_credential(&a11, &who, &shard.clone(), 1u32).unwrap();
			credential_unsigned.update_holder(from_date_index, min_balance);

			let minimum_amount = format!("{}", min_balance);
			let minimum_amount =
				AssertionLogic::new_item("$minimum_amount", Op::GreaterEq, &minimum_amount);
			let to_date = AssertionLogic::new_item("$to_date", Op::GreaterEq, &to_date);

			let assertion = AssertionLogic::new_and()
				.add_item(minimum_amount)
				.add_item(from_date)
				.add_item(to_date);

			assert_eq!(credential_unsigned.credential_subject.values[0], false);
			assert_eq!(credential_unsigned.credential_subject.assertions[0], assertion)
		}

		{
			// case 1: from_date_index = 1
			let from_date_index = 1_usize;

			let a11 = Assertion::A11(min_balance);
			let mut credential_unsigned =
				Credential::generate_unsigned_credential(&a11, &who, &shard.clone(), 1u32).unwrap();
			credential_unsigned.update_holder(from_date_index, min_balance);

			let minimum_amount = format!("{}", min_balance);
			let minimum_amount =
				AssertionLogic::new_item("$minimum_amount", Op::GreaterEq, &minimum_amount);
			let from_date = AssertionLogic::new_item("$from_date", Op::LessThan, "2018-01-01");
			let to_date = AssertionLogic::new_item("$to_date", Op::GreaterEq, &to_date);
			let assertion = AssertionLogic::new_and()
				.add_item(minimum_amount)
				.add_item(from_date)
				.add_item(to_date);

			assert_eq!(credential_unsigned.credential_subject.values[0], true);
			assert_eq!(credential_unsigned.credential_subject.assertions[0], assertion)
		}

		{
			// case 1: from_date_index = 7
			let from_date_index = 7_usize;

			let a11 = Assertion::A11(min_balance);
			let mut credential_unsigned =
				Credential::generate_unsigned_credential(&a11, &who, &shard.clone(), 1u32).unwrap();
			credential_unsigned.update_holder(from_date_index, min_balance);

			let minimum_amount = format!("{}", min_balance);
			let minimum_amount =
				AssertionLogic::new_item("$minimum_amount", Op::GreaterEq, &minimum_amount);
			let from_date = AssertionLogic::new_item("$from_date", Op::GreaterEq, "2023-01-01");
			let to_date = AssertionLogic::new_item("$to_date", Op::GreaterEq, &to_date);
			let assertion = AssertionLogic::new_and()
				.add_item(minimum_amount)
				.add_item(from_date)
				.add_item(to_date);

			assert_eq!(credential_unsigned.credential_subject.values[0], true);
			assert_eq!(credential_unsigned.credential_subject.assertions[0], assertion)
		}
	}
}
