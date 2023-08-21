import { Consumer } from "./consumer";
import { ContextManager } from "./context-manager";

export type JobQueue<Context, QueueResult> = Iterator<Consumer<Context>, QueueResult, never>;

export function* repeat<Context>(
    iterations: number,
    job: Consumer<Context>
): JobQueue<Context, void> {
    for (let i = iterations; i > 0; --i) {
        yield job;
    }
}

export async function processQueue<Context extends {}, QueueResult>(
    contextManager: ContextManager<Context>,
    queue: JobQueue<Context, QueueResult>,
    report: (error: unknown) => Promise<void>
): Promise<QueueResult> {
    while (true) {
        try {
            return contextManager.do(async (context) => {
                while (true) {
                    const nextResult = queue.next();
                    if (nextResult.done) {
                        return nextResult.value;
                    }
                    await nextResult.value(context);
                }
            });
        } catch (error) {
            await report(error);
        }
    }
}
