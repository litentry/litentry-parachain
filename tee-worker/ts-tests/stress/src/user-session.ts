import { LitentryPrimitivesIdentity } from 'sidechain-api';
import { Index } from '@polkadot/types/interfaces';
import { buildIdentityFromWallet, getSidechainNonce, Wallet, Api } from './litentry-api';
import { Runner } from './measurement';
import { setShieldingKey } from './steps';

export type UserSession = {
    primary: Wallet;
    userShieldingKey: `0x${string}`;
    subject: LitentryPrimitivesIdentity;
    nextNonce: () => Index;
};
export async function newUserSession(
    primary: Wallet,
    userShieldingKey: `0x${string}`,
    api: Api,
    log: WritableStream<string>,
    runner: Runner<string, boolean>
): Promise<UserSession> {
    const subject = await buildIdentityFromWallet(primary, api.sidechainRegistry);
    const initialNonce = await getSidechainNonce(
        api.teeWorker,
        api.parachainApi,
        api.mrEnclave,
        api.teeShieldingKey,
        subject,
        log
    );
    let currentNonce = initialNonce.toNumber();
    const nextNonce = () => api.parachainApi.createType('Index', currentNonce++);

    await setShieldingKey(
        runner,
        primary,
        api.sidechainRegistry,
        api.teeWorker,
        api.parachainApi,
        api.mrEnclave,
        api.teeShieldingKey,
        userShieldingKey,
        nextNonce(),
        subject,
        log
    );

    return {
        primary,
        userShieldingKey,
        subject,
        nextNonce,
    };
}
