import { Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { randomBytes } from 'crypto';
import { ethers } from 'ethers';
import { Wallet } from './litentry-api';

function randomSubstrateWallet(): KeyringPair {
    const keyring = new Keyring({ type: 'sr25519' });
    return keyring.addFromSeed(randomBytes(32));
}
function randomEvmWallet(): ethers.Wallet {
    return ethers.Wallet.createRandom();
}
export function randomWallet(): Wallet {
    return Math.random() > 0.5
        ? { type: 'substrate', keyringPair: randomSubstrateWallet() }
        : { type: 'evm', wallet: randomEvmWallet() };
}
