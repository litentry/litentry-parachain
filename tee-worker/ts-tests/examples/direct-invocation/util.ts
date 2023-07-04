import { ApiPromise, Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { BN, u8aToHex, hexToU8a, compactAddLength, bufferToU8a, u8aConcat, stringToU8a } from '@polkadot/util';
import { Codec } from '@polkadot/types/types';
import { Address, PubicKeyJson } from '../../common/type-definitions';
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
            const result = JSON.parse(data.toString()).result;
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
    parachainApi: ApiPromise,
    trustedCall: [string, string],
    account: KeyringPair,
    // hex-encoded mrenclave, retrieveable from parachain enclave registry
    // TODO: do we have a RPC getter from the enclave?
    mrenclave: string,
    nonce: Codec,
    params: any,
    withWrappedBytes = false
) => {
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
        [account.type]: u8aToHex(account.sign(payload)),
    });
    return parachainApi.createType('TrustedCallSigned', {
        call: call,
        index: nonce,
        signature: signature,
    });
};

export const createSignedTrustedGetter = (
    parachainApi: ApiPromise,
    trustedGetter: [string, string],
    account: KeyringPair,
    params: any
) => {
    const [variant, argType] = trustedGetter;
    const getter = parachainApi.createType('TrustedGetter', {
        [variant]: parachainApi.createType(argType, params),
    });
    const payload = getter.toU8a();
    const signature = parachainApi.createType('LitentryMultiSignature', {
        [account.type]: account.sign(payload),
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

export function createSignedTrustedCallBalanceTransfer(
    parachainApi: ApiPromise,
    mrenclave: string,
    nonce: Codec,
    from: KeyringPair,
    to: string,
    amount: BN
) {
    return createSignedTrustedCall(
        parachainApi,
        ['balance_transfer', '(AccountId, AccountId, Balance)'],
        from,
        mrenclave,
        nonce,
        [from.address, to, amount]
    );
}

// TODO: maybe use HexString?
export function createSignedTrustedCallSetUserShieldingKey(
    parachainApi: ApiPromise,
    mrenclave: string,
    nonce: Codec,
    who: KeyringPair,
    address: Address,
    key: string,
    hash: string,
    withWrappedBytes = false
) {
    return createSignedTrustedCall(
        parachainApi,
        ['set_user_shielding_key', '(LitentryMultiAddress, LitentryMultiAddress, UserShieldingKeyType, H256)'],
        who,
        mrenclave,
        nonce,
        [address, address, key, hash],
        withWrappedBytes
    );
}

export function createSignedTrustedCallLinkIdentity(
    parachainApi: ApiPromise,
    mrenclave: string,
    nonce: Codec,
    who: KeyringPair,
    address: Address,
    identity: string,
    validationData: string,
    keyNonce: string,
    hash: string
) {
    return createSignedTrustedCall(
        parachainApi,
        [
            'link_identity',
            '(LitentryMultiAddress, LitentryMultiAddress, LitentryIdentity, LitentryValidationData, UserShieldingKeyNonceType, H256)',
        ],
        who,
        mrenclave,
        nonce,
        [address, address, identity, validationData, keyNonce, hash]
    );
}

export function createSignedTrustedGetterUserShieldingKey(
    parachainApi: ApiPromise,
    who: KeyringPair,
    address: Address
) {
    const getterSigned = createSignedTrustedGetter(
        parachainApi,
        ['user_shielding_key', '(LitentryMultiAddress)'],
        who,
        address
    );
    return parachainApi.createType('Getter', { trusted: getterSigned });
}

export function createSignedTrustedGetterIdGraph(parachainApi: ApiPromise, who: KeyringPair, address: Address) {
    const getterSigned = createSignedTrustedGetter(parachainApi, ['id_graph', '(LitentryMultiAddress)'], who, address);
    return parachainApi.createType('Getter', { trusted: getterSigned });
}

export const getSidechainNonce = async (
    wsp: any,
    parachainApi: ApiPromise,
    mrenclave: string,
    teeShieldingKey: KeyObject,
    address: Address
) => {
    const getterPublic = createPublicGetter(parachainApi, ['nonce', '(LitentryMultiAddress)'], address);
    const getter = parachainApi.createType('Getter', { public: getterPublic });
    const nonce = await sendRequestFromGetter(wsp, parachainApi, mrenclave, teeShieldingKey, getter);
    const nonceValue = decodeNonce(nonce.value.toHex());
    return parachainApi.createType('Index', nonceValue);
};

export const sendRequestFromTrustedCall = async (
    wsp: any,
    parachainApi: ApiPromise,
    mrenclave: string,
    teeShieldingKey: KeyObject,
    call: Codec
) => {
    // construct trusted operation
    const trustedOperation = parachainApi.createType('TrustedOperation', { direct_call: call });
    console.log('top: ', trustedOperation.toJSON());
    // create the request parameter
    const requestParam = await createRequest(
        wsp,
        parachainApi,
        mrenclave,
        teeShieldingKey,
        false,
        trustedOperation.toU8a()
    );
    const request = {
        jsonrpc: '2.0',
        method: 'author_submitAndWatchExtrinsic',
        params: [u8aToHex(requestParam)],
        id: 1,
    };
    return sendRequest(wsp, request, parachainApi);
};

export const sendRequestFromGetter = async (
    wsp: any,
    parachainApi: ApiPromise,
    mrenclave: string,
    teeShieldingKey: KeyObject,
    getter: Codec
): Promise<WorkerRpcReturnValue> => {
    // important: we don't create the `TrustedOperation` type here, but use `Getter` type directly
    //            this is what `state_executeGetter` expects in rust
    const requestParam = await createRequest(wsp, parachainApi, mrenclave, teeShieldingKey, true, getter.toU8a());
    const request = {
        jsonrpc: '2.0',
        method: 'state_executeGetter',
        params: [u8aToHex(requestParam)],
        id: 1,
    };
    return sendRequest(wsp, request, parachainApi);
};

// get TEE's shielding key directly via RPC
export const getTeeShieldingKey = async (wsp: WebSocketAsPromised, parachainApi: ApiPromise) => {
    const request = { jsonrpc: '2.0', method: 'author_getShieldingKey', params: [], id: 1 };
    const res = await sendRequest(wsp, request, parachainApi);
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
    parachainApi: ApiPromise,
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

    return parachainApi.createType('Request', { shard: hexToU8a(mrenclave), cyphertext }).toU8a();
};

export function decodeNonce(nonceInHex: string) {
    const optionalType = Option(Vector(u8));
    const encodedNonce = optionalType.dec(nonceInHex) as number[];
    const nonce = u32.dec(new Uint8Array(encodedNonce));
    return nonce;
}

export function getKeyPair(accountName: string, keyring: Keyring): KeyringPair {
    switch (keyring.type) {
        case 'ethereum': {
            return keyring.addFromMnemonic('test_account' + accountName, { name: accountName });
        }
        case 'ecdsa':
        case 'ed25519':
        case 'sr25519': {
            return keyring.addFromUri('//' + accountName, { name: accountName });
        }
    }
}
