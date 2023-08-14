import fs from 'fs';
import '@polkadot/api-augment';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { TypeRegistry } from '@polkadot/types/create';
import { Bytes } from '@polkadot/types';
import { loadConfig, signAndSend } from './utils';
import { hexToU8a } from '@polkadot/util';

const account = process.argv[2];
const transferAmount = "100000000000000";


async function transferBalance(api: ApiPromise, config: any) {
    // Get keyring of Alice, who is also the sudo in dev chain spec
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    const tx = api.tx.balances.transfer(account, transferAmount);
    console.log(`Transferring balance to the Enclave Account`);
    return signAndSend(tx, alice);
}

(async () => {
    console.log('Transfer balance to enclave on parachain ...');
    const config = loadConfig();

    const provider = new WsProvider(config.parachain_ws);
    const api = await ApiPromise.create({
        provider: provider,
    });

    await transferBalance(api, config); 
    
    await api.disconnect();
    provider.on('disconnected', () => {
        console.log('Disconnect from relaychain');
        process.exit(0);
    });
})();

