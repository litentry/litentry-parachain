import { ApiPromise } from '@polkadot/api';
import { Metadata, TypeRegistry } from '@polkadot/types';
import type { Bytes } from '@polkadot/types-codec';
import { u8aToHex, hexToU8a, compactAddLength, compactStripLength, u8aToString, bufferToU8a } from '@polkadot/util';
import WebSocketAsPromised from 'websocket-as-promised';
import { HexString } from '@polkadot/util/types';
import { RequestBody } from './type-definitions';
import { WorkerRpcReturnValue } from '../interfaces/identity';

// send RPC request
export async function sendRequest(
    wsClient: WebSocketAsPromised,
    request: RequestBody,
    api: ApiPromise
): Promise<WorkerRpcReturnValue> {
    const rawRes = await wsClient.sendRequest(request, { requestId: 1, timeout: 6000 });
    const res: WorkerRpcReturnValue = api.createType('WorkerRpcReturnValue', rawRes.result) as any;

    if (res.status.isError) {
        throw new Error('RPC call error' + decodeRpcBytesAsString(res.value));
    }
    return res;
}

// decode the returned bytes as string
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
