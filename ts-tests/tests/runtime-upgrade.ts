//simulate runtime upgrad
// is a demo not impl

import fs from 'fs';
import '@polkadot/api-augment';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { TypeRegistry } from '@polkadot/types/create';
import { Bytes } from '@polkadot/types';

import { loadConfig, signAndSend } from './utils';

async function runtime_upgrade(api: ApiPromise, config: any) {
    // Get keyring of Alice, who is also the sudo in dev chain spec
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    const registry = new TypeRegistry();
    const code_path="./docker/*-parachain-runtime.compact.compressed.wasm"
    const tx = api.tx.sudo.sudoUncheckWeight(
            api.tx.system.setCode(
                    code_path
                )
        );

    console.log(`Parachain registration tx Sent!`);
    return signAndSend(tx, alice);
}

(async () => {
    console.log('Register parachain ...');
    const config = loadConfig();

    const provider = new WsProvider(config.relaychain_ws);
    const api = await ApiPromise.create({
        provider: provider,
    });

    await runtime_upgrade(api, config);
    await api.disconnect();
    provider.on('disconnected', () => {
        console.log('Disconnect from relaychain');
        process.exit(0);
    });
})();
