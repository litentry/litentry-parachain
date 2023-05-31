import { ApiPromise } from '@polkadot/api';
import { KeyObject } from 'crypto';
import WebSocketAsPromised from 'websocket-as-promised';
import { after, before, describe } from 'mocha';
import type { IntegrationTestContext, Web3Wallets } from '../type-definitions';
import type { Metadata, TypeRegistry } from '@polkadot/types';
import type { HexString } from '@polkadot/util/types';
import { initIntegrationTestContext } from './context';

export function describeLitentry(title: string, walletsNumber: number, cb: (context: IntegrationTestContext) => void) {
    describe(title, function () {
        // Set timeout to 6000 seconds
        this.timeout(6000000);

        let context: IntegrationTestContext = {
            mrEnclave: '0x11' as HexString,
            api: {} as ApiPromise,
            tee: {} as WebSocketAsPromised,
            teeShieldingKey: {} as KeyObject,
            ethersWallet: {},
            substrateWallet: {},
            metaData: {} as Metadata,
            sidechainRegistry: {} as TypeRegistry,
            web3Signers: [] as Web3Wallets[],
        };

        before('Starting Litentry(parachain&tee)', async function () {
            //env url
            const tmp = await initIntegrationTestContext(
                process.env.WORKER_END_POINT!,
                process.env.SUBSTRATE_END_POINT!,
                walletsNumber
            );
            context.mrEnclave = tmp.mrEnclave;
            context.api = tmp.api;
            context.tee = tmp.tee;
            context.teeShieldingKey = tmp.teeShieldingKey;
            context.ethersWallet = tmp.ethersWallet;
            context.substrateWallet = tmp.substrateWallet;
            context.metaData = tmp.metaData;
            context.sidechainRegistry = tmp.sidechainRegistry;
            context.web3Signers = tmp.web3Signers;
        });

        after(async function () {});

        cb(context);
    });
}
