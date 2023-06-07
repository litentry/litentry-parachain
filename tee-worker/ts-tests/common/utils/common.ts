import type { LitentryPrimitivesIdentity } from '@polkadot/types/lookup';
export function sleep(secs: number) {
    return new Promise((resolve) => {
        setTimeout(resolve, secs * 1000);
    });
}
