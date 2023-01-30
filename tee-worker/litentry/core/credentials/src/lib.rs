// Copyright 2020-2022 Litentry Technologies GmbH.
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
use litentry_primitives::{Assertion, AssertionLogic, Op, ParentchainBlockNumber};
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
	pub assertions: AssertionLogic,
	/// Results of each set of assertions
	pub values: Vec<bool>,
	/// The extrinsic on Parentchain for credential verification purpose
	pub endpoint: String,
}

impl CredentialSubject {
	pub fn is_empty(&self) -> bool {
		self.id.is_empty()
	}

	pub fn set_value(&mut self, value: bool) {
		self.values[0] = value;
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
	pub fn new(type_: ProofType, bn: ParentchainBlockNumber) -> Self {
		Self {
			created_block_number: bn,
			proof_type: type_,
			proof_purpose: PROOF_PURPOSE.to_string(),
			proof_value: "".to_string(),
			verification_method: "".to_string(),
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
	pub proof: Proof,
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
		vc.credential_subject.id = account_id_to_string(who);
		vc.issuance_block_number = bn;
		vc.proof.created_block_number = bn;
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
		self.id = format!("{}", HexDisplay::from(&vc_id.to_vec()));
	}

	pub fn to_json(&self) -> Result<String, Error> {
		let json_str =
			serde_json::to_string(&self).map_err(|err| Error::ParseError(format!("{}", err)))?;
		Ok(json_str)
	}

	pub fn get_index(&self) -> Result<[u8; 32], Error> {
		let index =
			hex::decode(self.id.as_bytes()).map_err(|err| Error::ParseError(format!("{}", err)))?;
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
		self.validate_unsigned()?;

		if self.id.is_empty() {
			return Err(Error::RuntimeError("Credential ID is invalid".to_string()))
		}

		if self.credential_subject.is_empty() {
			return Err(Error::EmptyCredentialSubject)
		}

		// ToDo: validate issuer
		if self.issuer.is_empty() {
			return Err(Error::EmptyCredentialIssuer)
		}

		if self.proof.is_empty() {
			return Err(Error::InvalidProof)
		}

		if self.proof.created_block_number == 0 {
			return Err(Error::EmptyProofBlockNumber)
		}

		// the proof bn that is must be equal or after issuance bn
		if self.proof.created_block_number < self.issuance_block_number {
			return Err(Error::InvalidProof)
		}

		//ToDo: validate proof signature

		let exported = self.to_json()?;
		if exported.len() > MAX_CREDENTIAL_SIZE {
			return Err(Error::CredentialIsTooLong)
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
				let credential: Credential = Credential::from_template(raw, who, shard, bn)?;
				Ok(credential)
			},
			_ => Err(Error::UnsupportedAssertion),
		}
	}

	pub fn add_assertion_a1(&mut self, web2_cnt: i32, web3_cnt: i32) {
		let web2_cnt = format!("{}", web2_cnt);
		let web3_cnt = format!("{}", web3_cnt);

		let web2_item = AssertionLogic::new_item("$web2_account_cnt", Op::GreaterEq, &web2_cnt);
		let web3_item = AssertionLogic::new_item("$web3_account_cnt", Op::GreaterThan, &web3_cnt);

		let assertion = AssertionLogic::new_or().add_item(web2_item).add_item(web3_item);

		self.credential_subject.assertions = assertion;
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
		assert_eq!(vc.proof.proof_purpose, "assertionMethod");
	}
}
