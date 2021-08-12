import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';

// Claim Assets for Alice
export async function assetClaim(api: ApiPromise, alice: KeyringPair) {
    console.log(`\nStep 3: Claim assets for Alice`);

    const transaction = await api.tx.offchainWorkerModule.assetClaim();

    const data = new Promise<{ block: string }>(async (resolve, reject) => {
        const unsub = await transaction.signAndSend(alice, (result) => {
            console.log(`Transfer is ${result.status}`);
            if (result.status.isInBlock) {
                console.log(`Transfer included at blockHash ${result.status.asInBlock}`);
                console.log(`Waiting for finalization... (can take a minute)`);
            } else if (result.status.isFinalized) {
                console.log(`Transfer finalized at blockHash ${result.status.asFinalized}`);
                unsub();
                resolve({
                    block: result.status.asFinalized.toString(),
                });
            }
        });
    });
    return data;
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
