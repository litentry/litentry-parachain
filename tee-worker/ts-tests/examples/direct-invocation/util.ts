import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { BN, u8aToHex, hexToU8a, u8aToBuffer, u8aToString, compactAddLength, bufferToU8a } from '@polkadot/util';
import { Codec } from '@polkadot/types/types';
import { assert } from 'chai';
import { WorkerRpcReturnValue, WorkerRpcReturnString, PubicKeyJson } from '../../common/type-definitions';
import { createPublicKey, publicEncrypt } from 'crypto';
import * as jose from 'jose';

const base58 = require('micro-base58');

type DirectExtrinsic = {};

export function toBalance(amountInt: number) {
    return new BN(amountInt).mul(new BN(10).pow(new BN(12)));
}

async function sendRequest(wsClient: any, request: any, api: ApiPromise) {
    const resp = await wsClient.sendRequest(request, { requestId: 1, timeout: 6000 });
    const resp_json = api.createType('WorkerRpcReturnValue', resp.result).toJSON() as WorkerRpcReturnValue;
    const resp_hex = api.createType('WorkerRpcReturnString', resp_json.value).toJSON() as WorkerRpcReturnString;
    return Buffer.from(resp_hex.vec.slice(2), 'hex').toString('utf-8');
}

// TrustedCalls are defined in:
// https://github.com/litentry/litentry-parachain/blob/d4be11716fdb46021194bbe9fe791b15249a369e/tee-worker/app-libs/stf/src/trusted_call.rs#L61
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

export async function sendRequestBalanceTransfer(
    wsp: any,
    parachain_api: ApiPromise,
    account: KeyringPair,
    to: string,
    mrenclave: string,
    amount: BN
) {
    // 1. create signed trusted call
    // TODO: get nonce from worker rpc
    const nonce = parachain_api.createType('Index', '0x01');
    const call = createSignedTrustedCall(
        parachain_api,
        ['balance_transfer', '(AccountId, AccountId, Balance)'],
        account,
        mrenclave,
        nonce,
        [account.address, to, amount]
    );
    // 2. construct trusted operation
    const trustedOperation = parachain_api.createType('TrustedOperation', { direct_call: call });
    // 3. encrypt it with TEE's shielding key
    let pk = await getTeeShieldingKey(wsp, parachain_api);
    const ciphertext = publicEncrypt(pk, trustedOperation.toU8a());
    // 4. send the direct request
    let balanceTransferRequest = u8aToHex(
        parachain_api.createType('Request', { shard: hexToU8a(mrenclave), ciphertext }).toU8a()
    );
    let request = { jsonrpc: '2.0', method: 'author_submitAndWatchExtrinsic', params: [balanceTransferRequest], id: 1 };
    let resp = await sendRequest(wsp, request, parachain_api);
    console.log('response: ', resp);
}

export const getTeeShieldingKey = async (wsp: any, parachain_api: ApiPromise) => {
    let request = { jsonrpc: '2.0', method: 'author_getShieldingKey', params: [], id: 1 };
    let resp = await sendRequest(wsp, request, parachain_api);
    const pubKeyJSON = JSON.parse(resp) as PubicKeyJson;

    console.log('Tee shielding key: ', pubKeyJSON);

    // `node-rsa` won't work, we also need to reverse the bytes
    // see https://github.com/integritee-network/worker/issues/987
    const pk = createPublicKey({
        key: {
            alg: 'RSA-OAEP',
            kty: 'RSA',
            use: 'enc',
            n: jose.base64url.encode(Buffer.from(pubKeyJSON.n).reverse()),
            e: jose.base64url.encode(Buffer.from(pubKeyJSON.e).reverse()),
        },
        format: 'jwk',
    });
    return pk;
};
