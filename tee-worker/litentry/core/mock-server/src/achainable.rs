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
	use warp::{http::Response, path::FullPath, Filter};

	pub(crate) fn query() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
	{
		let res_body = r#"
		{
			"result": true
		}"#;

		warp::post().and(warp::path::full()).and(warp::body::json()).map(
			|_p: FullPath, body: ReqBody| {
				match body.params {
					Params::ParamsBasicTypeWithAmount(_) => {
						// Total transaction
						if body.name == "Account total transactions under {amount}" {
							// Total transactions
							let total_txs = r#"
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

							return Response::builder().body(total_txs.to_string())
						} else {
							return Response::builder().body(res_body.to_string())
						};
					},
					_ => return Response::builder().body(res_body.to_string()),
				};
			},
		)
	}
}
