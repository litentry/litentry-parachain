#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use std::{
	string::{String, ToString},
	vec,
	vec::Vec,
};

const DIGICERT_GLOBAL_CA: &str = include_str!("../certificates/digicert_global_root_g2.crt.pem");
const AMAZON_CA: &str = include_str!("../certificates/amazon_root_ca1.pem");
const CLOUDFLARE_CA: &str = include_str!("../certificates/cloudflare_ecc_ca_3ca.pem");
const LETS_ENCRYPT_CA: &str = include_str!("../certificates/lets_encrypt.pem");

pub fn get_ca_certs() -> Vec<String> {
	vec![
		DIGICERT_GLOBAL_CA.to_string(),
		AMAZON_CA.to_string(),
		CLOUDFLARE_CA.to_string(),
		LETS_ENCRYPT_CA.to_string(),
	]
}
