import '@polkadot/api-augment';
import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import type { ISubmittableResult } from "@polkadot/types/types";
import {loadConfig} from './utils';

async function upgrade_parathread_to_parachain(api:ApiPromise){
    console.log("start onboarding parachain");
    const keyring = new Keyring({ type: 'sr25519' });
    const alice = keyring.addFromUri('//Alice');

    // user can set the number
    const Period_conut = 100;

    return new Promise(async (resolvePromise) => {
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
    console.log('update to parachain ...');
    const config = loadConfig();

    const provider = new WsProvider(config.relaychain_ws);
    const api = await ApiPromise.create({
        provider: provider,
    });

    await upgrade_parathread_to_parachain(api);
    await api.disconnect();
    provider.on('disconnected', () => {
        console.log('Disconnect from relaychain');
        process.exit(0);
    });
})();
