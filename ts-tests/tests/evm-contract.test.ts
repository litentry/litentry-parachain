import { assert, expect } from 'chai';
import { step } from 'mocha-steps';

import { signAndSend, describeLitentry, loadConfig, sleep } from './utils';
import { evmToAddress } from '@polkadot/util-crypto';
import Web3 from 'web3';

import { compiled } from './compile';

describeLitentry('Test EVM Module Contract', ``, (context) => {
    console.log(`Test EVM Module Contract`);

    step('Transfer Value from Eve to EVM external account', async function () {
        // In case evm is not enabled in Normal Mode, switch back to filterMode, after test.
        // We do not test mode in initialization since ts-test concerns filter function too.
        const filterMode = (await context.api.query.extrinsicFilter.mode()).toHuman();
        if ('Test' !== filterMode) {
            let extrinsic = context.api.tx.sudo.sudo(context.api.tx.extrinsicFilter.setMode('Test'));
            await signAndSend(extrinsic, context.alice);
        }

        let eveMappedEVMAccount = context.eve.publicKey.slice(0, 20);
        let eveMappedSustrateAccount = evmToAddress(eveMappedEVMAccount, 31);

        // Deposit money into substrate account's truncated EVM address's mapping substrate account
        const tx_init = context.api.tx.balances.transfer(eveMappedSustrateAccount, 30000000000000);
        await signAndSend(tx_init, context.eve);

        // Get the initial balance of Eve and EVM external account
        const { nonce: eveInitNonce, data: eveInitBalance } = await context.api.query.system.account(
            context.eve.address
        );
        // EVM module transfer for substrate account
        const evmAccountRaw = {
            privateKey: '0x01ab6e801c06e59ca97a14fc0a1978b27fa366fc87450e0b65459dd3515b7391',
            address: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771',
            mappedAddress: evmToAddress('0xaaafB3972B05630fCceE866eC69CdADd9baC2771', 31),
        };
        const { nonce: evmAccountInitNonce, data: evmAccountInitBalance } = await context.api.query.system.account(
            evmAccountRaw.mappedAddress
        );

        let value = 20000000000000; // 20 000 000 000 000
        // 25000 is min_gas_price setup
        const tx = context.api.tx.evm.call(
            eveMappedEVMAccount,
            evmAccountRaw.address,
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

        // If a substrate account using pallet_evm to trigger evm transaction,
        // it will bump 2 for nonce (one for substrate extrinsic, one for evm).
        // +1 nonce for original substrate account, plus another 1 nonce for original substrate account's truncated evm address's mapped susbtrate account.
        expect(eveCurrentNonce.toNumber()).to.equal(eveInitNonce.toNumber() + 1);
        expect(evmAccountCurrentBalance.free.toBigInt()).to.equal(
            evmAccountInitBalance.free.toBigInt() + BigInt(value)
        );

        // In case evm is not enabled in Normal Mode, switch back to filterMode, after test.
        let extrinsic = context.api.tx.sudo.sudo(context.api.tx.extrinsicFilter.setMode(filterMode));
        await signAndSend(extrinsic, context.alice);
    });

    step('Deploy and test contract by EVM external account', async function () {
        // We want evm works in Normal Mode, switch back to filterMode, after testing.
        // We do not test mode in initialization since ts-test concerns filter function too.
        const filterMode = (await context.api.query.extrinsicFilter.mode()).toHuman();
        if ('Normal' !== filterMode) {
            let extrinsic = context.api.tx.sudo.sudo(context.api.tx.extrinsicFilter.setMode('Normal'));
            await signAndSend(extrinsic, context.alice);
        }

        const evmAccountRaw = {
            privateKey: '0x01ab6e801c06e59ca97a14fc0a1978b27fa366fc87450e0b65459dd3515b7391',
            address: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771',
            mappedAddress: evmToAddress('0xaaafB3972B05630fCceE866eC69CdADd9baC2771', 31),
        };
        const { nonce: evmAccountInitNonce, data: evmAccountInitBalance } = await context.api.query.system.account(
            evmAccountRaw.mappedAddress
        );

        // Get the bytecode and API
        const bytecode = compiled.evm.bytecode.object;
        const abi = compiled.abi;
        // Create Web3 instance
        const web3 = new Web3('http://localhost:9944');

        // Create deploy function
        const deploy = async (accountFrom: any) => {
            console.log(`Attempting to deploy from account ${accountFrom.address}`);

            // Create contract instance
            const hello = new web3.eth.Contract(abi);

            // Create constructor tx
            const helloTx = hello.deploy({
                data: bytecode,
                arguments: [],
            });

            // Sign transacation and send
            const createTransaction = await web3.eth.accounts.signTransaction(
                {
                    data: helloTx.encodeABI(),
                    gas: await helloTx.estimateGas(),
                },
                accountFrom.privateKey
            );

            // Send tx and wait for receipt
            const createReceipt = await web3.eth.sendSignedTransaction(createTransaction.rawTransaction!);
            console.log(`Contract deployed at address: ${createReceipt.contractAddress}`);

            return createReceipt;
        };

        const deployed = await deploy(evmAccountRaw);
        if (!deployed.contractAddress) {
            console.log('deployed', deployed);
        }

        // Test get message contract method
        const sayMessage = async (contractAddress: string) => {
            // 4. Create contract instance
            const hello = new web3.eth.Contract(abi, contractAddress);
            console.log(`Making a call to contract at address: ${contractAddress}`);

            // 6. Call contract
            const data = await hello.methods.sayMessage().call();

            console.log(`The current message is: ${data}`);

            return data;
        };

        const message = await sayMessage(deployed.contractAddress!);
        const initialResult = message === 'Hello World' ? 1 : 0;
        assert.equal(1, initialResult, 'Contract initial storage query mismatch');

        // Test set message contract method
        const setMessage = async (contractAddress: string, accountFrom: any, message: string) => {
            console.log(`Calling the setMessage function in contract at address: ${contractAddress}`);

            // Create contract instance
            const hello = new web3.eth.Contract(abi, contractAddress);
            // Build tx
            const helloTx = hello.methods.setMessage(message);

            // Sign Tx with PK
            const createTransaction = await web3.eth.accounts.signTransaction(
                {
                    to: contractAddress,
                    data: helloTx.encodeABI(),
                    gas: await helloTx.estimateGas(),
                },
                accountFrom.privateKey
            );

            // Send Tx and Wait for Receipt
            const createReceipt = await web3.eth.sendSignedTransaction(createTransaction.rawTransaction!);
            console.log(`Tx successful with hash: ${createReceipt.transactionHash}`);
        };
        const setMsg = await setMessage(deployed.contractAddress!, evmAccountRaw, 'Goodbye World');
        const sayMsg = await sayMessage(deployed.contractAddress!);
        const setResult = sayMsg === 'Goodbye World' ? 1 : 0;
        assert.equal(1, setResult, 'Contract modified storage query mismatch');

        // In case evm is not enabled in Normal Mode, switch back to filterMode, after test.
        let extrinsic = context.api.tx.sudo.sudo(context.api.tx.extrinsicFilter.setMode(filterMode));
        await signAndSend(extrinsic, context.alice);
    });
});
