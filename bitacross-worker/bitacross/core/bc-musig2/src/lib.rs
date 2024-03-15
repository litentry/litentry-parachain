#![cfg_attr(not(feature = "std"), no_std)]

extern crate core;
#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

use k256::PublicKey;
use std::collections::HashMap;

// enclave public key is used as signer identifier
type SignerId = [u8; 32];


pub struct MuSig2Instance {
    signers: Vec<SignerId>,
    pubkeys: HashMap<SignerId, Vec<PublicKey>>
}


impl MuSig2Instance {

    pub fn new(signers: Vec<SignerId>) -> Self {
        Self {
            signers,
            pubkeys: Default::default()
        }
    }

}


#[cfg(test)]
pub mod test {
    use crate::MuSig2Instance;

    #[test]
    fn creates_new_instance() {
        let signer_1 = [0_u8; 32];
        let signer_2 = [1_u8; 32];

        let instance = MuSig2Instance::new(vec![signer_1, signer_2]);
    }


}

