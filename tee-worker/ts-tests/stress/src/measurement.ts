export type Measurement<Label, Result> = {
    label: Label;
    start: number;
    end: number;
    result: Result;
};

export type Runner<Label, Result> = (label: Label, step: () => Promise<Result>) => Promise<Result>;

export function newTimedRunner<Label, Result>(
    output: WritableStream<Measurement<Label, Result>>
): Runner<Label, Result> {
    return async (label, step) => {
        const start = Date.now();
        const result = await step();
        const end = Date.now();
        const writer = output.getWriter();
        await writer.write({ label, start, end, result });
        writer.releaseLock();
        return result;
    };
}

export function newPassthroughRunner<Result>(): Runner<unknown, Result> {
    return async (_, step) => await step();
}
