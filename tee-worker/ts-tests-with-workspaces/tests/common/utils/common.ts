import type { LitentryPrimitivesIdentity } from 'sidechain-api';
export function sleep(secs: number) {
    return new Promise((resolve) => {
        setTimeout(resolve, secs * 1000);
    });
}
