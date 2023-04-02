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

use std::collections::HashMap;
use warp::{http::Response, Filter};

pub(crate) fn check_follow(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get()
		.and(warp::path!("twitter" / "followers" / "verification"))
		.and(warp::query::<HashMap<String, String>>())
		.map(move |p: HashMap<String, String>| {
			let default = String::default();
			let handler1 = p.get("handler1").unwrap_or(&default);
			let handler2 = p.get("handler2").unwrap_or(&default);

			let body = r#"{ "data": false }"#;
			if handler1.as_str() == "litentry" && handler2 == "paritytech" {
				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}
