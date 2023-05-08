import { ApiPromise } from '@polkadot/api';
import { Metadata, TypeRegistry } from '@polkadot/types';
import WebSocketAsPromised from 'websocket-as-promised';
import { HexString } from '@polkadot/util/types';
import { RequestBody, WorkerRpcReturnValue } from '../common/type-definitions';
//rpc call
export async function sendRequest(
    wsClient: WebSocketAsPromised,
    request: RequestBody,
    api: ApiPromise
): Promise<WorkerRpcReturnValue> {
    const resp = await wsClient.sendRequest(request, { requestId: 1, timeout: 6000 });

    const resp_json = api.createType('WorkerRpcReturnValue', resp.result).toJSON() as WorkerRpcReturnValue;

    if (resp_json.status === 'Error') {
        throw new Error('RPC call error');
    }
    return resp_json;
}

export async function getMetadata(wsClient: WebSocketAsPromised, api: ApiPromise): Promise<Metadata> {
    let request = { jsonrpc: '2.0', method: 'state_getMetadata', params: [], id: 1 };
    let respJSON = await sendRequest(wsClient, request, api);
    const registry = new TypeRegistry();
    const metadata = new Metadata(registry, respJSON.value);
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
    let respJSON = await sendRequest(wsClient, request, api);
    return respJSON;
}
