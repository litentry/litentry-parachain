import { runExample } from './example';

(async () => {
    await runExample('ethereum').catch((e) => {
        console.error(e);
    });
    process.exit(0);
})();
