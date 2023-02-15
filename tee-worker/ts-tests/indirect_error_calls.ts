import { encryptWithTeeShieldingKey, listenEvent, sendTxUntilInBlock } from './utils';
import { KeyringPair } from '@polkadot/keyring/types';
import { HexString } from '@polkadot/util/types';
import { IntegrationTestContext } from './type-definitions';
import { expect } from 'chai';

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
        const events = await listenEvent(context.substrate, 'identityManagement', [
            'SetUserShieldingKeyHandlingFailed',
        ]);
        expect(events.length).to.be.equal(1);
        return events[0].method as string;
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
        const events = await listenEvent(context.substrate, 'identityManagement', ['CreateIdentityHandlingFailed']);
        expect(events.length).to.be.equal(1);
        return events[0].method as string;
    }
    return undefined;
}
