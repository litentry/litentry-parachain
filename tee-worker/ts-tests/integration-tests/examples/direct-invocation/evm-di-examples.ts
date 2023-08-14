import { runExample } from './example';

(async () => {
    await runExample('evm').catch((e) => {
        console.error(e);
    });
    process.exit(0);
})();
