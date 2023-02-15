import { encryptWithTeeShieldingKey, listenEncryptedEvents, sendTxUntilInBlock } from './utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { HexString } from '@polkadot/util/types';
import { IntegrationTestContext } from './type-definitions';
export async function setErrorUserShieldingKey(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean
): Promise<string | undefined> {
    const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, aesKey).toString('hex');

    const tx = context.substrate.tx.identityManagement.setUserShieldingKey(context.shard, `0x${ciphertext}`);

    await sendTxUntilInBlock(context.substrate, tx, signer);

    if (listening) {
        const result = await listenEncryptedEvents(context, aesKey, 'hex', {
            module: 'identityManagement',
            method: 'userShieldingKeySet',
            event: 'UserShieldingKeySet',
            errorEvent: 'SetUserShieldingKeyHandlingFailed',
        });

        return result as string;
    }
    return undefined;
}

export async function createErrorIdentity(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    errorCiphertext: string
): Promise<string | undefined> {
    const tx = context.substrate.tx.identityManagement.createIdentity(
        context.shard,
        signer.address,
        errorCiphertext,
        null
    );

    await sendTxUntilInBlock(context.substrate, tx, signer);

    if (listening) {
        const result = await listenEncryptedEvents(context, aesKey, 'hex', {
            module: 'identityManagement',
            method: 'identityCreated',
            event: 'IdentityCreated',
            errorEvent: 'CreateIdentityHandlingFailed',
        });

        return result as string;
    }
    return undefined;
}
