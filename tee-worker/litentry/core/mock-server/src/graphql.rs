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

use lc_data_providers::graphql::{
	VerifiedCredentialsIsHodlerIn, VerifiedCredentialsNetwork, VerifiedCredentialsTotalTxs,
};
use std::collections::HashMap;
use warp::{http::Response, Filter};

pub(crate) fn query() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get()
		.and(warp::path!("latest" / "graphql"))
		.and(warp::query::<HashMap<String, String>>())
		.map(|p: HashMap<String, String>| {
			let default = String::default();
			let query = p.get("query").unwrap_or(&default);

			let expected_query_total_txs = VerifiedCredentialsTotalTxs::new(
				vec!["EGP7XztdTosm1EmaATZVMjSWujGEj9nNidhjqA2zZtttkFg".to_string()],
				vec![VerifiedCredentialsNetwork::Kusama, VerifiedCredentialsNetwork::Polkadot],
			)
			.to_graphql();

			let expected_query_is_hodler = VerifiedCredentialsIsHodlerIn::new(
				vec![
					"0x61f2270153bb68dc0ddb3bc4e4c1bd7522e918ad".to_string(),
					"0x3394caf8e5ccaffb936e6407599543af46525e0b".to_string(),
				],
				"2022-10-16T00:00:00Z".to_string(),
				VerifiedCredentialsNetwork::Ethereum,
				"0xb59490aB09A0f526Cc7305822aC65f2Ab12f9723".to_string(),
				"0.00000056".into(),
			)
			.to_graphql();

			if query == &expected_query_total_txs {
				let body = r#"
{
	"data": {
		"kusama": [
			{
				"address": "EGP7XztdTosm1EmaATZVMjSWujGEj9nNidhjqA2zZtttkFg",
				"totalTransactions": 42
			}
		],
		"polkadot": [
			{
				"address": "EGP7XztdTosm1EmaATZVMjSWujGEj9nNidhjqA2zZtttkFg",
				"totalTransactions": 0
			}
		]
	}
}"#;
				Response::builder().body(body.to_string())
			} else if query == &expected_query_is_hodler {
				let body = r#"
{
  "data": {
    "VerifiedCredentialsIsHodler": [
      {
        "isHodler": false,
        "address": "0x61f2270153bb68dc0ddb3bc4e4c1bd7522e918ad"
      },
      {
        "isHodler": false,
        "address": "0x2eD157cd084Cee5861BdCC773c89881DdA373693"
      }
    ]
  }
}"#;
				Response::builder().body(body.to_string())
			} else {
				Response::builder().status(400).body(String::from("Error query"))
			}
		})
}
