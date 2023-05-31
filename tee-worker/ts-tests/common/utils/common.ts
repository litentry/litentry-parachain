import { LitentryIdentity } from '../type-definitions';
import { LitentryPrimitivesIdentity } from '@polkadot/types/lookup';
export function sleep(secs: number) {
    return new Promise((resolve) => {
        setTimeout(resolve, secs * 1000);
    });
}

export function isEqual(obj1: LitentryPrimitivesIdentity, obj2: LitentryPrimitivesIdentity) {
    return JSON.stringify(obj1) === JSON.stringify(obj2);
}

// campare two array of event_identities idgraph_identities whether equal
export function isArrayEqual(arr1: LitentryPrimitivesIdentity[], arr2: LitentryPrimitivesIdentity[]) {
    if (arr1.length !== arr2.length) {
        return false;
    }
    for (let i = 0; i < arr1.length; i++) {
        const obj1 = arr1[i];
        let found = false;

        for (let j = 0; j < arr2.length; j++) {
            const obj2 = arr2[j];

            if (isEqual(obj1, obj2)) {
                found = true;
                break;
            }
        }

        if (!found) {
            return false;
        }
    }
    return true;
}
