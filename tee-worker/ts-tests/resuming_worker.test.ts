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

type WorkerConfig = {
    untrusted_ws_port: number;
    commands: {
        first_launch: string;
        resume: string;
    };
};

function genCommands(nodeUrl: string, nodePort: number): { worker0: WorkerConfig; worker1: WorkerConfig } {
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

type WorkerParams = {
    muRaPort: number;
    untrustedHttpPort: number;
    trustedWorkerPort: number;
    untrustedWorkerPort: number;
};

const worker0Params: WorkerParams = {
    muRaPort: 3443,
    untrustedHttpPort: 4545,
    trustedWorkerPort: 2000,
    untrustedWorkerPort: 3000,
};

const worker1Params: WorkerParams = {
    muRaPort: 3444,
    untrustedHttpPort: 4546,
    trustedWorkerPort: 2001,
    untrustedWorkerPort: 3001,
};

type WorkerCommandParams = WorkerParams & {
    enableMockServer: boolean;
    cleanReset: boolean;
    requestState: boolean;
    dev: boolean;
};

const worker0LaunchParams: WorkerCommandParams = {
    ...worker0Params,
    cleanReset: true,
    dev: true,
    enableMockServer: true,
    requestState: false,
};

const worker1LaunchParams: WorkerCommandParams = {
    ...worker1Params,
    cleanReset: true,
    dev: true,
    enableMockServer: false,
    requestState: true,
};

const worker0ResumeParams: WorkerCommandParams = {
    ...worker0Params,
    cleanReset: false,
    dev: false,
    enableMockServer: false,
    requestState: false,
};

const worker1ResumeParams: WorkerCommandParams = {
    ...worker1Params,
    cleanReset: false,
    dev: false,
    enableMockServer: false,
    requestState: false,
};

function generateWorkerCommandArguments(nodeUrl: string, nodePort: number, params: WorkerCommandParams): string {
    return [
        '--running-mode mock',
        ...(params.enableMockServer ? ['--enable-mock-server'] : []),
        ...(params.cleanReset ? ['--clean-reset'] : []),
        '--mu-ra-external-address localhost',
        `--mu-ra-port ${params.muRaPort}`,
        `--untrusted-http-port ${params.untrustedHttpPort}`,
        '--ws-external',
        '--trusted-external-address wss://localhost',
        `--trusted-worker-port ${params.trustedWorkerPort}`,
        '--untrusted-external-address wss://localhost',
        `--untrusted-worker-port ${params.untrustedWorkerPort}`,
        `--node-url ${nodeUrl}`,
        `--node-port ${nodePort}`,
        `run`,
        `--skip-ra`,
        ...(params.requestState ? ['--request-state'] : []),
        ...(params.dev ? ['--dev'] : []),
    ].join(' ');
}

async function launchWorker(
    name: string,
    binaryDir: string,
    workingDir: string,
    command: string
): Promise<{ shard: `0x${string}`; process: ChildProcess }> {
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

async function resumeWorker(
    name: string,
    binaryDir: string,
    workingDir: string,
    command: string
): Promise<ChildProcess> {
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

    return await new Promise<ChildProcess>((resolve, reject) => {
        outputStream.on('line', (line: string) => {
            console.log(name, line);

            if (line.includes('Successfully initialized shard')) {
                reject(new Error('Shard should have been there from the previous run'));
            }

            if (line.includes('Untrusted RPC server is spawned on')) {
                resolve(job);
            }
        });
    });
}

function killWorker(worker: ChildProcess): Promise<void> {
    const pid = worker.pid;

    if (pid === undefined) {
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
            if (start_block_number === 0) {
                start_block_number = block_number;
            }

            console.log(`${connection.ws.url} current block: ${block_number}`);
        }
        await sleep(2);
    } while (block_number < start_block_number + atLeast);
    return block_number;
}

function getConfiguration(): { binaryDir: string; nodeUrl: string; nodePort: number } {
    const binaryDir = process.env.BINARY_DIR;
    if (binaryDir === undefined) {
        throw new Error('Environment variable BINARY_DIR not defined');
    }

    const [, nodeHost, nodePortRaw] = process.env.SUBSTRATE_END_POINT!.split(':');
    const nodePort = Number.parseInt(nodePortRaw);
    if (nodeHost === undefined || Number.isNaN(nodePort)) {
        throw new Error('Environment variable SUBSTRATE_END_POINT undefined or malformed');
    }

    return { binaryDir, nodeUrl: `ws:${nodeHost}`, nodePort };
}

describe('Resume worker', function () {
    this.timeout(6000000);

    const { binaryDir, nodeUrl, nodePort } = getConfiguration();
    const worker0Dir = path.join(__dirname, './tmp/worker0');
    const worker1Dir = path.join(__dirname, './tmp/worker1');
    const commands = genCommands(nodeUrl, nodePort);

    step('One worker', async function () {
        assert.strictEqual(
            commands.worker0.commands.first_launch,
            generateWorkerCommandArguments(nodeUrl, nodePort, worker0LaunchParams)
        );
        // first launch worker
        const { shard: shard, process: worker0 } = await launchWorker(
            'worker0',
            binaryDir,
            worker0Dir,
            generateWorkerCommandArguments(nodeUrl, nodePort, worker0LaunchParams)
        );
        const worker0Conn = await initWorkerConnection(`ws://localhost:${commands.worker0.untrusted_ws_port}`);
        const currentBlock = await waitWorkerProducingBlock(worker0Conn, shard, 4);
        await killWorker(worker0);
        console.log('=========== worker stopped ==================');

        // resume worker
        assert.strictEqual(
            commands.worker0.commands.resume,
            generateWorkerCommandArguments(nodeUrl, nodePort, worker0ResumeParams)
        );
        await resumeWorker(
            'worker0',
            binaryDir,
            worker0Dir,
            generateWorkerCommandArguments(nodeUrl, nodePort, worker0ResumeParams)
        );
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
        assert.strictEqual(
            commands.worker1.commands.first_launch,
            generateWorkerCommandArguments(nodeUrl, nodePort, worker1LaunchParams)
        );
        const { shard: shard, process: worker1 } = await launchWorker(
            'worker1',
            binaryDir,
            worker1Dir,
            generateWorkerCommandArguments(nodeUrl, nodePort, worker1LaunchParams)
        );
        const worker1Conn = await initWorkerConnection(`ws://localhost:${commands.worker1.untrusted_ws_port}`);
        const worker1CurrentBlock = await waitWorkerProducingBlock(worker1Conn, shard, 4);
        await killWorker(worker1);
        console.log('=========== worker1 stopped ==================');
        await sleep(20); // @fixme is this to allow `worker0` to get ahead? Does it need to be that long?

        // resume worker1
        assert.strictEqual(
            commands.worker1.commands.resume,
            generateWorkerCommandArguments(nodeUrl, nodePort, worker1ResumeParams)
        );
        await resumeWorker(
            'worker1',
            binaryDir,
            worker1Dir,
            generateWorkerCommandArguments(nodeUrl, nodePort, worker1ResumeParams)
        );
        await worker1Conn.open(); //reopen connection
        const resumeBlock = await latestBlock(worker1Conn, shard);
        assert.isNotEmpty(resumeBlock.result, "the latest block can't be empty");
        assert.isTrue(resumeBlock!.result!.number >= worker1CurrentBlock, 'failed to resume worker');
    });

    // @fixme don't we need to kill the children after the test!?!?
});
