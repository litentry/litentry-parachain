import fs from 'fs';
import '@polkadot/api-augment';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { TypeRegistry } from '@polkadot/types/create';
import { Bytes } from '@polkadot/types';
import type { ISubmittableResult } from "@polkadot/types/types";
import {loadConfig, signAndSend, sleep} from './utils';

async function registerParathread(api: ApiPromise, config: any) {
    // Get keyring of Alice, who is also the sudo in dev chain spec
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    const genesisHeadBytes = fs.readFileSync(config.genesis_state_path, 'utf8');
    const validationCodeBytes = fs.readFileSync(config.genesis_wasm_path, 'utf8');

    const registry = new TypeRegistry();

    const tx = api.tx.sudo.sudo(
        api.tx.parasSudoWrapper.sudoScheduleParaInitialize(process.env.PARACHAIN_ID, {
            genesisHead: new Bytes(registry, genesisHeadBytes),
            validationCode: new Bytes(registry, validationCodeBytes),
            parachain: true,
        })
    );

    console.log(`Parachain registration tx Sent!`);
    return signAndSend(tx, alice);
}

async function onboarding_parachain(api:ApiPromise,config: any){
    console.log("start onboarding parachain");
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    // user can set the number
    const Period_conut = 100;

    return new Promise(async (resolvePromise, reject) => {
        await api.tx.sudo
            .sudo(api.tx.slots.forceLease(process.env.PARACHAIN_ID, alice.address, 0, 0, Period_conut))
            .signAndSend(alice, ({ status }: ISubmittableResult) => {
                console.log(`Current status is ${status}`);
                if (status.isInBlock) {
                    console.log(`Transaction included at blockHash ${status.asInBlock}`);
                } else if (status.isFinalized) {
                    resolvePromise(0)
                }
            });
    })
}

(async () => {
    console.log('Register parachain ...');
    const config = loadConfig();

    const provider = new WsProvider(config.relaychain_ws);
    const api = await ApiPromise.create({
        provider: provider,
    });

    await registerParathread(api, config);
    await api.disconnect();
    provider.on('disconnected', () => {
        console.log('Disconnect from relaychain');
        process.exit(0);
    });
})();
