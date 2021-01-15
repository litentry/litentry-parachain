//! Based on https://github.com/sipa/bech32/tree/master/ref/rust
//! // Copyright (c) 2017 Clark Moody
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

use sp_std::prelude::*;
use sp_std::vec;

pub struct WitnessProgram {
    /// Witness program version
    pub version: u8,
    /// Witness program content
    pub program: Vec<u8>
}

impl WitnessProgram {
    /// Converts a Witness Program to a SegWit Address
    pub fn to_address(&self, hrp: Vec<u8>) -> Result<Vec<u8>, &'static str> {
        // Verify that the program is valid
        let mut data: Vec<u8> = vec![self.version];
        // Convert 8-bit program into 5-bit
        let p5 = self.program.to_base32();
        // let p5 = convert_bits(self.program.to_vec(), 8, 5, true)?;
        data.extend_from_slice(&p5);
        let b32 = data.encode(hrp)?;
        Ok(b32)
    }

    /// Extracts a WitnessProgram out of a provided script public key
    pub fn from_scriptpubkey(pubkey: &[u8]) -> Result<Self, &'static str> {
        // We need a version byte and a program length byte, with a program at 
        // least 2 bytes long.
        if pubkey.len() < 4 {
            return Err("TooShort")
        }
        let proglen: usize = pubkey[1] as usize;
        // Check that program length byte is consistent with pubkey length
        if pubkey.len() != 2 + proglen {
            return Err("InvalidLengthByte")
        }
        // Process script version
        let mut v: u8 = pubkey[0];
        if v > 0x50 {
            v -= 0x50;
        }
        let program = &pubkey[2..];
        Ok(WitnessProgram {
            version: v,
            program: program.to_vec()
        })
    }
}

const SEP: u8 = b'1';
const ALPHABET: &'static [u8] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";

pub trait Bech32 {
    fn encode(&self, hrp: Vec<u8>) -> Result<Vec<u8>, &'static str>;
}

impl Bech32 for [u8] {
    fn encode(&self, hrp: Vec<u8>) -> Result<Vec<u8>, &'static str> {
        if hrp.len() < 1 {
            return Err("invalidData")
        }

        let mut combined: Vec<u8> = self.clone().to_vec();
        combined.extend_from_slice(&create_checksum(&hrp, &self.to_vec()));
        let mut encoded = hrp;
        encoded.push(SEP);
        for p in combined {
            if p >= 32 {
                return Err("invalidData")
            }
            encoded.push(ALPHABET[p as usize]);
        }
        Ok(encoded)
    }
}

const GEN: [u32; 5] = [0x3b6a57b2, 0x26508e6d, 0x1ea119fa, 0x3d4233dd, 0x2a1462b3];

fn hrp_expand(hrp: &Vec<u8>) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();
    for b in hrp {
        v.push(*b >> 5);
    }
    v.push(0);
    for b in hrp {
        v.push(*b & 0x1f);
    }
    v
}

fn create_checksum(hrp: &Vec<u8>, data: &Vec<u8>) -> Vec<u8> {
    let mut values: Vec<u8> = hrp_expand(hrp);
    values.extend_from_slice(data);
    // Pad with 6 zeros
    values.extend_from_slice(&[0u8; 6]);
    let plm: u32 = polymod(values) ^ 1;
    let mut checksum: Vec<u8> = Vec::new();
    for p in 0..6 {
        checksum.push(((plm >> 5 * (5 - p)) & 0x1f) as u8);
    }
    checksum
}

fn polymod(values: Vec<u8>) -> u32 {
    let mut chk: u32 = 1;
    let mut b: u8;
    for v in values {
        b = (chk >> 25) as u8;
        chk = (chk & 0x1ffffff) << 5 ^ (v as u32);
        for i in 0..5 {
            if (b >> i) & 1 == 1 {
                chk ^= GEN[i]
            }
        }
    }
    chk
}

/// A trait for converting a value to base58 encoded string.
pub trait ToBase32 {
	/// Converts a value of `self` to a base58 value, returning the owned string.
	fn to_base32(&self) -> Vec<u8>;
}

impl ToBase32 for [u8] {
    // /// Convert between bit sizes
    // fn to_base32(&self) -> Vec<u8> {
    //     let from: u32 = 8;
    //     let to: u32 = 5;

    //     let mut acc: u32 = 0;
    //     let mut bits: u32 = 0;
    //     let mut ret: Vec<u8> = Vec::new();
    //     let maxv: u32 = (1<<to) - 1;
    //     for &value in self.into_iter() {
    //         let v: u32 = value as u32;

    //         acc = (acc << from) | v;
    //         bits += from;
    //         while bits >= to {
    //             bits -= to;
    //             ret.push(((acc >> bits) & maxv) as u8);
    //         }
    //     }
    //     if bits > 0 {
    //         ret.push(((acc << (to - bits)) & maxv) as u8);
    //     }

    //     ret
    // }

	fn to_base32(&self) -> Vec<u8> {
        // Amount of bits left over from last round, stored in buffer.
        let mut buffer_bits = 0u32;
        // Holds all unwritten bits left over from last round. The bits are stored beginning from
        // the most significant bit. E.g. if buffer_bits=3, then the byte with bits a, b and c will
        // look as follows: [a, b, c, 0, 0, 0, 0, 0]
        let mut buffer: u8 = 0;

        let mut result = Vec::new();


        for b in self.into_iter() {
            // Write first u5 if we have to write two u5s this round. That only happens if the
            // buffer holds too many bits, so we don't have to combine buffer bits with new bits
            // from this rounds byte.
            if buffer_bits >= 5 {
                result.push((buffer & 0b1111_1000) >> 3);
                buffer <<= 5;
                buffer_bits -= 5;
            }

            // Combine all bits from buffer with enough bits from this rounds byte so that they fill
            // a u5. Save remaining bits from byte to buffer.
            let from_buffer = buffer >> 3;
            let from_byte = b >> (3 + buffer_bits); // buffer_bits <= 4

            result.push(from_buffer | from_byte);
            buffer = b << (5 - buffer_bits);
            buffer_bits += 3;
        }

        // There can be at most two u5s left in the buffer after processing all bytes, write them.
        if buffer_bits >= 5 {
            result.push((buffer & 0b1111_1000) >> 3);
            buffer <<= 5;
            buffer_bits -= 5;
        }

        if buffer_bits != 0 {
            result.push(buffer >> 3);
        }

        result

	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::str::from_utf8;

	#[test]
	fn test_to_base32_basic() {
		assert_eq!(from_utf8(&vec![0x00, 0x01, 0x02].encode(b"bech32".to_vec()).unwrap()).unwrap(), "bech321qpz4nc4pe");
    }

    #[test]
    fn valid_address() {
        let pairs: Vec<(&str, Vec<u8>)> = vec![
            (
                "BC1QW508D6QEJXTDG4Y5R3ZARVARY0C5XW7KV8F3T4",
                vec![
                    0x00, 0x14, 0x75, 0x1e, 0x76, 0xe8, 0x19, 0x91, 0x96, 0xd4, 0x54,
                    0x94, 0x1c, 0x45, 0xd1, 0xb3, 0xa3, 0x23, 0xf1, 0x43, 0x3b, 0xd6
                ]
            ),
            (
                "bc1pw508d6qejxtdg4y5r3zarvary0c5xw7kw508d6qejxtdg4y5r3zarvary0c5xw7k7grplx",
                vec![
                    0x51, 0x28, 0x75, 0x1e, 0x76, 0xe8, 0x19, 0x91, 0x96, 0xd4, 0x54,
                    0x94, 0x1c, 0x45, 0xd1, 0xb3, 0xa3, 0x23, 0xf1, 0x43, 0x3b, 0xd6,
                    0x75, 0x1e, 0x76, 0xe8, 0x19, 0x91, 0x96, 0xd4, 0x54, 0x94, 0x1c,
                    0x45, 0xd1, 0xb3, 0xa3, 0x23, 0xf1, 0x43, 0x3b, 0xd6
                ]
            ),
            (
                "BC1SW50QA3JX3S",
                vec![
                   0x60, 0x02, 0x75, 0x1e
                ]
            ),
            (
                "bc1zw508d6qejxtdg4y5r3zarvaryvg6kdaj",
                vec![
                    0x52, 0x10, 0x75, 0x1e, 0x76, 0xe8, 0x19, 0x91, 0x96, 0xd4, 0x54,
                    0x94, 0x1c, 0x45, 0xd1, 0xb3, 0xa3, 0x23
                ]
            ),
        ];
        for p in pairs {
            let (address, scriptpubkey) = p;

            let hrp = b"bc".to_vec();

            let spk_result = WitnessProgram::from_scriptpubkey(&scriptpubkey);
            assert!(spk_result.is_ok());
            let prog = spk_result.unwrap();

            let enc_result = prog.to_address(hrp);
            assert!(enc_result.is_ok());

            let enc_address = enc_result.unwrap();
            assert_eq!(address.to_lowercase(), from_utf8(&enc_address).unwrap().to_lowercase());
        }
    }
    
}