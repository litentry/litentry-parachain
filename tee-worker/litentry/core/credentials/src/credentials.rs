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

// #[cfg(all(not(feature = "std"), feature = "sgx"))]
// use crate::sgx_reexport_prelude::*;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

//use crate::ocall::OcallApi;
use crate::{ensure, error::Error};
use chrono::{DateTime, FixedOffset};
use itp_attestation_handler::IntelAttestationHandler;
use litentry_primitives::Assertion;
use serde::{Deserialize, Serialize};
use std::{
	fmt::Debug,
	str::FromStr,
	string::{String, ToString},
	vec::Vec,
};

pub const PROOF_PURPOSE: &str = "assertionMethod";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct VCDateTime {
	// rfc3339
	date_time: DateTime<FixedOffset>,
	// Whether to use "Z" or "+00:00" when formatting the date-time in UTC
	use_z: bool,
}

impl FromStr for VCDateTime {
	type Err = chrono::format::ParseError;
	fn from_str(date_time: &str) -> Result<Self, Self::Err> {
		let use_z = date_time.ends_with('Z');
		let date_time = DateTime::parse_from_rfc3339(date_time)?;
		Ok(VCDateTime { date_time, use_z })
	}
}

impl TryFrom<String> for VCDateTime {
	type Error = chrono::format::ParseError;
	fn try_from(date_time: String) -> Result<Self, Self::Error> {
		Self::from_str(&date_time)
	}
}

impl From<VCDateTime> for String {
	fn from(z_date_time: VCDateTime) -> String {
		let VCDateTime { date_time, use_z } = z_date_time;
		date_time.to_rfc3339_opts(chrono::SecondsFormat::AutoSi, use_z)
	}
}

impl<Tz: chrono::TimeZone> From<DateTime<Tz>> for VCDateTime
where
	chrono::DateTime<chrono::FixedOffset>: From<chrono::DateTime<Tz>>,
{
	fn from(date_time: DateTime<Tz>) -> Self {
		Self { date_time: date_time.into(), use_z: true }
	}
}

impl<Tz: chrono::TimeZone> From<VCDateTime> for DateTime<Tz>
where
	chrono::DateTime<Tz>: From<chrono::DateTime<chrono::FixedOffset>>,
{
	fn from(vc_date_time: VCDateTime) -> Self {
		Self::from(vc_date_time.date_time)
	}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum ProofType {
	Ed25519Signature2020,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum CredentialType {
	VerifiableCredential,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataSoure {
	pub data_provider_id: u32,
	pub data_provider: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CredentialSubject {
	pub id: String,
	pub description: String,
	#[serde(rename = "type")]
	pub types: String,
	pub tag: Vec<String>,
	pub data_soure: Vec<DataSoure>,
	pub assertions: String,
	pub values: Vec<bool>,
	pub endpoint: String,
}

impl CredentialSubject {
	pub fn is_empty(&self) -> bool {
		self.id.is_empty()
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Proof {
	// rfc3339
	pub created: Option<VCDateTime>,
	#[serde(rename = "type")]
	pub proof_type: ProofType,
	pub proof_purpose: String,
	pub proof_value: String,
	pub verification_method: String,
}

impl Proof {
	pub fn new(type_: ProofType) -> Proof {
		Proof {
			created: None,
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

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ValidationResult {
	pub result: bool,
	pub msg: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
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
	pub issuance_date: Option<VCDateTime>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expiration_date: Option<VCDateTime>,
	pub proof: Proof,
}

impl Credential {
	pub fn from_json(s: &str) -> Result<Self, Error> {
		let vc: Self = serde_json::from_str(s)?;
		vc.validate()?;
		Ok(vc)
	}

	pub fn validate(&self) -> Result<(), Error> {
		self.validate_unsigned()?;
		if self.proof.is_empty() {
			return Err(Error::EmptyCredentialProof)
		}
		//ToDo: validate_proof
		Ok(())
	}

	pub fn validate_unsigned(&self) -> Result<(), Error> {
		if !self.types.contains(&CredentialType::VerifiableCredential) {
			return Err(Error::EmptyCredentialType)
		}

		if self.issuer.is_empty() {
			return Err(Error::EmptyCredentialIssuer)
		}

		if self.credential_subject.is_empty() {
			return Err(Error::EmptyCredentialSubject)
		}

		if self.issuance_date.is_none() {
			return Err(Error::EmptyIssuanceDate)
		}

		Ok(())
	}

	pub fn get_issuer(&self) -> Result<Issuer, Error> {
		//let attestation_handler = IntelAttestationHandler::new(OcallApi);

		Ok(())
	}

	pub fn sign_proof(&self) -> Result<Proof, Error> {
		if self.issuer.is_empty() {
			return Err(Error::EmptyCredentialIssuer)
		}

		Ok(())
	}

	pub fn validate_proof(&self) -> Result<ValidationResult, Error> {
		Ok(())
	}

	pub fn generate_credential(&self, assertion: Assertion) -> Result<Credential, Error> {
		match assertion {
			Assertion::A1 => {
				let data = include_str!("templates/a1.json");
				let vc: Credential = Credential::from_json(data).unwrap();
			},
			Assertion::A2(ParameterString, ParameterString) => {},
			Assertion::A3(ParameterString, ParameterString) => {},
			Assertion::A4(Balance, ParameterString) => {},
			Assertion::A5(ParameterString, ParameterString) => {},
			Assertion::A6 => {},
			Assertion::A7(Balance, u32) => {},
			Assertion::A8(u64) => {},
			Assertion::A9 => {},
			Assertion::A10(Balance, u32) => {},
			Assertion::A11(Balance, u32) => {},
			Assertion::A13(u32) => {},
			_ => return Err(Error::UnsupportedAssertion),
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn eval_simple_success() {
		let data = include_str!("templates/a1.json");

		let vc: Credential = Credential::from_json(data).unwrap();
		println!("{:?}", vc);
		assert_eq!(vc.proof.proof_purpose, "assertionMethod");
	}
}
