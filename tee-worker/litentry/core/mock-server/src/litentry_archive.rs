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

use std::collections::HashMap;
use warp::{http::Response, Filter};

pub(crate) fn query_user_joined_evm_campaign(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get()
		.and(warp::path!("events" / "does-user-joined-evm-campaign"))
		.and(warp::query::<HashMap<String, String>>())
		.map(move |p: HashMap<String, String>| {
			let default = String::default();
			let account = p.get("account").unwrap_or(&default);
			if account == "0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d" {
				let body = r#"{ "hasJoined": true }"#;
				Response::builder().body(body.to_string())
			} else {
				let body = r#"{ "hasJoined": false }"#;
				Response::builder().body(body.to_string())
			}
		})
}
