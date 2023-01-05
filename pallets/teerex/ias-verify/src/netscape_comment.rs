use crate::{utils::length_from_raw_data, CertDer, SgxQuoteAdd, SgxQuoteInputs};
use frame_support::ensure;
use sp_std::{convert::TryFrom, prelude::Vec};

pub struct NetscapeComment<'a> {
	pub attestation_raw: &'a [u8],
	pub sig: Vec<u8>,
	pub sig_cert: Vec<u8>,
	pub quote_add: Option<SgxQuoteAdd>,
}

pub const NS_CMT_OID: &[u8; 11] =
	&[0x06, 0x09, 0x60, 0x86, 0x48, 0x01, 0x86, 0xF8, 0x42, 0x01, 0x0D];

impl<'a> TryFrom<CertDer<'a>> for NetscapeComment<'a> {
	type Error = &'static str;

	fn try_from(value: CertDer<'a>) -> Result<Self, Self::Error> {
		// Search for Netscape Comment OID
		let cert_der = value.0;

		let mut offset = cert_der
			.windows(NS_CMT_OID.len())
			.position(|window| window == NS_CMT_OID)
			.ok_or("Certificate does not contain 'ns_cmt_oid'")?;

		offset += 12; // 11 + TAG (0x04)

		#[cfg(test)]
		println!("netscape");
		// Obtain Netscape Comment length
		let len = length_from_raw_data(cert_der, &mut offset)?;
		// Obtain Netscape Comment
		offset += 1;
		let netscape_raw = cert_der
			.get(offset..offset + len)
			.ok_or("Index out of bounds")?
			.split(|x| *x == 0x7C)
			.collect::<Vec<&[u8]>>();
		ensure!(netscape_raw.len() >= 3, "Invalid netscape payload");

		let sig = base64::decode(netscape_raw[1]).map_err(|_| "Signature Decoding Error")?;

		let sig_cert = base64::decode_config(netscape_raw[2], base64::STANDARD)
			.map_err(|_| "Cert Decoding Error")?;

		if netscape_raw.len() > 3 {
			let quote_add = Self::try_quote_add(&netscape_raw)?;

			return Ok(NetscapeComment {
				attestation_raw: netscape_raw[0],
				sig,
				sig_cert,
				quote_add: Some(quote_add),
			})
		}

		Ok(NetscapeComment { attestation_raw: netscape_raw[0], sig, sig_cert, quote_add: None })
	}
}

impl<'a> NetscapeComment<'a> {
	pub fn try_quote_add(netscape_raw: &Vec<&[u8]>) -> Result<SgxQuoteAdd, &'static str> {
		let spid = base64::decode_config(netscape_raw[3], base64::STANDARD)
			.map_err(|_| "Cert Decoding Error")?;

		let mut nonce = Vec::<u8>::new();
		if netscape_raw.len() > 4 {
			nonce = base64::decode_config(netscape_raw[4], base64::STANDARD)
				.map_err(|_| "Cert Decoding Error")?;
		}

		let mut sig_rl = Vec::<u8>::new();
		if netscape_raw.len() > 5 {
			sig_rl = base64::decode_config(netscape_raw[5], base64::STANDARD)
				.map_err(|_| "Cert Decoding Error")?;
		}

		#[cfg(test)]
		{
			println!("spid: {:?}", spid);
			println!("nonce : {:?}", nonce);
			println!("sig_rl: {:?}", sig_rl);
		}

		let mut d_spid = [0_u8; 16];
		d_spid.copy_from_slice(&spid[..16]);

		let mut d_nonce = [0_u8; 16];
		d_nonce.copy_from_slice(&nonce[..16]);

		let quote_inputs = SgxQuoteInputs { spid: d_spid, nonce: d_nonce, sig_rl };

		Ok(SgxQuoteAdd { quote_inputs })
	}
}
