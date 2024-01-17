import { sleep } from './../integration-tests/common/utils';

type RetryConfig = {
    isRetriable: (err: unknown) => boolean;
    maxRetries: number;
    initialDelaySeconds: number;
    backoffFactor: number;
};

async function withRetry<T>(
    task: () => Promise<T>,
    { isRetriable, maxRetries, initialDelaySeconds, backoffFactor }: RetryConfig
): Promise<T> {
    let attempt = 0;
    let delaySeconds = initialDelaySeconds;
    // eslint-disable-next-line no-constant-condition
    while (true) {
        try {
            return await task();
        } catch (err) {
            if (attempt > maxRetries || !isRetriable(err)) {
                throw err;
            }
            attempt += 1;
            await sleep(delaySeconds);
            delaySeconds *= backoffFactor;
        }
    }
}

describe('Resume worker', function () {
    console.log('works');
});
