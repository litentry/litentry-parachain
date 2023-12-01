#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use std::{
	string::{String, ToString},
	vec,
	vec::Vec,
};

const DIGI_CERT_GLOBAL_ROOT_CERTIFICATE: &str =
	include_str!("../certificates/DigiCertGlobalRootG2.crt.pem");

pub fn get_ca_certs() -> Vec<String> {
	vec![DIGI_CERT_GLOBAL_ROOT_CERTIFICATE.to_string()]
}
