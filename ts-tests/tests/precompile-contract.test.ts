import { assert, expect } from 'chai';
import { step } from 'mocha-steps';
import { AbiItem } from 'web3-utils'
import { signAndSend, describeLitentry, loadConfig, sleep } from './utils';
import { hexToU8a } from '@polkadot/util';
import { createPair, encodeAddress } from '@polkadot/keyring';
import Web3 from 'web3';
import ethers from 'ethers'
import { compiled } from './compile';
import precompileContractAbi from '../precompile/contracts/staking.json'
describeLitentry('Test Parachain Precompile Contract', ``, (context) => {
    const precompileContractAddress = '0x000000000000000000000000000000000000502d'
    const evmAccountRaw = {
            privateKey: '0x01ab6e801c06e59ca97a14fc0a1978b27fa366fc87450e0b65459dd3515b7391',
            address: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771',
            mappedAddress: '0xaaafB3972B05630fCceE866eC69CdADd9baC2771000000000000000000000000',
    };
    step('Init precompile contract', async function () {
        const web3 = new Web3('https://rpc.rococo-parachain.litentry.io');
        const precompileContract = new web3.eth.Contract(precompileContractAbi as AbiItem[], precompileContractAddress);
        // console.log(precompileContract.methods);

        // evmAccountRaw need lit test token to test precompile contract
        
        const nonce = await web3.eth.getTransactionCount(evmAccountRaw.address);
        console.log(nonce);



        // write contract sample

        // params: bytes32 _candidate, uint256 _amount, uint8 _autoCompound  see notion interfaces:https://web3builders.notion.site/Parachain-Precompile-Contract-0c34929e5f16408084446dcf3dd36006
        const delegateWithAutoCompoundTx = precompileContract.methods.delegateWithAutoCompound(
          "0x000000000000000000000000aaafB3972B05630fCceE866eC69CdADd9baC2771",1,1
        )

        // construct transaction
          const createTransaction = await web3.eth.accounts.signTransaction(
                {
                    to: precompileContractAddress,
                    data: delegateWithAutoCompoundTx.encodeABI(),
                    gas: await delegateWithAutoCompoundTx.estimateGas(),
                },
                evmAccountRaw.privateKey
          );
        
        console.log(createTransaction);
        

        // send transaction
        // const receipt = await web3.eth.sendSignedTransaction(createTransaction.rawTransaction!);

        

        // read contract sample o
        // params: bytes32 delegator, bytes32 candidate        
        const pending = await precompileContract.methods.delegationRequestIsPending(
            "0x000000000000000000000000aaafB3972B05630fCceE866eC69CdADd9baC2771",
            "0x000000000000000000000000aaafB3972B05630fCceE866eC69CdADd9baC2771"
        ).call();

        console.log(pending);
        
        
    });
    
   
    
});


