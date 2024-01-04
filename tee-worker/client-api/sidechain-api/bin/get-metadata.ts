import WebSocketAsPromised from "websocket-as-promised";
import Options from "websocket-as-promised/types/options";
import WebSocket from "ws";

async function initConnection() {
    const wsp = new WebSocketAsPromised("ws://141.95.2.55:2000/", <Options>(<unknown>{
        createWebSocket: (url: any) => new WebSocket(url),
        extractMessageData: (event: any) => event,
        packMessage: (data: any) => JSON.stringify(data),
        unpackMessage: (data: string | ArrayBuffer | Blob) => JSON.parse(data.toString()),
        attachRequestId: (data: any, requestId: string | number) => Object.assign({ id: requestId }, data),
        extractRequestId: (data: any) => data && data.id, // read requestId from message `id` field
    }));
    await wsp.open();
    return wsp;
}

async function getSgxMetadataRaw(wsp: WebSocketAsPromised) {
    const request = {
        jsonrpc: "2.0",
        method: "state_getSgxMetadataRaw",
        params: Uint8Array.from([]),
        id: 1,
    };
    const response = await wsp.sendRequest(request, { requestId: request.id, timeout: 6000 });
    return response;
}

async function main() {
    const wsp = await initConnection();
    const metadata = await getSgxMetadataRaw(wsp);
    console.log(metadata);
}

main();
