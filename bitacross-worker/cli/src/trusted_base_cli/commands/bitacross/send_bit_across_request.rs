use crate::{
	command_utils::get_worker_api_direct,
	trusted_cli::TrustedCli,
	trusted_command_utils::{get_identifiers, get_pair_from_str},
	Cli, CliResult, CliResultOk,
    trusted_operation::send_direct_bit_across_request
};
use litentry_primitives::{REQUEST_AES_KEY_LEN, RequestAesKey};
use itc_rpc_client::direct_client::DirectApi;
use sp_core::Pair;

#[derive(Parser)]
pub struct BitAcrossCommand;

impl BitAcrossCommand {
	pub(crate) fn run(&self, cli: &Cli, trusted_cli: &TrustedCli) -> CliResult {
        let (mrenclave, shard) = get_identifiers(trusted_cli, cli);
        let aes_key = Self::random_aes_key();
        // Replace random data with whatever struct you want, and then encode it 
        let random_data: Vec<u8> = vec![1,2,3,4];

        send_direct_bit_across_request::<String>(
            cli, 
            trusted_cli, 
            shard, 
            aes_key, 
            random_data
        ).unwrap();

		Ok(CliResultOk::None)
	}

    fn random_aes_key() -> RequestAesKey {
		let random: Vec<u8> = (0..REQUEST_AES_KEY_LEN).map(|_| rand::random::<u8>()).collect();
		random[0..REQUEST_AES_KEY_LEN].try_into().unwrap()
	}
}

