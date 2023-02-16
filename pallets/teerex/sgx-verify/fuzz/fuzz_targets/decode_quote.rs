#![no_main]

use codec::{Decode, Encode};
use libfuzzer_sys::fuzz_target;
use sgx_verify::DcapQuote;

fuzz_target!(|data: &[u8]| {
	let mut copy = data;
	let quote: Result<DcapQuote, codec::Error> = Decode::decode(&mut copy);

	// This assert is commented out because the fuzzer manages to find a "valid" quote that can
	// at least be decoded into memory. We would need additional verification steps (for example signature)
	// to enable this check.
	//assert!(quote.is_err());
});
