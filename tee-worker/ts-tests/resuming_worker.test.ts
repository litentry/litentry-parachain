import { ChildProcess, ChildProcessWithoutNullStreams, spawn } from 'child_process';
import * as readline from 'readline';
import fs from 'fs';
import * as path from 'path';
import * as process from 'process';
import { describe } from 'mocha';
import { step } from 'mocha-steps';
import WebSocketAsPromised from 'websocket-as-promised';
import { initWorkerConnection, sleep } from './common/utils';
import { assert } from 'chai';

type WorkerConfig = {
    name: string;
    muRaPort: number;
    untrustedHttpPort: number;
    trustedWorkerPort: number;
    untrustedWorkerPort: number;
    enableMockServer: boolean;
    requestStateOnLaunch: boolean;
};

const workerConfig: Record<'worker0' | 'worker1', WorkerConfig> = {
    worker0: {
        name: 'worker0',
        muRaPort: 3443,
        untrustedHttpPort: 4545,
        trustedWorkerPort: 2000,
        untrustedWorkerPort: 3000,
        enableMockServer: true,
        requestStateOnLaunch: false,
    },
    worker1: {
        name: 'worker1',
        muRaPort: 3444,
        untrustedHttpPort: 4546,
        trustedWorkerPort: 2001,
        untrustedWorkerPort: 3001,
        enableMockServer: false,
        requestStateOnLaunch: true,
    },
} as const;

type NodeConfig = {
    nodeUrl: string;
    nodePort: number;
};

type Command = 'launch' | 'resume';

function generateWorkerCommandArguments(
    command: Command,
    { nodeUrl, nodePort }: NodeConfig,
    workerParams: WorkerConfig
): string {
    const isLaunch = command === 'launch';

    return [
        '--running-mode mock',
        ...(workerParams.enableMockServer ? ['--enable-mock-server'] : []),
        ...(isLaunch ? ['--clean-reset'] : []),
        '--mu-ra-external-address localhost',
        `--mu-ra-port ${workerParams.muRaPort}`,
        `--untrusted-http-port ${workerParams.untrustedHttpPort}`,
        '--ws-external',
        '--trusted-external-address wss://localhost',
        `--trusted-worker-port ${workerParams.trustedWorkerPort}`,
        '--untrusted-external-address ws://localhost',
        `--untrusted-worker-port ${workerParams.untrustedWorkerPort}`,
        `--node-url ${nodeUrl}`,
        `--node-port ${nodePort}`,
        `run`,
        `--skip-ra`,
        ...(isLaunch && workerParams.requestStateOnLaunch ? ['--request-state'] : []),
        ...(isLaunch ? ['--dev'] : []),
    ].join(' ');
}

function initializeFiles(workingDir: string, binaryDir: string) {
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
            achainable_url: 'http://localhost:19527',
            achainable_auth_key: '',
        },
        null,
        4
    );
    fs.writeFileSync(`${workingDir}/worker-config-mock.json`, data);
}

type JobConfig = {
    workerConfig: WorkerConfig;
    nodeConfig: NodeConfig;
    workingDir: string;
};

async function spawnWorkerJob(
    command: Command,
    { workingDir, nodeConfig, workerConfig }: JobConfig,
    subprocessTracker: Set<number>
) {
    const { name } = workerConfig;

    const job = await new Promise<ChildProcessWithoutNullStreams>((resolve, reject) => {
        const childProcess = spawn(
            `./integritee-service`,
            [generateWorkerCommandArguments(command, nodeConfig, workerConfig)],
            {
                cwd: workingDir,
                shell: '/bin/sh',
                env: {
                    RUST_LOG: 'warn,sp_io::storage=error,substrate_api_client=warn',
                    ...process.env,
                },
                detached: true,
            }
        );

        if (childProcess.pid !== undefined) {
            subprocessTracker.add(childProcess.pid);
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

    return { outputStream, job };
}

type WorkerState = {
    shard: `0x${string}`;
    job: ChildProcess;
    connection: WebSocketAsPromised;
    latestSeenBlock: number;
};

async function launchWorker(
    binaryDir: string,
    jobConfig: JobConfig,
    subprocessTracker: Set<number>
): Promise<WorkerState> {
    initializeFiles(jobConfig.workingDir, binaryDir);

    const { outputStream, job } = await spawnWorkerJob('launch', jobConfig, subprocessTracker);

    const shard = await new Promise<`0x${string}`>((resolve, reject) => {
        let shard: `0x${string}` | undefined = undefined;

        outputStream.on('line', (line: string) => {
            console.log(jobConfig.workerConfig.name, line);

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
                resolve(shard);
            }
        });
    });

    const connection = await initWorkerConnection(`ws://localhost:${jobConfig.workerConfig.untrustedWorkerPort}`);
    const latestSeenBlock = await waitWorkerProducingBlock(connection, shard, 4);

    return { shard, job, connection, latestSeenBlock };
}

async function resumeWorker(
    jobConfig: JobConfig,
    connection: WebSocketAsPromised,
    subprocessTracker: Set<number>
): Promise<ChildProcess> {
    const { outputStream, job } = await spawnWorkerJob('resume', jobConfig, subprocessTracker);

    await new Promise<void>((resolve, reject) => {
        outputStream.on('line', (line: string) => {
            console.log(jobConfig.workerConfig.name, line);

            if (line.includes('Successfully initialized shard')) {
                reject(new Error('Shard should have been there from the previous run'));
            }

            if (line.includes('Untrusted RPC server is spawned on')) {
                resolve();
            }
        });
    });

    await connection.open();

    return job;
}

function killWorker(worker: ChildProcess, subprocessTracker: Set<number>): Promise<void> {
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
        worker.on('exit', () => {
            subprocessTracker.delete(pid);
            resolve();
        });
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

async function waitForBlock(
    connection: WebSocketAsPromised,
    shard: `0x${string}`,
    lowerBound: number
): Promise<number> {
    let waitIntervalSeconds = 2;
    const waitIncreaseFactor = 5;
    // eslint-disable-next-line no-constant-condition
    while (true) {
        const resp = await latestBlock(connection, shard);
        // lax comparison because `resp.result` could be `null` instead of `undefined` :P
        if (resp.result == undefined) {
            console.log(`${connection.ws.url} current block: undefined`);
            continue;
        }

        const blockNumber = resp.result.number;
        console.log(`${connection.ws.url} current block: ${blockNumber}`);

        if (blockNumber >= lowerBound) {
            return blockNumber;
        }

        await sleep(waitIntervalSeconds);
        waitIntervalSeconds = waitIntervalSeconds * waitIncreaseFactor;
    }
}

async function waitWorkerProducingBlock(
    connection: WebSocketAsPromised,
    shard: `0x${string}`,
    atLeast: number
): Promise<number> {
    const currentBlockNumber = await waitForBlock(connection, shard, 0);
    return await waitForBlock(connection, shard, currentBlockNumber + atLeast);
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
    const nodeConfig = { nodeUrl, nodePort };
    const jobConfig: Record<'worker0' | 'worker1', JobConfig> = {
        worker0: {
            workingDir: path.join(__dirname, './tmp/worker0'),
            nodeConfig,
            workerConfig: workerConfig.worker0,
        },
        worker1: {
            workingDir: path.join(__dirname, './tmp/worker1'),
            nodeConfig,
            workerConfig: workerConfig.worker1,
        },
    };

    const subprocessTracker: Set<number> = new Set();

    after(() => {
        subprocessTracker.forEach((pid) => {
            if (pid !== undefined) {
                process.kill(-pid, 'SIGTERM');
            }
        });
    });

    let worker0State: WorkerState | undefined = undefined;

    step('One worker', async function () {
        // first launch worker0
        worker0State = await launchWorker(binaryDir, jobConfig.worker0, subprocessTracker);

        // kill worker0
        await killWorker(worker0State.job, subprocessTracker);
        console.log('=========== worker0 stopped ==================');

        // resume worker0
        worker0State.job = await resumeWorker(jobConfig.worker0, worker0State.connection, subprocessTracker);

        // check block production
        worker0State.latestSeenBlock = await waitForBlock(
            worker0State.connection,
            worker0State.shard,
            worker0State.latestSeenBlock + 1
        );
    });

    let worker1State: WorkerState | undefined = undefined;

    step('Two workers & resume worker1', async function () {
        assert(worker0State);

        // first launch worker1
        worker1State = await launchWorker(binaryDir, jobConfig.worker1, subprocessTracker);

        // kill worker1
        await killWorker(worker1State.job, subprocessTracker);
        console.log('=========== worker1 stopped ==================');

        // let worker0 produce
        worker0State.latestSeenBlock = await waitForBlock(
            worker0State.connection,
            worker0State.shard,
            worker0State.latestSeenBlock + 1
        );

        // resume worker1
        worker1State.job = await resumeWorker(jobConfig.worker1, worker1State.connection, subprocessTracker);

        // check block production
        worker1State.latestSeenBlock = await waitForBlock(
            worker1State.connection,
            worker1State.shard,
            worker1State.latestSeenBlock + 1
        );
    });

    step('Kill and resume both workers', async function () {
        assert(worker0State);
        assert(worker1State);

        // kill both workers
        await killWorker(worker0State.job, subprocessTracker);
        console.log('=========== worker0 stopped ==================');
        await killWorker(worker1State.job, subprocessTracker);
        console.log('=========== worker1 stopped ==================');

        // resume and check worker1
        worker1State.job = await resumeWorker(jobConfig.worker1, worker1State.connection, subprocessTracker);
        worker1State.latestSeenBlock = await waitForBlock(
            worker1State.connection,
            worker1State.shard,
            worker1State.latestSeenBlock + 1
        );

        // resume and check worker0
        worker0State.job = await resumeWorker(jobConfig.worker0, worker0State.connection, subprocessTracker);
        worker0State.latestSeenBlock = await waitForBlock(
            worker0State.connection,
            worker0State.shard,
            worker0State.latestSeenBlock + 1
        );
    });
});
