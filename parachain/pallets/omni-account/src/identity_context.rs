use crate::{BlockNumberFor, Config, Web3Network};
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

#[derive(Clone, Eq, PartialEq, Default, Debug, Encode, Decode, TypeInfo)]
pub enum IdentityStatus {
	#[default]
	#[codec(index = 0)]
	Active,
	#[codec(index = 1)]
	Inactive,
}

// The context associated with the (litentry-account, did) pair
#[derive(Clone, Eq, PartialEq, Default, Debug, Encode, Decode, TypeInfo)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct IdentityContext<T: Config> {
	// the block number at which the identity is linked
	pub link_block: BlockNumberFor<T>,
	// a list of web3 networks on which the identity should be used
	pub web3networks: Vec<Web3Network>,
	// the identity status
	pub status: IdentityStatus,
}

impl<T: Config> IdentityContext<T> {
	pub fn new(link_block: BlockNumberFor<T>, web3networks: Vec<Web3Network>) -> Self {
		Self { link_block, web3networks: Self::dedup(web3networks), status: IdentityStatus::Active }
	}

	pub fn set_web3networks(&mut self, web3networks: Vec<Web3Network>) {
		self.web3networks = Self::dedup(web3networks);
	}

	pub fn deactivate(&mut self) {
		self.status = IdentityStatus::Inactive
	}

	pub fn activate(&mut self) {
		self.status = IdentityStatus::Active
	}

	// a small helper fn to apply mutable changes
	fn dedup(mut web3networks: Vec<Web3Network>) -> Vec<Web3Network> {
		web3networks.sort();
		web3networks.dedup();
		web3networks
	}
}
