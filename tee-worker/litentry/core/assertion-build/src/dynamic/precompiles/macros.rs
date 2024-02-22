#[macro_export]
macro_rules! http_get_precompile {
	($name:ident, $token:ident, $parse_fn_name:ident) => {
		pub struct $name;

		impl $name {
			pub fn execute(input: Vec<u8>) -> $crate::dynamic::precompiles::PrecompileResult {
				let mut reader = $crate::dynamic::precompiles::InputReader::new(input);
				let url = reader.read_string();
				let pointer = reader.read_string();
				let mut headers = itc_rest_client::rest_client::Headers::new();
				headers.insert(http::header::CONNECTION.as_str(), "close");
				let client = itc_rest_client::http_client::HttpClient::new(
					itc_rest_client::http_client::DefaultSend {},
					true,
					Some(core::time::Duration::from_secs(5)),
					Some(headers),
					None,
				);
				let resp = client
					.send_request_raw(
						itc_rest_client::rest_client::Url::parse(&url).unwrap(),
						itc_rest_client::rest_client::Method::GET,
						None,
					)
					.unwrap();
				let value: serde_json::Value = serde_json::from_slice(&resp.1).unwrap();
				let result = value.pointer(&pointer).unwrap();
				let encoded = ethabi::encode(&[ethabi::Token::$token(
					result.$parse_fn_name().unwrap().into(),
				)]);
				Ok(evm::executor::stack::PrecompileOutput {
					exit_status: evm::ExitSucceed::Returned,
					output: encoded,
				})
			}
		}
	};
}
