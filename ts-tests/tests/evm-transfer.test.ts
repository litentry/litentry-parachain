import { assert, expect } from 'chai';
import { step } from 'mocha-steps';

import { signAndSend, describeLitentry, loadConfig, sleep } from './utils';
import { hexToU8a, u8aToHex } from '@polkadot/util';
import { createPair, encodeAddress } from '@polkadot/keyring';
import Web3 from 'web3';

describeLitentry('Test EVM Module Transfer', ``, (context) => {
    console.log(`Test EVM Module Transfer`);

    step('Transfer Value from Eve to EVM external account', async function () {
        // In case evm is not enabled in Normal Mode, switch back to filterMode, after test.
        // We do not test mode in initialization since ts-test concerns filter function too.
        const filterMode = (await context.api.query.extrinsicFilter.mode()).toHuman();
        if ('Test' !== filterMode) {
            let extrinsic = context.api.tx.sudo.sudo(context.api.tx.extrinsicFilter.setMode('Test'));
            let temp = await context.api.rpc.chain.getBlock();
            console.log(`setMode await Before: ${temp.block.header.number}`);
            await signAndSend(extrinsic, context.alice);
            temp = await context.api.rpc.chain.getBlock();
            console.log(`setMode await end: ${temp.block.header.number}`);
        }

        // Get the initial balance of Eve and EVM external account
        const { nonce: eveInitNonce, data: eveInitBalance } = await context.api.query.system.account(
            context.eve.address
        );
        // EVM module transfer for substrate account
        const evmAccountRaw = {
            privateKey: '0x01ab6e801c06e59ca97a14fc0a1978b27fa366fc87450e0b65459dd3515b7391',
            address: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771',
            mappedAddress: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771000000000000000000000000',
        };
        const { nonce: evmAccountInitNonce, data: evmAccountInitBalance } = await context.api.query.system.account(
            evmAccountRaw.mappedAddress
        );

        let eveMappedAccount = context.eve.publicKey.slice(0, 20);
        console.log(`eve address: ${context.eve.publicKey}`);
        console.log(`eveMappedAccount: ${eveMappedAccount}`);

        let value = 20000000000000; // 20 000 000 000 000
        // 25000 is min_gas_price setup
        const tx = context.api.tx.evm.call(
            eveMappedAccount,
            evmAccountRaw.address,
            '0x',
            value,
            1000000,
            25000,
            null,
            null,
            []
        );
        let block = await context.api.rpc.chain.getBlock();
        const blockNumber = block.block.header.number;
        console.log(`evm call await before: ${block.block.header.number}`);
        await signAndSend(tx, context.eve);
        let temp = await context.api.rpc.chain.getBlock();
        console.log(`evm call await end: ${temp.block.header.number}`);

        const { nonce: eveCurrentNonce, data: eveCurrentBalance } = await context.api.query.system.account(
            context.eve.address
        );
        const { nonce: evmAccountCurrentNonce, data: evmAccountCurrentBalance } =
            await context.api.query.system.account(evmAccountRaw.mappedAddress);

        // If a substrate account using pallet_evm to trigger evm transaction,
        // it will bump 2 for nonce (one for substrate extrinsic, one for evm).
        expect(eveCurrentNonce.toNumber()).to.equal(eveInitNonce.toNumber() + 2);
        expect(evmAccountCurrentBalance.free.toBigInt()).to.equal(
            evmAccountInitBalance.free.toBigInt() + BigInt(value)
        );

        // In case evm is not enabled in Normal Mode, switch back to filterMode, after test.
        let extrinsic = context.api.tx.sudo.sudo(context.api.tx.extrinsicFilter.setMode(filterMode));
        await signAndSend(extrinsic, context.alice);
    });

    step('Transfer some value back to Eve Mapped account from EVM external account', async function () {
        // In case evm is not enabled in Normal Mode, switch back to filterMode, after test.
        // We do not test mode in initialization since ts-test concerns filter function too.
        const filterMode = (await context.api.query.extrinsicFilter.mode()).toHuman();
        if ('Test' !== filterMode) {
            let extrinsic = context.api.tx.sudo.sudo(context.api.tx.extrinsicFilter.setMode('Test'));
            let temp = await context.api.rpc.chain.getBlock();
            console.log(`setMode await Before: ${temp.block.header.number}`);
            await signAndSend(extrinsic, context.alice);
            temp = await context.api.rpc.chain.getBlock();
            console.log(`setMode await end: ${temp.block.header.number}`);
        }

        // Get the initial balance of Eve and EVM external account
        const { nonce: eveInitNonce, data: eveInitBalance } = await context.api.query.system.account(
            context.eve.address
        );
        const evmAccountRaw = {
            privateKey: '0x01ab6e801c06e59ca97a14fc0a1978b27fa366fc87450e0b65459dd3515b7391',
            address: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771',
            mappedAddress: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771000000000000000000000000',
        };
        const { nonce: evmAccountInitNonce, data: evmAccountInitBalance } = await context.api.query.system.account(
            evmAccountRaw.mappedAddress
        );
        console.log(`evmAccount Balance: ${evmAccountInitBalance}`);
        let eveMappedAccount = u8aToHex(context.eve.publicKey.slice(0, 20));

        // Create Web3 instance
        const web3 = new Web3('http://localhost:9944');

        let value = 100000000000;
        // ExistentialDeposit = 100 000 000 000 (0x174876E800)
        // Sign Tx with PK
        console.log(`Tx Signing with: ${evmAccountRaw.privateKey}`);
        const transferTransaction = await web3.eth.accounts.signTransaction(
            {
                from: evmAccountRaw.address,
                to: eveMappedAccount,
                value: value, // must be higher than ExistentialDeposit
                gasPrice: 25000,
                gas: 1000000,
            },
            evmAccountRaw.privateKey
        );
        console.log(`Tx Signed with: ${transferTransaction.rawTransaction}`);
        const transferReceipt = await web3.eth.sendSignedTransaction(transferTransaction.rawTransaction!);
        console.log(`Tx successful with hash: ${transferReceipt.transactionHash}`);

        const { nonce: eveCurrentNonce, data: eveCurrentBalance } = await context.api.query.system.account(
            context.eve.address
        );
        const { nonce: evmAccountCurrentNonce, data: evmAccountCurrentBalance } =
            await context.api.query.system.account(evmAccountRaw.mappedAddress);

        console.log(`evmAccount Balance: ${evmAccountCurrentBalance}`);

        expect(evmAccountCurrentNonce.toNumber()).to.equal(evmAccountInitNonce.toNumber() + 1);
        expect(eveCurrentBalance.free.toBigInt()).to.equal(eveInitBalance.free.toBigInt() + BigInt(100000000000));

        // In case evm is not enabled in Normal Mode, switch back to filterMode, after test.
        let extrinsic = context.api.tx.sudo.sudo(context.api.tx.extrinsicFilter.setMode(filterMode));
        await signAndSend(extrinsic, context.alice);
    });

    step('Test substrate signature can not access ultra vires evm/substrate account', async function () {
        // In case evm is not enabled in Normal Mode, switch back to filterMode, after test.
        // We do not test mode in initialization since ts-test concerns filter function too.
        const filterMode = (await context.api.query.extrinsicFilter.mode()).toHuman();
        if ('Test' !== filterMode) {
            let extrinsic = context.api.tx.sudo.sudo(context.api.tx.extrinsicFilter.setMode('Test'));
            await signAndSend(extrinsic, context.alice);
        }

        // Get the initial balance of Eve and EVM external account
        const { nonce: eveInitNonce, data: eveInitBalance } = await context.api.query.system.account(
            context.eve.address
        );
        // EVM module transfer for substrate account
        const evmAccountRaw = {
            privateKey: '0x01ab6e801c06e59ca97a14fc0a1978b27fa366fc87450e0b65459dd3515b7391',
            address: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771',
            mappedAddress: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771000000000000000000000000',
        };
        const { nonce: evmAccountInitNonce, data: evmAccountInitBalance } = await context.api.query.system.account(
            evmAccountRaw.mappedAddress
        );

        const evmAccount = createPair(
            { toSS58: encodeAddress, type: 'ethereum' },
            { publicKey: hexToU8a(evmAccountRaw.mappedAddress), secretKey: new Uint8Array([]) }
        );

        let eveMappedAccount = context.eve.publicKey.slice(0, 20);
        let value = 100000000000; // ExistentialDeposit = 100 000 000 000 (0x174876E800)
        // Sign Tx with substrate signature, try manipulate evm account out of substrate signature's control
        // 25000 is min_gas_price setup
        const tx = context.api.tx.evm.call(
            evmAccountRaw.address,
            eveMappedAccount,
            '0x',
            value,
            1000000,
            25000,
            null,
            null,
            []
        );
        await signAndSend(tx, context.eve);

        const { nonce: eveCurrentNonce, data: eveCurrentBalance } = await context.api.query.system.account(
            context.eve.address
        );
        const { nonce: evmAccountCurrentNonce, data: evmAccountCurrentBalance } =
            await context.api.query.system.account(evmAccountRaw.mappedAddress);

        // Extrinsic succeed with failed origin
        // So the evm transaction nonce bump will not be triggered
        expect(eveCurrentNonce.toNumber()).to.equal(eveInitNonce.toNumber() + 1);
        // Which means balance unchanged
        expect(evmAccountCurrentBalance.free.toBigInt()).to.equal(evmAccountInitBalance.free.toBigInt());

        // In case evm is not enabled in Normal Mode, switch back to filterMode, after test.
        let extrinsic = context.api.tx.sudo.sudo(context.api.tx.extrinsicFilter.setMode(filterMode));
        await signAndSend(extrinsic, context.alice);
    });
});
