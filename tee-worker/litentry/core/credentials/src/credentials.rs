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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

use crate::error::Error;
use chrono::{DateTime, FixedOffset, NaiveDateTime, SecondsFormat};
use codec::{Decode, Encode};
use itp_types::AccountId;
use itp_utils::stringify::account_id_to_string;
use litentry_primitives::Assertion;
use log::*;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use std::{
	fmt::Debug,
	string::{String, ToString},
	time::{SystemTime, UNIX_EPOCH},
	vec::Vec,
};

pub const PROOF_PURPOSE: &str = "assertionMethod";
pub const MAX_CREDENTIAL_SIZE: usize = 2048;

pub fn now() -> String {
	let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
	let naive = NaiveDateTime::from_timestamp_opt(ts.as_secs() as i64, ts.subsec_nanos()).unwrap();
	let datenow_time = DateTime::<FixedOffset>::from_utc(naive, FixedOffset::east_opt(0).unwrap());
	datenow_time.to_rfc3339_opts(SecondsFormat::Secs, true)
}

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
pub struct DataSoure {
	pub data_provider_id: u32,
	pub data_provider: String,
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct Issuer {
	pub id: String,
	pub name: String,
	pub enclave_id: String,
}

impl Issuer {
	pub fn is_empty(&self) -> bool {
		self.enclave_id.is_empty()
	}

	pub fn new(id: String, name: String, enclave_id: String) -> Self {
		Self { id, name, enclave_id }
	}
}

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct CredentialSubject {
	pub id: String,
	pub description: String,
	#[serde(rename = "type")]
	pub types: String,
	pub tag: Vec<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_soure: Option<Vec<DataSoure>>,
	pub assertions: String,
	pub values: Vec<bool>,
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

#[derive(Serialize, Deserialize, Encode, Decode, Clone, Debug, PartialEq, Eq, TypeInfo)]
#[serde(rename_all = "camelCase")]
pub struct Proof {
	pub created: Option<String>,
	#[serde(rename = "type")]
	pub proof_type: ProofType,
	pub proof_purpose: String,
	pub proof_value: String,
	pub verification_method: String,
}

impl Proof {
	pub fn new(type_: ProofType) -> Self {
		Self {
			created: Some(now()),
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
	#[serde(rename = "@context")]
	pub context: Vec<String>,
	pub id: String,
	pub subject: String,
	#[serde(rename = "type")]
	pub types: Vec<CredentialType>,
	pub credential_subject: CredentialSubject,
	pub issuer: Issuer,
	pub issuance_date: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expiration_date: Option<String>,
	pub proof: Proof,
}

impl Credential {
	pub fn from_template(s: &str, who: &AccountId) -> Result<Self, Error> {
		let mut vc: Self =
			serde_json::from_str(s).map_err(|err| Error::ParseError(format!("{}", err)))?;
		vc.subject = account_id_to_string(who);
		vc.issuance_date = Some(now());
		vc.validate_unsigned()?;
		Ok(vc)
	}

	pub fn to_json(&self) -> Result<String, Error> {
		let json_str =
			serde_json::to_string(&self).map_err(|err| Error::ParseError(format!("{}", err)))?;
		Ok(json_str)
	}

	pub fn validate_unsigned(&self) -> Result<(), Error> {
		if !self.types.contains(&CredentialType::VerifiableCredential) {
			return Err(Error::EmptyCredentialType)
		}

		if self.subject.is_empty() {
			return Err(Error::EmptySubject)
		}

		if self.issuance_date.is_none() {
			return Err(Error::EmptyIssuanceDate)
		}

		Ok(())
	}

	pub fn validate(&self) -> Result<(), Error> {
		self.validate_unsigned()?;

		if self.credential_subject.is_empty() {
			return Err(Error::EmptyCredentialSubject)
		}

		// ToDo: validate issuer
		// if self.issuer.is_empty() {
		// 	return Err(Error::EmptyCredentialIssuer)
		// }

		//ToDo: validate the proof timestamp that is must be equal or after issuance timestamp
		if self.proof.created.is_none() {
			return Err(Error::InvalidDateOrTimeError)
		}

		// ToDo: validate proof
		// if self.proof.is_empty() {
		// 	return Err(Error::EmptyCredentialProof)
		// }

		//ToDo: validate proof signature

		let exported = self.to_json()?;
		if exported.len() > MAX_CREDENTIAL_SIZE {
			return Err(Error::CredentialIsTooLong)
		}

		Ok(())
	}

	pub fn validate_schema(&self) -> Result<(), Error> {
		//ToDo: fetch and check schema status from Parentchain storage
		Ok(())
	}

	pub fn generate_unsigned_credential(
		assertion: &Assertion,
		who: &AccountId,
	) -> Result<Credential, Error> {
		debug!("generate unsigned credential {:?}", assertion);
		match assertion {
			Assertion::A1 => {
				let raw = include_str!("templates/a1.json");
				let credential: Credential = Credential::from_template(raw, who)?;
				Ok(credential)
			},
			_ => Err(Error::UnsupportedAssertion),
		}
	}

	pub fn add_assertion_a1(&mut self, _web2_cnt: i32, _web3_cnt: i32) {
		self.credential_subject.assertions = "\"or\": [{\"src\": \"$web2_account_cnt\", \"op\": \">\", \"dsc\": \"0\",},{\"src\": \"$web3_account_cnt\", \"op\": \">\", \"dsc\": \"0\",}]".to_string();
	}

	pub fn generate_issuer() -> Result<Issuer, Error> {
		let issuer = Issuer::new("".to_string(), "".to_string(), "".to_string());
		Ok(issuer)
	}

	pub fn sign_proof(&self) -> Result<Proof, Error> {
		if self.issuer.is_empty() {
			return Err(Error::EmptyCredentialIssuer)
		}
		let proof = Proof::new(ProofType::Ed25519Signature2020);
		Ok(proof)
	}

	pub fn validate_proof(&self) -> Result<(), Error> {
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn eval_simple_success() {
		let who = AccountId::from([0; 32]);
		let data = include_str!("templates/a1.json");

		let vc = Credential::from_template(data, &who).unwrap();
		assert!(vc.validate_unsigned().is_ok());
		assert_eq!(vc.proof.proof_purpose, "assertionMethod");
	}
}
