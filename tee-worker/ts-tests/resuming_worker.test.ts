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

function genCommands(node_url: string, node_port: string): { worker0: WorkerConfig; worker1: WorkerConfig } {
    return {
        worker0: {
            untrusted_ws_port: 3000,
            commands: {
                first_launch:
                    '--running-mode mock --enable-mock-server --clean-reset --mu-ra-external-address localhost --mu-ra-port 3443' +
                    ' --untrusted-http-port 4545 --ws-external --trusted-external-address wss://localhost' +
                    ' --trusted-worker-port 2000 --untrusted-external-address ws://localhost' +
                    ' --untrusted-worker-port 3000 --node-url ' +
                    node_url +
                    ' --node-port ' +
                    node_port +
                    ' run --skip-ra --dev',

                resume:
                    '--running-mode mock --enable-mock-server --mu-ra-external-address localhost --mu-ra-port 3443' +
                    ' --untrusted-http-port 4545 --ws-external --trusted-external-address wss://localhost' +
                    ' --trusted-worker-port 2000 --untrusted-external-address ws://localhost' +
                    ' --untrusted-worker-port 3000 --node-url ' +
                    node_url +
                    ' --node-port ' +
                    node_port +
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
                    node_url +
                    ' --node-port ' +
                    node_port +
                    ' run --skip-ra --request-state --dev',

                resume:
                    '--running-mode mock --mu-ra-external-address localhost --mu-ra-port 3444' +
                    ' --untrusted-http-port 4546 --ws-external --trusted-external-address wss://localhost' +
                    ' --trusted-worker-port 2001 --untrusted-external-address ws://localhost' +
                    ' --untrusted-worker-port 3001 --node-url ' +
                    node_url +
                    ' --node-port ' +
                    node_port +
                    ' run --skip-ra',
            },
        },
    };
}

async function launchWorker(
    name: string,
    binary_dir: string,
    working_dir: string,
    command: string,
    init_files: boolean
): Promise<{ shard: string; process: ChildProcess }> {
    // const logging = fs.createWriteStream(log, {flags: 'w+'});
    if (init_files) {
        fs.mkdirSync(working_dir, { recursive: true });
        fs.copyFileSync(`${binary_dir}/enclave.signed.so`, `${working_dir}/enclave.signed.so`);
        fs.copyFileSync(`${binary_dir}/integritee-service`, `${working_dir}/integritee-service`);
        fs.closeSync(fs.openSync(`${working_dir}/spid.txt`, 'w'));
        fs.closeSync(fs.openSync(`${working_dir}/key.txt`, 'w'));
        let data = JSON.stringify(
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
        fs.writeFileSync(`${working_dir}/worker-config-mock.json`, data);
    }

    return new Promise<{ shard: string; process: ChildProcess }>(async (resolve, reject) => {
        const job = spawn(`./integritee-service`, [command], {
            cwd: working_dir,
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
    return new Promise<number>(async (resolve, reject) => {
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

    let binary_dir = process.env.BINARY_DIR!;
    let [_, node_url, node_port] = process.env.SUBSTRATE_END_POINT!.split(':');
    let worker0_dir = path.join(__dirname, './tmp/worker0');
    let worker1_dir = path.join(__dirname, './tmp/worker1');
    let commands = genCommands(`ws:${node_url}`, node_port);

    step('One worker', async function () {
        // first launch worker
        let { shard: shard, process: worker0 } = await launchWorker(
            'worker0',
            binary_dir,
            worker0_dir,
            commands.worker0.commands.first_launch,
            true
        );
        let worker0_conn = await initWorkerConnection(`ws://localhost:${commands.worker0.untrusted_ws_port}`);
        const current_block = await waitWorkerProducingBlock(worker0_conn, shard, 4);
        await killWorker(worker0);
        console.log('=========== worker stopped ==================');

        // resume worker
        let { process: r_worker0 } = await launchWorker(
            'worker0',
            binary_dir,
            worker0_dir,
            commands.worker0.commands.resume,
            false
        );
        await worker0_conn.open(); //reopen connection
        const resume_block = await latestBlock(worker0_conn, shard);
        // TODO compare the block hash
        assert.isNotEmpty(resume_block.result, "the latest block can't be empty");
        assert.isTrue(resume_block!.result!.number >= current_block, 'failed to resume worker');
        // await killWorker(r_worker)
        await sleep(1);
    });

    // Continue with the above test case to test
    step('Two workers & resume worker1', async function () {
        // 2 workers were actually launched
        // first launch worker1
        let { shard: shard, process: worker1 } = await launchWorker(
            'worker1',
            binary_dir,
            worker1_dir,
            commands.worker1.commands.first_launch,
            true
        );
        let worker1_conn = await initWorkerConnection(`ws://localhost:${commands.worker1.untrusted_ws_port}`);
        const worker1_current_block = await waitWorkerProducingBlock(worker1_conn, shard, 4);
        await killWorker(worker1);
        console.log('=========== worker1 stopped ==================');
        await sleep(20);

        // resume worker1
        let { process: r_worker1 } = await launchWorker(
            'worker1',
            binary_dir,
            worker1_dir,
            commands.worker1.commands.resume,
            false
        );
        await worker1_conn.open(); //reopen connection
        const resume_block = await latestBlock(worker1_conn, shard);
        assert.isNotEmpty(resume_block.result, "the latest block can't be empty");
        assert.isTrue(resume_block!.result!.number >= worker1_current_block, 'failed to resume worker');
        await sleep(60);
    });
});
