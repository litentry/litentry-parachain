import { runExample } from './example';

(async () => {
    await runExample('sr25519').catch((e) => {
        console.error(e);
    });
    process.exit(0);
})();
