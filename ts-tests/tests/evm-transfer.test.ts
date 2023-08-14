import { expect } from 'chai';
import { step } from 'mocha-steps';

import { signAndSend, describeLitentry, loadConfig, sleep } from './utils';
import { addressToEvm, createPair, encodeAddress , hexToU8a, u8aToHex } from '@polkadot/util';
import Web3 from "web3";

describeLitentry('Test EVM Module Transfer', ``, (context) => {
    console.log(`Test Balance Transfer`);

    step('Transfer Value from Eve to EVM external account', async function () {
        // Get the initial balance of Eve and EVM external account
        const { nonce: eveInitNonce, data: eveInitBalance } = await context.api.query.system.account(
            context.eve.address
        );
        // EVM module transfer for substrate account
        const evmAccountRaw = {
            privateKey: '0x01ab6e801c06e59ca97a14fc0a1978b27fa366fc87450e0b65459dd3515b7391',
            address: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771',
            mappedAddress: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771000000000000000000000000'
        };
    
        const evmAccount = createPair({ encodeAddress, type: 'ethereum' }, { publicKey: hexToU8a(evmAccountRaw.mappedAddress), secretKey: new Uint8Array([]) });
    
        let eveMappedAccount = context.eve.address.slice(0, 42);
        let value = 200000000000; // ExistentialDeposit = 100 000 000 000 (0x174876E800)
        const tx = context.api.tx.evm.call(eveMappedAccount, evmAccount.address, '0x', value, 4294967295, 1, null);
        await signAndSend(tx, context.eve);

        const { nonce: eveCurrentNonce, data: eveCurrentBalance } = await context.api.query.system.account(
            context.eve.address
        );
        const { nonce: evmAccountCurrentNonce, data: evmAccountCurrentBalance } = await context.api.query.system.account(
            evmAccount.address
        );

        expect(eveCurrentNonce.toNumber()).to.equal(eveInitNonce.toNumber() + 1);
        expect(evmAccountCurrentNonce.free.toBigInt()).to.equal(BigInt(value));
    });

    step('Transfer 100 unit back to Eve Mapped account from EVM external account', async function () {
        // Get the initial balance of Eve and EVM external account
        const { nonce: eveInitNonce, data: eveInitBalance } = await context.api.query.system.account(
            context.eve.address
        );
        const evmAccountRaw = {
            privateKey: '0x01ab6e801c06e59ca97a14fc0a1978b27fa366fc87450e0b65459dd3515b7391',
            address: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771',
            mappedAddress: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771000000000000000000000000'
        };
        const evmAccount = createPair({ encodeAddress, type: 'ethereum' }, { publicKey: hexToU8a(evmAccountRaw.mappedAddress), secretKey: new Uint8Array([]) });
        const { nonce: evmAccountInitNonce, data: evmAccountInitBalance } = await context.api.query.system.account(
            evmAccount.address
        );

        let eveMappedAccount = context.eve.address.slice(0, 42);

        // Create Web3 instance
        const web3 = new Web3('http://localhost:9944');
        
        let value = "0x174876E800";
        // ExistentialDeposit = 100 000 000 000 (0x174876E800)
        // Sign Tx with PK
        const transferTransaction = await web3.eth.accounts.signTransaction(
            {
                from: evmAccountRaw.address,
                to: eveMappedAccount,
                value: value, // must be higher than ExistentialDeposit
                gasPrice: "0x3B9ACA00", // 1000000000,
				gas: "0x100000",
            },
            evmAccountRaw.privateKey
        );

        const transferReceipt = await web3.eth.sendSignedTransaction(transferTransaction.rawTransaction);
        console.log(`Tx successful with hash: ${transferReceipt.transactionHash}`);

        const { nonce: eveCurrentNonce, data: eveCurrentBalance } = await context.api.query.system.account(
            context.eve.address
        );
        const { nonce: evmAccountCurrentNonce, data: evmAccountCurrentBalance } = await context.api.query.system.account(
            evmAccount.address
        );

        expect(evmAccountCurrentNonce.toNumber()).to.equal(evmAccountInitNonce.toNumber() + 1);
        expect(eveCurrentBalance.free.toBigInt()).to.equal(eveInitBalance.free.toBigInt() + BigInt(100000000000));
    });
});
