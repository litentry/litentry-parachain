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

pub mod tag {
	use lc_data_providers::achainable::{Params, ReqBody};
	use std::collections::HashMap;
	use warp::{http::Response, path::FullPath, Filter};

	pub(crate) fn query() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
	{
		let body_false = r#"
		{
			"result": false
		}"#;

		let body_true = r#"
		{
			"result": true
		}"#;

		warp::post()
			.and(warp::query::<HashMap<String, String>>())
			.and(warp::path::full())
			.and(warp::body::json())
			.and(warp::body::content_length_limit(1024 * 16))
			.map(|_q: HashMap<_, _>, p: FullPath, body: ReqBody| {
				let path = p.as_str();
				let name = body.name;
				let address = body.address;

				match body.params {
					Params::AmountHoding(a) => {},
					Params::ClassOfYear(c) => {},
					Params::EthDrainedInLastFortnight(e) => {},

					Params::ParamsBasicType(a) => {},
					Params::ParamsBasicTypeWithAmount(a) => {},
					Params::ParamsBasicTypeWithAmounts(a) => {},
					Params::ParamsBasicTypeWithDate(a) => {},
					Params::ParamsBasicTypeWithAmountToken(a) => {},
					Params::ParamsBasicTypeWithBetweenPercents(a) => {},
					Params::ParamsBasicTypeWithDateInterval(_) => {},
					Params::ParamsBasicTypeWithToken(_) => {},
				}

				if path == "/v1/run/labels/74655d14-3abd-4a25-b3a4-cd592ae26f4c" {
					// total transactions
					let total_txs = r#"
					{
						"label": {
							"result": true,
							"display": [
								{
									"text": "Total transactions under 1 (Transactions: 41)",
									"result": true
								}
							],
							"runningCost": 1
						}
					}"#;

					return Response::builder().body(total_txs.to_string())
				}

				Response::builder().status(400).body(String::from("Error query"))
			})
	}
}
