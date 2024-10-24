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

use std::str::FromStr;

use alloy::network::EthereumWallet;
use alloy::primitives::{Address, U256};
use alloy::providers::{Provider, ProviderBuilder, WalletProvider};
use alloy::rpc::types::{TransactionInput, TransactionRequest};
use alloy::signers::local::PrivateKeySigner;
use async_trait::async_trait;
use executor_core::intent_executor::IntentExecutor;
use executor_core::primitives::Intent;
use log::{error, info};

/// Executes intents on Ethereum network.
pub struct EthereumIntentExecutor {
	rpc_url: String,
}

impl EthereumIntentExecutor {
	pub fn new(rpc_url: &str) -> Result<Self, ()> {
		Ok(Self { rpc_url: rpc_url.to_string() })
	}
}

#[async_trait]
impl IntentExecutor for EthereumIntentExecutor {
	async fn execute(&self, intent: Intent) -> Result<(), ()> {
		info!("Executin intent: {:?}", intent);
		// todo: this should be retrieved from key_store
		let signer = PrivateKeySigner::from_str(
			"0x59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d",
		)
		.unwrap();
		let wallet = EthereumWallet::from(signer);
		let provider = ProviderBuilder::new()
			.with_recommended_fillers()
			.wallet(wallet)
			.on_http(self.rpc_url.parse().map_err(|e| error!("Could not parse rpc url: {:?}", e))?);
		let account =
			provider.get_account(provider.signer_addresses().next().unwrap()).await.unwrap();
		match intent {
			Intent::TransferEthereum(to, value) => {
				let tx = TransactionRequest::default()
					.to(Address::from(to))
					.nonce(account.nonce)
					.value(U256::from_be_bytes(value));
				let pending_tx = provider.send_transaction(tx).await.map_err(|e| {
					error!("Could not send transaction: {:?}", e);
				})?;
				// wait for transaction to be included
				pending_tx.get_receipt().await.map_err(|e| {
					error!("Could not get transaction receipt: {:?}", e);
				})?;
			},
			Intent::CallEthereum(address, input) => {
				let tx = TransactionRequest::default()
					.to(Address::from(address))
					.nonce(account.nonce)
					.input(TransactionInput::from(input));
				let pending_tx = provider.send_transaction(tx).await.map_err(|e| {
					error!("Could not send transaction: {:?}", e);
				})?;
				// wait for transaction to be included
				pending_tx.get_receipt().await.map_err(|e| {
					error!("Could not get transaction receipt: {:?}", e);
				})?;
			},
		}
		Ok(())
	}
}
