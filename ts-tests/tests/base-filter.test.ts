import { expect } from 'chai';
import { step } from 'mocha-steps';
import { u8aConcat } from '@polkadot/util';

import { signAndSend, describeLitentry, loadConfig, sleep } from './utils';

describeLitentry('Test Base Filter', ``, (context) => {
    console.log(`Test Base Filter`);

    step('Transfer 1000 unit from Eve to Bob', async function () {
        // Get the initial balance of Eve and Bob
        const { nonce: eveInitNonce, data: eveInitBalance } = await context.api.query.system.account(
            context.eve.address
        );
        const { nonce: bobInitNonce, data: bobInitBalance } = await context.api.query.system.account(
            context.bob.address
        );

        const tx = context.api.tx.balances.transfer(context.bob.address, 1000);
        await signAndSend(tx, context.eve);

        const { nonce: eveCurrentNonce, data: eveCurrentBalance } = await context.api.query.system.account(
            context.eve.address
        );
        const { nonce: bobCurrentNonce, data: bobCurrentBalance } = await context.api.query.system.account(
            context.bob.address
        );

        expect(eveCurrentNonce.toNumber()).to.equal(eveInitNonce.toNumber() + 1);
		// The transfer should fail and Bob's balance stays unchanged
        expect(bobCurrentBalance.free.toBigInt()).to.equal(bobInitBalance.free.toBigInt());
    });

    step('Transfer 1000 unit from Alice to Bob with Sudo', async function () {
        // Get the initial balance of Alice and Bob
        const { nonce: aliceInitNonce, data: aliceInitBalance } = await context.api.query.system.account(
            context.alice.address
        );
        const { nonce: bobInitNonce, data: bobInitBalance } = await context.api.query.system.account(
            context.bob.address
        );

        const tx = context.api.tx.sudo.sudo(
			context.api.tx.balances.transfer(context.bob.address, 1000)
			);
        await signAndSend(tx, context.alice);

        const { nonce: aliceCurrentNonce, data: aliceCurrentBalance } = await context.api.query.system.account(
            context.alice.address
        );
        const { nonce: bobCurrentNonce, data: bobCurrentBalance } = await context.api.query.system.account(
            context.bob.address
        );

		// The transfer should succeed 
        expect(aliceCurrentNonce.toNumber()).to.equal(aliceInitNonce.toNumber() + 1);
        expect(bobCurrentBalance.free.toBigInt()).to.equal(bobInitBalance.free.toBigInt() + BigInt(1000));
    });

    step('Verify transaction fee distribution', async function () {
		// These are default proportions configured in runtime as constants
		const TREASURY_PROPORTION = 40;
		const AUTHOR_PROPORTION = 0;
		const BURNED_PROPORTION = 60;

		// Get Treasury account
		const treasuryAccount = u8aConcat(
			'modl',
			context.api.consts.treasury && context.api.consts.treasury.palletId
			  ? context.api.consts.treasury.palletId.toU8a(true)
			  : 'py/trsry',
			  new Uint8Array(32)
		  ).subarray(0, 32);

		// Get the initial balances of Alice and Treasury
        const { data: aliceInitBalance } = await context.api.query.system.account(
            context.alice.address
        );
        const { data: treasuryInitBalance } = await context.api.query.system.account(
            treasuryAccount
        );

	    // Send a sudo transaction from Alice
		const transferAmount = 1000;
        const txTransfer = context.api.tx.sudo.sudo(
			context.api.tx.balances.transfer(context.bob.address, transferAmount)
			);
        await signAndSend(txTransfer, context.alice);

		// Get the current balances of Alice and Treasury
        const { data: aliceCurrentBalance } = await context.api.query.system.account(
            context.alice.address
        );
        const { data: treasuryCurrentBalance } = await context.api.query.system.account(
            treasuryAccount
        );

		// Calculate transaction fee 
		const txFee = aliceInitBalance.free.toNumber() - aliceCurrentBalance.free.toNumber() - transferAmount;
		const treasuryBalanceIncrease = treasuryCurrentBalance.free.toNumber() - treasuryInitBalance.free.toNumber();

		console.log(`The actual transaction fee is ${txFee}, and the Treasury pot has an balance increase of ${treasuryBalanceIncrease}`)

		const totalProportion = TREASURY_PROPORTION + AUTHOR_PROPORTION + BURNED_PROPORTION;
		expect(treasuryBalanceIncrease).to.approximately(txFee / totalProportion * TREASURY_PROPORTION, 10);
    });
});
