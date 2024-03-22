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

/// Empty tests entry for production mode.
#[cfg(not(feature = "test"))]
#[no_mangle]
#[allow(clippy::unreachable)]
pub extern "C" fn test_main_entrance() -> sgx_types::types::size_t {
	unreachable!("Tests are not available when compiled in production mode.")
}
