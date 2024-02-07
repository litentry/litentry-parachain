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

use lc_data_providers::vip3::{LevelEntity, VIP3SBTInfoResponse};
use std::collections::HashMap;
use warp::{http::Response, Filter};
pub(crate) fn query_user_sbt_level(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get()
		.and(warp::path!("api" / "v1" / "sbt" / "info"))
		.and(warp::query::<HashMap<String, String>>())
		.map(move |p: HashMap<String, String>| {
			let default = String::default();
			let account = p.get("addr").unwrap_or(&default);
			if account == "0xff93b45308fd417df303d6515ab04d9e89a750ca" {
				let body = VIP3SBTInfoResponse {
					code: 0,
					msg: "success".to_string(),
					data: LevelEntity { level: 1 },
				};
				Response::builder().body(serde_json::to_string(&body).unwrap())
			} else {
				let body = VIP3SBTInfoResponse {
					code: 0,
					msg: "success".to_string(),
					data: LevelEntity { level: 0 },
				};
				Response::builder().body(serde_json::to_string(&body).unwrap())
			}
		})
}
