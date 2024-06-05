use super::*;

use crate::Runtime;
use frame_support::{traits::OnRuntimeUpgrade, weights::Weight};

// This is a place we triggre migrations
// For more details, see: https://www.notion.so/web3builders/How-to-upgrade-enclave-one-worker-edfaf5871b4441579f9471074032ed1e
pub struct Upgrade;
impl OnRuntimeUpgrade for Upgrade {
	fn on_runtime_upgrade() -> Weight {
		pallet_imt::migrations::migrate_to_v1::<Runtime, IdentityManagement>()
	}
}
