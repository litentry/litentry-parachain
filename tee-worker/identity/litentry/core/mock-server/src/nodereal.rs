// Copyright 2020-2024 Trust Computing GmbH.
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

use warp::{http::Response, Filter};

const RESPONSE_BNB_DOMAIN: &str = r#"{
					"nodeHash": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"bind": "0xr4b0bf28adfcee93c5069982a895785c9231c1fe",
					"name": "8",
					"expires": "2028-09-18T13:35:38Z"
				}
                "#;
pub(crate) fn query() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::post()
		.and(warp::path!(String / "spaceid" / "domain" / "names" / String))
		.map(|_: String, _: String| Response::builder().body(RESPONSE_BNB_DOMAIN.to_string()))
}
