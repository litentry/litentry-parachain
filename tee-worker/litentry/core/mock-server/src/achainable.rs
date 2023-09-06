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
const RES_BODY: &str = r#"
{
	"result": true
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

use lc_data_providers::achainable::ReqBody;
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
			} else {
				Response::builder().body(RES_BODY.to_string())
			}
		},
	)
}
