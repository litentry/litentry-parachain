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

use std::collections::HashMap;

use lc_data_providers::magic_craft::UserVerificationResponse;

use warp::{http::Response, Filter};

pub(crate) fn query() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get()
		.and(warp::path!("magic_craft" / "litentry" / "user"))
		.and(warp::query::<HashMap<String, String>>())
		.map(move |p: HashMap<String, String>| {
			let default = String::default();
			let address = p.get("wallet_address").unwrap_or(&default);

			if address == "0x49ad262c49c7aa708cc2df262ed53b64a17dd5ee" {
				let body = UserVerificationResponse { user: true };
				Response::builder().body(serde_json::to_string(&body).unwrap())
			} else {
				let body = UserVerificationResponse { user: false };
				Response::builder().body(serde_json::to_string(&body).unwrap())
			}
		})
}
