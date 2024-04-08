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
	parentchain::{ExtrinsicStatus, FilterEvents, HandleParentchainEvents},
	Address, RsaRequest, H256,
};
use log::*;
use std::vec::Vec;

/// Default filter we use for the Integritee-Parachain.
pub struct MockExtrinsicFilter<ExtrinsicParser> {
	_phantom: PhantomData<ExtrinsicParser>,
}

impl<ExtrinsicParser, NodeMetadata: NodeMetadataTrait> FilterIntoDataFrom<NodeMetadata>
	for MockExtrinsicFilter<ExtrinsicParser>
where
	ExtrinsicParser: ParseExtrinsic,
{
	type Output = IndirectCall;
	type ParseParentchainMetadata = ExtrinsicParser;

	fn filter_into_from_metadata(
		encoded_data: &[u8],
		metadata: &NodeMetadata,
	) -> Option<Self::Output> {
		let call_mut = &mut &encoded_data[..];

		// Todo: the filter should not need to parse, only filter. This should directly be configured
		// in the indirect executor.
		let xt = match Self::ParseParentchainMetadata::parse(call_mut) {
			Ok(xt) => xt,
			Err(e) => {
				log::error!("[InvokeFilter] Could not parse parentchain extrinsic: {:?}", e);
				return None
			},
		};
		let index = xt.call_index;
		let call_args = &mut &xt.call_args[..];
		log::trace!("[AndInvokeFilter] attempting to execute indirect call with index {:?}", index);
		if index == metadata.post_opaque_task_call_indexes().ok()? {
			log::debug!("executing invoke call");
			let args = InvokeArgs::decode(call_args).unwrap();
			Some(IndirectCall::Invoke(args))
		} else {
			None
		}
	}
}
pub struct ExtrinsicParser<SignedExtra> {
	_phantom: PhantomData<SignedExtra>,
}

/// Parses the extrinsics corresponding to the parentchain.
pub type MockParentchainExtrinsicParser = ExtrinsicParser<ParentchainSignedExtra>;

/// Partially interpreted extrinsic containing the `signature` and the `call_index` whereas
/// the `call_args` remain in encoded form.
///
/// Intended for usage, where the actual `call_args` form is unknown.
pub struct SemiOpaqueExtrinsic<'a> {
	/// Signature of the Extrinsic.
	pub signature: Signature,
	/// Call index of the dispatchable.
	pub call_index: CallIndex,
	/// Encoded arguments of the dispatchable corresponding to the `call_index`.
	pub call_args: &'a [u8],
}

/// Trait to extract signature and call indexes of an encoded [UncheckedExtrinsicV4].
pub trait ParseExtrinsic {
	/// Signed extra of the extrinsic.
	type SignedExtra;

	fn parse(encoded_call: &[u8]) -> Result<SemiOpaqueExtrinsic, codec::Error>;
}

impl<SignedExtra> ParseExtrinsic for ExtrinsicParser<SignedExtra>
where
	SignedExtra: Decode + Encode,
{
	type SignedExtra = SignedExtra;

	/// Extract a call index of an encoded call.
	fn parse(encoded_call: &[u8]) -> Result<SemiOpaqueExtrinsic, codec::Error> {
		let call_mut = &mut &encoded_call[..];

		// `()` is a trick to stop decoding after the call index. So the remaining bytes
		//  of `call` after decoding only contain the parentchain's dispatchable's arguments.
		let xt = UncheckedExtrinsicV4::<
            Address,
            (CallIndex, ()),
            PairSignature,
            Self::SignedExtra,
        >::decode(call_mut)?;

		Ok(SemiOpaqueExtrinsic {
			signature: xt.signature.unwrap().1,
			call_index: xt.function.0,
			call_args: call_mut,
		})
	}
}
/// The default indirect call (extrinsic-triggered) of the Integritee-Parachain.
#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub enum IndirectCall {
	Invoke(InvokeArgs),
}

impl<Executor: IndirectExecutor<TrustedCallSignedMock, Error>>
	IndirectDispatch<Executor, TrustedCallSignedMock> for IndirectCall
{
	type Args = ();
	fn dispatch(&self, executor: &Executor, args: Self::Args) -> ICResult<()> {
		trace!("dispatching indirect call {:?}", self);
		match self {
			IndirectCall::Invoke(invoke_args) => invoke_args.dispatch(executor, args),
		}
	}
}

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

	fn get_events<T: StaticEvent>(&self) -> core::result::Result<Vec<T>, Self::Error> {
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
	) -> core::result::Result<(), Error> {
		Ok(())
	}
}

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct InvokeArgs {
	request: RsaRequest,
}

impl<Executor: IndirectExecutor<TrustedCallSignedMock, Error>>
	IndirectDispatch<Executor, TrustedCallSignedMock> for InvokeArgs
{
	type Args = ();
	fn dispatch(&self, executor: &Executor, _args: Self::Args) -> ICResult<()> {
		log::debug!("Found trusted call extrinsic, submitting it to the top pool");
		executor.submit_trusted_call(self.request.shard(), self.request.payload().to_vec());
		Ok(())
	}
}
