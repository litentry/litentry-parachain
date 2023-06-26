import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { BN, u8aToHex, hexToU8a, compactAddLength, bufferToU8a, u8aConcat, stringToU8a } from '@polkadot/util';
import { Codec } from '@polkadot/types/types';
import { PubicKeyJson } from '../../common/type-definitions';
import { WorkerRpcReturnValue } from '../../parachain-interfaces/identity/types';
import { encryptWithTeeShieldingKey } from '../../common/utils';
import { decodeRpcBytesAsString } from '../../common/call';
import { createPublicKey, KeyObject } from 'crypto';
import WebSocketAsPromised from 'websocket-as-promised';
import { u32, Option, u8, Vector } from 'scale-ts';

export function toBalance(amountInt: number) {
    return new BN(amountInt).mul(new BN(10).pow(new BN(12)));
}

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
    request: any,
    api: ApiPromise
): Promise<WorkerRpcReturnValue> {
    const p = new Promise<WorkerRpcReturnValue>((resolve) =>
        wsClient.onMessage.addListener((data) => {
            let result = JSON.parse(data.toString()).result;
            const res: WorkerRpcReturnValue = api.createType('WorkerRpcReturnValue', result) as any;

            if (res.status.isError) {
                console.log('Rpc response error: ' + decodeRpcBytesAsString(res.value));
            }

            // unfortunately, the res.value only contains the hash of top
            if (res.status.isTrustedOperationStatus && res.status.asTrustedOperationStatus.isInvalid) {
                console.log('Rpc trusted operation execution failed, hash: ', res.value.toHex());
            }

            // resolve it once `do_watch` is false, meaning it's the final response
            if (res.do_watch.isFalse) {
                // TODO: maybe only remove this listener
                wsClient.onMessage.removeAllListeners();
                resolve(res);
            } else {
                // TODO: subscribe to parachain headers if the status is `Submitted`
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
export const createSignedTrustedCall = (
    parachain_api: ApiPromise,
    trustedCall: [string, string],
    account: KeyringPair,
    // hex-encoded mrenclave, retrieveable from parachain enclave registry
    // TODO: do we have a RPC getter from the enclave?
    mrenclave: string,
    nonce: Codec,
    params: any,
    withWrappedBytes: boolean = false
) => {
    const [variant, argType] = trustedCall;
    const call = parachain_api.createType('TrustedCall', {
        [variant]: parachain_api.createType(argType, params),
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
    const signature = parachain_api.createType('MultiSignature', {
        Sr25519: u8aToHex(account.sign(payload)),
    });
    return parachain_api.createType('TrustedCallSigned', {
        call: call,
        index: nonce,
        signature: signature,
    });
};

export const createSignedTrustedGetter = (
    parachain_api: ApiPromise,
    trustedGetter: [string, string],
    account: KeyringPair,
    params: any
) => {
    const [variant, argType] = trustedGetter;
    const getter = parachain_api.createType('TrustedGetter', {
        [variant]: parachain_api.createType(argType, params),
    });
    const payload = getter.toU8a();
    const signature = parachain_api.createType('MultiSignature', {
        Sr25519: account.sign(payload),
    });
    return parachain_api.createType('TrustedGetterSigned', {
        getter: getter,
        signature: signature,
    });
};

export const createPublicGetter = (parachain_api: ApiPromise, publicGetter: [string, string], params: any) => {
    const [variant, argType] = publicGetter;
    const getter = parachain_api.createType('PublicGetter', {
        [variant]: parachain_api.createType(argType, params),
    });

    return getter;
};

export function createSignedTrustedCallBalanceTransfer(
    parachain_api: ApiPromise,
    mrenclave: string,
    nonce: Codec,
    from: KeyringPair,
    to: string,
    amount: BN
) {
    return createSignedTrustedCall(
        parachain_api,
        ['balance_transfer', '(AccountId, AccountId, Balance)'],
        from,
        mrenclave,
        nonce,
        [from.address, to, amount]
    );
}

// TODO: maybe use HexString?
export function createSignedTrustedCallSetUserShieldingKey(
    parachain_api: ApiPromise,
    mrenclave: string,
    nonce: Codec,
    who: KeyringPair,
    key: string,
    hash: string,
    withWrappedBytes: boolean = false
) {
    return createSignedTrustedCall(
        parachain_api,
        ['set_user_shielding_key', '(AccountId, AccountId, UserShieldingKeyType, H256)'],
        who,
        mrenclave,
        nonce,
        [who.address, who.address, key, hash],
        withWrappedBytes
    );
}

export function createSignedTrustedCallLinkIdentity(
    parachain_api: ApiPromise,
    mrenclave: string,
    nonce: Codec,
    who: KeyringPair,
    identity: string,
    validation_data: string,
    key_nonce: string,
    hash: string
) {
    return createSignedTrustedCall(
        parachain_api,
        [
            'link_identity',
            '(AccountId, AccountId, LitentryIdentity, LitentryValidationData, UserShieldingKeyNonceType, H256)',
        ],
        who,
        mrenclave,
        nonce,
        [who.address, who.address, identity, validation_data, key_nonce, hash]
    );
}

export function createSignedTrustedGetterUserShieldingKey(parachain_api: ApiPromise, who: KeyringPair) {
    let getterSigned = createSignedTrustedGetter(
        parachain_api,
        ['user_shielding_key', '(AccountId)'],
        who,
        who.address
    );
    return parachain_api.createType('Getter', { trusted: getterSigned });
}

export function createSignedTrustedGetterIDGraph(parachain_api: ApiPromise, who: KeyringPair) {
    let getterSigned = createSignedTrustedGetter(parachain_api, ['id_graph', '(AccountId)'], who, who.address);
    return parachain_api.createType('Getter', { trusted: getterSigned });
}

export const getSidechainNonce = async (
    wsp: any,
    parachain_api: ApiPromise,
    mrenclave: string,
    teeShieldingKey: KeyObject,
    who: string
) => {
    let getterPublic = createPublicGetter(parachain_api, ['nonce', '(AccountId)'], who);
    let getter = parachain_api.createType('Getter', { public: getterPublic });
    const nonce = await sendRequestFromGetter(wsp, parachain_api, mrenclave, teeShieldingKey, getter);
    const NonceValue = decodeNonce(nonce.value.toHex());
    return parachain_api.createType('Index', NonceValue);
};

export const sendRequestFromTrustedCall = async (
    wsp: any,
    parachain_api: ApiPromise,
    mrenclave: string,
    teeShieldingKey: KeyObject,
    call: Codec
) => {
    // construct trusted operation
    const trustedOperation = parachain_api.createType('TrustedOperation', { direct_call: call });
    console.log('top: ', trustedOperation.toJSON());
    // create the request parameter
    let requestParam = await createRequest(
        wsp,
        parachain_api,
        mrenclave,
        teeShieldingKey,
        false,
        trustedOperation.toU8a()
    );
    let request = {
        jsonrpc: '2.0',
        method: 'author_submitAndWatchExtrinsic',
        params: [u8aToHex(requestParam)],
        id: 1,
    };
    return sendRequest(wsp, request, parachain_api);
};

export const sendRequestFromGetter = async (
    wsp: any,
    parachain_api: ApiPromise,
    mrenclave: string,
    teeShieldingKey: KeyObject,
    getter: Codec
): Promise<WorkerRpcReturnValue> => {
    // important: we don't create the `TrustedOperation` type here, but use `Getter` type directly
    //            this is what `state_executeGetter` expects in rust
    let requestParam = await createRequest(wsp, parachain_api, mrenclave, teeShieldingKey, true, getter.toU8a());
    let request = {
        jsonrpc: '2.0',
        method: 'state_executeGetter',
        params: [u8aToHex(requestParam)],
        id: 1,
    };
    return sendRequest(wsp, request, parachain_api);
};

// get TEE's shielding key directly via RPC
export const getTEEShieldingKey = async (wsp: WebSocketAsPromised, parachain_api: ApiPromise) => {
    let request = { jsonrpc: '2.0', method: 'author_getShieldingKey', params: [], id: 1 };
    let res = await sendRequest(wsp, request, parachain_api);
    const k = JSON.parse(decodeRpcBytesAsString(res.value)) as PubicKeyJson;

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

// given an encoded trusted operation, construct a request bytes that are sent in RPC request parameters
export const createRequest = async (
    wsp: WebSocketAsPromised,
    parachain_api: ApiPromise,
    mrenclave: string,
    teeShieldingKey: KeyObject,
    isGetter: boolean,
    top: Uint8Array
) => {
    let cyphertext;
    if (isGetter) {
        cyphertext = compactAddLength(top);
    } else {
        cyphertext = compactAddLength(bufferToU8a(encryptWithTeeShieldingKey(teeShieldingKey, top)));
    }

    return parachain_api.createType('Request', { shard: hexToU8a(mrenclave), cyphertext }).toU8a();
};

export function decodeNonce(nonceInHex: string) {
    const optionalType = Option(Vector(u8));
    const encodedNonce = optionalType.dec(nonceInHex) as number[];
    const nonce = u32.dec(new Uint8Array(encodedNonce));
    return nonce;
}
