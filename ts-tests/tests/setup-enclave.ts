import fs from 'fs';
import '@polkadot/api-augment';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { TypeRegistry } from '@polkadot/types/create';
import { Bytes } from '@polkadot/types';
import { loadConfig, signAndSend } from './utils';
import { hexToU8a } from '@polkadot/util';

const mrenclave = process.argv[2];
const block = process.argv[3];

async function setAliceAsAdmin(api: ApiPromise, config: any) {
    // Get keyring of Alice, who is also the sudo in dev chain spec
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    const tx = api.tx.sudo.sudo(api.tx.teerex.setAdmin('esqZdrqhgH8zy1wqYh1aLKoRyoRWLFbX9M62eKfaTAoK67pJ5')); 

    console.log(`Setting Alice as Admin for Teerex`);
    return signAndSend(tx, alice);
}

async function updateScheduledEnclave(api: ApiPromise, config: any) {
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');
    
    const tx = api.tx.teerex.updateScheduledEnclave(block, hexToU8a(`0x${mrenclave}`));

    console.log("Schedule Enclave Extrinsic sent"); 
    return signAndSend(tx, alice)
}

(async () => {
    console.log('Register parachain ...');
    const config = loadConfig();

    const provider = new WsProvider(config.parachain_ws);
    const api = await ApiPromise.create({
        provider: provider,
    });

    await setAliceAsAdmin(api, config);
    await updateScheduledEnclave(api, config);
    
    await api.disconnect();
    provider.on('disconnected', () => {
        console.log('Disconnect from relaychain');
        process.exit(0);
    });
})();

