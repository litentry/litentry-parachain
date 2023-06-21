import { ChildProcess, spawn } from 'child_process';
import fs from 'fs';
import { initWorkerConnection, sleep } from './common/utils';
import * as path from 'path';
import * as process from 'process';
import { describe } from 'mocha';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import WebSocketAsPromised from 'websocket-as-promised';

export type WorkerConfig = {
    untrusted_ws_port: number;
    commands: {
        first_launch: string;
        resume: string;
    };
};

function genCommands(nodeUrl: string, nodePort: string): { worker0: WorkerConfig; worker1: WorkerConfig } {
    return {
        worker0: {
            untrusted_ws_port: 3000,
            commands: {
                first_launch:
                    '--running-mode mock --enable-mock-server --clean-reset --mu-ra-external-address localhost --mu-ra-port 3443' +
                    ' --untrusted-http-port 4545 --ws-external --trusted-external-address wss://localhost' +
                    ' --trusted-worker-port 2000 --untrusted-external-address ws://localhost' +
                    ' --untrusted-worker-port 3000 --node-url ' +
                    nodeUrl +
                    ' --node-port ' +
                    nodePort +
                    ' run --skip-ra --dev',

                resume:
                    '--running-mode mock --enable-mock-server --mu-ra-external-address localhost --mu-ra-port 3443' +
                    ' --untrusted-http-port 4545 --ws-external --trusted-external-address wss://localhost' +
                    ' --trusted-worker-port 2000 --untrusted-external-address ws://localhost' +
                    ' --untrusted-worker-port 3000 --node-url ' +
                    nodeUrl +
                    ' --node-port ' +
                    nodePort +
                    ' run --skip-ra',
            },
        },
        worker1: {
            untrusted_ws_port: 3001,
            commands: {
                first_launch:
                    '--running-mode mock --clean-reset --mu-ra-external-address localhost --mu-ra-port 3444' +
                    ' --untrusted-http-port 4546 --ws-external --trusted-external-address wss://localhost' +
                    ' --trusted-worker-port 2001 --untrusted-external-address ws://localhost' +
                    ' --untrusted-worker-port 3001 --node-url ' +
                    nodeUrl +
                    ' --node-port ' +
                    nodePort +
                    ' run --skip-ra --request-state --dev',

                resume:
                    '--running-mode mock --mu-ra-external-address localhost --mu-ra-port 3444' +
                    ' --untrusted-http-port 4546 --ws-external --trusted-external-address wss://localhost' +
                    ' --trusted-worker-port 2001 --untrusted-external-address ws://localhost' +
                    ' --untrusted-worker-port 3001 --node-url ' +
                    nodeUrl +
                    ' --node-port ' +
                    nodePort +
                    ' run --skip-ra',
            },
        },
    };
}

async function launchWorker(
    name: string,
    binaryDir: string,
    workingDir: string,
    command: string,
    initFiles: boolean
): Promise<{ shard: string; process: ChildProcess }> {
    // const logging = fs.createWriteStream(log, {flags: 'w+'});
    if (initFiles) {
        fs.mkdirSync(workingDir, { recursive: true });
        fs.copyFileSync(`${binaryDir}/enclave.signed.so`, `${workingDir}/enclave.signed.so`);
        fs.copyFileSync(`${binaryDir}/integritee-service`, `${workingDir}/integritee-service`);
        fs.closeSync(fs.openSync(`${workingDir}/spid.txt`, 'w'));
        fs.closeSync(fs.openSync(`${workingDir}/key.txt`, 'w'));
        const data = JSON.stringify(
            {
                twitter_official_url: 'http://localhost:19527',
                twitter_litentry_url: 'http://localhost:19527',
                twitter_auth_token: '',
                discord_official_url: 'http://localhost:19527',
                discord_litentry_url: 'http://localhost:19527',
                discord_auth_token: '',
                graphql_url: 'http://localhost:19527',
                graphql_auth_key: '',
            },
            null,
            4
        );
        fs.writeFileSync(`${workingDir}/worker-config-mock.json`, data);
    }

    return new Promise<{ shard: string; process: ChildProcess }>((resolve) => {
        const job = spawn(`./integritee-service`, [command], {
            cwd: workingDir,
            shell: '/bin/sh',
            env: {
                RUST_LOG: 'warn,sp_io::storage=error,substrate_api_client=warn',
                ...process.env,
            },
            detached: true,
        });
        job.stdout.setEncoding('utf8');
        let shard = '';
        job.stdout.on('data', (data: string) => {
            if (data.includes('Successfully initialized shard')) {
                const regex = /^Successfully initialized shard (0x[\w\d]{64}).*/g;
                const groups = regex.exec(data);
                if (groups) {
                    shard = groups[1];
                }
            }
            if (data.includes('Untrusted RPC server is spawned on')) {
                resolve({ shard, process: job });
            }
            console.log(name, data);
        });
        job.stderr.setEncoding('utf8');
        job.stderr.on('data', (data: string) => {
            console.log(name, data);
        });
        job.on('close', (code) => {
            console.log(`${name} close: ${code}`);
        });
    });
}

async function killWorker(worker: ChildProcess) {
    // https://azimi.me/2014/12/31/kill-child_process-node-js.html
    if (worker.pid) {
        process.kill(-worker.pid, 9);
        await sleep(2);
    }
}

async function latestBlock(
    connection: WebSocketAsPromised,
    shard: string
): Promise<{ result: undefined | { number: number; hash: string } }> {
    return await connection.sendRequest(
        {
            jsonrpc: '2.0',
            method: 'sidechain_latestBlock',
            params: shard,
            id: 1,
        },
        { requestId: 1, timeout: 6000 }
    );
}

async function waitWorkerProducingBlock(
    connection: WebSocketAsPromised,
    shard: string,
    atLeast: number
): Promise<number> {
    // eslint-disable-next-line no-async-promise-executor
    return new Promise<number>(async (resolve) => {
        let block_number = 0;
        let start_block_number = 0;
        do {
            const resp = await latestBlock(connection, shard);
            if (resp.result) {
                block_number = resp.result.number;
                if (start_block_number == 0) {
                    start_block_number = block_number;
                }

                console.log(`${connection.ws.url} current block: ${block_number}`);
            }
            await sleep(2);
            // console.log(block_number >= (start_block_number + atLeast))
        } while (block_number < start_block_number + atLeast);
        resolve(block_number);
    });
}

describe('Resume worker', function () {
    this.timeout(6000000);

    const binaryDir = process.env.BINARYDIR!;
    const [, nodeUrl, nodePort] = process.env.SUBSTRATE_END_POINT!.split(':');
    const worker0Dir = path.join(__dirname, './tmp/worker0');
    const worker1Dir = path.join(__dirname, './tmp/worker1');
    const commands = genCommands(`ws:${nodeUrl}`, nodePort);

    step('One worker', async function () {
        // first launch worker
        const { shard: shard, process: worker0 } = await launchWorker(
            'worker0',
            binaryDir,
            worker0Dir,
            commands.worker0.commands.first_launch,
            true
        );
        const worker0Conn = await initWorkerConnection(`ws://localhost:${commands.worker0.untrusted_ws_port}`);
        const currentBlock = await waitWorkerProducingBlock(worker0Conn, shard, 4);
        await killWorker(worker0);
        console.log('=========== worker stopped ==================');

        // resume worker
        await launchWorker('worker0', binaryDir, worker0Dir, commands.worker0.commands.resume, false);
        await worker0Conn.open(); //reopen connection
        const resumeBlock = await latestBlock(worker0Conn, shard);
        // TODO compare the block hash
        assert.isNotEmpty(resumeBlock.result, "the latest block can't be empty");
        assert.isTrue(resumeBlock!.result!.number >= currentBlock, 'failed to resume worker');
        // await killWorker(r_worker)
        await sleep(1);
    });

    // Continue with the above test case to test
    step('Two workers & resume worker1', async function () {
        // 2 workers were actually launched
        // first launch worker1
        const { shard: shard, process: worker1 } = await launchWorker(
            'worker1',
            binaryDir,
            worker1Dir,
            commands.worker1.commands.first_launch,
            true
        );
        const worker1Conn = await initWorkerConnection(`ws://localhost:${commands.worker1.untrusted_ws_port}`);
        const worker1CurrentBlock = await waitWorkerProducingBlock(worker1Conn, shard, 4);
        await killWorker(worker1);
        console.log('=========== worker1 stopped ==================');
        await sleep(20);

        // resume worker1
        await launchWorker('worker1', binaryDir, worker1Dir, commands.worker1.commands.resume, false);
        await worker1Conn.open(); //reopen connection
        const resumeBlock = await latestBlock(worker1Conn, shard);
        assert.isNotEmpty(resumeBlock.result, "the latest block can't be empty");
        assert.isTrue(resumeBlock!.result!.number >= worker1CurrentBlock, 'failed to resume worker');
        await sleep(60);
    });
});
