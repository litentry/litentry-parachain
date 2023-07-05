import { ChildProcess, ChildProcessWithoutNullStreams, spawn } from 'child_process';
import * as readline from 'readline';
import fs from 'fs';
import * as path from 'path';
import * as process from 'process';
import { describe } from 'mocha';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import WebSocketAsPromised from 'websocket-as-promised';
import { initWorkerConnection, sleep } from './common/utils';

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
): Promise<{ shard: `0x${string}`; process: ChildProcess }> {
    if (initFiles) {
        fs.mkdirSync(workingDir, { recursive: true });
        fs.copyFileSync(path.join(binaryDir, 'enclave.signed.so'), path.join(workingDir, 'enclave.signed.so'));
        fs.copyFileSync(path.join(binaryDir, 'integritee-service'), path.join(workingDir, 'integritee-service'));
        fs.closeSync(fs.openSync(path.join(workingDir, 'spid.txt'), 'w'));
        fs.closeSync(fs.openSync(path.join(workingDir, 'key.txt'), 'w'));
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

    const job = await new Promise<ChildProcessWithoutNullStreams>((resolve, reject) => {
        const childProcess = spawn(`./integritee-service`, [command], {
            cwd: workingDir,
            shell: '/bin/sh',
            env: {
                RUST_LOG: 'warn,sp_io::storage=error,substrate_api_client=warn',
                ...process.env,
            },
            detached: true,
        });

        if (childProcess.pid !== undefined) {
            resolve(childProcess);
            return;
        }

        childProcess.on('error', (error) => reject(error));
    });

    job.stderr.setEncoding('utf8');
    const errorStream = readline.createInterface(job.stderr);
    errorStream.on('line', (line: string) => {
        console.warn(name, line);
    });

    job.stdout.setEncoding('utf8');
    const outputStream = readline.createInterface(job.stdout);

    job.on('close', (code) => {
        console.log(`${name} close: ${code}`);
    });

    return await new Promise<{ shard: `0x${string}`; process: ChildProcess }>((resolve, reject) => {
        let shard: `0x${string}` | undefined = undefined;

        outputStream.on('line', (line: string) => {
            console.log(name, line);

            const match = line.match(/^Successfully initialized shard (?<shard>0x[\w\d]{64}).*/);
            if (match !== null) {
                /**
                 * Assertions needed because regex contents aren't reflected in function typing;
                 * see e.g. https://github.com/microsoft/TypeScript/issues/32098.
                 *
                 * If the regexp match succeeds, the `groups` property is guaranteed to be present,
                 * as well as the corresponding named capturing groups. See
                 * https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/String/match
                 */
                shard = match.groups!.shard as `0x${string}`;
                return;
            }

            if (line.includes('Untrusted RPC server is spawned on')) {
                if (shard === undefined) {
                    reject(new Error('RPC server spawned before shard initialization'));
                    return;
                }
                resolve({ shard, process: job });
            }
        });
    });
}

function killWorker(worker: ChildProcess): Promise<void> {
    const pid = worker.pid;

    if (pid == undefined) {
        return Promise.reject(new Error('Attempted to kill an unspawned worker?'));
    }

    return new Promise((resolve, reject) => {
        /**
         * Kill each process in the worker's group;
         * see https://www.man7.org/linux/man-pages/man2/kill.2.html
         */
        process.kill(-pid, 'SIGKILL');
        worker.on('exit', () => resolve());
        worker.on('error', (error) => reject(error));
    });
}

async function latestBlock(
    connection: WebSocketAsPromised,
    shard: `0x${string}`
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
    shard: `0x${string}`,
    atLeast: number
): Promise<number> {
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
    } while (block_number < start_block_number + atLeast);
    return block_number;
}

function getConfiguration(): { binaryDir: string; nodeUrl: string; nodePort: string } {
    const binaryDir = process.env.BINARY_DIR;
    if (binaryDir === undefined) {
        throw new Error('Environment variable BINARY_DIR not defined');
    }

    const [, nodeUrl, nodePort] = process.env.SUBSTRATE_END_POINT!.split(':');
    if (nodeUrl === undefined || nodePort === undefined) {
        throw new Error('Environment variable SUBSTRATE_END_POINT undefined or malformed');
    }

    return { binaryDir, nodeUrl, nodePort };
}

describe('Resume worker', function () {
    this.timeout(6000000);

    const { binaryDir, nodeUrl, nodePort } = getConfiguration();
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
        await sleep(20); // @fixme is this to allow `worker0` to get ahead? Does it need to be that long?

        // resume worker1
        await launchWorker('worker1', binaryDir, worker1Dir, commands.worker1.commands.resume, false);
        await worker1Conn.open(); //reopen connection
        const resumeBlock = await latestBlock(worker1Conn, shard);
        assert.isNotEmpty(resumeBlock.result, "the latest block can't be empty");
        assert.isTrue(resumeBlock!.result!.number >= worker1CurrentBlock, 'failed to resume worker');
    });

    // @fixme don't we need to kill the children after the test!?!?
});
