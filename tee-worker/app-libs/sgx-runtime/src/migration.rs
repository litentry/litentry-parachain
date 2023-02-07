use super::*;

use crate::{pallet_imt::migrations, Runtime};
use frame_support::{traits::OnRuntimeUpgrade, weights::Weight};

pub struct Upgrade;
impl OnRuntimeUpgrade for Upgrade {
	fn on_runtime_upgrade() -> Weight {
		migrations::migrate_to_v1::<Runtime, IdentityManagement>()
	}
}
