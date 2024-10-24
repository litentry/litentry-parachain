/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

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

//! Implementation of the server part of the state provisioning.

use super::{authentication::ClientAuth, ClientProvisioningRequest, Opcode, TcpHeader};
use crate::{
	attestation::create_ra_report_and_signature,
	error::{Error as EnclaveError, Result as EnclaveResult},
	initialization::global_components::{
		EnclaveSealHandler, GLOBAL_INTEGRITEE_PARENTCHAIN_LIGHT_CLIENT_SEAL,
		GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT, GLOBAL_SIGNER_REGISTRY,
		GLOBAL_STATE_KEY_REPOSITORY_COMPONENT,
	},
	ocall::OcallApi,
	tls_ra::seal_handler::UnsealStateAndKeys,
	GLOBAL_STATE_HANDLER_COMPONENT,
};

use crate::initialization::global_components::GLOBAL_ENCLAVE_REGISTRY;
use codec::Decode;
use itp_attestation_handler::RemoteAttestationType;
use itp_component_container::ComponentGetter;
use itp_ocall_api::EnclaveAttestationOCallApi;
use itp_types::ShardIdentifier;
use litentry_primitives::WorkerMode;
use log::*;
use rustls::{ServerConfig, ServerSession, StreamOwned};
use sgx_types::*;
use std::{
	backtrace::{self, PrintFormat},
	io::{Read, Write},
	net::TcpStream,
	sync::Arc,
};

#[allow(dead_code)]
#[derive(Clone, Eq, PartialEq, Debug)]
enum ProvisioningPayload {
	Everything,
	ShieldingKeyAndLightClient,
}

impl From<WorkerMode> for ProvisioningPayload {
	fn from(m: WorkerMode) -> Self {
		match m {
			WorkerMode::OffChainWorker => ProvisioningPayload::Everything,
			WorkerMode::Sidechain => ProvisioningPayload::Everything,
		}
	}
}

/// Server part of the TCP-level connection and the underlying TLS-level session.
///
/// Includes a seal handler, which handles the reading part of the data to be sent.
struct TlsServer<StateAndKeyUnsealer> {
	tls_stream: StreamOwned<ServerSession, TcpStream>,
	seal_handler: StateAndKeyUnsealer,
	provisioning_payload: ProvisioningPayload,
}

impl<StateAndKeyUnsealer> TlsServer<StateAndKeyUnsealer>
where
	StateAndKeyUnsealer: UnsealStateAndKeys,
{
	fn new(
		tls_stream: StreamOwned<ServerSession, TcpStream>,
		seal_handler: StateAndKeyUnsealer,
		provisioning_payload: ProvisioningPayload,
	) -> Self {
		Self { tls_stream, seal_handler, provisioning_payload }
	}

	/// Sends all relevant data of the specific shard to the client.
	fn handle_shard_request_from_client(&mut self) -> EnclaveResult<()> {
		println!(
			"    [Enclave] (MU-RA-Server) handle_shard_request_from_client, calling read_shard()"
		);
		let request = self.await_shard_request_from_client()?;
		println!("    [Enclave] (MU-RA-Server) handle_shard_request_from_client, await_shard_request_from_client() OK");
		println!("    [Enclave] (MU-RA-Server) handle_shard_request_from_client, write_all()");
		self.write_provisioning_payloads(&request.shard)
	}

	/// Read the shard of the state the client wants to receive.
	fn await_shard_request_from_client(&mut self) -> EnclaveResult<ClientProvisioningRequest> {
		let mut request = [0u8; std::mem::size_of::<ClientProvisioningRequest>()];
		println!(
			"    [Enclave] (MU-RA-Server) await_shard_request_from_client, calling read_exact()"
		);
		self.tls_stream.read_exact(&mut request)?;
		ClientProvisioningRequest::decode(&mut request.as_slice())
			.map_err(|_| EnclaveError::Other("matching byte size can't fail to decode".into()))
	}

	/// Sends all relevant data to the client.
	fn write_provisioning_payloads(&mut self, shard: &ShardIdentifier) -> EnclaveResult<()> {
		debug!("Provisioning is set to: {:?}", self.provisioning_payload);
		match self.provisioning_payload {
			ProvisioningPayload::Everything => {
				self.write_shielding_key()?;
				self.write_signers()?;
				self.write_enclaves()?;
				self.write_state_key()?;
				self.write_state(shard)?;
				self.write_light_client_state()?;
			},
			ProvisioningPayload::ShieldingKeyAndLightClient => {
				self.write_shielding_key()?;
				self.write_light_client_state()?;
			},
		}

		debug!("Successfully provisioned all payloads to peer");
		Ok(())
	}

	fn write_shielding_key(&mut self) -> EnclaveResult<()> {
		let shielding_key = self.seal_handler.unseal_shielding_key()?;
		self.write(Opcode::ShieldingKey, &shielding_key)?;
		Ok(())
	}

	fn write_signers(&mut self) -> EnclaveResult<()> {
		let signers = self.seal_handler.unseal_signers()?;
		self.write(Opcode::Signers, &signers)?;
		Ok(())
	}

	fn write_enclaves(&mut self) -> EnclaveResult<()> {
		let enclaves = self.seal_handler.unseal_enclaves()?;
		self.write(Opcode::Enclaves, &enclaves)?;
		Ok(())
	}

	fn write_state_key(&mut self) -> EnclaveResult<()> {
		let state_key = self.seal_handler.unseal_state_key()?;
		self.write(Opcode::StateKey, &state_key)?;
		Ok(())
	}

	fn write_state(&mut self, shard: &ShardIdentifier) -> EnclaveResult<()> {
		let state = self.seal_handler.unseal_state(shard)?;
		self.write(Opcode::State, &state)?;
		Ok(())
	}

	fn write_light_client_state(&mut self) -> EnclaveResult<()> {
		let state = self.seal_handler.unseal_light_client_state()?;
		self.write(Opcode::LightClient, &state)?;
		Ok(())
	}

	/// Sends the header followed by the payload.
	fn write(&mut self, opcode: Opcode, bytes: &[u8]) -> EnclaveResult<()> {
		let payload_length = bytes.len() as u64;
		self.write_header(TcpHeader::new(opcode, payload_length))?;
		debug!("Write payload - opcode: {:?}, payload_length: {}", opcode, payload_length);
		self.tls_stream.write_all(bytes)?;
		Ok(())
	}

	/// Sends the header which includes the payload length and the Opcode indicating the payload type.
	fn write_header(&mut self, tcp_header: TcpHeader) -> EnclaveResult<()> {
		self.tls_stream.write_all(&tcp_header.opcode.to_bytes())?;
		self.tls_stream.write_all(&tcp_header.payload_length.to_be_bytes())?;
		debug!(
			"Write header - opcode: {:?}, payload length: {}",
			tcp_header.opcode, tcp_header.payload_length
		);
		Ok(())
	}
}

#[no_mangle]
pub unsafe extern "C" fn run_state_provisioning_server(
	socket_fd: c_int,
	sign_type: sgx_quote_sign_type_t,
	quoting_enclave_target_info: Option<&sgx_target_info_t>,
	quote_size: Option<&u32>,
	skip_ra: c_int,
) -> sgx_status_t {
	let _ = backtrace::enable_backtrace("enclave.signed.so", PrintFormat::Short);

	let state_handler = match GLOBAL_STATE_HANDLER_COMPONENT.get() {
		Ok(s) => s,
		Err(e) => {
			error!("{:?}", e);
			return sgx_status_t::SGX_ERROR_UNEXPECTED
		},
	};

	let state_key_repository = match GLOBAL_STATE_KEY_REPOSITORY_COMPONENT.get() {
		Ok(s) => s,
		Err(e) => {
			error!("{:?}", e);
			return sgx_status_t::SGX_ERROR_UNEXPECTED
		},
	};

	let shielding_key_repository = match GLOBAL_SHIELDING_KEY_REPOSITORY_COMPONENT.get() {
		Ok(s) => s,
		Err(e) => {
			error!("{:?}", e);
			return sgx_status_t::SGX_ERROR_UNEXPECTED
		},
	};

	let light_client_seal = match GLOBAL_INTEGRITEE_PARENTCHAIN_LIGHT_CLIENT_SEAL.get() {
		Ok(s) => s,
		Err(e) => {
			error!("{:?}", e);
			return sgx_status_t::SGX_ERROR_UNEXPECTED
		},
	};

	let signer_registry = match GLOBAL_SIGNER_REGISTRY.get() {
		Ok(s) => s,
		Err(e) => {
			error!("{:?}", e);
			return sgx_status_t::SGX_ERROR_UNEXPECTED
		},
	};
	let enclave_registry = match GLOBAL_ENCLAVE_REGISTRY.get() {
		Ok(s) => s,
		Err(e) => {
			error!("{:?}", e);
			return sgx_status_t::SGX_ERROR_UNEXPECTED
		},
	};

	let seal_handler = EnclaveSealHandler::new(
		state_handler,
		state_key_repository,
		shielding_key_repository,
		light_client_seal,
		signer_registry,
		enclave_registry,
	);

	if let Err(e) = run_state_provisioning_server_internal::<_>(
		socket_fd,
		sign_type,
		quoting_enclave_target_info,
		quote_size,
		skip_ra,
		seal_handler,
	) {
		error!("Failed to provision state due to: {:?}", e);
		return e.into()
	};

	sgx_status_t::SGX_SUCCESS
}

/// Internal [`run_state_provisioning_server`] function to be able to use the handy `?` operator.
pub(crate) fn run_state_provisioning_server_internal<StateAndKeyUnsealer: UnsealStateAndKeys>(
	socket_fd: c_int,
	sign_type: sgx_quote_sign_type_t,
	quoting_enclave_target_info: Option<&sgx_target_info_t>,
	quote_size: Option<&u32>,
	skip_ra: c_int,
	seal_handler: StateAndKeyUnsealer,
) -> EnclaveResult<()> {
	let server_config = tls_server_config(
		sign_type,
		quoting_enclave_target_info,
		quote_size,
		OcallApi,
		skip_ra == 1,
	)?;
	let (server_session, tcp_stream) = tls_server_session_stream(socket_fd, server_config)?;

	let provisioning = ProvisioningPayload::Everything;

	let mut server =
		TlsServer::new(StreamOwned::new(server_session, tcp_stream), seal_handler, provisioning);

	// todo: verify client signer belongs to a registered enclave on integritee network with a
	// matching or whitelisted MRENCLAVE as replacement for MU RA #1385

	println!("    [Enclave] (MU-RA-Server) MU-RA successful sending keys");
	println!(
		"    [Enclave] (MU-RA-Server) MU-RA successful, calling handle_shard_request_from_client()"
	);
	server.handle_shard_request_from_client()
}

fn tls_server_session_stream(
	socket_fd: i32,
	server_config: ServerConfig,
) -> EnclaveResult<(ServerSession, TcpStream)> {
	let sess = ServerSession::new(&Arc::new(server_config));
	let conn = TcpStream::new(socket_fd).map_err(|e| EnclaveError::Other(e.into()))?;
	Ok((sess, conn))
}

fn tls_server_config<A: EnclaveAttestationOCallApi + 'static>(
	sign_type: sgx_quote_sign_type_t,
	quoting_enclave_target_info: Option<&sgx_target_info_t>,
	quote_size: Option<&u32>,
	ocall_api: A,
	skip_ra: bool,
) -> EnclaveResult<ServerConfig> {
	#[cfg(not(feature = "dcap"))]
	let attestation_type = RemoteAttestationType::Epid;
	#[cfg(feature = "dcap")]
	let attestation_type = RemoteAttestationType::Dcap;

	// report will be signed with server enclave ed25519 signing key
	let (key_der, cert_der) = create_ra_report_and_signature(
		skip_ra,
		attestation_type,
		sign_type,
		quoting_enclave_target_info,
		quote_size,
	)?;

	// ClientAuth will perform MU RA as part of authentication process
	let mut cfg = rustls::ServerConfig::new(Arc::new(ClientAuth::new(true, skip_ra, ocall_api)));
	let certs = vec![rustls::Certificate(cert_der)];
	let privkey = rustls::PrivateKey(key_der);
	cfg.set_single_cert_with_ocsp_and_sct(certs, privkey, vec![], vec![])
		.map_err(|e| EnclaveError::Other(e.into()))?;
	Ok(cfg)
}
