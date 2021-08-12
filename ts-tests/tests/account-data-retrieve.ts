import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { signAndSend } from './utils';

// Claim Assets for Alice
export async function assetClaim(api: ApiPromise, alice: KeyringPair) {
    console.log(`\nStep 3: Claim assets for Alice`);

    const tx = await api.tx.offchainWorkerModule.assetClaim();

    return signAndSend(tx, alice);
}

// Retrieve assets balances of Alice
export async function getAssets(api: ApiPromise, alice: KeyringPair) {
    console.log(`\nStep 4: Retrieving assets of Alice`);

    // Retrieve Alice account with new nonce value
    const { nonce, data: balance } = await api.query.system.account(alice.address);
    console.log(`Alice Substrate Account (nonce: ${nonce}) balance, free: ${balance.free}`);

    const assetsBalances = await api.query.offchainWorkerModule.accountBalance(alice.address);
    console.log(`Linked Ethereum balances of Alice are: ${assetsBalances.toString()}`);

    return assetsBalances;
}
