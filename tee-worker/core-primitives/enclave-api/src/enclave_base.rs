/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG
	Copyright (C) 2017-2019 Baidu, Inc. All Rights Reserved.

	Licensed under the Apache License, Version 2.0 (the "License");
	you may not use this file except in compliance with the License.
	You may obtain a copy of the License at

		http://www.apache.org/licenses/LICENSE-2.0

	Unless required by applicable law or agreed to in writing, software
	distributed under the License is distributed on an "AS IS" BASIS,
	WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
	See the License for the specific language governing permissions and
	limitations under the License.

*/

use crate::EnclaveResult;
use codec::Decode;
use core::fmt::Debug;
use itc_parentchain::primitives::{ParentchainId, ParentchainInitParams};
use itp_types::ShardIdentifier;
use pallet_teebag::EnclaveFingerprint;
use sgx_crypto_helper::rsa3072::Rsa3072PubKey;
use sp_core::ed25519;

/// Trait for base/common Enclave API functions
pub trait EnclaveBase: Send + Sync + 'static {
	/// Initialize the enclave (needs to be called once at application startup).
	fn init(
		&self,
		mu_ra_addr: &str,
		untrusted_worker_addr: &str,
		base_dir: &str,
	) -> EnclaveResult<()>;

	/// Initialize the enclave sidechain components.
	fn init_enclave_sidechain_components(
		&self,
		fail_mode: Option<String>,
		fail_at: u64,
	) -> EnclaveResult<()>;

	/// Initialize the direct invocation RPC server.
	fn init_direct_invocation_server(&self, rpc_server_addr: String) -> EnclaveResult<()>;

	/// Initialize the light client (needs to be called once at application startup).
	fn init_parentchain_components<Header: Decode + Debug>(
		&self,
		params: ParentchainInitParams,
	) -> EnclaveResult<Header>;

	/// Initialize a new shard.
	fn init_shard(&self, shard: Vec<u8>) -> EnclaveResult<()>;

	/// Initialize a new shard vault account and register enclave signer as its proxy.
	fn init_proxied_shard_vault(
		&self,
		shard: &ShardIdentifier,
		parentchain_id: &ParentchainId,
	) -> EnclaveResult<()>;

	fn set_nonce(&self, nonce: u32, parentchain_id: ParentchainId) -> EnclaveResult<()>;

	fn set_node_metadata(
		&self,
		metadata: Vec<u8>,
		parentchain_id: ParentchainId,
	) -> EnclaveResult<()>;

	fn get_rsa_shielding_pubkey(&self) -> EnclaveResult<Rsa3072PubKey>;

	fn get_ecc_signing_pubkey(&self) -> EnclaveResult<ed25519::Public>;

	/// retrieve vault account from shard state
	fn get_ecc_vault_pubkey(&self, shard: &ShardIdentifier) -> EnclaveResult<ed25519::Public>;

	fn get_fingerprint(&self) -> EnclaveResult<EnclaveFingerprint>;

	// litentry
	fn migrate_shard(&self, old_shard: Vec<u8>, new_shard: Vec<u8>) -> EnclaveResult<()>;
}

/// EnclaveApi implementation for Enclave struct
#[cfg(feature = "implement-ffi")]
mod impl_ffi {
	use super::EnclaveBase;
	use crate::{error::Error, Enclave, EnclaveResult};
	use codec::{Decode, Encode};
	use core::fmt::Debug;
	use frame_support::ensure;
	use itc_parentchain::primitives::{ParentchainId, ParentchainInitParams};
	use itp_enclave_api_ffi as ffi;
	use itp_settings::worker::{
		HEADER_MAX_SIZE, MR_ENCLAVE_SIZE, SHIELDING_KEY_SIZE, SIGNING_KEY_SIZE,
	};
	use itp_types::ShardIdentifier;
	use log::*;
	use pallet_teebag::EnclaveFingerprint;
	use sgx_crypto_helper::rsa3072::Rsa3072PubKey;
	use sgx_types::*;
	use sp_core::ed25519;

	impl EnclaveBase for Enclave {
		fn init(
			&self,
			mu_ra_addr: &str,
			untrusted_worker_addr: &str,
			base_dir: &str,
		) -> EnclaveResult<()> {
			let mut retval = sgx_status_t::SGX_SUCCESS;

			let encoded_mu_ra_addr = mu_ra_addr.encode();
			let encoded_untrusted_worker_addr = untrusted_worker_addr.encode();
			let encoded_base_dir = base_dir.encode();

			let result = unsafe {
				ffi::init(
					self.eid,
					&mut retval,
					encoded_mu_ra_addr.as_ptr(),
					encoded_mu_ra_addr.len() as u32,
					encoded_untrusted_worker_addr.as_ptr(),
					encoded_untrusted_worker_addr.len() as u32,
					encoded_base_dir.as_ptr(),
					encoded_base_dir.len() as u32,
				)
			};

			ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
			ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));

			Ok(())
		}

		fn init_enclave_sidechain_components(
			&self,
			fail_mode: Option<String>,
			fail_at: u64,
		) -> EnclaveResult<()> {
			let mut retval = sgx_status_t::SGX_SUCCESS;
			let encoded_fail_mode = fail_mode.encode();
			let encoded_fail_at = fail_at.encode();

			let result = unsafe {
				ffi::init_enclave_sidechain_components(
					self.eid,
					&mut retval,
					encoded_fail_mode.as_ptr(),
					encoded_fail_mode.len() as u32,
					encoded_fail_at.as_ptr(),
					encoded_fail_at.len() as u32,
				)
			};

			ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
			ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));

			Ok(())
		}

		fn init_direct_invocation_server(&self, rpc_server_addr: String) -> EnclaveResult<()> {
			let mut retval = sgx_status_t::SGX_SUCCESS;

			let encoded_rpc_server_addr = rpc_server_addr.encode();

			let result = unsafe {
				ffi::init_direct_invocation_server(
					self.eid,
					&mut retval,
					encoded_rpc_server_addr.as_ptr(),
					encoded_rpc_server_addr.len() as u32,
				)
			};

			ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
			ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));

			Ok(())
		}

		fn init_parentchain_components<Header: Decode + Debug>(
			&self,
			params: ParentchainInitParams,
		) -> EnclaveResult<Header> {
			let latest_header_encoded = init_parentchain_components_ffi(self.eid, params.encode())?;

			let latest = Header::decode(&mut latest_header_encoded.as_slice())?;
			info!("Latest Header {:?}", latest);

			Ok(latest)
		}

		fn init_shard(&self, shard: Vec<u8>) -> EnclaveResult<()> {
			let mut retval = sgx_status_t::SGX_SUCCESS;

			let result = unsafe {
				ffi::init_shard(self.eid, &mut retval, shard.as_ptr(), shard.len() as u32)
			};

			ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
			ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));

			Ok(())
		}

		fn init_proxied_shard_vault(
			&self,
			shard: &ShardIdentifier,
			parentchain_id: &ParentchainId,
		) -> EnclaveResult<()> {
			let mut retval = sgx_status_t::SGX_SUCCESS;
			let parentchain_id_enc = parentchain_id.encode();
			let shard_bytes = shard.encode();
			let result = unsafe {
				ffi::init_proxied_shard_vault(
					self.eid,
					&mut retval,
					shard_bytes.as_ptr(),
					shard_bytes.len() as u32,
					parentchain_id_enc.as_ptr(),
					parentchain_id_enc.len() as u32,
				)
			};

			ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
			ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));

			Ok(())
		}

		fn set_nonce(&self, nonce: u32, parentchain_id: ParentchainId) -> EnclaveResult<()> {
			let mut retval = sgx_status_t::SGX_SUCCESS;

			let parentchain_id_enc = parentchain_id.encode();

			let result = unsafe {
				ffi::set_nonce(
					self.eid,
					&mut retval,
					&nonce,
					parentchain_id_enc.as_ptr(),
					parentchain_id_enc.len() as u32,
				)
			};

			ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
			ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));

			Ok(())
		}

		fn set_node_metadata(
			&self,
			metadata: Vec<u8>,
			parentchain_id: ParentchainId,
		) -> EnclaveResult<()> {
			let mut retval = sgx_status_t::SGX_SUCCESS;

			let parentchain_id_enc = parentchain_id.encode();

			let result = unsafe {
				ffi::set_node_metadata(
					self.eid,
					&mut retval,
					metadata.as_ptr(),
					metadata.len() as u32,
					parentchain_id_enc.as_ptr(),
					parentchain_id_enc.len() as u32,
				)
			};

			ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
			ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));

			Ok(())
		}

		fn get_rsa_shielding_pubkey(&self) -> EnclaveResult<Rsa3072PubKey> {
			let mut retval = sgx_status_t::SGX_SUCCESS;

			let pubkey_size = SHIELDING_KEY_SIZE;
			let mut pubkey = vec![0u8; pubkey_size];

			let result = unsafe {
				ffi::get_rsa_encryption_pubkey(
					self.eid,
					&mut retval,
					pubkey.as_mut_ptr(),
					pubkey.len() as u32,
				)
			};

			ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
			ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));

			let rsa_pubkey: Rsa3072PubKey =
				serde_json::from_slice(pubkey.as_slice()).expect("Invalid public key");
			debug!("got RSA pubkey {:?}", rsa_pubkey);
			Ok(rsa_pubkey)
		}

		fn get_ecc_signing_pubkey(&self) -> EnclaveResult<ed25519::Public> {
			let mut retval = sgx_status_t::SGX_SUCCESS;
			let mut pubkey = [0u8; SIGNING_KEY_SIZE];

			let result = unsafe {
				ffi::get_ecc_signing_pubkey(
					self.eid,
					&mut retval,
					pubkey.as_mut_ptr(),
					pubkey.len() as u32,
				)
			};

			ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
			ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));

			Ok(ed25519::Public::from_raw(pubkey))
		}

		fn get_ecc_vault_pubkey(&self, shard: &ShardIdentifier) -> EnclaveResult<ed25519::Public> {
			let mut retval = sgx_status_t::SGX_SUCCESS;
			let mut pubkey = [0u8; SIGNING_KEY_SIZE];
			let shard_bytes = shard.encode();

			let result = unsafe {
				ffi::get_ecc_vault_pubkey(
					self.eid,
					&mut retval,
					shard_bytes.as_ptr(),
					shard_bytes.len() as u32,
					pubkey.as_mut_ptr(),
					pubkey.len() as u32,
				)
			};

			ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
			ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));

			Ok(ed25519::Public::from_raw(pubkey))
		}

		fn get_fingerprint(&self) -> EnclaveResult<EnclaveFingerprint> {
			let mut retval = sgx_status_t::SGX_SUCCESS;
			let mut mr_enclave = [0u8; MR_ENCLAVE_SIZE];

			let result = unsafe {
				ffi::get_mrenclave(
					self.eid,
					&mut retval,
					mr_enclave.as_mut_ptr(),
					mr_enclave.len() as u32,
				)
			};

			ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
			ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));

			Ok(mr_enclave.into())
		}

		fn migrate_shard(&self, old_shard: Vec<u8>, new_shard: Vec<u8>) -> EnclaveResult<()> {
			let mut retval = sgx_status_t::SGX_SUCCESS;

			let result = unsafe {
				ffi::migrate_shard(
					self.eid,
					&mut retval,
					old_shard.as_ptr(),
					new_shard.as_ptr(),
					old_shard.len() as u32,
				)
			};

			ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
			ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));

			Ok(())
		}
	}

	fn init_parentchain_components_ffi(
		enclave_id: sgx_enclave_id_t,
		params: Vec<u8>,
	) -> EnclaveResult<Vec<u8>> {
		let mut retval = sgx_status_t::SGX_SUCCESS;

		let latest_header_size = HEADER_MAX_SIZE;
		let mut latest_header = vec![0u8; latest_header_size];

		let result = unsafe {
			ffi::init_parentchain_components(
				enclave_id,
				&mut retval,
				params.as_ptr(),
				params.len(),
				latest_header.as_mut_ptr(),
				latest_header.len(),
			)
		};

		ensure!(result == sgx_status_t::SGX_SUCCESS, Error::Sgx(result));
		ensure!(retval == sgx_status_t::SGX_SUCCESS, Error::Sgx(retval));

		Ok(latest_header)
	}
}
