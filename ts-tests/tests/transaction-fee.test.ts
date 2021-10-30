import { u8aConcat } from '@polkadot/util';
import { expect } from 'chai';
import { step } from 'mocha-steps';

import { signAndSend, describeLitentry, loadConfig, sleep } from './utils';

describeLitentry('Test Transaction Fee', ``, (context) => {
    console.log(`Test Transaction Fee`);

    step('Set transaction fee portion ', async function () {
        // Get the initial transaction fee ratio
        const ratio = await context.api.query.transactionPaymentInterface.ratio();

		const defaultRatio = "[50,30,20]";
		
		console.log(`The default transaction fee ratio is ${ratio}`);
        expect(ratio.toString()).to.equal(defaultRatio);

		//const treasuryPortion: U32 = 100;
		//const authorPortion: U32 = 20; 
		//const burnedPortion: U32 = 70;
		
		// Set transaction fee ratio to 100:20:80
        const txSetRatio = context.api.tx.sudo.sudo(
			context.api.tx.transactionPaymentInterface.setRatio(100, 20, 80)
			);
        await signAndSend(txSetRatio, context.alice);

        const newRatio = await context.api.query.transactionPaymentInterface.ratio();

		console.log(`The new transaction fee ratio is ${newRatio}`);
        expect(newRatio.toString()).to.equal("[100,20,80]");
    });

    step('Verify transaction fee distribution', async function () {
		// Get Treasury account
		const treasuryAccount = u8aConcat(
			'modl',
			context.api.consts.treasury && context.api.consts.treasury.palletId
			  ? context.api.consts.treasury.palletId.toU8a(true)
			  : 'py/trsry',
			  new Uint8Array(32)
		  ).subarray(0, 32);

		// Get the initial balances of Alice (the block author), Eve and Treasury
        const { data: aliceInitBalance } = await context.api.query.system.account(
            context.alice.address
        );
        const { data: eveInitBalance } = await context.api.query.system.account(
            context.eve.address
        );
        const { data: treasuryInitBalance } = await context.api.query.system.account(
            treasuryAccount
        );

	    // Send a transaction from Eve
		const transferAmount = 1000;
        const txTransfer = context.api.tx.balances.transfer(context.bob.address, transferAmount);
        await signAndSend(txTransfer, context.eve);

		// Get the current balances of Alice (the block author), Eve and Treasury
        const { data: aliceCurrentBalance } = await context.api.query.system.account(
            context.alice.address
        );
        const { data: eveCurrentBalance } = await context.api.query.system.account(
            context.eve.address
        );
        const { data: treasuryCurrentBalance } = await context.api.query.system.account(
            treasuryAccount
        );

		// Calculate transaction fee 
		const txFee = eveInitBalance.free.toNumber() - eveCurrentBalance.free.toNumber() - transferAmount;
		const treasuryBalanceIncrease = treasuryCurrentBalance.free.toNumber() - treasuryInitBalance.free.toNumber();
		const aliceBalanceIncrease = aliceCurrentBalance.free.toNumber() - aliceInitBalance.free.toNumber();

		console.log(`The actual transaction fee is ${txFee}, the Block author (Alice) has an balance increase of ${aliceBalanceIncrease}, and the Treasury pot has an balance increase of ${treasuryBalanceIncrease}`)

		expect(treasuryBalanceIncrease).to.approximately(txFee / 2, 10);
		expect(aliceBalanceIncrease).to.approximately(txFee / 10, 10);
    });

});
