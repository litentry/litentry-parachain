import { assert, expect } from 'chai';
import { step } from 'mocha-steps';

import { signAndSend, describeLitentry, loadConfig, sleep } from './utils';
import { hexToU8a } from '@polkadot/util';
import { createPair, encodeAddress } from '@polkadot/keyring';
import Web3 from "web3";

describeLitentry('Test EVM Module Transfer', ``, (context) => {
    console.log(`Test Balance Transfer`);

    step('Transfer Value from Eve to EVM external account', async function () {
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
            mappedAddress: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771000000000000000000000000'
        };
    
        const evmAccount = createPair({ toSS58: encodeAddress, type: 'ethereum' }, { publicKey: hexToU8a(evmAccountRaw.mappedAddress), secretKey: new Uint8Array([]) });
    
        let eveMappedAccount = context.eve.publicKey.slice(0, 20);
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

        
        // In case evm is not enabled in Normal Mode, switch back to filterMode, after test.
        let extrinsic = context.api.tx.sudo.sudo(context.api.tx.extrinsicFilter.setMode(filterMode));
        await signAndSend(extrinsic, context.alice);
    });

    step('Deploy and test contract by EVM external account', async function () {
        // In case evm is not enabled in Normal Mode, switch back to filterMode, after test.
        // We do not test mode in initialization since ts-test concerns filter function too.
        const filterMode = (await context.api.query.extrinsicFilter.mode()).toHuman();
        if ('Test' !== filterMode) {
            let extrinsic = context.api.tx.sudo.sudo(context.api.tx.extrinsicFilter.setMode('Test'));
            await signAndSend(extrinsic, context.alice);
        }

        const evmAccountRaw = {
            privateKey: '0x01ab6e801c06e59ca97a14fc0a1978b27fa366fc87450e0b65459dd3515b7391',
            address: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771',
            mappedAddress: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771000000000000000000000000'
        };
        const { nonce: evmAccountInitNonce, data: evmAccountInitBalance } = await context.api.query.system.account(
          evmAccountRaw.mappedAddress
        );

        const { compiled } = await import("./compile.mjs");
        // Get the bytecode and API
        const bytecode = compiled.evm.bytecode.object;
        const abi = compiled.abi;
        // Create Web3 instance
        const web3 = new Web3('http://localhost:9944');

        // Create deploy function
        const deploy = async (accountFrom) => {
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
        console.log('deployed', deployed);
        const result = (deployed.contractAddress === '0x687528e4BC4040DC9ADBA05C1f00aE3633faa731') ? 1 : 0;
        assert.equal(1, result, 'Contract address mismatch');

        // Test get message contract method
        const sayMessage = async (contractAddress) => {
          // 4. Create contract instance
          const hello = new web3.eth.Contract(abi, contractAddress);
          console.log(`Making a call to contract at address: ${contractAddress}`);
    
          // 6. Call contract
          const data = await hello.methods.sayMessage().call();
    
          console.log(`The current message is: ${data}`);
    
          return data;
        };

        const deployedContract = '0x687528e4BC4040DC9ADBA05C1f00aE3633faa731';
        const message = await sayMessage(deployedContract);
        const initialResult = (message === 'Hello World') ? 1 : 0;
        assert.equal(1, initialResult, 'Contract initial storage query mismatch');

        // Test set message contract method
        const setMessage = async (contractAddress, accountFrom, message) => {
          console.log(
              `Calling the setMessage function in contract at address: ${contractAddress}`
          );
    
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
        const setMsg = await setMessage(deployedContract, evmAccountRaw, 'Goodbye World');
        const sayMsg = await sayMessage(deployedContract);
        const setResult = (sayMsg === 'Goodbye World') ? 1 : 0;
        assert.equal(1, setResult, 'Contract modified storage query mismatch');

        // In case evm is not enabled in Normal Mode, switch back to filterMode, after test.
        let extrinsic = context.api.tx.sudo.sudo(context.api.tx.extrinsicFilter.setMode(filterMode));
        await signAndSend(extrinsic, context.alice);
    });
});