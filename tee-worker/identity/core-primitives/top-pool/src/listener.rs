// This file is part of Substrate.

// Copyright (C) 2018-2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#[cfg(all(not(feature = "std"), feature = "sgx"))]
use crate::sgx_reexport_prelude::*;

use crate::{primitives::TxHash, watcher::Watcher};

use itc_direct_rpc_server::SendRpcResponse;
use itp_types::BlockHash as SidechainBlockHash;
use linked_hash_map::LinkedHashMap;
use log::*;

use std::{collections::HashMap, string::String, sync::Arc, vec, vec::Vec};

/// Extrinsic pool default listener.
#[derive(Default)]
pub struct Listener<R>
where
	R: SendRpcResponse<Hash = TxHash>,
{
	watchers: HashMap<TxHash, Watcher<R>>,
	finality_watchers: LinkedHashMap<SidechainBlockHash, Vec<TxHash>>,
	rpc_response_sender: Arc<R>,
}

/// Maximum number of blocks awaiting finality at any time.
const MAX_FINALITY_WATCHERS: usize = 512;

impl<R> Listener<R>
where
	R: SendRpcResponse<Hash = TxHash>,
{
	pub fn new(rpc_response_sender: Arc<R>) -> Self {
		Listener {
			watchers: Default::default(),
			finality_watchers: Default::default(),
			rpc_response_sender,
		}
	}

	fn fire<F>(&mut self, hash: &TxHash, fun: F)
	where
		F: FnOnce(&mut Watcher<R>),
	{
		let clean = if let Some(h) = self.watchers.get_mut(hash) {
			fun(h);
			h.is_done()
		} else {
			false
		};

		if clean {
			self.watchers.remove(hash);
		}
	}

	/// Creates a new watcher for given verified extrinsic.
	///
	/// The watcher can be used to subscribe to life-cycle events of that extrinsic.
	pub fn create_watcher(&mut self, hash: TxHash) {
		let new_watcher = Watcher::new_watcher(hash, self.rpc_response_sender.clone());
		self.watchers.insert(hash, new_watcher);
	}

	/// Notify the listeners about extrinsic broadcast.
	pub fn broadcasted(&mut self, hash: &TxHash, peers: Vec<String>) {
		trace!(target: "txpool", "[{:?}] Broadcasted", hash);
		self.fire(hash, |watcher| watcher.broadcast(peers));
	}

	/// Notify listeners about top execution.
	pub fn top_executed(&mut self, hash: &TxHash, response: &[u8], force_wait: bool) {
		trace!(target: "txpool", "[{:?}] Top Executed", hash);
		self.fire(hash, |watcher| watcher.top_executed(response, force_wait));
	}

	/// New operation was added to the ready pool or promoted from the future pool.
	pub fn ready(&mut self, tx: &TxHash, old: Option<&TxHash>) {
		trace!(target: "txpool", "[{:?}] Ready (replaced with {:?})", tx, old);
		self.fire(tx, |watcher| watcher.ready());
		if let Some(old) = old {
			self.fire(old, |watcher| watcher.usurped());
		}
	}

	/// New operation was added to the future pool.
	pub fn future(&mut self, tx: &TxHash) {
		trace!(target: "txpool", "[{:?}] Future", tx);
		self.fire(tx, |watcher| watcher.future());
	}

	/// TrustedOperation was dropped from the pool because of the limit.
	pub fn dropped(&mut self, tx: &TxHash, by: Option<&TxHash>) {
		trace!(target: "txpool", "[{:?}] Dropped (replaced with {:?})", tx, by);
		self.fire(tx, |watcher| match by {
			Some(_) => watcher.usurped(),
			None => watcher.dropped(),
		})
	}

	/// TrustedOperation was removed as invalid.
	pub fn invalid(&mut self, tx: &TxHash) {
		self.fire(tx, |watcher| watcher.invalid());
	}

	/// TrustedOperation was pruned from the pool.
	#[allow(clippy::or_fun_call)]
	pub fn pruned(&mut self, block_hash: SidechainBlockHash, tx: &TxHash) {
		debug!(target: "txpool", "[{:?}] Pruned at {:?}", tx, block_hash);
		self.fire(tx, |s| s.in_block(block_hash));
		self.finality_watchers.entry(block_hash).or_insert(vec![]).push(*tx);

		while self.finality_watchers.len() > MAX_FINALITY_WATCHERS {
			if let Some((_hash, txs)) = self.finality_watchers.pop_front() {
				for tx in txs {
					self.fire(&tx, |s| s.finality_timeout());
				}
			}
		}
	}

	/// TrustedOperation in block.
	pub fn in_block(&mut self, tx: &TxHash, block_hash: SidechainBlockHash) {
		self.fire(tx, |s| s.in_block(block_hash));
	}

	/// The block this operation was included in has been retracted.
	pub fn retracted(&mut self, block_hash: SidechainBlockHash) {
		if let Some(hashes) = self.finality_watchers.remove(&block_hash) {
			for hash in hashes {
				self.fire(&hash, |s| s.retracted())
			}
		}
	}

	/// Notify all watchers that operations have been finalized
	pub fn finalized(&mut self, block_hash: SidechainBlockHash) {
		if let Some(hashes) = self.finality_watchers.remove(&block_hash) {
			for hash in hashes {
				debug!(target: "txpool", "[{:?}] Sent finalization event (block {:?})", hash, block_hash);
				self.fire(&hash, |s| s.finalized())
			}
		}
	}

	/// Litentry: set the rpc response value and force_wait flag for a given TrustedOperation `tx`.
	pub fn update_connection_state(
		&mut self,
		tx: &TxHash,
		encoded_value: Vec<u8>,
		force_wait: bool,
	) {
		self.fire(tx, |s| s.update_connection_state(encoded_value, force_wait));
	}

	// Direct vc request is not going through top pool. The hash is only added within "handle_message()" to
	// connection_registry. So Here use rpc_response_sender directly.
	pub fn send_rpc_response(&mut self, hash: TxHash, encoded_value: Vec<u8>, do_watch: bool) {
		if let Err(e) = self.rpc_response_sender.send_rpc_response(hash, encoded_value, do_watch) {
			warn!("failed to update connection state: {:?}", e);
		}
	}

	/// Litentry: swap the old hash with the new one in rpc connection registry
	pub fn swap_rpc_connection_hash(&mut self, old_hash: TxHash, new_hash: TxHash) {
		info!("Swapping connection {:?} to {:?}", &old_hash, &new_hash);
		// It's possible that the old top (hash) is already removed from the pool when we
		// request to swap hashes, in this case we just create one to facilitate the swap
		if let Some(w) = self.watchers.get(&old_hash) {
			w.swap_rpc_connection_hash(new_hash);
		} else {
			// do not insert it to `watchers`, will be deallocated if it goes out of scope
			Watcher::new_watcher(old_hash, self.rpc_response_sender.clone())
				.swap_rpc_connection_hash(new_hash);
		}
	}
}
