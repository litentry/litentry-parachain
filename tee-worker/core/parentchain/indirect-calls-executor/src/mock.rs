use crate::{
	error::{Error, Result as ICResult},
	filter_metadata::{EventsFromMetadata, FilterIntoDataFrom},
	IndirectDispatch,
};
use codec::{Decode, Encode};
use core::marker::PhantomData;
use litentry_primitives::DecryptableRequest;

use itp_api_client_types::{ParentchainSignedExtra, StaticEvent};
use itp_node_api::{
	api_client::{CallIndex, PairSignature, UncheckedExtrinsicV4},
	metadata::NodeMetadataTrait,
};
use itp_stf_primitives::{traits::IndirectExecutor, types::Signature};
use itp_test::mock::stf_mock::TrustedCallSignedMock;
use itp_types::{
	parentchain::{
		events::{
			ActivateIdentityRequested, DeactivateIdentityRequested, LinkIdentityRequested,
			OpaqueTaskPosted, ScheduledEnclaveRemoved, ScheduledEnclaveSet, VCRequested,
		},
		ExtrinsicStatus, FilterEvents, HandleParentchainEvents,
	},
	Address, RsaRequest, H256,
};
use log::*;
use std::vec::Vec;

pub struct TestEventCreator;

impl<NodeMetadata> EventsFromMetadata<NodeMetadata> for TestEventCreator {
	type Output = MockEvents;

	fn create_from_metadata(
		_metadata: NodeMetadata,
		_block_hash: H256,
		_events: &[u8],
	) -> Option<Self::Output> {
		Some(MockEvents)
	}
}

pub struct MockEvents;

impl FilterEvents for MockEvents {
	type Error = ();
	fn get_extrinsic_statuses(&self) -> core::result::Result<Vec<ExtrinsicStatus>, Self::Error> {
		Ok(Vec::from([ExtrinsicStatus::Success]))
	}

	fn get_opaque_task_posted_events(
		&self,
	) -> core::result::Result<Vec<OpaqueTaskPosted>, Self::Error> {
		let opaque_task_posted_event =
			OpaqueTaskPosted { request: RsaRequest::new(H256::default(), Vec::from([0u8; 32])) };
		Ok(Vec::from([opaque_task_posted_event]))
	}

	fn get_link_identity_events(
		&self,
	) -> core::result::Result<Vec<itp_types::parentchain::events::LinkIdentityRequested>, Self::Error>
	{
		Ok(Vec::new())
	}

	fn get_vc_requested_events(
		&self,
	) -> core::result::Result<Vec<itp_types::parentchain::events::VCRequested>, Self::Error> {
		Ok(Vec::new())
	}

	fn get_deactivate_identity_events(
		&self,
	) -> core::result::Result<Vec<DeactivateIdentityRequested>, Self::Error> {
		Ok(Vec::new())
	}

	fn get_activate_identity_events(
		&self,
	) -> core::result::Result<Vec<ActivateIdentityRequested>, Self::Error> {
		Ok(Vec::new())
	}

	fn get_scheduled_enclave_set_events(
		&self,
	) -> core::result::Result<Vec<itp_types::parentchain::events::ScheduledEnclaveSet>, Self::Error>
	{
		Ok(Vec::new())
	}

	fn get_scheduled_enclave_removed_events(
		&self,
	) -> core::result::Result<
		Vec<itp_types::parentchain::events::ScheduledEnclaveRemoved>,
		Self::Error,
	> {
		Ok(Vec::new())
	}
}

pub struct MockParentchainEventHandler {}

impl<Executor> HandleParentchainEvents<Executor, TrustedCallSignedMock, Error>
	for MockParentchainEventHandler
where
	Executor: IndirectExecutor<TrustedCallSignedMock, Error>,
{
	fn handle_events(
		_: &Executor,
		_: impl itp_types::parentchain::FilterEvents,
	) -> core::result::Result<Vec<H256>, Error> {
		Ok(Vec::from([H256::default()]))
	}
}
