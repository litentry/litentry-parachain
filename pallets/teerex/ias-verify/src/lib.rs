/*
	Copyright 2021 Integritee AG and Supercomputing Systems AG

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

#![cfg_attr(not(feature = "std"), no_std)]

use crate::netscape_comment::NetscapeComment;
use chrono::prelude::*;
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde_json::Value;
use sp_std::{
	convert::{TryFrom, TryInto},
	prelude::*,
	str::FromStr,
};

mod ephemeral_key;
mod netscape_comment;
#[cfg(test)]
mod tests;
mod utils;

const SGX_REPORT_DATA_SIZE: usize = 64;
#[derive(Encode, Decode, Copy, Clone, TypeInfo)]
pub struct SgxReportData {
	d: [u8; SGX_REPORT_DATA_SIZE],
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, TypeInfo, sp_core::RuntimeDebug)]
pub struct SGXAttributes {
	flags: u64,
	xfrm: u64,
}

// see Intel SGX SDK https://github.com/intel/linux-sgx/blob/master/common/inc/sgx_report.h
const SGX_REPORT_BODY_RESERVED1_BYTES: usize = 12;
const SGX_REPORT_BODY_RESERVED2_BYTES: usize = 32;
const SGX_REPORT_BODY_RESERVED3_BYTES: usize = 32;
const SGX_REPORT_BODY_RESERVED4_BYTES: usize = 42;
const SGX_FLAGS_DEBUG: u64 = 0x0000000000000002;

#[derive(Encode, Decode, Copy, Clone, TypeInfo)]
pub struct SgxReportBody {
	cpu_svn: [u8; 16],    /* (  0) Security Version of the CPU */
	misc_select: [u8; 4], /* ( 16) Which fields defined in SSA.MISC */
	reserved1: [u8; SGX_REPORT_BODY_RESERVED1_BYTES], /* ( 20) */
	isv_ext_prod_id: [u8; 16], /* ( 32) ISV assigned Extended Product ID */
	attributes: SGXAttributes, /* ( 48) Any special Capabilities the Enclave possess */
	mr_enclave: [u8; 32], /* ( 64) The value of the enclave's ENCLAVE measurement */
	reserved2: [u8; SGX_REPORT_BODY_RESERVED2_BYTES], /* ( 96) */
	mr_signer: [u8; 32],  /* (128) The value of the enclave's SIGNER measurement */
	reserved3: [u8; SGX_REPORT_BODY_RESERVED3_BYTES], /* (160) */
	config_id: [u8; 64],  /* (192) CONFIGID */
	isv_prod_id: u16,     /* (256) Product ID of the Enclave */
	isv_svn: u16,         /* (258) Security Version of the Enclave */
	config_svn: u16,      /* (260) CONFIGSVN */
	reserved4: [u8; SGX_REPORT_BODY_RESERVED4_BYTES], /* (262) */
	isv_family_id: [u8; 16], /* (304) ISV assigned Family ID */
	report_data: SgxReportData, /* (320) Data provided by the user */
}

impl SgxReportBody {
	pub fn sgx_build_mode(&self) -> SgxBuildMode {
		#[cfg(test)]
		println!("attributes flag : {:x}", self.attributes.flags);
		if self.attributes.flags & SGX_FLAGS_DEBUG == SGX_FLAGS_DEBUG {
			SgxBuildMode::Debug
		} else {
			SgxBuildMode::Production
		}
	}
}

// signature_len: u32, // 432
// signature: [u8; 64] // 436
// must be hard-coded for SCALE codec */
// see Intel SGX SDK https://github.com/intel/linux-sgx/blob/master/common/inc/sgx_quote.h
#[derive(Encode, Decode, Copy, Clone, TypeInfo)]
pub struct SgxQuote {
	version: u16,               /* 0 */
	sign_type: u16,             /* 2 */
	epid_group_id: u32,         /* 4 */
	qe_svn: u16,                /* 8 */
	pce_svn: u16,               /* 10 */
	xeid: u32,                  /* 12 */
	basename: [u8; 32],         /* 16 */
	report_body: SgxReportBody, /* 48 */
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub enum SgxBuildMode {
	Debug,
	Production,
}
impl Default for SgxBuildMode {
	fn default() -> Self {
		SgxBuildMode::Production
	}
}

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub enum SgxStatus {
	Invalid,
	Ok,
	GroupOutOfDate,
	GroupRevoked,
	ConfigurationNeeded,
	SignatureRevoked,
	KeyRevoked,
	SigrlVersionMismatch,
	SwHardeningNeeded,
	ConfigurationAndSwHardeningNeeded,
}
impl Default for SgxStatus {
	fn default() -> Self {
		SgxStatus::Invalid
	}
}

impl FromStr for SgxStatus {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"OK" => Ok(SgxStatus::Ok),
			"GROUP_OUT_OF_DATE" => Ok(SgxStatus::GroupOutOfDate),
			"GROUP_REVOKED" => Ok(SgxStatus::GroupRevoked),
			"CONFIGURATION_NEEDED" => Ok(SgxStatus::ConfigurationNeeded),
			"SIGNATURE_REVOKED" => Ok(SgxStatus::SignatureRevoked),
			"KEY_REVOKED" => Ok(SgxStatus::KeyRevoked),
			"SIGRL_VERSION_MISMATCH" => Ok(SgxStatus::SigrlVersionMismatch),
			"SW_HARDENING_NEEDED" => Ok(SgxStatus::SwHardeningNeeded),
			"CONFIGURATION_AND_SW_HARDENING_NEEDED" =>
				Ok(SgxStatus::ConfigurationAndSwHardeningNeeded),

			_ => Ok(SgxStatus::Invalid),
		}
	}
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, sp_core::RuntimeDebug, TypeInfo)]
pub struct SgxReport {
	pub mr_enclave: [u8; 32],
	pub pubkey: [u8; 32],
	pub status: SgxStatus,
	pub timestamp: u64, // unix timestamp in milliseconds
	pub build_mode: SgxBuildMode,
	pub metadata: SgxEnclaveMetadata,
}

#[derive(Encode, Decode, Clone, TypeInfo, PartialEq, Eq, sp_core::RuntimeDebug, Default)]
pub struct SgxQuoteInputs {
	pub spid: [u8; 16],
	pub nonce: [u8; 16],

	// the revocation list
	pub sig_rl: Vec<u8>,
}

impl SgxQuoteInputs {
	pub fn new(spid: Vec<u8>, nonce: Vec<u8>, sig_rl: Vec<u8>) -> Self {
		let mut d_spid = [0_u8; 16];
		if spid.len() >= 16 {
			d_spid.copy_from_slice(&spid[..16]);
		}

		let mut d_nonce = [0_u8; 16];
		if nonce.len() >= 16 {
			d_nonce.copy_from_slice(&nonce[..16]);
		}

		SgxQuoteInputs { spid: d_spid, nonce: d_nonce, sig_rl }
	}
}

#[derive(Encode, Decode, Clone, TypeInfo, Default, sp_core::RuntimeDebug)]
pub struct SgxQuoteAdd {
	pub quote_inputs: SgxQuoteInputs,
	pub quote: Vec<u8>,
}
impl SgxQuoteAdd {
	pub fn new(quote_inputs: SgxQuoteInputs, quote: Vec<u8>) -> Self {
		SgxQuoteAdd { quote_inputs, quote }
	}
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, sp_core::RuntimeDebug)]
pub struct SgxTargetInfo {
	// target info
	pub mr_enclave: [u8; 32],
	pub attributes: SGXAttributes,
	pub reserved1: [u8; SGX_REPORT_BODY_RESERVED1_BYTES],
	pub config_svn: u16,
	pub misc_select: [u8; 4],
	pub reserved2: [u8; SGX_REPORT_BODY_RESERVED2_BYTES],
	pub config_id: [u8; 64],
	pub reserved3: [u8; SGX_REPORT_BODY_RESERVED3_BYTES],
}
impl Default for SgxTargetInfo {
	fn default() -> Self {
		SgxTargetInfo {
			mr_enclave: [0_u8; 32],
			attributes: SGXAttributes { flags: 0_u64, xfrm: 0_u64 },
			reserved1: [0_u8; SGX_REPORT_BODY_RESERVED1_BYTES],
			config_svn: 0_u16,
			misc_select: [0_u8; 4],
			reserved2: [0_u8; SGX_REPORT_BODY_RESERVED2_BYTES],
			config_id: [0_u8; 64],
			reserved3: [0_u8; SGX_REPORT_BODY_RESERVED3_BYTES],
		}
	}
}

#[derive(Encode, Decode, Clone, TypeInfo, PartialEq, Eq, Default, sp_core::RuntimeDebug)]
pub struct SgxReportInputs {
	pub target_info: SgxTargetInfo,
	pub report_data: [u8; 32],
}

#[derive(Encode, Decode, Clone, TypeInfo, PartialEq, Eq, Default, sp_core::RuntimeDebug)]
pub struct SgxEnclaveMetadata {
	pub report_inputs: SgxReportInputs,
	pub quote_inputs: SgxQuoteInputs,
	pub isv_enclave_quote: Vec<u8>,
	pub sgx_status: SgxStatus,
}
impl SgxEnclaveMetadata {
	pub fn new(
		report_inputs: SgxReportInputs,
		quote_inputs: SgxQuoteInputs,
		isv_enclave_quote: Vec<u8>,
		sgx_status: SgxStatus,
	) -> Self {
		SgxEnclaveMetadata { report_inputs, quote_inputs, isv_enclave_quote, sgx_status }
	}

	pub fn build_report_inputs(sgx_quote: &SgxQuote) -> SgxReportInputs {
		let target_info = SgxTargetInfo {
			mr_enclave: sgx_quote.report_body.mr_enclave,
			attributes: sgx_quote.report_body.attributes,
			reserved1: sgx_quote.report_body.reserved1,
			config_svn: sgx_quote.report_body.config_svn,
			misc_select: sgx_quote.report_body.misc_select,
			reserved2: sgx_quote.report_body.reserved2,
			config_id: sgx_quote.report_body.config_id,
			reserved3: sgx_quote.report_body.reserved3,
		};

		let mut report_data = [0u8; 32];
		report_data.copy_from_slice(&sgx_quote.report_body.report_data.d[..32]);

		SgxReportInputs { target_info, report_data }
	}

	pub fn build_sgx_metadata(
		netscape: &NetscapeComment,
		sgx_quote: &SgxQuote,
		sgx_status: SgxStatus,
	) -> Self {
		match netscape.quote_add.as_ref() {
			Some(quote_add) => {
				let report_inputs = SgxEnclaveMetadata::build_report_inputs(sgx_quote);
				let quote_inputs = quote_add.clone().quote_inputs;
				let isv_enclave_quote = quote_add.quote.clone();

				SgxEnclaveMetadata::new(report_inputs, quote_inputs, isv_enclave_quote, sgx_status)
			},
			None => SgxEnclaveMetadata::default(),
		}
	}
}

type SignatureAlgorithms = &'static [&'static webpki::SignatureAlgorithm];
static SUPPORTED_SIG_ALGS: SignatureAlgorithms = &[
	//&webpki::ECDSA_P256_SHA256,
	//&webpki::ECDSA_P256_SHA384,
	//&webpki::ECDSA_P384_SHA256,
	//&webpki::ECDSA_P384_SHA384,
	&webpki::RSA_PKCS1_2048_8192_SHA256,
	&webpki::RSA_PKCS1_2048_8192_SHA384,
	&webpki::RSA_PKCS1_2048_8192_SHA512,
	&webpki::RSA_PKCS1_3072_8192_SHA384,
];

//pub const IAS_REPORT_CA: &[u8] = include_bytes!("../AttestationReportSigningCACert.pem");

pub static IAS_SERVER_ROOTS: webpki::TLSServerTrustAnchors = webpki::TLSServerTrustAnchors(&[
	/*
	 * -----BEGIN CERTIFICATE-----
	 * MIIFSzCCA7OgAwIBAgIJANEHdl0yo7CUMA0GCSqGSIb3DQEBCwUAMH4xCzAJBgNV
	 * BAYTAlVTMQswCQYDVQQIDAJDQTEUMBIGA1UEBwwLU2FudGEgQ2xhcmExGjAYBgNV
	 * BAoMEUludGVsIENvcnBvcmF0aW9uMTAwLgYDVQQDDCdJbnRlbCBTR1ggQXR0ZXN0
	 * YXRpb24gUmVwb3J0IFNpZ25pbmcgQ0EwIBcNMTYxMTE0MTUzNzMxWhgPMjA0OTEy
	 * MzEyMzU5NTlaMH4xCzAJBgNVBAYTAlVTMQswCQYDVQQIDAJDQTEUMBIGA1UEBwwL
	 * U2FudGEgQ2xhcmExGjAYBgNVBAoMEUludGVsIENvcnBvcmF0aW9uMTAwLgYDVQQD
	 * DCdJbnRlbCBTR1ggQXR0ZXN0YXRpb24gUmVwb3J0IFNpZ25pbmcgQ0EwggGiMA0G
	 * CSqGSIb3DQEBAQUAA4IBjwAwggGKAoIBgQCfPGR+tXc8u1EtJzLA10Feu1Wg+p7e
	 * LmSRmeaCHbkQ1TF3Nwl3RmpqXkeGzNLd69QUnWovYyVSndEMyYc3sHecGgfinEeh
	 * rgBJSEdsSJ9FpaFdesjsxqzGRa20PYdnnfWcCTvFoulpbFR4VBuXnnVLVzkUvlXT
	 * L/TAnd8nIZk0zZkFJ7P5LtePvykkar7LcSQO85wtcQe0R1Raf/sQ6wYKaKmFgCGe
	 * NpEJUmg4ktal4qgIAxk+QHUxQE42sxViN5mqglB0QJdUot/o9a/V/mMeH8KvOAiQ
	 * byinkNndn+Bgk5sSV5DFgF0DffVqmVMblt5p3jPtImzBIH0QQrXJq39AT8cRwP5H
	 * afuVeLHcDsRp6hol4P+ZFIhu8mmbI1u0hH3W/0C2BuYXB5PC+5izFFh/nP0lc2Lf
	 * 6rELO9LZdnOhpL1ExFOq9H/B8tPQ84T3Sgb4nAifDabNt/zu6MmCGo5U8lwEFtGM
	 * RoOaX4AS+909x00lYnmtwsDVWv9vBiJCXRsCAwEAAaOByTCBxjBgBgNVHR8EWTBX
	 * MFWgU6BRhk9odHRwOi8vdHJ1c3RlZHNlcnZpY2VzLmludGVsLmNvbS9jb250ZW50
	 * L0NSTC9TR1gvQXR0ZXN0YXRpb25SZXBvcnRTaWduaW5nQ0EuY3JsMB0GA1UdDgQW
	 * BBR4Q3t2pn680K9+QjfrNXw7hwFRPDAfBgNVHSMEGDAWgBR4Q3t2pn680K9+Qjfr
	 * NXw7hwFRPDAOBgNVHQ8BAf8EBAMCAQYwEgYDVR0TAQH/BAgwBgEB/wIBADANBgkq
	 * hkiG9w0BAQsFAAOCAYEAeF8tYMXICvQqeXYQITkV2oLJsp6J4JAqJabHWxYJHGir
	 * IEqucRiJSSx+HjIJEUVaj8E0QjEud6Y5lNmXlcjqRXaCPOqK0eGRz6hi+ripMtPZ
	 * sFNaBwLQVV905SDjAzDzNIDnrcnXyB4gcDFCvwDFKKgLRjOB/WAqgscDUoGq5ZVi
	 * zLUzTqiQPmULAQaB9c6Oti6snEFJiCQ67JLyW/E83/frzCmO5Ru6WjU4tmsmy8Ra
	 * Ud4APK0wZTGtfPXU7w+IBdG5Ez0kE1qzxGQaL4gINJ1zMyleDnbuS8UicjJijvqA
	 * 152Sq049ESDz+1rRGc2NVEqh1KaGXmtXvqxXcTB+Ljy5Bw2ke0v8iGngFBPqCTVB
	 * 3op5KBG3RjbF6RRSzwzuWfL7QErNC8WEy5yDVARzTA5+xmBc388v9Dm21HGfcC8O
	 * DD+gT9sSpssq0ascmvH49MOgjt1yoysLtdCtJW/9FZpoOypaHx0R+mJTLwPXVMrv
	 * DaVzWh5aiEx+idkSGMnX
	 * -----END CERTIFICATE-----
	 */
	webpki::TrustAnchor {
		subject: b"1\x0b0\t\x06\x03U\x04\x06\x13\x02US1\x0b0\t\x06\x03U\x04\x08\x0c\x02CA1\x140\x12\x06\x03U\x04\x07\x0c\x0bSanta Clara1\x1a0\x18\x06\x03U\x04\n\x0c\x11Intel Corporation100.\x06\x03U\x04\x03\x0c\'Intel SGX Attestation Report Signing CA",
		spki: b"0\r\x06\t*\x86H\x86\xf7\r\x01\x01\x01\x05\x00\x03\x82\x01\x8f\x000\x82\x01\x8a\x02\x82\x01\x81\x00\x9f<d~\xb5w<\xbbQ-\'2\xc0\xd7A^\xbbU\xa0\xfa\x9e\xde.d\x91\x99\xe6\x82\x1d\xb9\x10\xd51w7\twFjj^G\x86\xcc\xd2\xdd\xeb\xd4\x14\x9dj/c%R\x9d\xd1\x0c\xc9\x877\xb0w\x9c\x1a\x07\xe2\x9cG\xa1\xae\x00IHGlH\x9fE\xa5\xa1]z\xc8\xec\xc6\xac\xc6E\xad\xb4=\x87g\x9d\xf5\x9c\t;\xc5\xa2\xe9ilTxT\x1b\x97\x9euKW9\x14\xbeU\xd3/\xf4\xc0\x9d\xdf\'!\x994\xcd\x99\x05\'\xb3\xf9.\xd7\x8f\xbf)$j\xbe\xcbq$\x0e\xf3\x9c-q\x07\xb4GTZ\x7f\xfb\x10\xeb\x06\nh\xa9\x85\x80!\x9e6\x91\tRh8\x92\xd6\xa5\xe2\xa8\x08\x03\x19>@u1@N6\xb3\x15b7\x99\xaa\x82Pt@\x97T\xa2\xdf\xe8\xf5\xaf\xd5\xfec\x1e\x1f\xc2\xaf8\x08\x90o(\xa7\x90\xd9\xdd\x9f\xe0`\x93\x9b\x12W\x90\xc5\x80]\x03}\xf5j\x99S\x1b\x96\xdei\xde3\xed\"l\xc1 }\x10B\xb5\xc9\xab\x7f@O\xc7\x11\xc0\xfeGi\xfb\x95x\xb1\xdc\x0e\xc4i\xea\x1a%\xe0\xff\x99\x14\x88n\xf2i\x9b#[\xb4\x84}\xd6\xff@\xb6\x06\xe6\x17\x07\x93\xc2\xfb\x98\xb3\x14X\x7f\x9c\xfd%sb\xdf\xea\xb1\x0b;\xd2\xd9vs\xa1\xa4\xbdD\xc4S\xaa\xf4\x7f\xc1\xf2\xd3\xd0\xf3\x84\xf7J\x06\xf8\x9c\x08\x9f\r\xa6\xcd\xb7\xfc\xee\xe8\xc9\x82\x1a\x8eT\xf2\\\x04\x16\xd1\x8cF\x83\x9a_\x80\x12\xfb\xdd=\xc7M%by\xad\xc2\xc0\xd5Z\xffo\x06\"B]\x1b\x02\x03\x01\x00\x01",
		name_constraints: None
	},

]);

/// Contains an unvalidated ias remote attestation certificate.
///
/// Wrapper to implemented parsing and verification traits on it.
pub struct CertDer<'a>(&'a [u8]);

// make sure this function doesn't panic!
pub fn verify_ias_report(cert_der: &[u8]) -> Result<SgxReport, &'static str> {
	// Before we reach here, the runtime already verified the extrinsic is properly signed by the
	// extrinsic sender Hence, we skip: EphemeralKey::try_from(cert)?;

	#[cfg(test)]
	println!("verifyRA: start verifying RA cert");

	let cert = CertDer(cert_der);
	let netscape = NetscapeComment::try_from(cert)?;
	let sig_cert = webpki::EndEntityCert::from(&netscape.sig_cert).map_err(|_| "Bad der")?;

	verify_signature(&sig_cert, netscape.attestation_raw, &netscape.sig)?;

	// FIXME: now hardcoded. but certificate renewal would have to be done manually anyway...
	// chain wasm update or by some sudo call
	let valid_until = webpki::Time::from_seconds_since_unix_epoch(1573419050);
	verify_server_cert(&sig_cert, valid_until)?;

	parse_report(&netscape)
}

fn parse_report(netscape: &NetscapeComment) -> Result<SgxReport, &'static str> {
	// fn parse_report(report_raw: &[u8]) -> Result<SgxReport, &'static str> {
	// parse attestation report
	let attn_report: Value = match serde_json::from_slice(netscape.attestation_raw) {
		Ok(report) => report,
		Err(_) => return Err("RA report parsing error"),
	};

	let _ra_timestamp = match &attn_report["timestamp"] {
		Value::String(time) => {
			let time_fixed = time.clone() + "+0000";
			match DateTime::parse_from_str(&time_fixed, "%Y-%m-%dT%H:%M:%S%.f%z") {
				Ok(d) => d.timestamp(),
				Err(_) => return Err("RA report timestamp parsing error"),
			}
		},
		_ => return Err("Failed to fetch timestamp from attestation report"),
	};

	// in milliseconds
	let ra_timestamp: u64 = (_ra_timestamp * 1000)
		.try_into()
		.map_err(|_| "Error converting report.timestamp to u64")?;

	#[cfg(test)]
	println!("verifyRA attestation timestamp [unix epoch]: {}", ra_timestamp);

	// get quote status (mandatory field)
	let ra_status = match &attn_report["isvEnclaveQuoteStatus"] {
		Value::String(quote_status) => SgxStatus::from_str(quote_status).unwrap(),
		_ => return Err("Failed to fetch isvEnclaveQuoteStatus from attestation report"),
	};

	#[cfg(test)]
	println!("verifyRA attestation status is: {:?}", ra_status);
	// parse quote body
	if let Value::String(quote_raw) = &attn_report["isvEnclaveQuoteBody"] {
		#[cfg(test)]
		println!("quote_raw : {}", quote_raw);

		let quote = match base64::decode(quote_raw) {
			Ok(q) => q,
			Err(_) => return Err("Quote Decoding Error"),
		};

		#[cfg(test)]
		println!("Quote read. len={}", quote.len());
		// TODO: lack security check here
		let sgx_quote: SgxQuote = match Decode::decode(&mut &quote[..]) {
			Ok(q) => q,
			Err(_) => return Err("could not decode quote"),
		};

		#[cfg(test)]
		{
			println!("sgx quote version = {}", sgx_quote.version);
			println!("sgx quote signature type = {}", sgx_quote.sign_type);
			//println!("sgx quote report_data = {:?}", sgx_quote.report_body.report_data.d[..32]);
			println!("sgx quote mr_enclave = {:?}", sgx_quote.report_body.mr_enclave);
			println!("sgx quote mr_signer = {:?}", sgx_quote.report_body.mr_signer);
			println!("sgx quote report_data = {:?}", sgx_quote.report_body.report_data.d.to_vec());
		}

		let mut xt_signer_array = [0u8; 32];
		xt_signer_array.copy_from_slice(&sgx_quote.report_body.report_data.d[..32]);
		Ok(SgxReport {
			mr_enclave: sgx_quote.report_body.mr_enclave,
			status: ra_status,
			pubkey: xt_signer_array,
			timestamp: ra_timestamp,
			build_mode: sgx_quote.report_body.sgx_build_mode(),
			metadata: SgxEnclaveMetadata::build_sgx_metadata(netscape, &sgx_quote, ra_status),
		})
	} else {
		Err("Failed to parse isvEnclaveQuoteBody from attestation report")
	}
}

pub fn verify_signature(
	entity_cert: &webpki::EndEntityCert,
	attestation_raw: &[u8],
	signature: &[u8],
) -> Result<(), &'static str> {
	match entity_cert.verify_signature(
		&webpki::RSA_PKCS1_2048_8192_SHA256,
		attestation_raw,
		signature,
	) {
		Ok(()) => {
			#[cfg(test)]
			println!("IAS signature is valid");
			Ok(())
		},
		Err(_e) => {
			#[cfg(test)]
			println!("RSA Signature ERROR: {}", _e);
			Err("bad signature")
		},
	}
}

pub fn verify_server_cert(
	sig_cert: &webpki::EndEntityCert,
	timestamp_valid_until: webpki::Time,
) -> Result<(), &'static str> {
	let chain: Vec<&[u8]> = Vec::new();
	match sig_cert.verify_is_valid_tls_server_cert(
		SUPPORTED_SIG_ALGS,
		&IAS_SERVER_ROOTS,
		&chain,
		timestamp_valid_until,
	) {
		Ok(()) => {
			#[cfg(test)]
			println!("CA is valid");
			Ok(())
		},
		Err(_e) => {
			#[cfg(test)]
			println!("CA ERROR: {}", _e);
			Err("CA verification failed")
		},
	}
}
