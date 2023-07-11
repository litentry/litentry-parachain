import { runExamples } from './example';

(async () => {
    await runExamples('sr25519').catch((e) => {
        console.error(e);
    });
    process.exit(0);
})();
