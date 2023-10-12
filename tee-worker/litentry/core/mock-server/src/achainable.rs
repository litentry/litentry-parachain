// Copyright 2020-2023 Trust Computing GmbH.
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
#![allow(opaque_hidden_inferred_bound)]

const RES_TOTALTRANSACTIONS: &str = r#"
{
	"result": true,
	"display": [
		{
			"text": "Total transactions under 1 (Transactions: 41)",
			"result": true
		}
	],
	"runningCost": 1
}"#;
const RES_BODY_TRUE: &str = r#"
{
	"result": true
}"#;
const RES_BODY_FALSE: &str = r#"
{
	"result": false
}"#;
const RES_BODY_INVALID_CLASS_OF_YEAR: &str = r#"
{
    "name": "Account created between {dates}",
    "result": false,
    "display": [],
    "analyticsDisplay": [],
    "metadata": [
        null
    ],
    "runningCost": 1
}
"#;
const RES_BODY_OK_CLASS_OF_YEAR: &str = r#"
{
    "name": "Account created between {dates}",
    "result": true,
    "display": [
        {
            "text": "Account created between 01/01/2015 and 01/09/2023",
            "result": true
        }
    ],
    "analyticsDisplay": [],
    "metadata": [
        "2017-10-27T07:38:14.000Z"
    ],
    "runningCost": 1
}
"#;
const RES_ERRBODY: &str = r#"Error request."#;

use lc_data_providers::achainable::{Params, ReqBody};
use warp::{http::Response, path::FullPath, Filter};

pub(crate) fn query() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post().and(warp::path::full()).and(warp::body::json()).map(
		|p: FullPath, body: ReqBody| {
			if p.as_str() != "/v1/run/system-labels" {
				return Response::builder().status(400).body(RES_ERRBODY.to_string())
			}

			if body.name == "Account total transactions under {amount}" {
				// Total transaction
				Response::builder().body(RES_TOTALTRANSACTIONS.to_string())
			} else if body.name == "Account created between {dates}" {
				// Class of year invalid address
				if body.address == "0x06e23f8209eCe9a33E24fd81440D46B08517adb5" {
					Response::builder().body(RES_BODY_INVALID_CLASS_OF_YEAR.to_string())
				} else {
					Response::builder().body(RES_BODY_OK_CLASS_OF_YEAR.to_string())
				}
			}
			// HoldingTime
			else if body.name == "Balance hodling {amount} since {date}"
				|| body.name == "ERC20 hodling {amount} of {token} since {date}"
			{
				println!(">>> body: {:?}", body);

				let date = match body.params {
					Params::ParamsBasicTypeWithAmountHolding(a) => a.date,
					_ => "".to_string(),
				};

				if body.address == "11111111111111111111111111" && date == "2017-01-01"
					|| body.address == "333333333333333333" && date == "2018-07-01"
					|| body.address == "111111111111111111111111112" && date == "2019-01-01"
					|| body.address == "222222222222222222222223" && date == "2020-07-01"
					|| body.address == "111111111111111111111111114" && date == "2018-01-01"
					|| body.address == "222222222222222222222225" && date == "2022-07-01"
					|| body.address == "111111111111111111111111116" && date == "2023-01-01"
					|| body.address == "222222222222222222222227" && date == "2023-07-01"
				{
					Response::builder().body(RES_BODY_TRUE.to_string())
				} else if body.address == "22222222222222222222222"
					|| body.address == "4444444444444444444444"
					|| body.address == "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
				{
					Response::builder().body(RES_BODY_FALSE.to_string())
				} else if body.address == "0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84"
					|| body.address == "0x4b978322643F9Bf6C15bf26d866B81E99F26b8DA"
				{
					Response::builder().body(RES_BODY_TRUE.to_string())
				} else {
					Response::builder().body(RES_BODY_FALSE.to_string())
				}
			} else {
				Response::builder().body(RES_BODY_TRUE.to_string())
			}
		},
	)
}
