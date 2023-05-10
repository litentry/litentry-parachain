import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { BN, u8aToHex, hexToU8a, compactAddLength, bufferToU8a } from '@polkadot/util';
import { Codec } from '@polkadot/types/types';
import { WorkerRpcReturnValue, WorkerRpcReturnString, PubicKeyJson } from '../../common/type-definitions';
import { encryptWithTeeShieldingKey } from '../../common/utils';
import { createPublicKey, KeyObject } from 'crypto';
import WebSocketAsPromised from 'websocket-as-promised';

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
            const res = api.createType('WorkerRpcReturnValue', result).toJSON() as WorkerRpcReturnValue;
            console.log('response status: ', res.status);
            if (res.status === 'Error') {
                throw new Error('ws response error: ' + res.value);
            }
            // resolve it once `do_watch` is false, meaning it's the final response
            if (!res.do_watch) {
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
    params: Array<any>
) => {
    const [variant, argType] = trustedCall;
    const call = parachain_api.createType('TrustedCall', {
        [variant]: parachain_api.createType(argType, params),
    });
    const payload = Uint8Array.from([
        ...call.toU8a(),
        ...nonce.toU8a(),
        ...hexToU8a(mrenclave),
        ...hexToU8a(mrenclave), // should be shard, but it's the same as MRENCLAVE in our case
    ]);
    const signature = parachain_api.createType('MultiSignature', {
        Sr25519: u8aToHex(account.sign(payload)),
    });
    return parachain_api.createType('TrustedCallSigned', {
        call: call,
        index: nonce,
        signature: signature,
    });
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
    hash: string
) {
    return createSignedTrustedCall(
        parachain_api,
        ['set_user_shielding_key_direct', '(AccountId, UserShieldingKeyType, H256)'],
        who,
        mrenclave,
        nonce,
        [who.address, key, hash]
    );
}

export function createSignedTrustedCallCreateIdentity(
    parachain_api: ApiPromise,
    mrenclave: string,
    nonce: Codec,
    who: KeyringPair,
    identity: string,
    metadata: string,
    bn: string,
    hash: string
) {
    return createSignedTrustedCall(
        parachain_api,
        ['create_identity_direct', '(AccountId, LitentryIdentity, Option<Vec<u8>>, u32, H256)'],
        who,
        mrenclave,
        nonce,
        [who.address, identity, metadata, bn, hash]
    );
}

export const sendRequestFromTrustedCall = async (
    wsp: any,
    parachain_api: ApiPromise,
    mrenclave: string,
    teeShieldingKey: KeyObject,
    call: Codec
) => {
    // construct trusted operation
    const trustedOperation = parachain_api.createType('TrustedOperation', { direct_call: call });
    // create the request parameter
    let requestParam = await createRequest(wsp, parachain_api, mrenclave, teeShieldingKey, trustedOperation.toU8a());
    let request = {
        jsonrpc: '2.0',
        method: 'author_submitAndWatchExtrinsic',
        params: [u8aToHex(requestParam)],
        id: 1,
    };
    await sendRequest(wsp, request, parachain_api);
};

// get TEE's shielding key directly via RPC
export const getTEEShieldingKey = async (wsp: WebSocketAsPromised, parachain_api: ApiPromise) => {
    let request = { jsonrpc: '2.0', method: 'author_getShieldingKey', params: [], id: 1 };
    let resp = await sendRequest(wsp, request, parachain_api);
    const resp_hex = parachain_api.createType('WorkerRpcReturnString', resp.value).toJSON() as WorkerRpcReturnString;
    const k = JSON.parse(Buffer.from(resp_hex.vec.slice(2), 'hex').toString('utf-8')) as PubicKeyJson;

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
    top: Uint8Array
) => {
    let cyphertext = compactAddLength(bufferToU8a(encryptWithTeeShieldingKey(teeShieldingKey, top)));
    return parachain_api.createType('Request', { shard: hexToU8a(mrenclave), cyphertext }).toU8a();
};
