import { step } from 'mocha-steps';
import { describeLitentry } from './common/utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { ethers } from 'ethers';
const { Keyring } = require('@polkadot/keyring');
import { u8aToHex } from '@polkadot/util';
import { Assertion } from './common/type-definitions';
import { batchCall } from './indirect_calls';
import { handleEvent } from './common/utils'
const assertion = <Assertion>{
    A1: 'A1',
    A2: ['A2'],
    A3: ['A3', 'A3', 'A3'],
    A4: [10],
    A7: [10],
    A8: ['litentry'],
    A10: [10],
    A11: [10],
};
const wallets = require('./wallets.json');
describeLitentry('multiple accounts test', async (context) => {
    step('create accounts', async () => {
        const count = 10;
        const seed = 'test seed';
        const addresses: {
            substrateWallet: KeyringPair;
            ethereumWallet: ethers.Wallet;
        }[] = [];
        const keyring = new Keyring({ type: 'sr25519' });

        for (let i = 0; i < count; i++) {
            const substratePair = keyring.addFromUri(`${seed}//${i}`);
            const ethereumWallet = ethers.Wallet.createRandom();
            addresses.push({
                substrateWallet: substratePair,
                ethereumWallet: ethereumWallet,
            });
        }
        const fs = require('fs');
        const fileName = `wallets.json`;
        fs.writeFileSync(fileName, JSON.stringify(addresses));
    });
    step('send a test token to each account.', async () => {
        const transfer_token_txs: any = [];
        for (let i = 0; i < wallets.length; i++) {
            const tx = context.api.tx.balances.transfer(wallets[i].substrateWallet.address, '1000000000000');
            transfer_token_txs.push(tx);
        }
        const events = await batchCall(context, context.substrateWallet.alice, transfer_token_txs, 'balances', [
            'Transfer',
        ]);
        await handleEvent(events)

    });
    step('request VC(A1)', async () => {

    });
});
