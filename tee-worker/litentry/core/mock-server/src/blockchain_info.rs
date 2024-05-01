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

use lc_data_providers::blockchain_info::{
	GetMultiAddressesResponse, GetMultiAddressesResponseWallet, GetSingleAddressResponse,
};

use warp::{http::Response, Filter};

pub(crate) fn query_rawaddr(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get()
		.and(warp::path!("blockchain_info" / "rawaddr" / String))
		.and(warp::query::<HashMap<String, String>>())
		.map(move |address, _| {
			if address == "bc1pgr5fw4p9gl9me0vzjklnlnap669caxc0gsk4j62gff2qktlw6naqm4m3d0" {
				let body = GetSingleAddressResponse { final_balance: 185123167511 };
				Response::builder().body(serde_json::to_string(&body).unwrap())
			} else {
				let body = GetSingleAddressResponse { final_balance: 0 };
				Response::builder().body(serde_json::to_string(&body).unwrap())
			}
		})
}

pub(crate) fn query_multiaddr(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get()
		.and(warp::path!("blockchain_info" / "multiaddr"))
		.and(warp::query::<HashMap<String, String>>())
		.map(move |_| {
			let body = GetMultiAddressesResponse {
				wallet: GetMultiAddressesResponseWallet { final_balance: 185123167511 },
			};
			Response::builder().body(serde_json::to_string(&body).unwrap())
		})
}
