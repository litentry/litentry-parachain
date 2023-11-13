import { LitentryPrimitivesIdentity, TypeRegistry as SidechainTypeRegistry } from 'sidechain-api';
import {
    Wallet,
    buildIdentityFromWallet,
    buildValidation,
    createSignedTrustedCallActivateIdentity,
    createSignedTrustedCallDeactivateIdentity,
    createSignedTrustedCallLinkIdentity,
    createSignedTrustedCallRequestVc,
    createSignedTrustedCallSetUserShieldingKey,
    sendRequestFromTrustedCall,
    subscribeToEventsWithExtHash,
} from './litentry-api';
import WebSocketAsPromised from 'websocket-as-promised';
import { ApiPromise as ParachainApiPromise } from 'parachain-api';
import crypto, { randomBytes } from 'crypto';
import { Index } from '@polkadot/types/interfaces';
import { Measurement, Runner } from './measurement';

const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';

export async function setShieldingKey(
    runner: Runner<string, boolean>,
    primary: Wallet,
    sidechainRegistry: SidechainTypeRegistry,
    teeWorker: WebSocketAsPromised,
    parachainApi: ParachainApiPromise,
    mrEnclave: string,
    teeShieldingKey: crypto.KeyObject,
    userShieldingKey: string,
    nonce: Index,
    subject: LitentryPrimitivesIdentity,
    log: WritableStream<string>
): Promise<void> {
    const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;

    const setUserShieldingKeyCall = await createSignedTrustedCallSetUserShieldingKey(
        parachainApi,
        mrEnclave,
        nonce,
        primary,
        subject,
        userShieldingKey,
        requestIdentifier
    );

    const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, parachainApi);

    await runner('setShieldingKey', async () => {
        await sendRequestFromTrustedCall(
            teeWorker,
            parachainApi,
            mrEnclave,
            teeShieldingKey,
            setUserShieldingKeyCall,
            log
        );

        const events = await eventsPromise;

        return events
            .map(({ event }) => event)
            .some((event) => parachainApi.events.identityManagement.UserShieldingKeySet.is(event));
    });
}

export async function linkIdentity(
    runner: Runner<string, boolean>,
    primary: Wallet,
    secondary: Wallet,
    sidechainRegistry: SidechainTypeRegistry,
    teeWorker: WebSocketAsPromised,
    parachainApi: ParachainApiPromise,
    mrEnclave: string,
    teeShieldingKey: crypto.KeyObject,
    nonce: Index,
    subject: LitentryPrimitivesIdentity,
    log: WritableStream<string>
): Promise<void> {
    const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
    const primarySubject = await buildIdentityFromWallet(primary, sidechainRegistry);
    const secondaryIdentity = await buildIdentityFromWallet(secondary, sidechainRegistry);
    const secondaryNetworks = parachainApi.createType(
        'Vec<Web3Network>',
        secondary.type === 'evm' ? ['Ethereum', 'Bsc'] : ['Litentry', 'Polkadot']
    );

    const secondaryValidation = await buildValidation(
        parachainApi,
        sidechainRegistry,
        subject,
        secondaryIdentity,
        nonce.toNumber(),
        secondary
    );

    const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
        parachainApi,
        mrEnclave,
        parachainApi.createType('Index', nonce),
        primary,
        primarySubject,
        secondaryIdentity.toHex(),
        secondaryValidation.toHex(),
        secondaryNetworks.toHex(),
        parachainApi.createType('Option<RequestAesKey>', aesKey).toHex(),
        requestIdentifier
    );
    const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, parachainApi);

    await runner('linkIdentity', async () => {
        await sendRequestFromTrustedCall(teeWorker, parachainApi, mrEnclave, teeShieldingKey, linkIdentityCall, log);

        const events = await eventsPromise;
        return events
            .map(({ event }) => event)
            .some((event) => parachainApi.events.identityManagement.IdentityLinked.is(event));
    });
}

export async function requestVc1(
    runner: Runner<string, boolean>,
    primary: Wallet,
    sidechainRegistry: SidechainTypeRegistry,
    teeWorker: WebSocketAsPromised,
    parachainApi: ParachainApiPromise,
    mrEnclave: string,
    teeShieldingKey: crypto.KeyObject,
    nonce: Index,
    subject: LitentryPrimitivesIdentity,
    log: WritableStream<string>
): Promise<void> {
    const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
    const requestVcCall = await createSignedTrustedCallRequestVc(
        parachainApi,
        mrEnclave,
        nonce,
        primary,
        subject,
        parachainApi.createType('Assertion', { A1: null }),
        parachainApi.createType('Option<RequestAesKey>', aesKey).toHex(),
        requestIdentifier
    );

    const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, parachainApi);

    await runner('requestVc1', async () => {
        await sendRequestFromTrustedCall(teeWorker, parachainApi, mrEnclave, teeShieldingKey, requestVcCall, log);

        const events = await eventsPromise;

        return events.map(({ event }) => event).some((event) => parachainApi.events.vcManagement.VCIssued.is(event));
    });
}

export async function requestVc4(
    runner: Runner<string, boolean>,
    primary: Wallet,
    sidechainRegistry: SidechainTypeRegistry,
    teeWorker: WebSocketAsPromised,
    parachainApi: ParachainApiPromise,
    mrEnclave: string,
    teeShieldingKey: crypto.KeyObject,
    nonce: Index,
    subject: LitentryPrimitivesIdentity,
    log: WritableStream<string>
): Promise<void> {
    const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
    const requestVcCall = await createSignedTrustedCallRequestVc(
        parachainApi,
        mrEnclave,
        nonce,
        primary,
        subject,
        parachainApi.createType('Assertion', { A4: '10' }),
        parachainApi.createType('Option<RequestAesKey>', aesKey).toHex(),
        requestIdentifier
    );

    const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, parachainApi);

    await runner('requestVc4', async () => {
        await sendRequestFromTrustedCall(teeWorker, parachainApi, mrEnclave, teeShieldingKey, requestVcCall, log);

        const events = await eventsPromise;

        return events.map(({ event }) => event).some((event) => parachainApi.events.vcManagement.VCIssued.is(event));
    });
}

export async function deactivateIdentity(
    runner: Runner<string, boolean>,
    primary: Wallet,
    secondary: Wallet,
    sidechainRegistry: SidechainTypeRegistry,
    teeWorker: WebSocketAsPromised,
    parachainApi: ParachainApiPromise,
    mrEnclave: string,
    teeShieldingKey: crypto.KeyObject,
    userShieldingKey: string,
    nonce: Index,
    subject: LitentryPrimitivesIdentity,
    log: WritableStream<string>
): Promise<void> {
    const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
    const secondaryIdentity = await buildIdentityFromWallet(secondary, sidechainRegistry);

    const deactivateIdentityCall = await createSignedTrustedCallDeactivateIdentity(
        parachainApi,
        mrEnclave,
        parachainApi.createType('Index', nonce),
        primary,
        subject,
        secondaryIdentity,
        requestIdentifier
    );
    const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, parachainApi);
    await runner('deactivateIdentity', async () => {
        await sendRequestFromTrustedCall(
            teeWorker,
            parachainApi,
            mrEnclave,
            teeShieldingKey,
            deactivateIdentityCall,
            log
        );

        const events = await eventsPromise;
        return events
            .map(({ event }) => event)
            .some((event) => parachainApi.events.identityManagement.IdentityDeactivated.is(event));
    });
}

export async function activateIdentity(
    runner: Runner<string, boolean>,
    primary: Wallet,
    secondary: Wallet,
    sidechainRegistry: SidechainTypeRegistry,
    teeWorker: WebSocketAsPromised,
    parachainApi: ParachainApiPromise,
    mrEnclave: string,
    teeShieldingKey: crypto.KeyObject,
    nonce: Index,
    subject: LitentryPrimitivesIdentity,
    log: WritableStream<string>
): Promise<void> {
    const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
    const secondaryIdentity = await buildIdentityFromWallet(secondary, sidechainRegistry);

    const activateIdentityCall = await createSignedTrustedCallActivateIdentity(
        parachainApi,
        mrEnclave,
        parachainApi.createType('Index', nonce),
        primary,
        subject,
        secondaryIdentity,
        requestIdentifier
    );
    const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, parachainApi);
    await runner('activateIdentity', async () => {
        await sendRequestFromTrustedCall(
            teeWorker,
            parachainApi,
            mrEnclave,
            teeShieldingKey,
            activateIdentityCall,
            log
        );

        const events = await eventsPromise;
        return events
            .map(({ event }) => event)
            .some((event) => parachainApi.events.identityManagement.IdentityActivated.is(event));
    });
}
