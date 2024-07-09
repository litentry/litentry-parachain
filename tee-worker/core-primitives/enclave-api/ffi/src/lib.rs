//! FFI's that call into the enclave. These functions need to be added to the
// enclave edl file and be implemented within the enclave.
use sgx_types::{error::*, types::*};

extern "C" {

	pub fn generate_dcap_ra_extrinsic_from_quote(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		w_url: *const u8,
		w_url_size: u32,
		quote: *const u8,
		quote_size: u32,
		unchecked_extrinsic: *mut u8,
		unchecked_extrinsic_max_size: u32,
		unchecked_extrinsic_size: *mut u32,
	) -> SgxStatus;

	pub fn init(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		mu_ra_addr: *const u8,
		mu_ra_addr_size: u32,
		untrusted_worker_addr: *const u8,
		untrusted_worker_addr_size: u32,
		encoded_base_dir_str: *const u8,
		encoded_base_dir_size: u32,
	) -> SgxStatus;

	pub fn init_enclave_sidechain_components(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		fail_mode: *const u8,
		fail_mode_size: u32,
		fail_at: *const u8,
		fail_at_size: u32,
	) -> SgxStatus;

	pub fn init_direct_invocation_server(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		server_addr: *const u8,
		server_addr_size: u32,
	) -> SgxStatus;

	pub fn init_parentchain_components(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		params: *const u8,
		params_size: usize,
		latest_header: *mut u8,
		latest_header_size: usize,
	) -> SgxStatus;

	pub fn init_shard(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		shard: *const u8,
		shard_size: u32,
	) -> SgxStatus;

	pub fn init_shard_creation_parentchain_header(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		shard: *const u8,
		shard_size: u32,
		parentchain_id: *const u8,
		parentchain_id_size: u32,
		header: *const u8,
		header_size: u32,
	) -> SgxStatus;

	pub fn get_shard_creation_info(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		shard: *const u8,
		shard_size: u32,
		creation: *mut u8,
		creation_size: u32,
	) -> SgxStatus;

	pub fn execute_trusted_calls(eid: EnclaveId, retval: *mut SgxStatus) -> SgxStatus;

	pub fn sync_parentchain(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		blocks: *const u8,
		blocks_size: usize,
		events: *const u8,
		events_size: usize,
		events_proofs: *const u8,
		events_proofs_size: usize,
		parentchain_id: *const u8,
		parentchain_id_size: u32,
		immediate_import: c_int,
	) -> SgxStatus;

	pub fn set_nonce(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		nonce: *const u32,
		parentchain_id: *const u8,
		parentchain_id_size: u32,
	) -> SgxStatus;

	pub fn set_node_metadata(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		node_metadata: *const u8,
		node_metadata_size: u32,
		parentchain_id: *const u8,
		parentchain_id_size: u32,
	) -> SgxStatus;

	pub fn get_rsa_encryption_pubkey(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		pubkey: *mut u8,
		pubkey_size: u32,
	) -> SgxStatus;

	pub fn get_ecc_signing_pubkey(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		pubkey: *mut u8,
		pubkey_size: u32,
	) -> SgxStatus;

	pub fn get_mrenclave(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		mrenclave: *mut u8,
		mrenclave_size: u32,
	) -> SgxStatus;

	pub fn generate_ias_ra_extrinsic(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		w_url: *const u8,
		w_url_size: u32,
		unchecked_extrinsic: *mut u8,
		unchecked_extrinsic_max_size: u32,
		unchecked_extrinsic_size: *mut u32,
		skip_ra: c_int,
	) -> SgxStatus;

	pub fn generate_dcap_ra_extrinsic(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		w_url: *const u8,
		w_url_size: u32,
		unchecked_extrinsic: *mut u8,
		unchecked_extrinsic_max_size: u32,
		unchecked_extrinsic_size: *mut u32,
		skip_ra: c_int,
		quoting_enclave_target_info: Option<&TargetInfo>,
		quote_size: Option<&u32>,
	) -> SgxStatus;

	pub fn generate_dcap_ra_quote(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		skip_ra: c_int,
		quoting_enclave_target_info: &TargetInfo,
		quote_size: u32,
		dcap_quote_p: *mut u8,
		dcap_quote_size: u32,
	) -> SgxStatus;

	pub fn generate_register_quoting_enclave_extrinsic(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		collateral: *const CQlQveCollateral,
		unchecked_extrinsic: *mut u8,
		unchecked_extrinsic_max_size: u32,
		unchecked_extrinsic_size: *mut u32,
	) -> SgxStatus;

	pub fn generate_register_tcb_info_extrinsic(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		collateral: *const CQlQveCollateral,
		unchecked_extrinsic: *mut u8,
		unchecked_extrinsic_max_size: u32,
		unchecked_extrinsic_size: *mut u32,
	) -> SgxStatus;

	pub fn dump_ias_ra_cert_to_disk(eid: EnclaveId, retval: *mut SgxStatus) -> SgxStatus;

	pub fn dump_dcap_ra_cert_to_disk(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		quoting_enclave_target_info: &TargetInfo,
		quote_size: u32,
	) -> SgxStatus;

	pub fn dump_dcap_collateral_to_disk(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		collateral: *const CQlQveCollateral,
	) -> SgxStatus;

	pub fn test_main_entrance(eid: EnclaveId, retval: *mut SgxStatus) -> SgxStatus;

	pub fn call_rpc_methods(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		request: *const u8,
		request_len: u32,
		response: *mut u8,
		response_len: u32,
	) -> SgxStatus;

	pub fn run_state_provisioning_server(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		socket_fd: c_int,
		sign_type: QuoteSignType,
		quoting_enclave_target_info: Option<&TargetInfo>,
		quote_size: Option<&u32>,
		skip_ra: c_int,
	) -> SgxStatus;

	pub fn request_state_provisioning(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		socket_fd: c_int,
		sign_type: QuoteSignType,
		quoting_enclave_target_info: Option<&TargetInfo>,
		quote_size: Option<&u32>,
		shard: *const u8,
		shard_size: u32,
		skip_ra: c_int,
	) -> SgxStatus;

	// litentry
	pub fn migrate_shard(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		new_shard: *const u8,
		shard_size: u32,
	) -> SgxStatus;

	pub fn ignore_parentchain_block_import_validation_until(
		eid: EnclaveId,
		retval: *mut SgxStatus,
		until: *const u32,
	) -> SgxStatus;

}
