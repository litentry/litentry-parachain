import { ApiPromise } from '@polkadot/api';
import { hexToU8a, compactStripLength, u8aToString } from '@polkadot/util';
import WebSocketAsPromised from 'websocket-as-promised';
import { HexString } from '@polkadot/util/types';
import type { RequestBody } from '../common/type-definitions';
import type { WorkerRpcReturnValue } from '../parachain-interfaces/identity/types';
import { Metadata, TypeRegistry } from '@polkadot/types';
import type { Bytes } from '@polkadot/types-codec';
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
        console.log('Rpc trusted operation execution failed, hash: ', res.value);
    }

    return res;
}

// decode the returned bytes as string
// TODO: is it same as `String::decode` in rust?
// please note we shouldn't use toU8a(), which encodes the Bytes instead of converting
export function decodeRpcBytesAsString(value: Bytes): string {
    return u8aToString(compactStripLength(hexToU8a(value.toHex()))[1]);
}

export async function getSidechainMetadata(
    wsClient: WebSocketAsPromised,
    api: ApiPromise
): Promise<{ metaData: Metadata; sidechainRegistry: TypeRegistry }> {
    let request = { jsonrpc: '2.0', method: 'state_getMetadata', params: [], id: 1 };
    let respJSON = await sendRequest(wsClient, request, api);
    const sidechainRegistry = new TypeRegistry();
    const metaData = new Metadata(sidechainRegistry, respJSON.value);
    sidechainRegistry.setMetadata(metaData);
    return { metaData, sidechainRegistry };
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
