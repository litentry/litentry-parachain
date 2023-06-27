import { after, before, describe } from 'mocha';
import type { IntegrationTestContext } from '../type-definitions';
import { initIntegrationTestContext } from './context';

export function describeLitentry(title: string, walletsNumber: number, cb: (context: IntegrationTestContext) => void) {
    describe(title, function () {
        // Set timeout to 6000 seconds
        this.timeout(6000000);

        let context: IntegrationTestContext = undefined as unknown as IntegrationTestContext;

        before('Starting Litentry(parachain&tee)', async function () {
            //env url
            context = await initIntegrationTestContext(
                process.env.WORKER_END_POINT!,
                process.env.SUBSTRATE_END_POINT!,
                walletsNumber
            );
        });

        after(async function () {});

        cb(context);
    });
}
