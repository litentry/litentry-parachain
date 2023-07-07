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

use lc_data_providers::achainable::{
	ToAchainable, VerifiedCredentialsIsHodlerIn, VerifiedCredentialsTotalTxs,
};
use litentry_primitives::SupportedNetwork;
use std::collections::HashMap;
use warp::{http::Response, Filter};

pub(crate) fn query() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	warp::get()
		.and(warp::path!("latest" / "achainable"))
		.and(warp::query::<HashMap<String, String>>())
		.map(|p: HashMap<String, String>| {
			let default = String::default();
			let query = p.get("query").unwrap_or(&default);

			let expected_query_total_txs = VerifiedCredentialsTotalTxs::new(
				vec!["EGP7XztdTosm1EmaATZVMjSWujGEj9nNidhjqA2zZtttkFg".to_string()],
				vec![SupportedNetwork::Kusama, SupportedNetwork::Polkadot],
			)
			.to_achainable();

			let expected_query_is_hodler = VerifiedCredentialsIsHodlerIn::new(
				vec![
					"0x61f2270153bb68dc0ddb3bc4e4c1bd7522e918ad".to_string(),
					"0x3394caf8e5ccaffb936e6407599543af46525e0b".to_string(),
				],
				"2022-10-16T00:00:00Z".to_string(),
				SupportedNetwork::Ethereum,
				"0xb59490aB09A0f526Cc7305822aC65f2Ab12f9723".to_string(),
				"0.00000056".into(),
			)
			.to_achainable();

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

pub mod tag {
	use lc_data_providers::achainable::ReqBody;
	use std::collections::HashMap;
	use warp::{http::Response, path::FullPath, Filter};

	pub(crate) fn query() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
	{
		warp::post()
			.and(warp::query::<HashMap<String, String>>())
			.and(warp::path::full())
			.and(warp::body::json())
			.and(warp::body::content_length_limit(1024 * 16))
			.map(|_q: HashMap<_, _>, p: FullPath, body: ReqBody| {
				let path = p.as_str();
				let address = body.params.address;
				println!(">>>path: {path}");

				let body_false = r#"
		{
			"result": false
		}"#;

				let body_true = r#"
		{
			"result": true
		}"#;

				// false
				if (path == "/v1/run/label/1de85e1d215868788dfc91a9f04d7afd"
					&& address == *"0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5".to_string())
					|| (path == "/v1/run/label/8a6e26b90dee869634215683ea2dad0d"
						&& address == *"0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5".to_string())
					|| (path == "/v1/run/label/9343efca78222a4fad82c635ab697ca0"
						&& address == *"0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5".to_string())
					|| (path == "/v1/run/label/6808c28c26908eb695f63b089cfdae80"
						&& address == *"0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5".to_string())
					|| (path == "/v1/run/label/6a0424e7544696a3e774dfc7e260dd6e"
						&& address
							== *"1soESeTVLfse9e2G8dRSMUyJ2SWad33qhtkjQTv9GMToRvP".to_string())
					|| (path == "/v1/run/label/68390df24e8ac5d0984a8e9c0725a964"
						&& address
							== *"1soESeTVLfse9e2G8dRSMUyJ2SWad33qhtkjQTv9GMToRvP".to_string())
					|| (path == "/v1/run/label/2bf33f5b3ae60293bf93784b80251129"
						&& address
							== *"CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq".to_string())
					|| (path == "/v1/run/label/f55a4c5a19b6817ad4faf90385f4df6e"
						&& address == *"0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84".to_string())
					|| (path == "/v1/run/label/0f26a13d7ff182641f9bb9168a3f1d84"
						&& address == *"0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84".to_string())
					|| (path == "/v1/run/label/3f0469170cd271ebaac4ed2c92754479"
						&& address == *"0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84".to_string())
					|| (path == "/v1/run/label/0dc166e3b588fb45a9cca36c60c61f79"
						&& address == *"0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84".to_string())
					|| (path == "/v1/run/label/d4748f8b162a78a195cbbc6669333545"
						&& address
							== *"5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW".to_string())
					|| (path == "/v1/run/label/d7cf879652ea3bcab1c043828f4d4478"
						&& address
							== *"CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq".to_string())
					|| (path == "/v1/run/label/fbf7f158c78d7eb95cb872b1a8d5fe07"
						&& address
							== *"5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW".to_string())
					|| (path == "/v1/run/label/1181944a66c746042c2914080eb7155b"
						&& address
							== *"CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq".to_string())
					|| (path == "/v1/run/label/1829e887a62fa97113dd0cee977aa8d5"
						&& address
							== *"5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW".to_string())
					|| (path == "/v1/run/label/3564145e6ca3f13185b2cd1490db65fc"
						&& address
							== *"CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq".to_string())
					|| (path == "/v1/run/label/ce5e845483b2fcbe42021ff91198b92b"
						&& address
							== *"5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW".to_string())
					|| (path == "/v1/run/label/ee1a4e4a1e3e63e3e9d1c5af1674e15b"
						&& address
							== *"CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq".to_string())
					|| (path == "/v1/run/label/5c1a272ce054e729f1eca5c5a47bcbdd"
						&& address
							== *"5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW".to_string())
					|| (path == "/v1/run/label/4aa1a72b5d1fae6dd0417671193fffe1"
						&& address
							== *"CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq".to_string())
					|| (path == "/v1/run/label/9b03668237a0a4a7bbdd45c839dbb0fd"
						&& address
							== *"5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW".to_string())
					|| (path == "/v1/run/label/91b0529b323d6c1207dc601d0f677414"
						&& address
							== *"CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq".to_string())
					|| (path == "/v1/run/label/8cb42563adaacf8fd4609d6641ce7670"
						&& address
							== *"5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW".to_string())
					|| (path == "/v1/run/label/50e6094ebf3df2e8bf2d2b41b2737ba0"
						&& address
							== *"CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq".to_string())
					|| (path == "/v1/run/label/e5bcdbdb20c07ffd9ff68ce206fb64d5"
						&& address
							== *"5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW".to_string())
					|| (path == "/v1/run/label/a27e414ae882a5e5b291b437376e266a"
						&& address
							== *"CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq".to_string())
					|| (path == "/v1/run/label/1f39ff71595b1f0ff9f196b8f64f04e3"
						&& address
							== *"5CAGKg1NAArpEgze7F7KEnw8T2uFVcAWf6mJNTWeg9PWkdVW".to_string())
					|| (path == "/v1/run/label/6ecc10647157f1c34fe7d3734ba3d89f"
						&& address
							== *"CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq".to_string())
					|| (path == "/v1/run/label/17769b1442bb26a1604c85ad49045f1b"
						&& address == *"0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b".to_string())
					|| (path == "/v1/run/label/aa7d5d57430bfb68708417aca6fa2e16"
						&& address == *"0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84".to_string())
					|| (path == "/v1/run/label/05265d4009703337e7a57764b09d39d2"
						&& address == *"0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84".to_string())
					|| (path == "/v1/run/label/1dcd359e078fb8fac92b76d2e9d720c8"
						&& address == *"0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84".to_string())
					|| (path == "/v1/run/label/5a2a14a93b7352e93a6cf84a460c2c50"
						&& address == *"0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84".to_string())
					|| (path == "/v1/run/label/b9256d66b76ad62b9ec25f27775e6d83"
						&& address == *"0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b".to_string())
					|| (path == "/v1/run/label/2f0470c59799e58f91929678d2a62c2b"
						&& address == *"0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b".to_string())
					|| (path == "/v1/run/label/c090d9694c902141673c85a8f64d7f78"
						&& address == *"0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b".to_string())
					|| (path == "/v1/run/label/054625c2a49a73876831b797c5c41cd3"
						&& address == *"0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b".to_string())
					|| (path == "/v1/run/label/7d7c271af78ebf94d7f3b1ff6df30142"
						&& address == *"0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b".to_string())
					|| (path == "/v1/run/label/2c3d7189e1783880916cc56a1277cb13"
						&& address == *"0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b".to_string())
					|| (path == "/v1/run/label/112860d373ee427d80b2d687ca54dc8e"
						&& address == *"0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b".to_string())
				{
					return Response::builder().body(body_false.to_string())
				}

				// true
				if (path == "/v1/run/label/a4ee0c9e44cbc7b8a4b2074b3b8fb912"
					&& address == *"0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5".to_string())
					|| (path == "/v1/run/label/3ace29836b372ae66a218dec16e37b62"
						&& address == *"0x3f349bBaFEc1551819B8be1EfEA2fC46cA749aA1".to_string())
					|| (path == "/v1/run/label/eb66927e8f56fd7f9a8917d380e6100d"
						&& address
							== *"17bR6rzVsVrzVJS1hM4dSJU43z2MUmz7ZDpPLh8y2fqVg7m".to_string())
					|| (path == "/v1/run/label/a0d213ff009e43b4ecd0cae67bbabae9"
						&& address
							== *"ESRBbWstgpPV1pVBsqjMo717rA8HLrtQvEUVwAGeFZyKcia".to_string())
					|| (path == "/v1/run/label/3e226ee1bfb0d33564efe7f28f5015bd"
						&& address
							== *"CsCrPSvLBPSSxJuQmDr18FFZPAQCtKVmsMu9YRTe5VToGeq".to_string())
					|| (path == "/v1/run/label/fee8171e2001d1605e018c74f64352da"
						&& address == *"0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84".to_string())
					|| (path == "/v1/run/label/4a35e107005f1ea4077f119c10d18503"
						&& address == *"0x2A038e100F8B85DF21e4d44121bdBfE0c288A869".to_string())
					|| (path == "/v1/run/label/8657c801983aed40012e387900d75726"
						&& address == *"0x4b978322643F9Bf6C15bf26d866B81E99F26b8DA".to_string())
					|| (path == "/v1/run/label/e4724ad5b7354ef85332887ee7852800"
						&& address == *"0x082aB5505CdeA46caeF670754E962830Aa49ED2C".to_string())
					|| (path == "/v1/run/label/83d16c4c31c55ae535472643e63f49ce"
						&& address == *"0x4b978322643F9Bf6C15bf26d866B81E99F26b8DA".to_string())
					|| (path == "/v1/run/label/53b54e51090a3663173c2a97039ebf69"
						&& address == *"0x4b978322643F9Bf6C15bf26d866B81E99F26b8DA".to_string())
					|| (path == "/v1/run/label/7bf72e9190098776817afa763044ac1b"
						&& address == *"0x4b978322643F9Bf6C15bf26d866B81E99F26b8DA".to_string())
					|| (path == "/v1/run/label/6650ee41cda375e6d2a4d27746ce4805"
						&& address == *"0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84".to_string())
					|| (path == "/v1/run/label/9e394bae4a87c67d1073d930e0dee58c"
						&& address == *"0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84".to_string())
					|| (path == "/v1/run/label/e3da6466ef2e710b39f1139872a69eed"
						&& address == *"0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b".to_string())
					|| (path == "/v1/run/label/3a0a5230a42c5dd2b3581218766cc7fb"
						&& address == *"0xa94586fBB5B736a3f6AF31f84EEcc7677D2e7F84".to_string())
					|| (path == "/v1/run/label/e79d42db5a0e1571262e5d7c056111ed"
						&& address == *"0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b".to_string())
					|| (path == "/v1/run/label/5061d6de2687378f303b2f38538b350d"
						&& address == *"0x335c0552eb130f3Dfbe6efcB4D2895aED1E9938b".to_string())
				{
					return Response::builder().body(body_true.to_string())
				}

				Response::builder().status(400).body(String::from("Error query"))
			})
	}
}
