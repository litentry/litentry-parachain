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

//#[cfg(all(not(feature = "std"), feature = "sgx"))]
//use crate::sgx_reexport_prelude::*;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

use crate::error::Error;
use chrono::{DateTime, FixedOffset, NaiveDateTime};
use itp_types::AccountId;
use itp_utils::stringify::account_id_to_string;
use litentry_primitives::Assertion;
use log::*;
use serde::{Deserialize, Serialize};
use std::{
	fmt::Debug,
	str::FromStr,
	string::{String, ToString},
	time::{SystemTime, UNIX_EPOCH},
	vec::Vec,
};

pub const PROOF_PURPOSE: &str = "assertionMethod";
pub const MAX_CREDENTIAL_SIZE: usize = 2048;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct VCDateTime {
	date_time: DateTime<FixedOffset>, // rfc3339
	use_z: bool, // Whether to use "Z" or "+00:00" when formatting the date-time in UTC
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

impl VCDateTime {
	pub fn now() -> Self {
		let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
		let naive =
			NaiveDateTime::from_timestamp_opt(ts.as_secs() as i64, ts.subsec_nanos() as u32)
				.unwrap();
		let now = DateTime::from_utc(naive, FixedOffset::east_opt(0).unwrap());
		Self { date_time: now, use_z: true }
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

	pub fn new(id: String, name: String, enclave_id: String) -> Self {
		Self { id, name, enclave_id }
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
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(skip_deserializing)]
	pub data_soure: Option<Vec<DataSoure>>,
	pub assertions: String,
	pub values: Vec<bool>,
	pub endpoint: String,
}

impl CredentialSubject {
	pub fn is_empty(&self) -> bool {
		self.id.is_empty()
	}

	pub fn set_subject_id(&mut self, who: &AccountId) {
		self.id = account_id_to_string(who);
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Proof {
	#[serde(skip_deserializing)]
	pub created: Option<VCDateTime>,
	#[serde(rename = "type")]
	pub proof_type: ProofType,
	pub proof_purpose: String,
	pub proof_value: String,
	pub verification_method: String,
}

impl Proof {
	pub fn new(type_: ProofType) -> Self {
		Self {
			created: Some(VCDateTime::now()),
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
	#[serde(skip_deserializing)]
	pub issuance_date: Option<VCDateTime>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(skip_deserializing)]
	pub expiration_date: Option<VCDateTime>,
	pub proof: Proof,
}

impl Credential {
	pub fn from_template(s: &str, who: &AccountId) -> Result<Self, Error> {
		let mut vc: Self =
			serde_json::from_str(s).map_err(|e| Error::Other(format!("{:?}", e).into()))?;
		vc.issuance_date = Some(VCDateTime::now());
		vc.credential_subject.set_subject_id(who);
		vc.validate_unsigned()?;
		Ok(vc)
	}

	pub fn to_json(&self) -> Result<String, Error> {
		Ok(serde_json::to_string(&self).map_err(|e| Error::Other(format!("{:?}", e).into()))?)
	}

	pub fn validate(&self) -> Result<(), Error> {
		self.validate_unsigned()?;

		// if self.issuer.is_empty() {
		// 	return Err(Error::EmptyCredentialIssuer)
		// }

		// validate proof
		// if self.proof.is_empty() {
		// 	return Err(Error::EmptyCredentialProof)
		// }

		// if self.proof.created.is_none() {
		// 	return Err(Error::InvalidDateOrTimeError)
		// }

		//ToDo: validate proof signature

		//ToDo: validate the timestamp of proof, it's must be after credential issuance timestamp

		let exported = self.to_json().unwrap();
		if exported.len() > MAX_CREDENTIAL_SIZE {
			return Err(Error::CredentialIsTooLong)
		}

		Ok(())
	}

	pub fn validate_unsigned(&self) -> Result<(), Error> {
		if !self.types.contains(&CredentialType::VerifiableCredential) {
			return Err(Error::EmptyCredentialType)
		}

		if self.credential_subject.is_empty() {
			return Err(Error::EmptyCredentialSubject)
		}

		if self.issuance_date.is_none() {
			return Err(Error::EmptyIssuanceDate)
		}

		Ok(())
	}

	pub fn validate_schema(&self) -> Result<(), Error> {
		//ToDo: fetch schema status from Parentchain storage

		Ok(())
	}

	pub fn generate_unsigned_credential(
		who: &AccountId,
		assertion: &Assertion,
	) -> Result<Credential, Error> {
		info!("start generate_unsigned_credential {:?}", assertion);
		match assertion {
			Assertion::A1 => {
				let raw = include_str!("templates/a1.json");
				let vc: Credential = Credential::from_template(raw, who)?;
				return Ok(vc)
			},
			_ => return Err(Error::UnsupportedAssertion),
		}
	}

	pub fn generate_issuer(&self) -> Result<Issuer, Error> {
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

		let vc: Credential = Credential::from_template(data, &who).unwrap();
		println!("{:?}", vc);
		assert_eq!(vc.proof.proof_purpose, "assertionMethod");

		let exported = vc.to_json().unwrap();
		println!("{}", exported);
	}
}
