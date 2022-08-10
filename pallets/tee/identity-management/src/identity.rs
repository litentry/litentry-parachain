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

// TODO: maybe move this file to the tee-worker repo as it's only used by TEE-worker
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};

#[derive(Clone, Encode, Decode, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Web2ValidationData {
	pub link: String,
}

#[derive(Clone, Encode, Decode, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Web3ValidationData {
	pub message: String,
	pub signature: String,
	pub timestamp: String,
}

#[derive(Clone, Encode, Decode, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
	pub did: String,
	pub metadata: Option<String>,
	#[serde(default)]
	pub need_verification: bool,
	pub web2_validation_data: Option<Web2ValidationData>,
	pub web3_validation_data: Option<Web3ValidationData>,
}

#[cfg(test)]
mod tests {
	use super::Identity;
	#[test]
	fn identity_full_json_parsed_correctly() {
		let id: Identity = serde_json::from_str(
			r#"
		{
			"did": "did:polkadot:web3:substrate:0x1234",
			"metadata": "0xabc",
			"needVerification": true,
			"web2ValidationData": {
                "link": "www.litentry.com"
            },
            "web3ValidationData": {
               "message": "this is a message",
               "signature": "this is a signature",
               "timestamp": "this is timestamp"
            }
		}
		"#,
		)
		.unwrap();
		assert!(id.need_verification);
		assert_eq!(id.metadata.unwrap(), String::from("0xabc"));
		assert_eq!(id.web2_validation_data.unwrap().link, String::from("www.litentry.com"));
	}

	#[test]
	fn identity_partial_json_parsed_correctly() {
		let id: Identity = serde_json::from_str(
			r#"
		{
			"did": "did:polkadot:web3:substrate:0x1234"
		}
		"#,
		)
		.unwrap();
		assert!(!id.need_verification);
		assert!(id.metadata.is_none());
		assert!(id.web2_validation_data.is_none());
		assert!(id.web3_validation_data.is_none());
	}
}
