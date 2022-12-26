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

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

// re-export module to properly feature gate sgx and regular std environment
#[cfg(all(not(feature = "std"), feature = "sgx"))]
pub mod sgx_reexport_prelude {
	pub use futures_sgx as futures;
	pub use hex_sgx as hex;
	pub use thiserror_sgx as thiserror;
	pub use url_sgx as url;
}

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub const PROOF_PURPOSE: &str = "assertionMethod";
pub const CREDENTIAL_TYPE: &str = "CredentialType";

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
	#[error("Empty Credential Proof")]
	EmptyCredentialProof,
	#[error("Empty Credential Type")]
	EmptyCredentialType,
	#[error("Empty Credential Issuer")]
	EmptyCredentialIssuer,
	#[error("Empty Credential Subject")]
	EmptyCredentialSubject,
	#[error("Empty Issuance Date")]
	EmptyIssuanceDate,
	#[error("Pass Error: {0}")]
	ParseError(String),
	#[error("Runtime Error: {0}")]
	RuntimeError(String),
	#[error(transparent)]
	Json(#[from] serde_json::Error),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Context {
	#[serde(rename = "@context")]
	context: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(try_from = "String")]
#[serde(into = "String")]
pub struct VCDateTime {
	/// The date-time
	date_time: DateTime<FixedOffset>,
	/// Whether to use "Z" or "+00:00" when formatting the date-time in UTC
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

// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub enum CredentialType {
// 	VerifiableCredential,
// }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ProofType {
	Ed25519Signature2020,
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
		//let datetime = Utc::now();

		Proof {
			created: None,
			proof_type: type_,
			proof_purpose: PROOF_PURPOSE.to_string(),
			proof_value: "".to_owned(),
			verification_method: "".to_owned(),
		}
	}

	pub fn is_empty(&self) -> bool {
		self.proof_value.is_empty()
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ValidationResult {
	pub result: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Credential {
	#[serde(rename = "@context")]
	pub context: Context,
	pub id: String,
	pub subject: String,
	#[serde(rename = "type")]
	pub types: Vec<String>,
	pub credential_subject: CredentialSubject,
	pub issuer: Issuer,
	// rfc3339
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
		Ok(())
	}

	pub fn validate_unsigned(&self) -> Result<(), Error> {
		if !self.types.contains(&CREDENTIAL_TYPE.to_string()) {
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

	// pub fn sign_proof(&self) -> Result<Proof, Error> {}

	//pub fn validate_proof(&self) -> Result<ValidationResult, Error> {}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn eval_simple_success() {
		let data = r#"
        {
            "@context": [
                "https://www.w3.org/2018/credentials/v1", 
                "https://w3id.org/security/suites/ed25519-2020/v1"
            ], 
            "id": "http://litentry.com/2022/credentials/twitter/follower", 
            "type": [
                "VerifiableCredential"
            ], 
            "issuer": {
                "id": "did:litentry:7f8ca8982f6cc6e8ea087bd9457ab8024bd2", 
                "enclaveId": "enclave id or registered hash", 
                "name": "Litentry TEE Worker"
            }, 
            "subject": "did:litentry:owner's Litentry address", 
            "issuanceDate": "2022-09-01T12:01:20Z", 
            "expirationDate": "2022-09-14T12:01:20Z", 
            "credentialSubject": {
                "id": "did:litentry:97c30de767f084ce3080168ee293053ba33b235d71", 
                "description": "1000-2000 Twitter followers", 
                "type": "TwitterFollower", 
                "tag": [
                    "Twitter", 
                    "IDHub"
                ], 
                "dataSoure": [
                    {
                        "dataProvider": "https://litentry.com/endpoint/graphql", 
                        "dataProviderId": 1
                    }
                ], 
                "assertions": "return 1+20", 
                "values": [
                    true
                ], 
                "endpoint": "https://litentry.com/parachain/extrinsic"
            }, 
            "proof": {
                "created": "2022-09-01T12:01:20Z", 
                "type": "Ed25519Signature2020", 
                "proofPurpose": "assertionMethod", 
                "proofValue": "f66944a454904a19f30a2b045ea80534547ffb522cdf2f8d9b949c76331d9d2c8359c4668b0775362d697985f52645d2479fbde0792dacdad9fdea09c4120c0d", 
                "verificationMethod": "did:litentry:issuer's Litentry pubkey"
            }
        }
        "#;

		let vc: Credential = Credential::from_json(data).unwrap();
		println!("{:?}", vc);
		assert_eq!(vc.subject, "did:litentry:owner's Litentry address");
		assert_eq!(vc.proof.proof_purpose, "assertionMethod");
		assert_eq!(
			vc.credential_subject.id,
			"did:litentry:97c30de767f084ce3080168ee293053ba33b235d71"
		);
	}
}
