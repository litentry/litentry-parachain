import { ApiPromise } from '@polkadot/api';
import { Metadata, TypeRegistry } from '@polkadot/types';
import WebSocketAsPromised from 'websocket-as-promised';
import { HexString } from '@polkadot/util/types';
import { RequestBody, WorkerRpcReturnValue } from '../common/type-definitions';
import sidechainMetaData from '../litentry-sidechain-metadata.json'
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

export async function getSidechainMetadata(wsClient: WebSocketAsPromised, api: ApiPromise): Promise<{ metaData: Metadata, sidechainRegistry: TypeRegistry }> {
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
    let respJSON = await sendRequest(wsClient, request, api);
    return respJSON;
}
