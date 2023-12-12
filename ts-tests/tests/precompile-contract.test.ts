import { assert, expect } from 'chai';
import { step } from 'mocha-steps';
import { AbiItem } from 'web3-utils';
import { signAndSend, describeLitentry, loadConfig, sleep } from './utils';
import { hexToU8a } from '@polkadot/util';
import { createPair, encodeAddress } from '@polkadot/keyring';
import Web3 from 'web3';
import ethers from 'ethers';
import { compiled } from './compile';
import precompileContractAbi from '../precompile/contracts/staking.json';

describeLitentry('Test Parachain Precompile Contract', ``, (context) => {
    const precompileContractAddress = '0x000000000000000000000000000000000000502d';
    // 10000 lit test token
    const evmAccountRaw = {
        privateKey: '0x01ab6e801c06e59ca97a14fc0a1978b27fa366fc87450e0b65459dd3515b7391',
        address: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771',
        mappedAddress: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771000000000000000000000000',
    };

    // candidate: collator address:5GKVBdNwNY6H7uqKtx3gks3t9aSpjG6uHEsFYKWUJtexxkyQ
    // transform to bytes32(public key) reference:https://polkadot.subscan.io/tools/format_transform?input=5GKVBdNwNY6H7uqKtx3gks3t9aSpjG6uHEsFYKWUJtexxkyQ&type=All
    const candidateCollator = '0xbc36f40fcf0e5f8f5245195f4c522a5f1ea85b1c279eedc22437de0568bc1f2a';
    const rococoParachainURL = 'https://rpc.rococo-parachain.litentry.io';
    const web3 = new Web3(rococoParachainURL);
    const precompileContract = new web3.eth.Contract(precompileContractAbi as AbiItem[], precompileContractAddress);

    const executeTransaction = async (delegateTransaction: any, label = '') => {
        console.log('Executing transaction ', label);
        const transaction = await web3.eth.accounts.signTransaction(
            {
                to: precompileContractAddress,
                data: delegateTransaction.encodeABI(),
                gas: await delegateTransaction.estimateGas(),
            },
            evmAccountRaw.privateKey
        );
        console.log('Raw transaction ', transaction.rawTransaction);
        const result = await web3.eth.sendSignedTransaction(transaction.rawTransaction!);
        console.log('Result: ', result);
        return result;
    };

    // To see full params types for the interfaces, check notion page: https://web3builders.notion.site/Parachain-Precompile-Contract-0c34929e5f16408084446dcf3dd36006
    step('Test precompile contract', async function () {
        // evmAccountRaw need lit test token to test precompile contract
        const { nonce: evmAccountCurrentNonce, data: evmAccountCurrentBalance } =
            await context.api.query.system.account(evmAccountRaw.mappedAddress);

        console.log(`evmAccount Balance: ${evmAccountCurrentBalance}`);

        // delegateWithAutoCompound(collator, amount, percent)
        const delegateWithAutoCompound = precompileContract.methods.delegateWithAutoCompound(candidateCollator, 1, 1);
        // construct transaction
        let result = await executeTransaction(delegateWithAutoCompound, 'delegateWithAutoCompound');
        const { nonce, data: balance } = await context.api.query.system.account(evmAccountRaw.mappedAddress);
        console.log(`evmAccount Balance after tx: ${balance.free.toBigInt()} ${nonce} nonce`);

        // delegatorBondMore(collator, amount)
        const delegatorBondMore = precompileContract.methods.delegatorBondMore(candidateCollator, 1);
        result = await executeTransaction(delegatorBondMore, 'delegatorBondMore');

        // scheduleDelegatorBondLess(collator, amount)
        const scheduleDelegatorBondLess = precompileContract.methods.scheduleDelegatorBondLess(candidateCollator, 1);
        result = await executeTransaction(scheduleDelegatorBondLess, 'scheduleDelegatorBondLess');

        // delegate(collator, amount)
        const delegate = precompileContract.methods.delegate(candidateCollator, 1);
        result = await executeTransaction(delegate, 'delegate');

        // setAutoCompound(collator, percent)
        const setAutoCompound = precompileContract.methods.setAutoCompound(candidateCollator);
        result = await executeTransaction(setAutoCompound, 'setAutoCompound');

        // scheduleRevokeDelegation(collator)
        const scheduleRevokeDelegation = precompileContract.methods.scheduleRevokeDelegation(candidateCollator);
        result = await executeTransaction(scheduleRevokeDelegation, 'scheduleRevokeDelegation');

        // executeDelegationRequest(delegator, collator)
        const executeDelegationRequest = precompileContract.methods.executeDelegationRequest(
            evmAccountRaw.mappedAddress,
            candidateCollator
        );
        result = await executeTransaction(executeDelegationRequest, 'executeDelegationRequest');

        // cancelDelegationRequest(collator)
        const cancelDelegationRequest = precompileContract.methods.cancelDelegationRequest(candidateCollator);
        result = await executeTransaction(cancelDelegationRequest, 'cancelDelegationRequest');

        // read contract sample
        // params: bytes32 delegator(user address), bytes32 candidate(collator address, see above)
        // why doesn't it work? fails with:
        //  ** Test precompile contract:
        //  ** Error: Returned values aren't valid, did it run Out of Gas?
        //  ** You might also see this error if you are not using the correct ABI for the contract you are retrieving data from, requesting data from a block number that does not exist, or querying a node which is not fully synced.**
        // const pending = await precompileContract.methods
        //     .delegationRequestIsPending(evmAccountRaw.mappedAddress, candidateCollator)
        //     .call();

        // console.log('pending', pending);
    });
});
