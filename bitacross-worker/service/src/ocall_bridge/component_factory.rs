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

use crate::{
	globals::tokio_handle::GetTokioHandle,
	ocall_bridge::{
		bridge_api::{
			GetOCallBridgeComponents, IpfsBridge, MetricsBridge, RemoteAttestationBridge,
			WorkerOnChainBridge,
		},
		ipfs_ocall::IpfsOCall,
		metrics_ocall::MetricsOCall,
		remote_attestation_ocall::RemoteAttestationOCall,
		worker_on_chain_ocall::WorkerOnChainOCall,
	},
	prometheus_metrics::ReceiveEnclaveMetrics,
	worker_peers_registry::PeersRegistry,
};
use itp_enclave_api::{enclave_base::EnclaveBase, remote_attestation::RemoteAttestationCallBacks};
use itp_node_api::node_api_factory::CreateNodeApi;
use std::sync::Arc;

/// Concrete implementation, should be moved out of the OCall Bridge, into the worker
/// since the OCall bridge itself should not know any concrete types to ensure
/// our dependency graph is worker -> ocall bridge
pub struct OCallBridgeComponentFactory<
	NodeApi,
	EnclaveApi,
	WorkerPeersRegistry,
	TokioHandle,
	MetricsReceiver,
> {
	integritee_rpc_api_factory: Arc<NodeApi>,
	target_a_parentchain_rpc_api_factory: Option<Arc<NodeApi>>,
	target_b_parentchain_rpc_api_factory: Option<Arc<NodeApi>>,
	enclave_api: Arc<EnclaveApi>,
	peers_registry: Arc<WorkerPeersRegistry>,
	tokio_handle: Arc<TokioHandle>,
	metrics_receiver: Arc<MetricsReceiver>,
}

impl<NodeApi, EnclaveApi, WorkerPeersRegistry, TokioHandle, MetricsReceiver>
	OCallBridgeComponentFactory<NodeApi, EnclaveApi, WorkerPeersRegistry, TokioHandle, MetricsReceiver>
{
	#[allow(clippy::too_many_arguments)]
	pub fn new(
		integritee_rpc_api_factory: Arc<NodeApi>,
		target_a_parentchain_rpc_api_factory: Option<Arc<NodeApi>>,
		target_b_parentchain_rpc_api_factory: Option<Arc<NodeApi>>,
		enclave_api: Arc<EnclaveApi>,
		peers_registry: Arc<WorkerPeersRegistry>,
		tokio_handle: Arc<TokioHandle>,
		metrics_receiver: Arc<MetricsReceiver>,
	) -> Self {
		OCallBridgeComponentFactory {
			integritee_rpc_api_factory,
			target_a_parentchain_rpc_api_factory,
			target_b_parentchain_rpc_api_factory,
			enclave_api,
			peers_registry,
			tokio_handle,
			metrics_receiver,
		}
	}
}

impl<NodeApi, EnclaveApi, WorkerPeersRegistry, TokioHandle, MetricsReceiver>
	GetOCallBridgeComponents
	for OCallBridgeComponentFactory<
		NodeApi,
		EnclaveApi,
		WorkerPeersRegistry,
		TokioHandle,
		MetricsReceiver,
	> where
	NodeApi: CreateNodeApi + 'static,
	EnclaveApi: EnclaveBase + RemoteAttestationCallBacks + 'static,
	WorkerPeersRegistry: PeersRegistry + 'static,
	TokioHandle: GetTokioHandle + 'static,
	MetricsReceiver: ReceiveEnclaveMetrics + 'static,
{
	fn get_ra_api(&self) -> Arc<dyn RemoteAttestationBridge> {
		Arc::new(RemoteAttestationOCall::new(self.enclave_api.clone()))
	}

	fn get_oc_api(&self) -> Arc<dyn WorkerOnChainBridge> {
		Arc::new(WorkerOnChainOCall::new(
			self.enclave_api.clone(),
			self.integritee_rpc_api_factory.clone(),
			self.target_a_parentchain_rpc_api_factory.clone(),
			self.target_b_parentchain_rpc_api_factory.clone(),
		))
	}

	fn get_ipfs_api(&self) -> Arc<dyn IpfsBridge> {
		Arc::new(IpfsOCall {})
	}

	fn get_metrics_api(&self) -> Arc<dyn MetricsBridge> {
		Arc::new(MetricsOCall::new(self.metrics_receiver.clone()))
	}
}
