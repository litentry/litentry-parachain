use itc_peer_top_broadcaster::PeerUpdater;
use sgx_tstd::{string::String, vec::Vec};

pub struct PeerUpdaterMock {}

impl PeerUpdater for PeerUpdaterMock {
	fn update(&self, _peers: Vec<String>) {}
}
