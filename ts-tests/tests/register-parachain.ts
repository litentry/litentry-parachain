import fs from 'fs';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { TypeRegistry } from '@polkadot/types/create';
import { Bytes } from '@polkadot/types';

import { loadConfig } from './utils';


async function registerParachain(api: ApiPromise, config: any) {
    // Get keyring of Alice
    // Keyring needed to sign using Alice account
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
    // Get keyring of Sudo
    const sudoKey = await api.query.sudo.key();

    const sudo = keyring.addFromAddress(sudoKey);

    const genesisHeadBytes = fs.readFileSync(config.parachain_genesis_path, 'utf8');
    const validationCodeBytes = fs.readFileSync(config.parachain_wasm_path, 'utf8');

    const registry = new TypeRegistry();

    const tx = api.tx.sudo.sudo(
        api.tx.parasSudoWrapper.sudoScheduleParaInitialize(2022, {
            genesisHead: new Bytes(registry, genesisHeadBytes),
            validationCode: new Bytes(registry, validationCodeBytes),
            parachain: true,
        })
    );

    console.log(`Parachain registration tx Sent!`);
    const parachainRegister = new Promise<{ block: string }>(async (resolve, reject) => {
        const unsub = await tx.signAndSend(alice, (result) => {
            console.log(`Parachain registration is ${result.status}`);
            if (result.status.isInBlock) {
                console.log(`Parachain registration included at blockHash ${result.status.asInBlock}`);
                console.log(`Waiting for finalization... (can take a minute)`);
            } else if (result.status.isFinalized) {
                console.log(`Transfer finalized at blockHash ${result.status.asFinalized}`);
                unsub();
                resolve({
                    block: result.status.asFinalized.toString(),
                });
            }
        });
    });

    return parachainRegister;
}


(async () => {
    console.log('Register parachain ...');
    const config = loadConfig();

    const provider = new WsProvider(config.relaynode_ws);
    const api = await ApiPromise.create({
        provider: provider,
        types: {
            // mapping the actual specified address format
            Address: 'MultiAddress',
            // mapping the lookup
            LookupSource: 'MultiAddress',
        },
    });

    await registerParachain(api, config);
    await api.disconnect();
    provider.on('disconnected', () => {
        console.log('Disconnect from relaychain');
        process.exit(0);
    });
})();
