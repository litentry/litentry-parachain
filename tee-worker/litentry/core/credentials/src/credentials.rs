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

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

//proof_purpose
pub const PROOF_PURPOSE: &str = "assertionMethod";

#[derive(Debug, thiserror::Error, Clone)]
pub enum Error {
	EmptyProof,
	EmptyCredentialType,
	EmptyIssuer,
	EmptyCredentialSubject,
	EmptyIssuanceDate,
	ParseError(String),
	RuntimeError(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Context {
	Vec<String>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CredentialType {
	VerifiableCredential,
}

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
	pub created: DateTime<Utc>,
	#[serde(rename = "type")]
	pub proof_type: ProofType,
	pub proof_purpose: String,
	pub proof_value: String,
	pub verification_method: String,
}

impl Proof {
	pub fn new(type_: ProofType) -> Self {
		Self {
			None,
			type_,
			PROOF_PURPOSE.to_string(),
			None,
			None,
		}
	}

	pub fn is_empty(&self) -> bool {
		self.proof_value.is_empty()
	}
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ValidationResult {
	pub result: bool
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Credential {
	#[serde(rename = "@context")]
    pub context: Context,
	pub id: String,
	pub subject: String,
	#[serde(rename = "type")]
	pub types: Vec<CredentialType>,
	pub credential_subject: CredentialSubject,
	pub issuer: Issuer,
	// rfc3339
	pub issuance_date: DateTime<Utc>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub expiration_date: Option<DateTime<Utc>>,
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
            return Err(Error::EmptyProof);
        }
        Ok(())
    }

	pub fn validate_unsigned(&self) -> Result<(), Error> {
        if !self.types.contains(&"VerifiableCredential".to_string()) {
            return Err(Error::EmptyCredentialType);
        }

		if self.issuer.is_empty() {
            return Err(Error::EmptyIssuer);
        }

		if self.credential_subject.is_empty() {
            return Err(Error::EmptyCredentialSubject);
        }

		if self.issuance_date.is_empty() {
            return Err(Error::EmptyIssuanceDate);
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

		let vc: VC = Credential::from_json(data).unwrap();
		println!("{:?}", vc);
		assert_eq!(vc.subject, "did:litentry:owner's Litentry address");
		assert_eq!(vc.proof.proof_purpose, "assertionMethod");
		assert_eq!(
			vc.credential_subject.id,
			"did:litentry:97c30de767f084ce3080168ee293053ba33b235d71"
		);
	}
}
