import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { expect } from 'chai';
import { step } from 'mocha-steps';

import { signAndSend, describeLitentry, loadConfig, sleep } from './utils';

describeLitentry('Test Balance Transfer', ``, (context) => {
    console.log(`Test Balance Transfer`);

    step('Transfer 1000 unit from Alice to Bob', async function () {
        // Get the initial balance of Alice and Bob
        const { nonce: aliceInitNonce, data: aliceInitBalance } = await context.api.query.system.account(
            context.alice.address
        );
        const { nonce: bobInitNonce, data: bobInitBalance } = await context.api.query.system.account(
            context.bob.address
        );

        const tx = context.api.tx.balances.transfer(context.bob.address, 1000);
        await signAndSend(tx, context.alice);

        const { nonce: aliceCurrentNonce, data: aliceCurrentBalance } = await context.api.query.system.account(
            context.alice.address
        );
        const { nonce: bobCurrentNonce, data: bobCurrentBalance } = await context.api.query.system.account(
            context.bob.address
        );

        expect(aliceCurrentNonce.toNumber()).to.equal(aliceInitNonce.toNumber() + 1);
        expect(bobCurrentBalance.free.toBigInt()).to.equal(bobInitBalance.free.toBigInt() + BigInt(1000));
    });

    step('Transfer 1000 unit back to Alice from Bob', async function () {
        // Get the initial balance of Alice and Bob
        const { nonce: aliceInitNonce, data: aliceInitBalance } = await context.api.query.system.account(
            context.alice.address
        );
        const { nonce: bobInitNonce, data: bobInitBalance } = await context.api.query.system.account(
            context.bob.address
        );

        const tx = context.api.tx.balances.transfer(context.alice.address, 1000);
        await signAndSend(tx, context.bob);

        const { nonce: aliceCurrentNonce, data: aliceCurrentBalance } = await context.api.query.system.account(
            context.alice.address
        );
        const { nonce: bobCurrentNonce, data: bobCurrentBalance } = await context.api.query.system.account(
            context.bob.address
        );

        expect(bobCurrentNonce.toNumber()).to.equal(bobInitNonce.toNumber() + 1);
        expect(aliceCurrentBalance.free.toBigInt()).to.equal(aliceInitBalance.free.toBigInt() + BigInt(1000));
    });
});
