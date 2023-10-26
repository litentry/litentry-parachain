use codec::{Decode, Encode};
use lc_stf_task_sender::AssertionBuildRequest;
use sp_core::H256;
use std::{
	sync::{mpsc::Sender, Arc},
	vec::Vec,
};

#[derive(Debug, Clone)]
pub struct VCRequest {
	pub assertion: AssertionBuildRequest,
	pub sender: Sender<Vec<u8>>,
}

#[derive(Encode, Decode)]
pub struct VCResponse {
	pub assertion_request: AssertionBuildRequest,
	pub vc_hash: H256,
	pub vc_payload: Vec<u8>,
	pub vc_index: H256,
}
