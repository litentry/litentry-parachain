import crypto from "crypto";
import dotenv from "dotenv";
import { u8aToHex } from "@polkadot/util";
import { cryptoWaitReady } from "@polkadot/util-crypto";
import { apiContextManager } from "../src/api-context-manager";
import { Config } from "../src/config";
import { processQueue, repeat } from "../src/job-queue";
import { Measurement, newTimedRunner } from "../src/measurement";
import { randomWallet } from "../src/random-wallet";
import {
    activateIdentity,
    deactivateIdentity,
    linkIdentity,
    requestVc1,
    requestVc4,
} from "../src/steps";
import { newUserSession } from "../src/user-session";
import { assert } from "chai";

// eslint-disable-next-line @typescript-eslint/no-var-requires, no-undef
dotenv.config({ path: `.env.${process.env.NODE_ENV}` });

function getConfig(): Config {
    return {
        connections: 3,
        iterations: 2,
        workerEndpoint: process.env.WORKER_ENDPOINT!, // @fixme evil assertion; centralize env access
        substrateEndpoint: process.env.NODE_ENDPOINT!, // @fixme evil assertion; centralize env access
    };
}

describe("load test runner", function () {
    this.timeout(6000000);

    it("starts threads, runs tests, and collects results", async function () {
        /**
         * The test will start some threads and count the measurements here;
         * we check at the end that the expected number of requests was counted for each type.
         */
        const measurementCounts = new Map<string, number>();

        /**
         * Setup global context (shared across threads)
         */
        await cryptoWaitReady();
        const config = getConfig();
        const log = new WritableStream<string>({
            write: (chunk) => {
                process.stderr.write(chunk);
            },
        });
        const measurementOutput = new WritableStream<Measurement<string, boolean>>({
            write: (measurement) => {
                if (!measurement.result) {
                    process.stderr.write(`Measurement failed for ${measurement.label}\n`);
                    return;
                }
                process.stderr.write(".");
                const current = measurementCounts.get(measurement.label) ?? 0;
                measurementCounts.set(measurement.label, current + 1);
            },
        });
        const runner = newTimedRunner(measurementOutput);

        /**
         * Define the job to be ran by each thread
         */
        const newProcess = async () => {
            /**
             * Setup thread-specific context
             */
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
                return {
                    api,
                    session: await newUserSession(primary, userShieldingKey, api, log, runner),
                };
            });

            /**
             * Start the execution loop
             */
            await processQueue(
                contextManager,
                repeat(config.iterations, async ({ api, session }) => {
                    const secondary = randomWallet();

                    await linkIdentity(
                        runner,
                        session.primary,
                        secondary,
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

                    await deactivateIdentity(
                        runner,
                        primary,
                        secondary,
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

                    await activateIdentity(
                        runner,
                        primary,
                        secondary,
                        api.sidechainRegistry,
                        api.teeWorker,
                        api.parachainApi,
                        api.mrEnclave,
                        api.teeShieldingKey,
                        session.nextNonce(),
                        session.subject,
                        log
                    );
                }),
                async (error) => {
                    // Log the error, but continue to the next iteration
                    const writer = log.getWriter();
                    await writer.write(
                        `${error instanceof Error ? error.message : JSON.stringify(error)}\n`
                    );
                    writer.releaseLock();
                }
            );
        };

        /**
         * Start all the threads and await completion
         */
        const jobs: Promise<void>[] = [];
        for (let c = config.connections; c; c--) {
            jobs.push(newProcess());
        }
        await Promise.all(jobs);

        /**
         * Check for successful execution.
         */
        assert.equal(measurementCounts.size, 6);
        assert.equal(measurementCounts.get("setShieldingKey"), 3);
        [
            "linkIdentity",
            "requestVc1",
            "requestVc4",
            "deactivateIdentity",
            "activateIdentity",
        ].forEach((label) => {
            assert.equal(measurementCounts.get(label), 6);
        });
    });
});
