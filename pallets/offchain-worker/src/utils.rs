use sp_std::prelude::*;

// u128 number string to u128
pub fn chars_to_u128(vec: &Vec<char>) -> Result<u128, &'static str> {
	// Check if the number string is decimal or hexadecimal (whether starting with 0x or not) 
	let base = if vec.len() >= 2 && vec[0] == '0' && vec[1] == 'x' {
		// This is a hexadecimal number
		16
	} else {
		// This is a decimal number
		10
	};

	let mut result: u128 = 0;
	for (i, item) in vec.iter().enumerate() {
		// Skip the 0 and x digit for hex. 
		// Using skip here instead of a new vec build to avoid an unnecessary copy operation
		if base == 16 && i < 2 {
			continue;
		}

		let n = item.to_digit(base);
		match n {
			Some(i) => {
				let i_64 = i as u128; 
				result = result * base as u128 + i_64;
				if result < i_64 {
					return Err("Wrong u128 balance data format");
				}
			},
			None => return Err("Wrong u128 balance data format"),
		}
	}
	return Ok(result)
}

// number byte to string byte
pub fn u8_to_str_byte(a: u8) -> u8{
	if a < 10 {
		return a + 48 as u8;
	}
	else {
		return a + 87 as u8;
	}
}

// address to string bytes
pub fn address_to_string(address: &[u8; 20]) -> Vec<u8> {

	let mut vec_result: Vec<u8> = Vec::new();
	for item in address {
		let a: u8 = item & 0x0F;
		let b: u8 = item >> 4;
		vec_result.push(u8_to_str_byte(b));
		vec_result.push(u8_to_str_byte(a));
	}
	return vec_result;
}
