import { z } from 'zod';
import { readFileSync } from 'fs';

export const Config = z.object({
    connections: z.number().int().positive(),
    iterations: z.number(),
    parachainEndpoint: z.string().url(),
    enclaveEndpoint: z.string().url(),
});

export type Config = z.infer<typeof Config>;

export function loadConfig(path: string): Config {
    return Config.parse(JSON.parse(readFileSync(path, { encoding: 'utf8' })));
}
