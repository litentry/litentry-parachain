import { assert, expect } from 'chai';
import { step } from 'mocha-steps';
import { AbiItem } from 'web3-utils';
import { signAndSend, describeLitentry, loadConfig, sleep } from './utils';
import { hexToU8a, hexToString, u8aToString, u8aToHex } from '@polkadot/util';
import Keyring, { createPair, encodeAddress } from '@polkadot/keyring';
import Web3 from 'web3';
import ethers from 'ethers';
import { compiled } from './compile';
import precompileContractAbi from '../precompile/contracts/staking.json';
import { mnemonicGenerate, mnemonicToMiniSecret, decodeAddress, evmToAddress } from '@polkadot/util-crypto';

const toBigNumber = (int: number) => int * 1e12;

describeLitentry('Test Parachain Precompile Contract', ``, (context) => {
    const config = loadConfig();

    const precompileContractAddress = '0x000000000000000000000000000000000000502d';
    const evmAccountRaw = {
        privateKey: '0x01ab6e801c06e59ca97a14fc0a1978b27fa366fc87450e0b65459dd3515b7391',
        address: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771',
        mappedAddress: evmToAddress('0xaaafB3972B05630fCceE866eC69CdADd9baC2771', 31),
    };

    // candidate: collator address: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
    // transform to bytes32(public key) reference:https://polkadot.subscan.io/tools/format_transform?input=5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY&type=All
    const collatorPublicKey = '0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d';

    const web3 = new Web3(config.parachain_ws);
    const precompileContract = new web3.eth.Contract(precompileContractAbi as AbiItem[], precompileContractAddress);

    const executeTransaction = async (delegateTransaction: any, label = '') => {
        console.log('Executing transaction ', label);

        // estimate gas doesn't work
        // const gas = await delegateTransaction.estimateGas();
        // console.log("gas", gas);

        const transaction = await web3.eth.accounts.signTransaction(
            {
                to: precompileContractAddress,
                data: delegateTransaction.encodeABI(),
                gas: 1000000,
            },
            evmAccountRaw.privateKey
        );
        const result = await web3.eth.sendSignedTransaction(transaction.rawTransaction!);
        console.log('Result: ', result);
        return result;
    };

    const printBalance = (label: string, bl: any) => {
        console.group("====", label, "====");
        console.log('free', bl.free.toNumber() / 1e12, 'reserved', bl.reserved.toNumber() / 1e12);
        console.groupEnd();
    };

    const transferTokens = async (from: typeof context.alice, to: any) => {
        const filterMode = (await context.api.query.extrinsicFilter.mode()).toHuman();
        if ('Test' !== filterMode) {
            let extrinsic = context.api.tx.sudo.sudo(context.api.tx.extrinsicFilter.setMode('Test'));
            let temp = await context.api.rpc.chain.getBlock();
            console.log(`setMode await Before: ${temp.block.header.number}`);
            await signAndSend(extrinsic, context.alice);
            temp = await context.api.rpc.chain.getBlock();
            console.log(`setMode await end: ${temp.block.header.number}`);
        }

        const aliceEVMMappedAccount = from.publicKey.slice(0, 20); // pretend to be evm
        console.log(`alice address: ${from.publicKey}`);
        console.log(`aliceEVMMappedAccount: ${aliceEVMMappedAccount}`);

        let aliceMappedSustrateAccount = evmToAddress(aliceEVMMappedAccount, 31);

        console.log("transfer from Alice to alice EMV");
        
        // Deposit money into substrate account's truncated EVM address's mapping substrate account
        const tx_init = context.api.tx.balances.transfer(aliceMappedSustrateAccount, 65 * 1e12);
        await signAndSend(tx_init, context.alice);
        console.log("transfered from Alice to alice EMV");

        // 1 - substrate Alice has 70 tokens
        // 2 - alice need to turn to own evm address account

        // 25000 is min_gas_price setup
        const tx = context.api.tx.evm.call(
            aliceEVMMappedAccount, // evm like
            to.address, // evm like
            '0x',
            toBigNumber(65),
            1000000,
            25000,
            null,
            null,
            []
        );
        let block = await context.api.rpc.chain.getBlock();
        console.log(`evm call await before: ${block.block.header.number}`);
        await signAndSend(tx, from);
        let temp = await context.api.rpc.chain.getBlock();
        console.log(`evm call await end: ${temp.block.header.number}`);
    };

    step('Address with not sufficient amount of tokens', async function () {
        // Create valid Substrate-compatible seed from mnemonic
        const randomSeed = mnemonicToMiniSecret(mnemonicGenerate());
        const secretKey = Buffer.from(randomSeed).toString('hex');

        const delegateWithAutoCompound = precompileContract.methods.delegateWithAutoCompound(
            collatorPublicKey,
            toBigNumber(60),
            1
        );

        try {
            await web3.eth.accounts.signTransaction(
                {
                    to: precompileContractAddress,
                    data: delegateWithAutoCompound.encodeABI(),
                    gas: await delegateWithAutoCompound.estimateGas(),
                },
                secretKey
            );
            expect(true).to.eq(false); // test should fail here
        } catch (e) {
            expect(e).to.be.instanceof(Error);
        }
    });

    // To see full params types for the interfaces, check notion page: https://web3builders.notion.site/Parachain-Precompile-Contract-0c34929e5f16408084446dcf3dd36006
    step('Test precompile contract', async function () {
        const { data } = await context.api.query.system.account(evmAccountRaw.mappedAddress);
        printBalance('initial balance', data);
        // top up LITs if not sufficient amount for staking (require: 50 LITs minimum)
        if (data.free.toNumber() < toBigNumber(60)) {
            console.log('transferring more tokens');

            await transferTokens(context.alice, evmAccountRaw);
            printBalance('after balance balance', (await context.api.query.system.account(evmAccountRaw.mappedAddress)).data);
        }

        //// TESTS

        // const isPendingRequest = await precompileContract.methods
        //     .delegationRequestIsPending(evmAccountRaw.mappedAddress, collatorPublicKey)
        //     .call();

        // console.log('pending staking', isPendingRequest);


        // // cancelDelegationRequest(collator)
        // const cancelDelegationRequest = precompileContract.methods.cancelDelegationRequest(collatorPublicKey);
        // await executeTransaction(cancelDelegationRequest, 'cancelDelegationRequest');
        // const { data: balanceAfterCancel } = await context.api.query.system.account(evmAccountRaw.mappedAddress);
        // printBalance('balanceAfterCancel', balanceAfterCancel);


        // const shouldBeFalse = await precompileContract.methods
        //     .delegationRequestIsPending(evmAccountRaw.mappedAddress, collatorPublicKey)
        //     .call();

        // console.log('should be false', shouldBeFalse);
        // delegateWithAutoCompound(collator, amount, percent)
        const delegateWithAutoCompound = precompileContract.methods.delegateWithAutoCompound(
            collatorPublicKey,
            toBigNumber(60),
            20
        );
        // construct transaction
        await executeTransaction(delegateWithAutoCompound, 'delegateWithAutoCompound');
        const { data: balance } = await context.api.query.system.account(evmAccountRaw.mappedAddress);
        printBalance('balance', balance);
        // expect(balance.free.toNumber()).to.lt(balance.free.toNumber() - toBigNumber(60));
        // expect(balance.reserved.toNumber()).to.eq(toBigNumber(60));

        // delegatorBondMore(collator, amount)
        const delegatorBondMore = precompileContract.methods.delegatorBondMore(collatorPublicKey, toBigNumber(1));
        await executeTransaction(delegatorBondMore, 'delegatorBondMore');

        const { data: balanceAfterBondMore } = await context.api.query.system.account(evmAccountRaw.mappedAddress);
        // expect(balanceAfterBondMore.free.toNumber()).to.eq(balanceAfterBondMore.free.toNumber() - toBigNumber(1));
        // expect(balanceAfterBondMore.reserved.toNumber()).to.eq(toBigNumber(61));
        printBalance('balanceAfterBondMore', balanceAfterBondMore);

        // Ask minqi should it be triggered after execute
        // scheduleDelegatorBondLess(collator, amount)
        const scheduleDelegatorBondLess = precompileContract.methods.scheduleDelegatorBondLess(
            collatorPublicKey,
            toBigNumber(5)
        );
        await executeTransaction(scheduleDelegatorBondLess, 'scheduleDelegatorBondLess');
        const { data: balanceAfterBondLess } = await context.api.query.system.account(evmAccountRaw.mappedAddress);
        printBalance('balanceAfterBondLess', balanceAfterBondLess);

        // cancelDelegationRequest(collator)
        const cancelDelegationRequest = precompileContract.methods.cancelDelegationRequest(collatorPublicKey);
        await executeTransaction(cancelDelegationRequest, 'cancelDelegationRequest');
        const { data: balanceAfterCancel } = await context.api.query.system.account(evmAccountRaw.mappedAddress);
        printBalance('balanceAfterCancel', balanceAfterCancel);

        // delegate(collator, amount)
        const delegate = precompileContract.methods.delegate(collatorPublicKey, toBigNumber(55));
        await executeTransaction(delegate, 'delegate');
        const { data: balanceAfterDelegate } = await context.api.query.system.account(evmAccountRaw.mappedAddress);
        printBalance('balanceAfterDelegate', balanceAfterDelegate);

        // setAutoCompound(collator, percent)
        const setAutoCompound = precompileContract.methods.setAutoCompound(collatorPublicKey);
        await executeTransaction(setAutoCompound, 'setAutoCompound');
        
        // scheduleRevokeDelegation(collator)
        const scheduleRevokeDelegation = precompileContract.methods.scheduleRevokeDelegation(collatorPublicKey);
        await executeTransaction(scheduleRevokeDelegation, 'scheduleRevokeDelegation');
        const { data: balanceAfterRevoke } = await context.api.query.system.account(evmAccountRaw.mappedAddress);
        printBalance('balanceAfterRevoke', balanceAfterRevoke);

        // executeDelegationRequest(delegator, collator)
        const executeDelegationRequest = precompileContract.methods.executeDelegationRequest(
            evmAccountRaw.mappedAddress,
            collatorPublicKey
        );
        await executeTransaction(executeDelegationRequest, 'executeDelegationRequest');
        const { data: balanceAfterExecuteDelegation } = await context.api.query.system.account(
            evmAccountRaw.mappedAddress
        );
        printBalance('balanceAfterExecuteDelegation', balanceAfterExecuteDelegation);

        const stakedResult = await precompileContract.methods
            .delegationRequestIsPending(evmAccountRaw.mappedAddress, collatorPublicKey)
            .call();

        console.log('pending', stakedResult);
    });
});
