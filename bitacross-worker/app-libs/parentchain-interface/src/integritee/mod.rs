/*
	Copyright 2021 Integritee AG

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

mod event_filter;
mod event_handler;

pub use event_filter::FilterableEvents;
pub use event_handler::ParentchainEventHandler;
pub use itp_types::{
	parentchain::{AccountId, Balance, Hash},
	CallIndex, H256,
};
use sp_runtime::traits::BlakeTwo256;

pub type BlockNumber = u32;
pub type Header = sp_runtime::generic::Header<BlockNumber, BlakeTwo256>;
pub type Signature = sp_runtime::MultiSignature;
