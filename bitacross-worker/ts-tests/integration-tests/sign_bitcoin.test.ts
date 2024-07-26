import WebSocket from 'ws';
import { assert } from 'chai';
import dotenv from 'dotenv';

function getWorkerUrls(env: String): String[] {
    if (env == 'local') {
        let workerEndpoint = process.env.WORKER_ENDPOINT!;
        const workerEndpointParts = workerEndpoint.split(':');
        let url = workerEndpointParts[0] + ':' + workerEndpointParts[1];
        let port = parseInt(workerEndpointParts[2]);
        return [url + ':' + port, url + ':' + (port + 10), url + ':' + (port + 20)];
    } else {
        return ['wss://bitacross-worker-1:2011', 'wss://bitacross-worker-2:2011', 'wss://bitacross-worker-3:2011'];
    }
}

function sleep(time) {
    return new Promise((resolve) => setTimeout(resolve, time));
}

describe('test-bitcoin', async () => {
    // eslint-disable-next-line @typescript-eslint/no-var-requires, no-undef
    dotenv.config({ path: `.env.${process.env.NODE_ENV || 'local'}` });

    const workerUrls = getWorkerUrls(process.env.NODE_ENV as string);
    console.log('Using worker urls: ' + workerUrls);
    console.log('Start: ' + Date.now());
    // it needs to wait for workers to be ready, todo: use is_initialized
    await sleep(60 * 1000);
    console.log('Run: ' + Date.now());

    it('should pass on all workers', async () => {
        const worker1 = new WebSocket(workerUrls[0], {
            perMessageDeflate: false,
            rejectUnauthorized: false,
        });
        const worker2 = new WebSocket(workerUrls[1], {
            perMessageDeflate: false,
            rejectUnauthorized: false,
        });
        const worker3 = new WebSocket(workerUrls[2], {
            perMessageDeflate: false,
            rejectUnauthorized: false,
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
