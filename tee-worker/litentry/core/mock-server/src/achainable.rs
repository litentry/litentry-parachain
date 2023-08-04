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
const RES_ERRBODY: &str = r#"Error request."#;

use lc_data_providers::achainable::ReqBody;
use warp::{http::Response, path::FullPath, Filter};

pub(crate) fn query() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post().and(warp::path::full()).and(warp::body::json()).map(
		|p: FullPath, body: ReqBody| {
			if p.as_str() != "/v1/run/system-labels" {
				return Response::builder().status(400).body(RES_ERRBODY.to_string())
			}

			// Total transaction
			if body.name == "Account total transactions under {amount}" {
				Response::builder().body(RES_TOTALTRANSACTIONS.to_string())
			} else {
				Response::builder().body(RES_BODY.to_string())
			}
		},
	)
}
