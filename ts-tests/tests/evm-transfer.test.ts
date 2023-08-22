import { assert, expect } from 'chai';
import { step } from 'mocha-steps';

import { signAndSend, describeLitentry, loadConfig, sleep } from './utils';
import { hexToU8a } from '@polkadot/util';
import { createPair, encodeAddress } from '@polkadot/keyring';
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
    
        const evmAccount = createPair({ toSS58: encodeAddress, type: 'ethereum' }, { publicKey: hexToU8a(evmAccountRaw.mappedAddress), secretKey: new Uint8Array([]) });
    
        let eveMappedAccount = context.eve.address.slice(0, 20);
        let value = 200000000000; // ExistentialDeposit = 100 000 000 000 (0x174876E800)
        // 25000 is min_gas_price setup
        const tx = context.api.tx.evm.call(eveMappedAccount, evmAccountRaw.address, '0x', value, 1000000, 25000, null, null, []);
        await signAndSend(tx, context.eve);

        const { nonce: eveCurrentNonce, data: eveCurrentBalance } = await context.api.query.system.account(
            context.eve.address
        );
        const { nonce: evmAccountCurrentNonce, data: evmAccountCurrentBalance } = await context.api.query.system.account(
            evmAccountRaw.mappedAddress
        );

        expect(eveCurrentNonce.toNumber()).to.equal(eveInitNonce.toNumber() + 1);
        expect(evmAccountCurrentBalance.free.toBigInt()).to.equal(BigInt(value));
    });

    step('Transfer some value back to Eve Mapped account from EVM external account', async function () {
        // Get the initial balance of Eve and EVM external account
        const { nonce: eveInitNonce, data: eveInitBalance } = await context.api.query.system.account(
            context.eve.address
        );
        const evmAccountRaw = {
            privateKey: '0x01ab6e801c06e59ca97a14fc0a1978b27fa366fc87450e0b65459dd3515b7391',
            address: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771',
            mappedAddress: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771000000000000000000000000'
        };
        const evmAccount = createPair({ toSS58: encodeAddress, type: 'ethereum' }, { publicKey: hexToU8a(evmAccountRaw.mappedAddress), secretKey: new Uint8Array([]) });
        const { nonce: evmAccountInitNonce, data: evmAccountInitBalance } = await context.api.query.system.account(
            evmAccountRaw.mappedAddress
        );

        let eveMappedAccount = context.eve.address.slice(0, 20);

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

        const transferReceipt = await web3.eth.sendSignedTransaction(transferTransaction.rawTransaction!);
        console.log(`Tx successful with hash: ${transferReceipt.transactionHash}`);

        const { nonce: eveCurrentNonce, data: eveCurrentBalance } = await context.api.query.system.account(
            context.eve.address
        );
        const { nonce: evmAccountCurrentNonce, data: evmAccountCurrentBalance } = await context.api.query.system.account(
            evmAccountRaw.mappedAddress
        );

        expect(evmAccountCurrentNonce.toNumber()).to.equal(evmAccountInitNonce.toNumber() + 1);
        expect(eveCurrentBalance.free.toBigInt()).to.equal(eveInitBalance.free.toBigInt() + BigInt(100000000000));
    });

    step('Test evm signature can not access ultra vires evm account', async function () {
        // EVM account 1
        const evmAccountRaw = {
            privateKey: '0x7daadde6e9d1377640070b143cfbde103b078c008d35ee2c7ed989878f2187c7',
            address: '0x297f658F438C9c657c45fd6B1b0dB4222f1983B0',
            mappedAddress: '0x297f658F438C9c657c45fd6B1b0dB4222f1983B0000000000000000000000000'
        };
    
        const evmAccount = createPair({ toSS58: encodeAddress, type: 'ethereum' }, { publicKey: hexToU8a(evmAccountRaw.mappedAddress), secretKey: new Uint8Array([]) });
        // Create Web3 instance
        const web3 = new Web3('http://localhost:9944');
        
        let value = "0x174876E800";
        // ExistentialDeposit = 100 000 000 000 (0x174876E800)
        // Sign Tx with PK, try manipulate evm account out of private key's control
        const transferTransaction = await web3.eth.accounts.signTransaction(
            {
                from: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771',
                to: evmAccountRaw.address,
                value: value, // must be higher than ExistentialDeposit
                gasPrice: "0x3B9ACA00", // 1000000000,
				gas: "0x100000",
            },
            evmAccountRaw.privateKey
        );
        const transferReceipt = await web3.eth.sendSignedTransaction(transferTransaction.rawTransaction!);
        // Expect EVM revert
        assert(!transferReceipt.status, "Transaction with wrong signature succeed");

    });

    step('Test substrate signature can not access ultra vires evm/substrate account', async function () {
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
        const { nonce: evmAccountInitNonce, data: evmAccountInitBalance } = await context.api.query.system.account(
            evmAccountRaw.mappedAddress
        );
    
        const evmAccount = createPair({ toSS58: encodeAddress, type: 'ethereum' }, { publicKey: hexToU8a(evmAccountRaw.mappedAddress), secretKey: new Uint8Array([]) });
    
        let eveMappedAccount = context.eve.address.slice(0, 20);
        let value = 100000000000; // ExistentialDeposit = 100 000 000 000 (0x174876E800)
        // Sign Tx with substrate signature, try manipulate evm account out of substrate signature's control
        // 25000 is min_gas_price setup
        const tx = context.api.tx.evm.call(evmAccountRaw.address, eveMappedAccount, '0x', value, 1000000, 25000, null, null, []);
        await signAndSend(tx, context.eve);

        const { nonce: eveCurrentNonce, data: eveCurrentBalance } = await context.api.query.system.account(
            context.eve.address
        );
        const { nonce: evmAccountCurrentNonce, data: evmAccountCurrentBalance } = await context.api.query.system.account(
            evmAccountRaw.mappedAddress
        );
        
        // Extrinsic succeed with failed origin
        expect(eveCurrentNonce.toNumber()).to.equal(eveInitNonce.toNumber() + 1);
        // Which means balance unchanged
        expect(evmAccountCurrentBalance.free.toBigInt()).to.equal(evmAccountInitBalance.free.toBigInt());
    });
});
