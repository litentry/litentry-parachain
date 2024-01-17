import { ChildProcess, spawn } from 'child_process';
import * as readline from 'readline';
import fs from 'fs';
import * as path from 'path';
import * as process from 'process';
import { step } from 'mocha-steps';
import WebSocketAsPromised from 'websocket-as-promised';
import os from 'os';
import { initWorkerConnection, sleep } from './../integration-tests/common/utils';
import { assert } from 'chai';
import type { HexString } from '@polkadot/util/types';
import * as base58 from 'micro-base58';
import { u8aToHex } from '@polkadot/util';

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

describe('Resume worker', function () {
    console.log('works');
});
