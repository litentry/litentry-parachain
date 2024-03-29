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

use codec::{Decode, Encode};
use ita_stf::TrustedCallSigned;
use itc_parentchain_indirect_calls_executor::{
	error::{Error, Result},
	IndirectDispatch,
};
use itp_stf_primitives::traits::IndirectExecutor;
use itp_types::{DecryptableRequest, RsaRequest};

#[derive(Debug, Clone, Encode, Decode, Eq, PartialEq)]
pub struct InvokeArgs {
	request: RsaRequest,
}

impl<Executor: IndirectExecutor<TrustedCallSigned, Error>>
	IndirectDispatch<Executor, TrustedCallSigned> for InvokeArgs
{
	type Args = ();
	fn dispatch(&self, executor: &Executor, _args: Self::Args) -> Result<()> {
		log::debug!("Found trusted call extrinsic, submitting it to the top pool");
		executor.submit_trusted_call(self.request.shard(), self.request.payload().to_vec());
		Ok(())
	}
}
