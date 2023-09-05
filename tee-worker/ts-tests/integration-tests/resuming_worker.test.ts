import { ChildProcess, spawn } from 'child_process';
import * as readline from 'readline';
import fs from 'fs';
import * as path from 'path';
import * as process from 'process';
import { describe } from 'mocha';
import { step, xstep } from 'mocha-steps';
import WebSocketAsPromised from 'websocket-as-promised';
import os from 'os';
import { initWorkerConnection, sleep } from './common/utils';
import { assert } from 'chai';
import type { HexString } from '@polkadot/util/types';

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
        `--dev`,
        ...(isLaunch && workerParams.requestStateOnLaunch ? ['--request-state'] : []),
    ].join(' ');
}

function initializeFiles(workingDir: string, binaryDir: string) {
    fs.mkdirSync(workingDir, { recursive: true });
    fs.copyFileSync(path.join(binaryDir, 'enclave.signed.so'), path.join(workingDir, 'enclave.signed.so'));
    fs.copyFileSync(path.join(binaryDir, 'litentry-worker'), path.join(workingDir, 'litentry-worker'));
    fs.closeSync(fs.openSync(path.join(workingDir, 'spid.txt'), 'w'));
    fs.closeSync(fs.openSync(path.join(workingDir, 'key.txt'), 'w'));
}

type RetryConfig = {
    isRetriable: (err: unknown) => boolean;
    maxRetries: number;
    initialDelaySeconds: number;
    backoffFactor: number;
};

async function withRetry<T>(
    task: () => Promise<T>,
    { isRetriable, maxRetries, initialDelaySeconds, backoffFactor }: RetryConfig
): Promise<T> {
    let attempt = 0;
    let delaySeconds = initialDelaySeconds;
    // eslint-disable-next-line no-constant-condition
    while (true) {
        try {
            return await task();
        } catch (err) {
            if (attempt > maxRetries || !isRetriable(err)) {
                throw err;
            }
            attempt += 1;
            await sleep(delaySeconds);
            delaySeconds *= backoffFactor;
        }
    }
}

type JobConfig = {
    workerConfig: WorkerConfig;
    nodeConfig: NodeConfig;
    workingDir: string;
};

class SidechainDbLockUnavailable extends Error {
    constructor() {
        super('sidechain_db lock unavailable');
    }
}

async function spawnWorkerJob(
    command: Command,
    { workingDir, nodeConfig, workerConfig }: JobConfig,
    subprocessTracker: Set<number>
): Promise<{ job: ChildProcess; shard: HexString | undefined }> {
    const { name } = workerConfig;
    const task = () =>
        new Promise<{ job: ChildProcess; shard: HexString | undefined }>((resolve, reject) => {
            let shard: HexString | undefined = undefined;

            const job = spawn(
                `./litentry-worker`,
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

            job.on('error', (error) => reject(error));

            const pid = job.pid;
            if (!pid) {
                return;
            }

            subprocessTracker.add(pid);
            job.on('exit', () => {
                subprocessTracker.delete(pid);
            });

            job.on('close', (code) => {
                console.log(`${name} close: ${code}`);
            });

            job.stderr.setEncoding('utf8');
            const errorStream = readline.createInterface(job.stderr);
            errorStream.on('line', (line: string) => {
                console.warn(name, line);
                if (line.includes('lock file: sidechain_db/LOCK: Resource temporarily unavailable')) {
                    reject(new SidechainDbLockUnavailable());
                }
            });

            job.stdout.setEncoding('utf8');
            const outputStream = readline.createInterface(job.stdout);
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
                    shard = match.groups!.shard as HexString;
                    return;
                }

                if (line.includes('Untrusted RPC server is spawned on')) {
                    resolve({ job, shard });
                }
            });
        });

    return withRetry(task, {
        isRetriable: (err) => err instanceof SidechainDbLockUnavailable,
        maxRetries: 3,
        initialDelaySeconds: 10,
        backoffFactor: 1.5,
    });
}

type WorkerState = {
    shard: HexString;
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

    const { shard, job } = await spawnWorkerJob('launch', jobConfig, subprocessTracker);

    if (shard === undefined) {
        throw new Error('RPC server spawned before shard initialization');
    }

    const connection = await initWorkerConnection(`ws://localhost:${jobConfig.workerConfig.untrustedWorkerPort}`);
    const latestSeenBlock = await waitWorkerProducingBlock(connection, shard, 4);

    return { shard, job, connection, latestSeenBlock };
}

async function resumeWorker(
    jobConfig: JobConfig,
    connection: WebSocketAsPromised,
    subprocessTracker: Set<number>
): Promise<ChildProcess> {
    const { shard, job } = await spawnWorkerJob('resume', jobConfig, subprocessTracker);

    if (shard !== undefined) {
        throw new Error('Shard should have been reused from the previous run');
    }

    await connection.open();

    return job;
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
        worker.on('exit', () => {
            resolve();
        });
        worker.on('error', (error) => reject(error));
    });
}

async function latestBlock(
    connection: WebSocketAsPromised,
    shard: HexString
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

async function waitForBlock(connection: WebSocketAsPromised, shard: HexString, lowerBound: number): Promise<number> {
    const task = async () => {
        const resp = await latestBlock(connection, shard);
        const blockNumber = resp.result?.number;
        console.log(`${connection.ws.url} current block: ${blockNumber}`);
        if (blockNumber != undefined && blockNumber >= lowerBound) {
            return blockNumber;
        }
        throw new Error(`waiting for block ${lowerBound}; got ${blockNumber} instead`);
    };

    return withRetry(task, { isRetriable: () => true, maxRetries: 10, initialDelaySeconds: 5, backoffFactor: 1.5 });
}

async function waitWorkerProducingBlock(
    connection: WebSocketAsPromised,
    shard: HexString,
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

    const [, nodeHost, nodePortRaw] = process.env.NODE_ENDPOINT!.split(':');
    const nodePort = Number.parseInt(nodePortRaw);
    if (nodeHost === undefined || Number.isNaN(nodePort)) {
        throw new Error('Environment variable NODE_ENDPOINT undefined or malformed');
    }

    return { binaryDir, nodeUrl: `ws:${nodeHost}`, nodePort };
}

describe('Resume worker', function () {
    this.timeout(6000000);

    const { binaryDir, nodeUrl, nodePort } = getConfiguration();
    const nodeConfig = { nodeUrl, nodePort };

    const tempDir = os.tmpdir();
    const worker0Dir = fs.mkdtempSync(path.join(tempDir, 'worker0-'));
    const worker1Dir = fs.mkdtempSync(path.join(tempDir, 'worker1-'));

    const jobConfig: Record<'worker0' | 'worker1', JobConfig> = {
        worker0: {
            workingDir: worker0Dir,
            nodeConfig,
            workerConfig: workerConfig.worker0,
        },
        worker1: {
            workingDir: worker1Dir,
            nodeConfig,
            workerConfig: workerConfig.worker1,
        },
    };

    const subprocessTracker: Set<number> = new Set();

    after(() => {
        fs.rmSync(worker0Dir, { recursive: true, force: true });
        fs.rmSync(worker1Dir, { recursive: true, force: true });
        subprocessTracker.forEach((pid) => {
            if (pid !== undefined) {
                try {
                    process.kill(-pid, 'SIGTERM');
                } catch (error) {
                    if ((error as { code: unknown }).code === 'ESRCH') {
                        return; // Process has already died; nothing to do
                    }
                    console.warn(error);
                }
            }
        });
    });

    let worker0State: WorkerState | undefined = undefined;

    step('One worker', async function () {
        // first launch worker0
        worker0State = await launchWorker(binaryDir, jobConfig.worker0, subprocessTracker);
        console.log('=========== worker0 launched and produced blocks ==================');

        // kill worker0
        await killWorker(worker0State.job);
        console.log('=========== worker0 stopped ==================');

        // resume worker0
        worker0State.job = await resumeWorker(jobConfig.worker0, worker0State.connection, subprocessTracker);
        console.log('=========== worker0 resumed ==================');

        // check block production
        worker0State.latestSeenBlock = await waitForBlock(
            worker0State.connection,
            worker0State.shard,
            worker0State.latestSeenBlock + 1
        );
        console.log('=========== worker0 produced blocks ==================');
    });

    let worker1State: WorkerState | undefined = undefined;

    // #fixme #1524 multiworker not supported
    xstep('Two workers & resume worker1', async function () {
        assert(worker0State);

        // first launch worker1
        worker1State = await launchWorker(binaryDir, jobConfig.worker1, subprocessTracker);
        console.log('=========== worker1 launched and produced blocks ==================');

        // kill worker1
        await killWorker(worker1State.job);
        console.log('=========== worker1 stopped ==================');

        // let worker0 produce
        worker0State.latestSeenBlock = await waitForBlock(
            worker0State.connection,
            worker0State.shard,
            worker0State.latestSeenBlock + 1
        );
        console.log('=========== worker0 still produces blocks ==================');

        // resume worker1
        worker1State.job = await resumeWorker(jobConfig.worker1, worker1State.connection, subprocessTracker);
        console.log('=========== worker1 resumed ==================');

        // check block production
        worker1State.latestSeenBlock = await waitForBlock(
            worker1State.connection,
            worker1State.shard,
            worker1State.latestSeenBlock + 1
        );
        console.log('=========== worker1 produced blocks ==================');
    });

    // #fixme #1524 multiworker not supported
    xstep('Kill and resume both workers', async function () {
        assert(worker0State);
        assert(worker1State);

        // kill both workers
        await killWorker(worker0State.job);
        console.log('=========== worker0 stopped ==================');
        await killWorker(worker1State.job);
        console.log('=========== worker1 stopped ==================');

        // resume and check worker1
        worker1State.job = await resumeWorker(jobConfig.worker1, worker1State.connection, subprocessTracker);
        console.log('=========== worker1 resumed ==================');
        worker1State.latestSeenBlock = await waitForBlock(
            worker1State.connection,
            worker1State.shard,
            worker1State.latestSeenBlock + 1
        );
        console.log('=========== worker1 produced blocks ==================');

        // resume and check worker0
        worker0State.job = await resumeWorker(jobConfig.worker0, worker0State.connection, subprocessTracker);
        console.log('=========== worker0 resumed ==================');
        worker0State.latestSeenBlock = await waitForBlock(
            worker0State.connection,
            worker0State.shard,
            worker0State.latestSeenBlock + 1
        );
        console.log('=========== worker0 produced blocks ==================');

        // check worker1 health
        worker1State.latestSeenBlock = await waitForBlock(
            worker1State.connection,
            worker1State.shard,
            worker1State.latestSeenBlock + 1
        );
        console.log('=========== worker1 still produces blocks ==================');
    });

    step('Tidy up', async function () {
        // kill both workers
        if (worker0State) {
            await killWorker(worker0State.job);
            console.log('=========== worker0 stopped ==================');
        }
        if (worker1State) {
            await killWorker(worker1State.job);
            console.log('=========== worker1 stopped ==================');
        }
    });
});
