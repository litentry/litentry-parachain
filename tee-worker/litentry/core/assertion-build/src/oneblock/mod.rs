#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use crate::*;
use http_req::response::Headers;
use itc_rest_client::{error::Error as RestClientError, RestGet, RestPath};
use lc_data_providers::build_client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OneBlockResponse {
	#[serde(flatten)]
	data: serde_json::Value,
}

impl RestPath<String> for OneBlockResponse {
	fn get_path(path: String) -> core::result::Result<String, RestClientError> {
		Ok(path)
	}
}
pub fn build(req: &AssertionBuildRequest) -> Result<Credential> {
	match Credential::new(&req.who, &req.shard) {
		Ok(mut credential_unsigned) => {
			let mut headers = Headers::new();
			headers.insert(
				"Authorization",
				"Bearer secret_Lq3i0MshwhytNjIYulKKvOfHrnTaUYQFinL3csCIyjw",
			);
			headers.insert("Notion-Version", "2022-06-28");
			let mut client = build_client(
		"https://api.notion.com/v1/blocks/32d13293a4dc46d4b042ee7b18189569/children?page_size=100",
		headers,
	);
			let get_response = client.get::<String, OneBlockResponse>(String::default()).unwrap();
			debug!("get_response: {:?}", get_response);
			// add subject info

			Ok(credential_unsigned)
		},
		Err(e) => {
			error!("Generate unsigned credential failed {:?}", e);
			Err(Error::RequestVCFailed(Assertion::Oneblock, e.into_error_detail()))
		},
	}
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	pub fn get_notion() {
		let mut headers = Headers::new();
		headers
			.insert("Authorization", "Bearer secret_Lq3i0MshwhytNjIYulKKvOfHrnTaUYQFinL3csCIyjw");
		headers.insert("Notion-Version", "2022-06-28");
		let mut client = build_client(
	"https://api.notion.com/v1/blocks/32d13293a4dc46d4b042ee7b18189569/children?page_size=100",
	headers,
);
		let get_response = client.get::<String, OneBlockResponse>(String::default()).unwrap();
		let results = &get_response.data["results"];
		let results_len = results.as_array().unwrap().len();
		println!("get_response: {:?}", results[0]["table_row"]["cells"]);
		for it in results.as_array().unwrap().iter() {
			let column_len = it["table_row"]["cells"].as_array().unwrap().len();
			for i in 0..column_len {
				println!("get_response: {:?}", it["table_row"]["cells"][i][0]["plain_text"]);
			}
		}

		assert_eq!(results.as_array().unwrap().len(), 3);
	}
}
