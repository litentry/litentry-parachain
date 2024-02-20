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

use lc_data_providers::moralis::{GetNftsByWalletResult, MoralisPageResponse};

use warp::{http::Response, Filter};

pub(crate) fn query() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get()
		.and(warp::path!("moralis" / String / "nft"))
		.and(warp::query::<HashMap<String, String>>())
		.map(move |address, _| {
			if address == "0x49ad262c49c7aa708cc2df262ed53b64a17dd5ee" {
				let body = MoralisPageResponse::<GetNftsByWalletResult> {
					status: "SYNCED".into(),
					cursor: Some("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9".into()),
					page: 1,
					page_size: 100,
					result: vec![GetNftsByWalletResult {
						amount: "1".into(),
						token_id: "5021".into(),
						token_address: "0xfff54e6fe44fd47c8814c4b1d62c924c54364ad3".into(),
						contract_type: "ERC721".into(),
						owner_of: "0xff3879b8a363aed92a6eaba8f61f1a96a9ec3c1e".into(),
					}],
				};
				Response::builder().body(serde_json::to_string(&body).unwrap())
			} else {
				let body = MoralisPageResponse::<GetNftsByWalletResult> {
					status: "SYNCED".into(),
					cursor: None,
					page: 1,
					page_size: 100,
					result: vec![],
				};
				Response::builder().body(serde_json::to_string(&body).unwrap())
			}
		})
}
