import WebSocket from 'ws';
import { assert } from 'chai';
import dotenv from 'dotenv';

describe('test-bitcoin', () => {
    // eslint-disable-next-line @typescript-eslint/no-var-requires, no-undef
    dotenv.config({ path: `.env.${process.env.NODE_ENV || 'local'}` });

    let workerEndpoint = process.env.WORKER_ENDPOINT!;
    const workerEndpointParts = workerEndpoint.split(':');
    let url = workerEndpointParts[0] + ':' + workerEndpointParts[1];
    let port = parseInt(workerEndpointParts[2]);

    it('should pass on all workers', async () => {
        const worker1 = new WebSocket(url + ':' + port, {
            perMessageDeflate: false,
        });
        const worker2 = new WebSocket(url + ':' + (port + 10), {
            perMessageDeflate: false,
        });
        const worker3 = new WebSocket(url + ':' + (port + 20), {
            perMessageDeflate: false,
        });

        let worker1Resolve: any;
        let worker1Result = new Promise<boolean>((resolve, reject) => {
            worker1Resolve = resolve;
        });
        let worker2Resolve: any;
        let worker2Result = new Promise<boolean>((resolve, reject) => {
            worker2Resolve = resolve;
        });
        let worker3Resolve: any;
        let worker3Result = new Promise<boolean>((resolve, reject) => {
            worker3Resolve = resolve;
        });

        worker1.on('message', (message: any) => {
            worker1Resolve(message == '{"jsonrpc":"2.0","result":"0x04010000","id":1}');
        });
        worker2.on('message', (message: any) => {
            worker2Resolve(message == '{"jsonrpc":"2.0","result":"0x04010000","id":1}');
        });
        worker3.on('message', (message: any) => {
            worker3Resolve(message == '{"jsonrpc":"2.0","result":"0x04010000","id":1}');
        });

        worker1.on('open', () => {
            worker1.send('{"id":1,"jsonrpc":"2.0","method":"bitacross_checkSignBitcoin","params":[]}');
        });
        worker2.on('open', () => {
            worker2.send('{"id":1,"jsonrpc":"2.0","method":"bitacross_checkSignBitcoin","params":[]}');
        });
        worker3.on('open', () => {
            worker3.send('{"id":1,"jsonrpc":"2.0","method":"bitacross_checkSignBitcoin","params":[]}');
        });

        await Promise.all([worker1Result, worker2Result, worker3Result]).then(([w1, w2, w3]) => {
            assert(w1 && w2 && w3);
        });
    });
});
