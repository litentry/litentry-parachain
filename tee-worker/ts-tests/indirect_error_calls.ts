import { IntegrationTestContext } from './type-definitions';
import { encryptWithTeeShieldingKey, listenErrorEvents, sendTxUntilInBlock } from './utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { HexString } from '@polkadot/util/types';

export async function setErrorUserShieldingKey(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean
): Promise<string | undefined> {
    const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, aesKey).toString('hex');

    await context.substrate.tx.identityManagement
        .setUserShieldingKey(context.shard, `0x${ciphertext}`)
        .paymentInfo(signer);

    const tx = context.substrate.tx.identityManagement.setUserShieldingKey(context.shard, `0x1`);

    //The purpose of paymentInfo is to check whether the version of polkadot/api is suitable for the current test and to determine whether the transaction is successful.
    await tx.paymentInfo(signer);

    await sendTxUntilInBlock(context.substrate, tx, signer);

    if (listening) {
        const result = await listenErrorEvents(context, aesKey, {
            module: 'identityManagement',
            method: 'userShieldingKeySet',
            event: 'UserShieldingKeySet',
            errorEvent: 'SetUserShieldingKeyHandlingFailed',
        });

        return result;
    }
    return undefined;
}
