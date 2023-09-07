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

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

pub mod course;

use crate::*;
use http::header::{AUTHORIZATION, CONNECTION};
use http_req::response::Headers;
use itc_rest_client::{error::Error as RestClientError, RestGet, RestPath};
use lc_data_providers::{build_client, GLOBAL_DATA_PROVIDER_CONFIG};
use serde::{Deserialize, Serialize};
use std::string::ToString;

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

fn fetch_data_from_notion(course_type: &OneBlockCourseType) -> Result<OneBlockResponse> {
	let mut headers = Headers::new();
	headers.insert(CONNECTION.as_str(), "close");
	headers.insert("Notion-Version", "2022-06-28");
	headers.insert(
		AUTHORIZATION.as_str(),
		GLOBAL_DATA_PROVIDER_CONFIG.read().unwrap().oneblock_notion_key.clone().as_str(),
	);

	let mut client = build_client(
		GLOBAL_DATA_PROVIDER_CONFIG.read().unwrap().oneblock_notion_url.clone().as_str(),
		headers,
	);

	client.get::<String, OneBlockResponse>(String::default()).map_err(|e| {
		Error::RequestVCFailed(
			Assertion::Oneblock(course_type.clone()),
			ErrorDetail::DataProviderError(ErrorString::truncate_from(
				format!("{e:?}").as_bytes().to_vec(),
			)),
		)
	})
}

fn oneblock_course_result(
	data: &serde_json::Value,
	course_type: &OneBlockCourseType,
	addresses: Vec<String>,
) -> Result<bool> {
	let get_results = |data: &serde_json::Value| -> Option<Vec<serde_json::Value>> {
		data.get("results").and_then(|results| results.as_array()).cloned()
	};

	// A object contains 6 rows of content
	let get_table_row = |object: &serde_json::Value| -> Option<Vec<serde_json::Value>> {
		object
			.get("table_row")
			.and_then(|data| data.get("cells"))
			.and_then(|data| data.as_array())
			.filter(|data| data.len() == 6)
			.cloned()
	};

	let get_cell_text = |cells: &serde_json::Value| -> String {
		cells
			.as_array()
			.filter(|cells| cells.len() == 1)
			.and_then(|cell_data| {
				cell_data[0].get("text").and_then(|data| {
					data.get("content").and_then(|data| data.as_str()).map(|data| data.to_string())
				})
			})
			.unwrap_or_default()
	};

	// If the first colume content of an table cell is not a numeric digit, then this is a invalid cell, no need to parse data.
	// Using this method to differentiate student data.
	let is_student_info_cell = |cells: &Vec<serde_json::Value>| -> bool {
		get_cell_text(&cells[0]).parse::<u32>().is_ok()
	};

	let get_student_address =
		|cells: &Vec<serde_json::Value>| -> String { get_cell_text(&cells[2]) };

	let level = |cells: &Vec<serde_json::Value>, course_type: &OneBlockCourseType| -> bool {
		let cell_index: usize = match course_type {
			OneBlockCourseType::CourseCompletion => 4,
			OneBlockCourseType::CourseExcellenceCompletion => 5,
			OneBlockCourseType::CourseParticipation => 3,
		};
		let text = get_cell_text(&cells[cell_index]);

		// TODO: HOW TO CHECK IF THIS TYPE OF COURSE VC IS VALID???
		// JUST CHECK IF THIS BLANK IS EMPTY OR NOT???
		!text.is_empty()
	};

	if let Some(results) = get_results(data) {
		for object in results.iter() {
			if let Some(cells) = get_table_row(object) {
				if is_student_info_cell(&cells) && addresses.contains(&get_student_address(&cells))
				{
					return Ok(level(&cells, course_type))
				}
			}
		}
	}

	Ok(false)
}

pub fn query_oneblock_status(
	course_type: &OneBlockCourseType,
	addresses: Vec<String>,
) -> Result<bool> {
	let oneblock_response = fetch_data_from_notion(course_type)?;
	debug!("OneBlock Assertion Response: {oneblock_response:?}");

	oneblock_course_result(&oneblock_response.data, course_type, addresses)
}

#[cfg(test)]
mod tests {
	use super::*;

	const RESPONSE_ONEBLOCK: &str = r#"
{"object":"list","results":[{"object":"block","id":"8b371d31-1e60-4060-9f63-3527fa8cfe0c","parent":{"type":"block_id","block_id":"e4068e6a-3262-4346-8f35-dcdc0c43f111"},"created_time":"2023-09-05T03:21:00.000Z","last_edited_time":"2023-09-05T04:11:00.000Z","created_by":{"object":"user","id":"03bab8a4-8794-44d8-8843-961ee4c33485"},"last_edited_by":{"object":"user","id":"c208e40c-306a-4b31-a249-49139ff24411"},"has_children":false,"archived":false,"type":"table_row","table_row":{"cells":[[{"type":"text","text":{"content":"学号","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"学号","href":null}],[{"type":"text","text":{"content":"姓名","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"姓名","href":null}],[{"type":"text","text":{"content":"substrate地址","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"substrate地址","href":null}],[{"type":"text","text":{"content":"课程观看进度","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"课程观看进度","href":null}],[{"type":"text","text":{"content":"是否毕业","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"是否毕业","href":null}],[{"type":"text","text":{"content":"是否优秀毕业","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"是否优秀毕业","href":null}]]}},{"object":"block","id":"00d4a220-db1f-4d24-8b4f-4d3815dab3c5","parent":{"type":"block_id","block_id":"e4068e6a-3262-4346-8f35-dcdc0c43f111"},"created_time":"2023-09-05T03:21:00.000Z","last_edited_time":"2023-09-05T04:11:00.000Z","created_by":{"object":"user","id":"03bab8a4-8794-44d8-8843-961ee4c33485"},"last_edited_by":{"object":"user","id":"c208e40c-306a-4b31-a249-49139ff24411"},"has_children":false,"archived":false,"type":"table_row","table_row":{"cells":[[{"type":"text","text":{"content":"Team1","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"Team1","href":null}],[],[],[],[],[]]}},{"object":"block","id":"71130f91-0326-468a-8c42-c64c17e779af","parent":{"type":"block_id","block_id":"e4068e6a-3262-4346-8f35-dcdc0c43f111"},"created_time":"2023-09-05T03:21:00.000Z","last_edited_time":"2023-09-05T06:25:00.000Z","created_by":{"object":"user","id":"03bab8a4-8794-44d8-8843-961ee4c33485"},"last_edited_by":{"object":"user","id":"c208e40c-306a-4b31-a249-49139ff24411"},"has_children":false,"archived":false,"type":"table_row","table_row":{"cells":[[{"type":"text","text":{"content":"1264","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"1264","href":null}],[{"type":"text","text":{"content":"Student1","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"Student1","href":null}],[{"type":"text","text":{"content":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQQ","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQQ","href":null}],[{"type":"text","text":{"content":"♥","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"♥","href":null}],[],[]]}},{"object":"block","id":"72cdaf64-9dc5-4e71-8285-bba8d03abd99","parent":{"type":"block_id","block_id":"e4068e6a-3262-4346-8f35-dcdc0c43f111"},"created_time":"2023-09-05T03:21:00.000Z","last_edited_time":"2023-09-05T04:11:00.000Z","created_by":{"object":"user","id":"03bab8a4-8794-44d8-8843-961ee4c33485"},"last_edited_by":{"object":"user","id":"c208e40c-306a-4b31-a249-49139ff24411"},"has_children":false,"archived":false,"type":"table_row","table_row":{"cells":[[{"type":"text","text":{"content":"1263","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"1263","href":null}],[{"type":"text","text":{"content":"Clement Tam","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"Clement Tam","href":null}],[{"type":"text","text":{"content":"5HYaWcUJvX1xjNnduouJnD3F5q5X7uCpGxHV2yCRZEurymEE","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"5HYaWcUJvX1xjNnduouJnD3F5q5X7uCpGxHV2yCRZEurymEE","href":null}],[],[],[]]}},{"object":"block","id":"42d0ada7-1e3d-493a-bd2f-17821d2e54a2","parent":{"type":"block_id","block_id":"e4068e6a-3262-4346-8f35-dcdc0c43f111"},"created_time":"2023-09-05T03:21:00.000Z","last_edited_time":"2023-09-05T04:11:00.000Z","created_by":{"object":"user","id":"03bab8a4-8794-44d8-8843-961ee4c33485"},"last_edited_by":{"object":"user","id":"c208e40c-306a-4b31-a249-49139ff24411"},"has_children":false,"archived":false,"type":"table_row","table_row":{"cells":[[{"type":"text","text":{"content":"1262","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"1262","href":null}],[{"type":"text","text":{"content":"Student2","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"Student2","href":null}],[{"type":"text","text":{"content":"12zh1QyBrqddzgLbxBHjmoCnna8XsT9pQ8MPCUt1f7WtC1f5","link":null},"annotations":{"bold":false,"italic":false,"strikethrough":false,"underline":false,"code":false,"color":"default"},"plain_text":"12zh1QyBrqddzgLbxBHjmoCnna8XsT9pQ8MPCUt1f7WtC1f5","href":null}],[],[],[]]}}],"next_cursor":null,"has_more":false,"type":"block","block":{}}
"#;

	#[test]
	fn oneblock_course_result_works() {
		let address = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQQ".to_string();
		let oneblock_response: serde_json::Value = serde_json::from_str(RESPONSE_ONEBLOCK).unwrap();

		let value = oneblock_course_result(
			&oneblock_response,
			&OneBlockCourseType::CourseParticipation,
			vec![address],
		);
		assert!(value.is_ok());
		assert_eq!(value.unwrap(), true);
	}
}
