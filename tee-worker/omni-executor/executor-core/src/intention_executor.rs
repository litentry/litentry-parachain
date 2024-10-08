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

use async_trait::async_trait;
use tokio::sync::mpsc;

use crate::primitives::Intention;

/// Used to perform intention on destination chain
#[async_trait]
pub trait IntentionExecutor: Send {
	async fn execute(&self, intention: Intention) -> Result<(), ()>;
}

pub struct MockedIntentionExecutor {
	sender: mpsc::UnboundedSender<()>,
}

impl MockedIntentionExecutor {
	pub fn new() -> (Self, mpsc::UnboundedReceiver<()>) {
		let (sender, receiver) = mpsc::unbounded_channel();
		(Self { sender }, receiver)
	}
}

#[async_trait]
impl IntentionExecutor for MockedIntentionExecutor {
	async fn execute(&self, _intention: Intention) -> Result<(), ()> {
		self.sender.send(()).map_err(|_| ())
	}
}
