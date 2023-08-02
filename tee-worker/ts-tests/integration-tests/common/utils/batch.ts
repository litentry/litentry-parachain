import { ExtrinsicStatus } from '@polkadot/types/interfaces';
import { ApiPromise, SubmittableResult } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';

export async function batchAndWait(api: ApiPromise, signer: KeyringPair, txs: any[]): Promise<ExtrinsicStatus> {
    return await new Promise((resolve, reject) => {
        api.tx.utility.batch(txs).signAndSend(signer, (result: SubmittableResult) => {
            console.log(`Current status is ${result.status}`);
            if (result.status.isFinalized) {
                resolve(result.status);
            } else if (result.status.isInvalid) {
                console.log(`Transaction is ${result.status}`);
                reject(result.status);
            }
        });
    });
}
