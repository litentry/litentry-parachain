import { runExamples } from './example';

(async () => {
    await runExamples('ethereum').catch((e) => {
        console.error(e);
    });
    process.exit(0);
})();
