import { u8aToHex } from "@polkadot/util";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import crypto from "crypto";
import { Api } from "../src/litentry-api";
import { processQueue, repeat } from "../src/job-queue";
import { Measurement, Runner, newTimedRunner } from "../src/measurement";
import { linkIdentity, requestVc1, requestVc4 } from "../src/steps";
import { apiContextManager } from "../src/api-context-manager";
import { UserSession, newUserSession } from "../src/user-session";
import { randomWallet } from "../src/random-wallet";
import { loadConfig } from "../src/config";

const newStepGenerator = (
    api: Api,
    session: UserSession,
    log: WritableStream<string>,
    runner: Runner<string, boolean>
) => {
    const choices: (() => Promise<void>)[] = [
        async () => {
            await linkIdentity(
                runner,
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
            );
        },
        async () => {
            await requestVc1(
                runner,
                session.primary,
                api.sidechainRegistry,
                api.teeWorker,
                api.parachainApi,
                api.mrEnclave,
                api.teeShieldingKey,
                session.nextNonce(),
                session.subject,
                log
            );
        },
        async () => {
            await requestVc4(
                runner,
                session.primary,
                api.sidechainRegistry,
                api.teeWorker,
                api.parachainApi,
                api.mrEnclave,
                api.teeShieldingKey,
                session.nextNonce(),
                session.subject,
                log
            );
        },
    ];

    return { nextStep: choices[Math.floor(choices.length * Math.random())] };
};

async function main() {
    console.warn(process.argv);
    const config = loadConfig(process.argv[process.argv.length - 1]);

    const log = new WritableStream<string>({
        write: (chunk) => {
            process.stderr.write(chunk);
        },
    });

    const measurementOutput = new WritableStream<Measurement<string, boolean>>({
        write: ({ label, start, end, result }) => {
            process.stderr.write(".");
            process.stdout.write(`"${label}", ${start}, ${end}, ${result}\n`);
        },
    });
    const runner = newTimedRunner(measurementOutput);

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

        const contextManager = apiContextManager(config, log).map(async (api) => {
            const userSession = await newUserSession(primary, userShieldingKey, api, log, runner);
            const stepGenerator = newStepGenerator(api, userSession, log, runner);
            return stepGenerator;
        });

        await processQueue(
            contextManager,
            repeat(config.iterations, async ({ nextStep }) => {
                await nextStep();
            }),
            async (error) => {
                const writer = log.getWriter();
                await writer.write(
                    `${error instanceof Error ? error.message : JSON.stringify(error)}\n`
                );
                writer.releaseLock();
            }
        );
    };

    const jobs: Promise<void>[] = [];
    for (let c = config.connections; c; c--) {
        jobs.push(newProcess());
    }
    await Promise.all(jobs);
}

main();
