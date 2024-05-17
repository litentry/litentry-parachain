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

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

use crate::{
	achainable::web3_network_to_chain, build_client_with_cert, DataProviderConfig, Error,
	HttpError, ReqPath, RetryOption, RetryableRestGet,
};
use http::header::CONNECTION;
use http_req::response::Headers;
use itc_rest_client::{
	http_client::{HttpClient, SendWithCertificateVerification},
	rest_client::RestClient,
	RestPath,
};
use litentry_primitives::Web3Network;
use log::debug;
use serde::{Deserialize, Serialize};
use std::{
	format, str,
	string::{String, ToString},
	vec,
	vec::Vec,
};

#[derive(Debug)]
pub struct MoralisRequest {
	path: String,
	query: Option<Vec<(String, String)>>,
}

enum ClientType {
	Evm,
	Solana,
}

pub struct MoralisClient {
	retry_option: RetryOption,
	client: RestClient<HttpClient<SendWithCertificateVerification>>,
	solana_client: RestClient<HttpClient<SendWithCertificateVerification>>,
}

impl MoralisClient {
	pub fn new(data_provider_config: &DataProviderConfig) -> Self {
		let api_key = data_provider_config.moralis_api_key.clone();
		let api_retry_delay = data_provider_config.moralis_api_retry_delay;
		let api_retry_times = data_provider_config.moralis_api_retry_times;
		let api_url = data_provider_config.moralis_api_url.clone();
		let solana_api_url = data_provider_config.moralis_solana_api_url.clone();
		let retry_option =
			RetryOption { retry_delay: Some(api_retry_delay), retry_times: Some(api_retry_times) };

		let mut headers = Headers::new();
		headers.insert(CONNECTION.as_str(), "close");
		headers.insert("X-API-Key", api_key.as_str());
		let client = build_client_with_cert(api_url.as_str(), headers.clone());
		let solana_client = build_client_with_cert(solana_api_url.as_str(), headers.clone());

		MoralisClient { retry_option, client, solana_client }
	}

	fn get<T>(
		&mut self,
		client_type: ClientType,
		params: MoralisRequest,
		fast_fail: bool,
	) -> Result<T, Error>
	where
		T: serde::de::DeserializeOwned + for<'a> RestPath<ReqPath<'a>>,
	{
		let retry_option: Option<RetryOption> =
			if fast_fail { None } else { Some(self.retry_option.clone()) };
		let client = match client_type {
			ClientType::Evm => &mut self.client,
			ClientType::Solana => &mut self.solana_client,
		};
		if let Some(query) = params.query {
			let transformed_query: Vec<(&str, &str)> =
				query.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect();
			client.get_with_retry::<ReqPath, T>(
				ReqPath::new(params.path.as_str()),
				&transformed_query,
				retry_option,
			)
		} else {
			client.get_retry::<ReqPath, T>(ReqPath::new(params.path.as_str()), retry_option)
		}
	}
}

trait MoralisChain {
	fn get_chain(&self) -> String;
}

impl MoralisChain for Web3Network {
	fn get_chain(&self) -> String {
		match self {
			Self::Ethereum => "eth".into(),
			Self::Bsc => "bsc".into(),
			Self::Polygon => "polygon".into(),
			Self::Arbitrum => "arbitrum".into(),
			_ => "".into(),
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoralisChainParam {
	value: String,
}

impl MoralisChainParam {
	pub fn new(network: &Web3Network) -> Self {
		Self { value: network.get_chain() }
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoralisPageResponse<T> {
	pub status: String,
	pub page: usize,
	pub page_size: usize,
	pub cursor: Option<String>,
	pub result: Vec<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNftsByWalletParam {
	pub address: String,
	pub chain: MoralisChainParam,
	pub token_addresses: Option<Vec<String>>,
	// max: 100, default: 100
	pub limit: Option<usize>,
	pub cursor: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetNftsByWalletResult {
	pub amount: String,
	pub token_id: String,
	pub token_address: String,
	pub contract_type: String,
	pub owner_of: String,
}

pub type GetNftsByWalletResponse = MoralisPageResponse<GetNftsByWalletResult>;

impl<'a> RestPath<ReqPath<'a>> for GetNftsByWalletResponse {
	fn get_path(path: ReqPath) -> Result<String, HttpError> {
		Ok(path.path.into())
	}
}

pub trait NftApiList {
	fn get_nfts_by_wallet(
		&mut self,
		param: &GetNftsByWalletParam,
		fast_fail: bool,
	) -> Result<GetNftsByWalletResponse, Error>;
}

impl NftApiList for MoralisClient {
	// https://docs.moralis.io/web3-data-api/evm/reference/get-wallet-nfts
	fn get_nfts_by_wallet(
		&mut self,
		param: &GetNftsByWalletParam,
		fast_fail: bool,
	) -> Result<GetNftsByWalletResponse, Error> {
		let mut query: Vec<(String, String)> =
			vec![("chain".to_string(), param.chain.value.clone())];
		if let Some(token_addresses) = param.token_addresses.clone() {
			for (index, address) in token_addresses.iter().enumerate() {
				query.push((format!("token_addresses[{}]", index), address.clone()));
			}
		}

		if let Some(limit) = param.limit {
			query.push(("limit".to_string(), limit.to_string()));
		}

		if let Some(cursor) = param.cursor.clone() {
			query.push(("cursor".to_string(), cursor));
		}

		let params = MoralisRequest { path: format!("{}/nft", param.address), query: Some(query) };

		debug!("get_nfts_by_wallet, params: {:?}", params);

		match self.get::<GetNftsByWalletResponse>(ClientType::Evm, params, fast_fail) {
			Ok(resp) => {
				debug!("get_nfts_by_wallet, response: {:?}", resp);
				Ok(resp)
			},
			Err(e) => {
				debug!("get_nfts_by_wallet, error: {:?}", e);
				Err(e)
			},
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSolanaNativeBalanceByWalletResponse {
	pub lamports: String,
	pub solana: String,
}

impl<'a> RestPath<ReqPath<'a>> for GetSolanaNativeBalanceByWalletResponse {
	fn get_path(path: ReqPath) -> Result<String, HttpError> {
		Ok(path.path.into())
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetSolanaTokenBalanceByWalletResponse {
	// token address
	pub mint: String,
	pub amount: String,
}

impl<'a> RestPath<ReqPath<'a>> for Vec<GetSolanaTokenBalanceByWalletResponse> {
	fn get_path(path: ReqPath) -> Result<String, HttpError> {
		Ok(path.path.into())
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetEvmTokenBalanceByWalletResponse {
	pub token_address: String,
	pub symbol: String,
	pub name: String,
	pub logo: Option<String>,      // logo url(string type), MAY BE null
	pub thumbnail: Option<String>, // thumbnail url(string type), MAY BE null
	pub decimals: u64,
	pub balance: String,
	pub possible_spam: bool,
	pub verified_contract: bool,
	pub total_supply: String,
	pub total_supply_formatted: String,
	pub percentage_relative_to_total_supply: f64,
}

impl<'a> RestPath<ReqPath<'a>> for Vec<GetEvmTokenBalanceByWalletResponse> {
	fn get_path(path: ReqPath) -> Result<String, HttpError> {
		Ok(path.path.into())
	}
}

pub trait BalanceApiList {
	fn get_solana_native_balance_by_wallet(
		&mut self,
		address: String,
		fast_fail: bool,
	) -> Result<GetSolanaNativeBalanceByWalletResponse, Error>;

	fn get_solana_tokens_balance_by_wallet(
		&mut self,
		address: String,
		fast_fail: bool,
	) -> Result<Vec<GetSolanaTokenBalanceByWalletResponse>, Error>;

	// https://docs.moralis.io/web3-data-api/evm/reference/wallet-api/get-token-balances-by-wallet
	fn get_evm_tokens_balance_by_wallet(
		&mut self,
		address: String,
		network: &Web3Network,
		fast_fail: bool,
	) -> Result<Vec<GetEvmTokenBalanceByWalletResponse>, Error>;
}

impl BalanceApiList for MoralisClient {
	// https://docs.moralis.io/web3-data-api/solana/reference/sol-balance
	fn get_solana_native_balance_by_wallet(
		&mut self,
		address: String,
		fast_fail: bool,
	) -> Result<GetSolanaNativeBalanceByWalletResponse, Error> {
		let params =
			MoralisRequest { path: format!("account/mainnet/{}/balance", address), query: None };

		debug!("get_solana_native_balance_by_wallet, address: {:?}", address);

		match self.get::<GetSolanaNativeBalanceByWalletResponse>(
			ClientType::Solana,
			params,
			fast_fail,
		) {
			Ok(resp) => {
				debug!("get_solana_native_balance_by_wallet, response: {:?}", resp);
				Ok(resp)
			},
			Err(e) => {
				debug!("get_solana_native_balance_by_wallet, error: {:?}", e);
				Err(e)
			},
		}
	}

	// https://docs.moralis.io/web3-data-api/solana/reference/get-spl
	fn get_solana_tokens_balance_by_wallet(
		&mut self,
		address: String,
		fast_fail: bool,
	) -> Result<Vec<GetSolanaTokenBalanceByWalletResponse>, Error> {
		let params =
			MoralisRequest { path: format!("account/mainnet/{}/tokens", address), query: None };

		debug!("get_solana_tokens_balance_by_wallet, address: {:?}", address);

		match self.get::<Vec<GetSolanaTokenBalanceByWalletResponse>>(
			ClientType::Solana,
			params,
			fast_fail,
		) {
			Ok(resp) => {
				debug!("get_solana_tokens_balance_by_wallet, response: {:?}", resp);
				Ok(resp)
			},
			Err(e) => {
				debug!("get_solana_tokens_balance_by_wallet, error: {:?}", e);
				Err(e)
			},
		}
	}

	fn get_evm_tokens_balance_by_wallet(
		&mut self,
		address: String,
		network: &Web3Network,
		fast_fail: bool,
	) -> Result<Vec<GetEvmTokenBalanceByWalletResponse>, Error> {
		debug!("get_evm_tokens_balance_by_wallet, address: {}", address);

		let query = Some(vec![("chain".to_string(), web3_network_to_chain(network))]);
		let params = MoralisRequest { path: format!("{}/erc20", address), query };

		match self.get::<Vec<GetEvmTokenBalanceByWalletResponse>>(
			ClientType::Evm,
			params,
			fast_fail,
		) {
			Ok(resp) => {
				debug!("get_evm_tokens_balance_by_wallet, response: {:?}", resp);
				Ok(resp)
			},
			Err(e) => {
				debug!("get_evm_tokens_balance_by_wallet, error: {:?}", e);
				Err(e)
			},
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use lc_mock_server::run;

	fn init() -> DataProviderConfig {
		let _ = env_logger::builder().is_test(true).try_init();
		let url = run(0).unwrap();

		let mut config = DataProviderConfig::new().unwrap();
		config.set_moralis_api_key("d416f55179dbd0e45b1a8ed030e3".to_string());
		config.set_moralis_api_url(url.clone() + "/moralis/").unwrap();
		config.set_moralis_solana_api_url(url.clone() + "/moralis_solana/").unwrap();
		config
	}

	#[test]
	fn does_get_nfts_by_wallet_works() {
		let config = init();
		let mut client = MoralisClient::new(&config);
		let param = GetNftsByWalletParam {
			address: "0x49ad262c49c7aa708cc2df262ed53b64a17dd5ee".into(),
			chain: MoralisChainParam::new(&Web3Network::Ethereum),
			token_addresses: Some(vec!["0x9401518f4ebba857baa879d9f76e1cc8b31ed197".into()]),
			limit: None,
			cursor: None,
		};
		let result = client.get_nfts_by_wallet(&param, true).unwrap();
		assert_eq!(result.cursor.unwrap(), "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9");
		assert_eq!(result.page, 1);
		assert_eq!(result.page_size, 100);
		assert_eq!(result.result.len(), 1);
		assert_eq!(result.result[0].amount, "1");
		assert_eq!(result.result[0].token_id, "5021");
		assert_eq!(result.result[0].token_address, "0xfff54e6fe44fd47c8814c4b1d62c924c54364ad3");
		assert_eq!(result.result[0].contract_type, "ERC721");
		assert_eq!(result.result[0].owner_of, "0xff3879b8a363aed92a6eaba8f61f1a96a9ec3c1e");
	}

	#[test]
	fn does_get_solana_native_balance_by_wallet_works() {
		let config = init();
		let mut client = MoralisClient::new(&config);
		let mut result = client
			.get_solana_native_balance_by_wallet(
				"EJpLyTeE8XHG9CeREeHd6pr6hNhaRnTRJx4Z5DPhEJJ6".into(),
				true,
			)
			.unwrap();
		assert_eq!(result.lamports, "5903457912");
		assert_eq!(result.solana, "5.903457912");
		result = client
			.get_solana_native_balance_by_wallet(
				"EJpLyTeE8XHG9CeREeHd6pr6hNhaRnTRJx4Z5DPhEJJ1".into(),
				true,
			)
			.unwrap();
		assert_eq!(result.lamports, "0");
		assert_eq!(result.solana, "0");
	}

	#[test]
	fn does_get_solana_tokens_balance_by_wallet_works() {
		let config = init();
		let mut client = MoralisClient::new(&config);
		let mut result = client
			.get_solana_tokens_balance_by_wallet(
				"EJpLyTeE8XHG9CeREeHd6pr6hNhaRnTRJx4Z5DPhEJJ6".into(),
				true,
			)
			.unwrap();
		assert_eq!(result.len(), 2);
		assert_eq!(result[0].mint, "FADm4QuSUF1K526LvTjvbJjKzeeipP6bj5bSzp3r6ipq");
		assert_eq!(result[0].amount, "405219.979008");
		assert_eq!(result[1].mint, "BNrgKeLwMUwWQYovZpANYQNCC7Aw8FgvFL3GQut1gL6B");
		assert_eq!(result[1].amount, "31");
		result = client
			.get_solana_tokens_balance_by_wallet(
				"EJpLyTeE8XHG9CeREeHd6pr6hNhaRnTRJx4Z5DPhEJJ1".into(),
				true,
			)
			.unwrap();
		assert_eq!(result.len(), 0);
	}
}
