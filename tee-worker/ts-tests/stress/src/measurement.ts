export type Measurement = {
    start: number;
    end: number;
    success: boolean;
};

export type MeasurementTracker = (step: string, measurement: Measurement) => Promise<void>;

export function newMeasurementTracker(epoch: number): MeasurementTracker {
    const stream = new WritableStream<string>({
        write: (chunk) => {
            process.stdout.write(chunk);
        },
    });
    return async (step: string, measurement: Measurement) => {
        process.stderr.write(".");
        const start = measurement.start - epoch;
        const end = measurement.end - epoch;
        const csvLine = `"${step}", ${start}, ${end}, ${measurement.success}\n`;
        const writer = stream.getWriter();
        await writer.write(csvLine);
        writer.releaseLock();
    };
}

export async function timed(step: () => Promise<boolean>): Promise<Measurement> {
    const start = Date.now();
    const success = await step();
    const end = Date.now();
    return { start, end, success };
}
