import { Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { u8aToHex } from "@polkadot/util";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import crypto, { randomBytes } from "crypto";
import { LitentryPrimitivesIdentity, TypeRegistry as SidechainTypeRegistry } from "sidechain-api";
import {
    ApiPromise as ParachainApiPromise,
    WsProvider as ParachainWsProvider,
    definitions as teeTypes,
} from "parachain-api";
import WebSocketAsPromised from "websocket-as-promised";
import { Index } from "@polkadot/types/interfaces";
import { z } from "zod";
import { readFileSync } from "fs";
import path, { dirname } from "path";
import { fileURLToPath } from "url";
import { ethers } from "ethers";
import { Wallet as EvmWallet } from "ethers";
import {
    initWorkerConnection,
    getSidechainMetadata,
    getEnclave,
    buildIdentityFromWallet,
    getSidechainNonce,
    createSignedTrustedCallSetUserShieldingKey,
    subscribeToEventsWithExtHash,
    sendRequestFromTrustedCall,
    buildValidation,
    createSignedTrustedCallLinkIdentity,
    keyNonce,
    createSignedTrustedCallRequestVc,
} from "../src/api";

export function logLine(...data: unknown[]): void {
    process.stderr.write(`\n${data.join()}\n`);
}

function sleep(ms: number) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}

async function withRetry<T>(maxAttempts: number, task: () => Promise<T>): Promise<T> {
    if (maxAttempts < 1) {
        throw new Error(`Invalid number of attempts ${maxAttempts}`);
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

type Factory<T> = () => Promise<T>;
type Consumer<T> = (arg: T) => Promise<void>;

type ContextFactory<Context> = Factory<{
    context: Context;
    exit: Consumer<void>;
}>;
class ContextManager<Context extends {}> {
    readonly enter: ContextFactory<Context>;
    constructor(enter: ContextFactory<Context>) {
        this.enter = enter;
    }

    async do<Result>(task: (context: Context) => Promise<Result>) {
        const { context, exit } = await this.enter();
        try {
            return await task(context);
        } finally {
            exit();
        }
    }

    static blank(): ContextManager<{}> {
        return new ContextManager(async () => ({ context: {}, exit: async () => {} }));
    }

    map<NewContext extends {}>(
        mapping: (context: Context) => Promise<NewContext>
    ): ContextManager<NewContext> {
        return new ContextManager(async () => {
            const { context, exit } = await this.enter();
            return { context: await mapping(context), exit };
        });
    }

    extend<Extension extends {}>(
        acquire: (context: Context) => Promise<Extension>,
        dispose: Consumer<Extension>
    ): ContextManager<Omit<Context, keyof Extension> & Extension> {
        return new ContextManager(async () => {
            const { context, exit } = await this.enter();
            try {
                const extension = await acquire(context);
                return {
                    context: { ...context, ...extension },
                    exit: async () => {
                        await dispose(extension);
                        await exit();
                    },
                };
            } catch (error) {
                await exit();
                throw error;
            }
        });
    }
}

type JobQueue<Context, QueueResult> = Iterator<Consumer<Context>, QueueResult, never>;

async function processQueue<Context extends {}, QueueResult>(
    contextManager: ContextManager<Context>,
    queue: JobQueue<Context, QueueResult>,
    report: (error: unknown) => Promise<void>
): Promise<QueueResult> {
    while (true) {
        try {
            return contextManager.do(async (context) => {
                while (true) {
                    const nextResult = queue.next();
                    if (nextResult.done) {
                        return nextResult.value;
                    }
                    await nextResult.value(context);
                }
            });
        } catch (error) {
            await report(error);
        }
    }
}

function* repeat<Context>(iterations: number, job: Consumer<Context>): JobQueue<Context, void> {
    for (let i = iterations; i > 0; --i) {
        yield job;
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

export type Wallet =
    | {
          type: "substrate";
          keyringPair: KeyringPair;
      }
    | { type: "evm"; wallet: EvmWallet };

function randomSubstrateWallet(): KeyringPair {
    const keyring = new Keyring({ type: "sr25519" });
    return keyring.addFromSeed(randomBytes(32));
}
function randomEvmWallet(): EvmWallet {
    return ethers.Wallet.createRandom();
}
function randomWallet(): Wallet {
    return Math.random() > 0.5
        ? { type: "substrate", keyringPair: randomSubstrateWallet() }
        : { type: "evm", wallet: randomEvmWallet() };
}

type MeasurementTracker = (step: string, measurement: Measurement) => Promise<void>;

function newMeasurementTracker(epoch: number): MeasurementTracker {
    const stream = new WritableStream({
        write: (chunk) => {
            process.stdout.write(chunk);
        },
    });
    return async (step: string, measurement: Measurement) => {
        process.stderr.write(".");
        const start = measurement.start - epoch;
        const end = measurement.end - epoch;
        const csvLine = `"${step}", ${start}, ${end}, ${measurement.success}\n`;
        const writer = stream.getWriter();
        await writer.write(csvLine);
        writer.releaseLock();
    };
}

type Api = {
    parachainApi: ParachainApiPromise;
    mrEnclave: `0x${string}`;
    teeShieldingKey: crypto.KeyObject;
    teeWorker: WebSocketAsPromised;
    sidechainRegistry: SidechainTypeRegistry;
};

const apiContextManager = (config: {
    substrateEndpoint: string;
    workerEndpoint: string;
}): ContextManager<Api> => {
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
                teeWorker: await withRetry(5, () => initWorkerConnection(config.workerEndpoint)),
            }),
            async ({ teeWorker }) => {
                await teeWorker.close();
            }
        )
        .extend(
            async ({ parachainApi, teeWorker }) => {
                const { mrEnclave, teeShieldingKey } = await getEnclave(parachainApi);
                const { sidechainRegistry } = await getSidechainMetadata(teeWorker, parachainApi);
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
    api: Api
): Promise<UserSession> {
    const subject = await buildIdentityFromWallet(primary, api.sidechainRegistry);
    const initialNonce = await getSidechainNonce(
        api.teeWorker,
        api.parachainApi,
        api.mrEnclave,
        api.teeShieldingKey,
        subject
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
        subject
    );

    return {
        primary,
        userShieldingKey,
        subject,
        nextNonce,
    };
}

const newStepGenerator = (api: Api, session: UserSession) => {
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
                    session.subject
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
                    session.subject
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
                    session.subject
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

        const contextManager = apiContextManager(config).map(async (api) => {
            const userSession = await newUserSession(primary, userShieldingKey, api);
            const stepGenerator = newStepGenerator(api, userSession);
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

type Measurement = {
    start: number;
    end: number;
    success: boolean;
};

async function timed(step: () => Promise<boolean>): Promise<Measurement> {
    const start = Date.now();
    const success = await step();
    const end = Date.now();
    return { start, end, success };
}

async function swallowErrors<Result>(
    fallback: Result,
    task: () => Promise<Result>
): Promise<Result> {
    try {
        return await task();
    } catch (error) {
        logLine(
            `Swallowed error: ${error instanceof Error ? error.message : JSON.stringify(error)}`
        );
        return fallback;
    }
}

async function setShieldingKey(
    primary: Wallet,
    sidechainRegistry: SidechainTypeRegistry,
    teeWorker: WebSocketAsPromised,
    parachainApi: ParachainApiPromise,
    mrEnclave: string,
    teeShieldingKey: crypto.KeyObject,
    userShieldingKey: string,
    nonce: Index,
    subject: LitentryPrimitivesIdentity
): Promise<Measurement> {
    const requestIdentifier = `0x${randomBytes(32).toString("hex")}`;

    const setUserShieldingKeyCall = await createSignedTrustedCallSetUserShieldingKey(
        parachainApi,
        mrEnclave,
        nonce,
        primary,
        subject,
        userShieldingKey,
        requestIdentifier
    );

    const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, parachainApi);

    return await timed(async () => {
        await sendRequestFromTrustedCall(
            teeWorker,
            parachainApi,
            mrEnclave,
            teeShieldingKey,
            setUserShieldingKeyCall
        );

        const events = await eventsPromise;

        return events
            .map(({ event }) => event)
            .some((event) => parachainApi.events.identityManagement.UserShieldingKeySet.is(event));
    });
}

async function linkIdentity(
    primary: Wallet,
    secondary: Wallet,
    sidechainRegistry: SidechainTypeRegistry,
    teeWorker: WebSocketAsPromised,
    parachainApi: ParachainApiPromise,
    mrEnclave: string,
    teeShieldingKey: crypto.KeyObject,
    userShieldingKey: string,
    nonce: Index,
    subject: LitentryPrimitivesIdentity
): Promise<Measurement> {
    const requestIdentifier = `0x${randomBytes(32).toString("hex")}`;
    const primarySubject = await buildIdentityFromWallet(primary, sidechainRegistry);
    const secondaryIdentity = await buildIdentityFromWallet(secondary, sidechainRegistry);
    const secondaryNetworks = parachainApi.createType(
        "Vec<Web3Network>",
        secondary.type === "evm" ? ["Ethereum", "Bsc"] : ["Litentry", "Polkadot"]
    );

    const secondaryValidation = await buildValidation(
        parachainApi,
        sidechainRegistry,
        subject,
        secondaryIdentity,
        nonce.toNumber(),
        userShieldingKey,
        secondary
    );

    const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
        parachainApi,
        mrEnclave,
        parachainApi.createType("Index", nonce),
        primary,
        primarySubject,
        secondaryIdentity.toHex(),
        secondaryValidation.toHex(),
        secondaryNetworks.toHex(),
        keyNonce,
        requestIdentifier
    );
    const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, parachainApi);

    return await timed(async () => {
        await sendRequestFromTrustedCall(
            teeWorker,
            parachainApi,
            mrEnclave,
            teeShieldingKey,
            linkIdentityCall
        );

        const events = await eventsPromise;
        return events
            .map(({ event }) => event)
            .some((event) => parachainApi.events.identityManagement.IdentityLinked.is(event));
    });
}

async function requestVc1(
    primary: Wallet,
    sidechainRegistry: SidechainTypeRegistry,
    teeWorker: WebSocketAsPromised,
    parachainApi: ParachainApiPromise,
    mrEnclave: string,
    teeShieldingKey: crypto.KeyObject,
    nonce: Index,
    subject: LitentryPrimitivesIdentity
): Promise<Measurement> {
    const requestIdentifier = `0x${randomBytes(32).toString("hex")}`;
    const requestVcCall = await createSignedTrustedCallRequestVc(
        parachainApi,
        mrEnclave,
        nonce,
        primary,
        subject,
        parachainApi.createType("Assertion", { A1: null }),
        requestIdentifier
    );

    const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, parachainApi);

    return await timed(async () => {
        await sendRequestFromTrustedCall(
            teeWorker,
            parachainApi,
            mrEnclave,
            teeShieldingKey,
            requestVcCall
        );

        const events = await eventsPromise;

        return events
            .map(({ event }) => event)
            .some((event) => parachainApi.events.vcManagement.VCIssued.is(event));
    });
}

async function requestVc4(
    primary: Wallet,
    sidechainRegistry: SidechainTypeRegistry,
    teeWorker: WebSocketAsPromised,
    parachainApi: ParachainApiPromise,
    mrEnclave: string,
    teeShieldingKey: crypto.KeyObject,
    nonce: Index,
    subject: LitentryPrimitivesIdentity
): Promise<Measurement> {
    const requestIdentifier = `0x${randomBytes(32).toString("hex")}`;
    const requestVcCall = await createSignedTrustedCallRequestVc(
        parachainApi,
        mrEnclave,
        nonce,
        primary,
        subject,
        parachainApi.createType("Assertion", { A4: "10" }),
        requestIdentifier
    );

    const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, parachainApi);

    return await timed(async () => {
        await sendRequestFromTrustedCall(
            teeWorker,
            parachainApi,
            mrEnclave,
            teeShieldingKey,
            requestVcCall
        );

        const events = await eventsPromise;

        return events
            .map(({ event }) => event)
            .some((event) => parachainApi.events.vcManagement.VCIssued.is(event));
    });
}

/**
 * NOTE:
 * These steps are not available in the `internal` branch which is what we're testing against
 */

// async function deactiveIdentity(
//     primary: KeyringPair,
//     secondary: Wallet,
//     sidechainRegistry: SidechainTypeRegistry,
//     teeWorker: WebSocketAsPromised,
//     parachainApi: ParachainApiPromise,
//     mrEnclave: string,
//     teeShieldingKey: crypto.KeyObject
// ): Promise<Measurement> {
//     const primarySubject = await buildIdentityFromKeypair(primary, sidechainRegistry);
//     const evmIdentity = await buildIdentityHelper(secondary.address, "Evm", sidechainRegistry);

//     const nonce = await getSidechainNonce(
//         teeWorker,
//         parachainApi,
//         mrEnclave,
//         teeShieldingKey,
//         primarySubject
//     );
//     const requestIdentifier = `0x${randomBytes(32).toString("hex")}`;

//     const deactivateIdentityCall = createSignedTrustedCallDeactivateIdentity(
//         parachainApi,
//         mrEnclave,
//         parachainApi.createType("Index", nonce),
//         primary,
//         primarySubject,
//         evmIdentity.toHex(),
//         requestIdentifier
//     );
//     const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, parachainApi);
//     return timed(async () => {
//         await sendRequestFromTrustedCall(
//             teeWorker,
//             parachainApi,
//             mrEnclave,
//             teeShieldingKey,
//             deactivateIdentityCall
//         );

//         const events = await eventsPromise;
//         return events
//             .map(({ event }) => event)
//             .some((event) => parachainApi.events.identityManagement.IdentityDeactivated.is(event));
//     });
// }

// async function activateIdentity(
//     primary: KeyringPair,
//     secondary: Wallet,
//     sidechainRegistry: SidechainTypeRegistry,
//     teeWorker: WebSocketAsPromised,
//     parachainApi: ParachainApiPromise,
//     mrEnclave: string,
//     teeShieldingKey: crypto.KeyObject
// ): Promise<Measurement> {
//     const primarySubject = await buildIdentityFromKeypair(primary, sidechainRegistry);
//     const evmIdentity = await buildIdentityHelper(secondary.address, "Evm", sidechainRegistry);

//     const nonce = await getSidechainNonce(
//         teeWorker,
//         parachainApi,
//         mrEnclave,
//         teeShieldingKey,
//         primarySubject
//     );
//     const requestIdentifier = `0x${randomBytes(32).toString("hex")}`;

//     const deactivateIdentityCall = createSignedTrustedCallActivateIdentity(
//         parachainApi,
//         mrEnclave,
//         parachainApi.createType("Index", nonce),
//         primary,
//         primarySubject,
//         evmIdentity.toHex(),
//         requestIdentifier
//     );
//     const eventsPromise = subscribeToEventsWithExtHash(requestIdentifier, parachainApi);
//     return timed(async () => {
//         await sendRequestFromTrustedCall(
//             teeWorker,
//             parachainApi,
//             mrEnclave,
//             teeShieldingKey,
//             deactivateIdentityCall
//         );

//         const events = await eventsPromise;
//         return events
//             .map(({ event }) => event)
//             .some((event) => parachainApi.events.identityManagement.IdentityActivated.is(event));
//     });
// }

main();
