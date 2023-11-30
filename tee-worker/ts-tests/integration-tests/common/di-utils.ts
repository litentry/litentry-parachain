import { ApiPromise } from '@polkadot/api';
import { u8aToHex, hexToU8a, compactAddLength, bufferToU8a, u8aConcat, stringToU8a } from '@polkadot/util';
import { Codec } from '@polkadot/types/types';
import { TypeRegistry } from '@polkadot/types';
import { Bytes } from '@polkadot/types-codec';
import { IntegrationTestContext, JsonRpcRequest } from './common-types';
import { WorkerRpcReturnValue, TrustedCallSigned, Getter } from 'parachain-api';
import { encryptWithTeeShieldingKey, Signer, encryptWithAes } from './utils';
import { aesKey, decodeRpcBytesAsString, keyNonce } from './call';
import { createPublicKey, KeyObject } from 'crypto';
import WebSocketAsPromised from 'websocket-as-promised';
import { u32, Option, u8, Vector } from 'scale-ts';
import { Index } from '@polkadot/types/interfaces';
import { blake2AsHex } from '@polkadot/util-crypto';
import type { LitentryPrimitivesIdentity, PalletIdentityManagementTeeIdentityContext } from 'sidechain-api';
import { createJsonRpcRequest, nextRequestId } from './helpers';

// Send the request to worker ws
// we should perform different actions based on the returned status:
//
// `Submitted`:
// the request is submitted to the top pool, we should start to subscribe to parachain headers to wait for async parachain event
//
// `InSidechainBlock`
// the request is included in a sidechain block: the state mutation of sidechain is done, the promise is resolved
// the corresponding parachain event should be emitted **around** that, it's not guaranteed if it's before or after this status
// due to block inclusion delays from the parachain
//
async function sendRequest(
    wsClient: WebSocketAsPromised,
    request: JsonRpcRequest,
    api: ApiPromise
): Promise<WorkerRpcReturnValue> {
    const p = new Promise<WorkerRpcReturnValue>((resolve) =>
        wsClient.onMessage.addListener((data) => {
            const parsed = JSON.parse(data);
            if (parsed.id === request.id) {
                const result = parsed.result;
                const res: WorkerRpcReturnValue = api.createType('WorkerRpcReturnValue', result) as any;

                console.log('Got response: ' + JSON.stringify(res.toHuman()));

                if (res.status.isError) {
                    console.log('Rpc response error: ' + decodeRpcBytesAsString(res.value));
                }

                if (res.status.isTrustedOperationStatus && res.status.asTrustedOperationStatus[0].isInvalid) {
                    console.log('Rpc trusted operation execution failed, hash: ', res.value.toHex());
                }

                // resolve it once `do_watch` is false, meaning it's the final response
                if (res.do_watch.isFalse) {
                    // TODO: maybe only remove this listener
                    wsClient.onMessage.removeAllListeners();
                    resolve(res);
                } else {
                    // `do_watch` is true means: hold on - there's still something coming
                    console.log('do_watch is true, continue watching ...');
                }
            }
        })
    );

    wsClient.sendRequest(request);
    return p;
}

// TrustedCalls are defined in:
// https://github.com/litentry/litentry-parachain/blob/d4be11716fdb46021194bbe9fe791b15249a369e/tee-worker/app-libs/stf/src/trusted_call.rs#L61
//
// About the signature, it's signed with `KeyringPair` here.
// In reality we need to get the user's signature on the `payload`.
export const createSignedTrustedCall = async (
    parachainApi: ApiPromise,
    trustedCall: [string, string],
    signer: Signer,
    // hex-encoded mrenclave, retrieveable from parachain enclave registry
    // TODO: do we have a RPC getter from the enclave?
    mrenclave: string,
    nonce: Codec,
    params: any,
    withWrappedBytes = false
): Promise<TrustedCallSigned> => {
    const [variant, argType] = trustedCall;
    const call = parachainApi.createType('TrustedCall', {
        [variant]: parachainApi.createType(argType, params),
    });
    let payload = Uint8Array.from([
        ...call.toU8a(),
        ...nonce.toU8a(),
        ...hexToU8a(mrenclave),
        ...hexToU8a(mrenclave), // should be shard, but it's the same as MRENCLAVE in our case
    ]);
    if (withWrappedBytes) {
        payload = u8aConcat(stringToU8a('<Bytes>'), payload, stringToU8a('</Bytes>'));
    }
    const signature = parachainApi.createType('LitentryMultiSignature', {
        [signer.type()]: u8aToHex(await signer.sign(payload)),
    });
    return parachainApi.createType('TrustedCallSigned', {
        call: call,
        index: nonce,
        signature: signature,
    }) as unknown as TrustedCallSigned;
};

export const createSignedTrustedGetter = async (
    parachainApi: ApiPromise,
    trustedGetter: [string, string],
    signer: Signer,
    params: any
) => {
    const [variant, argType] = trustedGetter;
    const getter = parachainApi.createType('TrustedGetter', {
        [variant]: parachainApi.createType(argType, params),
    });
    const payload = getter.toU8a();
    const signature = parachainApi.createType('LitentryMultiSignature', {
        [signer.type()]: u8aToHex(await signer.sign(payload)),
    });
    return parachainApi.createType('TrustedGetterSigned', {
        getter: getter,
        signature: signature,
    });
};

export const createPublicGetter = (parachainApi: ApiPromise, publicGetter: [string, string], params: any) => {
    const [variant, argType] = publicGetter;
    const getter = parachainApi.createType('PublicGetter', {
        [variant]: parachainApi.createType(argType, params),
    });

    return getter;
};

export async function createSignedTrustedCallLinkIdentity(
    parachainApi: ApiPromise,
    mrenclave: string,
    nonce: Codec,
    signer: Signer,
    subject: LitentryPrimitivesIdentity,
    identity: string,
    validationData: string,
    web3networks: string,
    aesKey: string,
    hash: string
) {
    return createSignedTrustedCall(
        parachainApi,
        [
            'link_identity',
            '(LitentryIdentity, LitentryIdentity, LitentryIdentity, LitentryValidationData, Vec<Web3Network>, Option<RequestAesKey>, H256)',
        ],
        signer,
        mrenclave,
        nonce,
        [subject.toHuman(), subject.toHuman(), identity, validationData, web3networks, aesKey, hash]
    );
}

export async function createSignedTrustedCallSetIdentityNetworks(
    parachainApi: ApiPromise,
    mrenclave: string,
    nonce: Codec,
    signer: Signer,
    subject: LitentryPrimitivesIdentity,
    identity: string,
    web3networks: string,
    hash: string
) {
    return createSignedTrustedCall(
        parachainApi,
        ['set_identity_networks', '(LitentryIdentity, LitentryIdentity, LitentryIdentity, Vec<Web3Network>, H256)'],
        signer,
        mrenclave,
        nonce,
        [subject.toHuman(), subject.toHuman(), identity, web3networks, hash]
    );
}

export async function createSignedTrustedCallRequestVc(
    parachainApi: ApiPromise,
    mrenclave: string,
    nonce: Codec,
    signer: Signer,
    subject: LitentryPrimitivesIdentity,
    assertion: string,
    aesKey: string,
    hash: string
) {
    return await createSignedTrustedCall(
        parachainApi,
        ['request_vc', '(LitentryIdentity, LitentryIdentity, Assertion, Option<RequestAesKey>, H256)'],
        signer,
        mrenclave,
        nonce,
        [subject.toHuman(), subject.toHuman(), assertion, aesKey, hash]
    );
}
export async function createSignedTrustedCallDeactivateIdentity(
    parachainApi: ApiPromise,
    mrenclave: string,
    nonce: Codec,
    signer: Signer,
    subject: LitentryPrimitivesIdentity,
    identity: string,
    hash: string
) {
    return createSignedTrustedCall(
        parachainApi,
        ['deactivate_identity', '(LitentryIdentity, LitentryIdentity, LitentryIdentity, H256)'],
        signer,
        mrenclave,
        nonce,
        [subject.toHuman(), subject.toHuman(), identity, hash]
    );
}
export async function createSignedTrustedCallActivateIdentity(
    parachainApi: ApiPromise,
    mrenclave: string,
    nonce: Codec,
    signer: Signer,
    subject: LitentryPrimitivesIdentity,
    identity: string,
    hash: string
) {
    return createSignedTrustedCall(
        parachainApi,
        ['activate_identity', '(LitentryIdentity, LitentryIdentity, LitentryIdentity, H256)'],
        signer,
        mrenclave,
        nonce,
        [subject.toHuman(), subject.toHuman(), identity, hash]
    );
}

export async function createSignedTrustedGetterIdGraph(
    parachainApi: ApiPromise,
    signer: Signer,
    subject: LitentryPrimitivesIdentity
): Promise<Getter> {
    const getterSigned = await createSignedTrustedGetter(
        parachainApi,
        ['id_graph', '(LitentryIdentity)'],
        signer,
        subject.toHuman()
    );
    return parachainApi.createType('Getter', { trusted: getterSigned }) as unknown as Getter; // @fixme 1878;
}

export const getSidechainNonce = async (
    context: IntegrationTestContext,
    teeShieldingKey: KeyObject,
    subject: LitentryPrimitivesIdentity
): Promise<Index> => {
    const getterPublic = createPublicGetter(context.api, ['nonce', '(LitentryIdentity)'], subject.toHuman());
    const getter = context.api.createType('Getter', { public: getterPublic }) as unknown as Getter; // @fixme 1878
    const nonce = await sendRequestFromGetter(context, teeShieldingKey, getter);
    const nonceValue = decodeNonce(nonce.value.toHex());
    return context.api.createType('Index', nonceValue) as Index;
};

export const sendRequestFromTrustedCall = async (
    context: IntegrationTestContext,
    teeShieldingKey: KeyObject,
    call: TrustedCallSigned
) => {
    // construct trusted operation
    const trustedOperation = context.api.createType('TrustedOperation', { direct_call: call });
    console.log('top: ', trustedOperation.toJSON());
    console.log('top hash', blake2AsHex(trustedOperation.toU8a()));
    // create the request parameter
    const requestParam = await createAesRequest(
        context.api,
        context.mrEnclave,
        teeShieldingKey,
        hexToU8a(aesKey),
        trustedOperation.toU8a()
    );
    const request = createJsonRpcRequest(
        'author_submitAndWatchAesRequest',
        [u8aToHex(requestParam)],
        nextRequestId(context)
    );
    return sendRequest(context.tee, request, context.api);
};

export const sendRequestFromGetter = async (
    context: IntegrationTestContext,
    teeShieldingKey: KeyObject,
    getter: Getter
): Promise<WorkerRpcReturnValue> => {
    // important: we don't create the `TrustedOperation` type here, but use `Getter` type directly
    //            this is what `state_executeGetter` expects in rust
    const requestParam = await createRsaRequest(context.api, context.mrEnclave, teeShieldingKey, true, getter.toU8a());
    const request = createJsonRpcRequest('state_executeGetter', [u8aToHex(requestParam)], nextRequestId(context));
    return sendRequest(context.tee, request, context.api);
};

// get TEE's shielding key directly via RPC
export const getTeeShieldingKey = async (context: IntegrationTestContext) => {
    const request = createJsonRpcRequest('author_getShieldingKey', Uint8Array.from([]), nextRequestId(context));
    const res = await sendRequest(context.tee, request, context.api);
    const k = JSON.parse(decodeRpcBytesAsString(res.value)) as {
        n: Uint8Array;
        e: Uint8Array;
    };

    return createPublicKey({
        key: {
            alg: 'RSA-OAEP-256',
            kty: 'RSA',
            use: 'enc',
            n: Buffer.from(k.n.reverse()).toString('base64url'),
            e: Buffer.from(k.e.reverse()).toString('base64url'),
        },
        format: 'jwk',
    });
};

// given an encoded trusted operation, construct a rsa request bytes that are sent in RPC request parameters
export const createRsaRequest = async (
    parachainApi: ApiPromise,
    mrenclave: string,
    teeShieldingKey: KeyObject,
    isGetter: boolean,
    top: Uint8Array
) => {
    let payload;
    if (isGetter) {
        payload = compactAddLength(top);
    } else {
        payload = compactAddLength(bufferToU8a(encryptWithTeeShieldingKey(teeShieldingKey, top)));
    }

    return parachainApi.createType('RsaRequest', { shard: hexToU8a(mrenclave), payload }).toU8a();
};

// given an encoded trusted operation, construct an aes request bytes that are sent in RPC request parameters
export const createAesRequest = async (
    parachainApi: ApiPromise,
    mrenclave: string,
    teeShieldingKey: KeyObject,
    aesKey: Uint8Array,
    top: Uint8Array
) => {
    const encryptedAesKey = compactAddLength(bufferToU8a(encryptWithTeeShieldingKey(teeShieldingKey, aesKey)));
    return parachainApi
        .createType('AesRequest', {
            shard: hexToU8a(mrenclave),
            key: encryptedAesKey,
            payload: parachainApi
                .createType('AesOutput', {
                    ciphertext: compactAddLength(
                        hexToU8a(encryptWithAes(u8aToHex(aesKey), hexToU8a(keyNonce), Buffer.from(top)))
                    ),
                    add: hexToU8a('0x'),
                    nonce: hexToU8a(keyNonce),
                })
                .toU8a(),
        })
        .toU8a();
};

export function decodeNonce(nonceInHex: string) {
    const optionalType = Option(Vector(u8));
    const encodedNonce = optionalType.dec(nonceInHex) as number[];
    const nonce = u32.dec(new Uint8Array(encodedNonce));
    return nonce;
}

export function decodeIdGraph(sidechainRegistry: TypeRegistry, value: Bytes) {
    const idgraphBytes = sidechainRegistry.createType('Option<Bytes>', hexToU8a(value.toHex()));
    return sidechainRegistry.createType(
        'Vec<(LitentryPrimitivesIdentity, PalletIdentityManagementTeeIdentityContext)>',
        idgraphBytes.unwrap()
    ) as unknown as [LitentryPrimitivesIdentity, PalletIdentityManagementTeeIdentityContext][];
}

export function getTopHash(parachainApi: ApiPromise, call: TrustedCallSigned) {
    const trustedOperation = parachainApi.createType('TrustedOperation', { direct_call: call });
    return blake2AsHex(trustedOperation.toU8a());
}
