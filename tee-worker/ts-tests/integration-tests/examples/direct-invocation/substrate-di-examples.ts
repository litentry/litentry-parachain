import { runExample } from './example';

(async () => {
    await runExample('substrate').catch((e) => {
        console.error(e);
    });
    process.exit(0);
})();
