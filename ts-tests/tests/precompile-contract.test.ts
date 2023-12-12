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
    step('Test precompile contract', async function () {
        const web3 = new Web3('https://rpc.rococo-parachain.litentry.io');
        const precompileContract = new web3.eth.Contract(precompileContractAbi as AbiItem[], precompileContractAddress);
        // console.log(precompileContract.methods);

        // evmAccountRaw need lit test token to test precompile contract

        const nonce = await web3.eth.getTransactionCount(evmAccountRaw.address);
        console.log(nonce);

        // write contract sample
        // params: bytes32 _candidate, uint256 _amount, uint8 _autoCompound  see notion interfaces:https://web3builders.notion.site/Parachain-Precompile-Contract-0c34929e5f16408084446dcf3dd36006

        // candidate: collator address:5GKVBdNwNY6H7uqKtx3gks3t9aSpjG6uHEsFYKWUJtexxkyQ transform to bytes32: 0xbc36f40fcf0e5f8f5245195f4c522a5f1ea85b1c279eedc22437de0568bc1f2a(public key) reference:https://polkadot.subscan.io/tools/format_transform?input=5GKVBdNwNY6H7uqKtx3gks3t9aSpjG6uHEsFYKWUJtexxkyQ&type=All
        const delegateWithAutoCompoundTx = precompileContract.methods.delegateWithAutoCompound(
            '0xbc36f40fcf0e5f8f5245195f4c522a5f1ea85b1c279eedc22437de0568bc1f2a',
            1,
            1
        );

        // construct transaction
        const createTransaction = await web3.eth.accounts.signTransaction(
            {
                to: precompileContractAddress,
                data: delegateWithAutoCompoundTx.encodeABI(),
                gas: await delegateWithAutoCompoundTx.estimateGas(),
            },
            evmAccountRaw.privateKey
        );

        console.log(createTransaction.rawTransaction);

        // send transaction
        const receipt = await web3.eth.sendSignedTransaction(createTransaction.rawTransaction!);
            console.log(receipt);
            
        // read contract sample
        // params: bytes32 delegator(user address), bytes32 candidate(collator address, see above)
        // why doesn't it work?
        const pending = await precompileContract.methods
            .delegationRequestIsPending(
                '0xaaafB3972B05630fCceE866eC69CdADd9baC2771000000000000000000000000',
                '0xbc36f40fcf0e5f8f5245195f4c522a5f1ea85b1c279eedc22437de0568bc1f2a',
            )
            .call();

        console.log(pending);
    });
});
