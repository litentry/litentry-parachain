import {
    ApiPromise as ParachainApiPromise,
    WsProvider as ParachainWsProvider,
    definitions as teeTypes,
} from 'parachain-api';
import { initWorkerConnection, getSidechainMetadata, getEnclave, Api } from './litentry-api';
import { ContextManager } from './context-manager';
import { withRetry } from './util/with-retry';

export const apiContextManager = (
    config: {
        parachainEndpoint: string;
        enclaveEndpoint: string;
    },
    log: WritableStream<string>
): ContextManager<Api> => {
    return ContextManager.blank()
        .extend(
            async () => ({
                provider: new ParachainWsProvider(config.parachainEndpoint),
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
                teeWorker: await withRetry(5, () => initWorkerConnection(config.enclaveEndpoint, log)),
            }),
            async ({ teeWorker }) => {
                await teeWorker.close();
            }
        )
        .extend(
            async ({ parachainApi, teeWorker }) => {
                const { mrEnclave, teeShieldingKey } = await getEnclave(parachainApi);
                const { sidechainRegistry } = await getSidechainMetadata(teeWorker, parachainApi, log);
                return { mrEnclave, teeShieldingKey, sidechainRegistry };
            },
            async () => {}
        );
};
