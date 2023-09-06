import { sleep } from "./sleep";

export async function withRetry<T>(maxAttempts: number, task: () => Promise<T>): Promise<T> {
    if (maxAttempts < 1) {
        throw new Error(`Invalid number of attempts: ${maxAttempts}`);
    }
    let attemptsLeft = maxAttempts;
    let waitMilliseconds = 1000;
    let errors: unknown[] = [];
    while (true) {
        try {
            return await task();
        } catch (error) {
            errors.push(error);
        }
        if (--attemptsLeft < 1) {
            const messages = errors.map((error) =>
                error instanceof Error ? error.message : JSON.stringify(error)
            );
            throw new Error(`Failed after ${maxAttempts} attempts:\n${messages.join("\n")}`);
        }
        waitMilliseconds *= 1 + Math.random(); // randomize backoff to avoid herd effect
        await sleep(waitMilliseconds);
    }
}
