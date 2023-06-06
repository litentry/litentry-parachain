import { ApiPromise } from '@polkadot/api';
import { Metadata, TypeRegistry } from '@polkadot/types';
import type { Bytes } from '@polkadot/types-codec';
import { hexToU8a, compactStripLength, u8aToString } from '@polkadot/util';
import WebSocketAsPromised from 'websocket-as-promised';
import { HexString } from '@polkadot/util/types';
import { RequestBody } from './type-definitions';
import { WorkerRpcReturnValue } from '../interfaces/identity';

// TODO:
// - better place to put these constants?
// - maybe randomise it in test initialisation
//
// the user shielding key
export const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
// the key nonce to calculate the verification message
// copied from hardcoded mock-server
// MOCK_VERIFICATION_NONCE: UserShieldingKeyNonceType = [1u8; 12];
export const keyNonce = '0x010101010101010101010101';

// send RPC request
export async function sendRequest(
    wsClient: WebSocketAsPromised,
    request: RequestBody,
    api: ApiPromise
): Promise<WorkerRpcReturnValue> {
    const rawRes = await wsClient.sendRequest(request, { requestId: 1, timeout: 6000 });
    const res: WorkerRpcReturnValue = api.createType('WorkerRpcReturnValue', rawRes.result) as any;

    if (res.status.isError) {
        console.log('Rpc response error: ' + decodeRpcBytesAsString(res.value));
    }

    // unfortunately, the res.value only contains the hash of top
    if (res.status.isTrustedOperationStatus && res.status.asTrustedOperationStatus.isInvalid) {
        console.log('Rpc trusted operation execution failed, hash: ', res.value.toHex());
    }

    return res;
}

// decode the returned bytes as string
// TODO: is it same as `String::decode` in rust?
// please note we shouldn't use toU8a(), which encodes the Bytes instead of converting
export function decodeRpcBytesAsString(value: Bytes): string {
    return u8aToString(compactStripLength(hexToU8a(value.toHex()))[1]);
}

export async function getMetadata(wsClient: WebSocketAsPromised, api: ApiPromise): Promise<Metadata> {
    let request = { jsonrpc: '2.0', method: 'state_getMetadata', params: [], id: 1 };
    let res = await sendRequest(wsClient, request, api);
    const registry = new TypeRegistry();
    const metadata = new Metadata(registry, res.value);
    registry.setMetadata(metadata);
    return metadata;
}

export async function getSideChainStorage(
    wsClient: WebSocketAsPromised,
    rpcMethod: string,
    api: ApiPromise,
    mrenclave: HexString,
    storageKey: string
): Promise<WorkerRpcReturnValue> {
    let request = { jsonrpc: '2.0', method: rpcMethod, params: [mrenclave, storageKey], id: 1 };
    return await sendRequest(wsClient, request, api);
}
