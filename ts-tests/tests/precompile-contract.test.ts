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
    
    const precompileContractAddress='0x000000000000000000000000000000000000502d'

    step('Init precompile contract', async function () {
        const web3 = new Web3('http://localhost:9944');
        const precompileContract = new web3.eth.Contract(precompileContractAbi as AbiItem[], precompileContractAddress);
        console.log(precompileContract.methods);
        
     });
    

});
