import { Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { u8aToHex } from "@polkadot/util";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import crypto, { randomBytes } from "crypto";
import { LitentryPrimitivesIdentity } from "sidechain-api";
import {
    ApiPromise as ParachainApiPromise,
    WsProvider as ParachainWsProvider,
    definitions as teeTypes,
} from "parachain-api";
import { Index } from "@polkadot/types/interfaces";
import { z } from "zod";
import { readFileSync } from "fs";
import path, { dirname } from "path";
import { fileURLToPath } from "url";
import { ethers } from "ethers";
import {
    initWorkerConnection,
    getSidechainMetadata,
    getEnclave,
    buildIdentityFromWallet,
    getSidechainNonce,
    Wallet,
    Api,
} from "../src/api";
import { ContextManager } from "src/context-manager";
import { processQueue, repeat } from "src/job-queue";
import { Measurement, newMeasurementTracker } from "src/measurement";
import { linkIdentity, requestVc1, requestVc4, setShieldingKey } from "src/steps";

function logLine(...data: unknown[]): void {
    process.stderr.write(`\n${data.join()}\n`);
}

function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

async function withRetry<T>(maxAttempts: number, task: () => Promise<T>): Promise<T> {
    if (maxAttempts < 1) {
        throw new Error(`Invalid number of attempts: ${maxAttempts}`);
    }
    let attemptsLeft = maxAttempts;
    let waitMilliseconds = 1000;
    let errors: unknown[] = [];
    while (true) {
        try {
            return await task();
        } catch (error) {
            errors.push(error);
        }
        if (--attemptsLeft < 1) {
            const messages = errors.map((error) =>
                error instanceof Error ? error.message : JSON.stringify(error)
            );
            throw new Error(`Failed after ${maxAttempts} attempts:\n${messages.join("\n")}`);
        }
        waitMilliseconds *= 1 + Math.random(); // randomize backoff to avoid herd effect
        await sleep(waitMilliseconds);
    }
}

const Config = z.object({
    timerEpoch: z.number(), // Arbitrary epoch to avoid precision loss when exporting/importing time info
    connections: z.number().int().positive(),
    iterations: z.number(),
    substrateEndpoint: z.string().url(),
    workerEndpoint: z.string().url(),
    waitMilliseconds: z
        .object({ min: z.number().nonnegative(), max: z.number().nonnegative() })
        .refine(({ min, max }) => min <= max),
});
type Config = z.infer<typeof Config>;

function loadConfig(path: string): Config {
    return Config.parse(JSON.parse(readFileSync(path, { encoding: "utf8" })));
}

function randomSubstrateWallet(): KeyringPair {
    const keyring = new Keyring({ type: "sr25519" });
    return keyring.addFromSeed(randomBytes(32));
}
function randomEvmWallet(): ethers.Wallet {
    return ethers.Wallet.createRandom();
}
function randomWallet(): Wallet {
    return Math.random() > 0.5
        ? { type: "substrate", keyringPair: randomSubstrateWallet() }
        : { type: "evm", wallet: randomEvmWallet() };
}

const apiContextManager = (
    config: {
        substrateEndpoint: string;
        workerEndpoint: string;
    },
    log: WritableStream<string>
): ContextManager<Api> => {
    return ContextManager.blank()
        .extend(
            async () => ({
                provider: new ParachainWsProvider(config.substrateEndpoint),
            }),
            async ({ provider }) => {
                await provider.disconnect();
            }
        )
        .extend(
            async ({ provider }) => ({
                parachainApi: await ParachainApiPromise.create({
                    provider,
                    types: teeTypes.types,
                }),
            }),
            async ({ parachainApi }) => {
                await parachainApi.disconnect();
            }
        )
        .extend(
            async () => ({
                teeWorker: await withRetry(5, () =>
                    initWorkerConnection(config.workerEndpoint, log)
                ),
            }),
            async ({ teeWorker }) => {
                await teeWorker.close();
            }
        )
        .extend(
            async ({ parachainApi, teeWorker }) => {
                const { mrEnclave, teeShieldingKey } = await getEnclave(parachainApi);
                const { sidechainRegistry } = await getSidechainMetadata(
                    teeWorker,
                    parachainApi,
                    log
                );
                return { mrEnclave, teeShieldingKey, sidechainRegistry };
            },
            async () => {}
        );
};

type UserSession = {
    primary: Wallet;
    userShieldingKey: `0x${string}`;
    subject: LitentryPrimitivesIdentity;
    nextNonce: () => Index;
};

async function newUserSession(
    primary: Wallet,
    userShieldingKey: `0x${string}`,
    api: Api,
    log: WritableStream<string>
): Promise<UserSession> {
    const subject = await buildIdentityFromWallet(primary, api.sidechainRegistry);
    const initialNonce = await getSidechainNonce(
        api.teeWorker,
        api.parachainApi,
        api.mrEnclave,
        api.teeShieldingKey,
        subject,
        log
    );
    let currentNonce = initialNonce.toNumber();
    const nextNonce = () => api.parachainApi.createType("Index", currentNonce++);

    await setShieldingKey(
        primary,
        api.sidechainRegistry,
        api.teeWorker,
        api.parachainApi,
        api.mrEnclave,
        api.teeShieldingKey,
        userShieldingKey,
        nextNonce(),
        subject,
        log
    );

    return {
        primary,
        userShieldingKey,
        subject,
        nextNonce,
    };
}

const newStepGenerator = (api: Api, session: UserSession, log: WritableStream<string>) => {
    const choices: (() => Promise<{ step: string; measurement: Measurement }>)[] = [
        async () => {
            return {
                step: "linkIdentity",
                measurement: await linkIdentity(
                    session.primary,
                    randomWallet(),
                    api.sidechainRegistry,
                    api.teeWorker,
                    api.parachainApi,
                    api.mrEnclave,
                    api.teeShieldingKey,
                    session.userShieldingKey,
                    session.nextNonce(),
                    session.subject,
                    log
                ),
            };
        },
        async () => {
            return {
                step: "requestVc1",
                measurement: await requestVc1(
                    session.primary,
                    api.sidechainRegistry,
                    api.teeWorker,
                    api.parachainApi,
                    api.mrEnclave,
                    api.teeShieldingKey,
                    session.nextNonce(),
                    session.subject,
                    log
                ),
            };
        },
        async () => {
            return {
                step: "requestVc4",
                measurement: await requestVc4(
                    session.primary,
                    api.sidechainRegistry,
                    api.teeWorker,
                    api.parachainApi,
                    api.mrEnclave,
                    api.teeShieldingKey,
                    session.nextNonce(),
                    session.subject,
                    log
                ),
            };
        },
    ];

    return { nextStep: () => choices[Math.floor(choices.length * Math.random())]() };
};

async function main() {
    const scriptDirectory = dirname(fileURLToPath(import.meta.url));
    const config = loadConfig(path.join(scriptDirectory, "../config.json"));

    const minWait = config.waitMilliseconds.min;
    const deltaWait = config.waitMilliseconds.max - minWait;
    const randomWait = () => Math.floor(minWait + Math.random() * deltaWait);

    const emit = newMeasurementTracker(config.timerEpoch);

    await cryptoWaitReady();

    const newProcess = async () => {
        const primary = randomWallet();
        const cryptoKey = await crypto.subtle.generateKey(
            {
                name: "AES-GCM",
                length: 256,
            },
            true,
            ["encrypt", "decrypt"]
        );
        const exportedKey = await crypto.subtle.exportKey("raw", cryptoKey);
        const userShieldingKey = u8aToHex(new Uint8Array(exportedKey));

        const log = new WritableStream<string>({
            write: (chunk) => {
                process.stderr.write(chunk);
            },
        });

        const contextManager = apiContextManager(config, log).map(async (api) => {
            const userSession = await newUserSession(primary, userShieldingKey, api, log);
            const stepGenerator = newStepGenerator(api, userSession, log);
            return stepGenerator;
        });

        await processQueue(
            contextManager,
            repeat(config.iterations, async ({ nextStep }) => {
                const { step, measurement } = await nextStep();
                await emit(step, measurement);
                await sleep(randomWait());
            }),
            async (error) => logLine(error instanceof Error ? error.message : JSON.stringify(error))
        );
    };

    const jobs: Promise<void>[] = [];
    for (let c = config.connections; c; c--) {
        jobs.push(newProcess());
    }
    await Promise.all(jobs);
}

main();
