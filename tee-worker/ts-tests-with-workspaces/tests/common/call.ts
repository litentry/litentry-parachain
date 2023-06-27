import { ApiPromise as ParachainApiPromise } from 'parachain-api';
import { hexToU8a, compactStripLength, u8aToString } from '@polkadot/util';
import WebSocketAsPromised from 'websocket-as-promised';
import { HexString } from '@polkadot/util/types';
import type { RequestBody } from '../common/type-definitions';
import type { WorkerRpcReturnValue } from 'parachain-api';
import { Metadata as SidechainMetadata, TypeRegistry as SidechainTypeRegistry } from 'sidechain-api';
import type { Bytes } from 'parachain-api';

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
    api: ParachainApiPromise
): Promise<WorkerRpcReturnValue> {
    const rawRes = await wsClient.sendRequest(request, { requestId: 1, timeout: 6000 });
    const res: WorkerRpcReturnValue = api.createType('WorkerRpcReturnValue', rawRes.result);
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
    api: ParachainApiPromise
): Promise<{ metaData: SidechainMetadata; sidechainRegistry: SidechainTypeRegistry }> {
    let request = { jsonrpc: '2.0', method: 'state_getMetadata', params: [], id: 1 };
    let respJSON = await sendRequest(wsClient, request, api);
    const sidechainRegistry = new SidechainTypeRegistry();
    const metaData = new SidechainMetadata(sidechainRegistry, respJSON.value);
    sidechainRegistry.setMetadata(metaData);
    return { metaData, sidechainRegistry };
}

export async function getSideChainStorage(
    wsClient: WebSocketAsPromised,
    rpcMethod: string,
    api: ParachainApiPromise,
    mrenclave: HexString,
    storageKey: string
): Promise<WorkerRpcReturnValue> {
    let request = { jsonrpc: '2.0', method: rpcMethod, params: [mrenclave, storageKey], id: 1 };
    return await sendRequest(wsClient, request, api);
}
