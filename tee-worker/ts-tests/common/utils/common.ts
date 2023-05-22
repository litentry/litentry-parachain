import { LitentryIdentity } from '../type-definitions';

export function sleep(secs: number) {
    return new Promise((resolve) => {
        setTimeout(resolve, secs * 1000);
    });
}

export function isEqual(obj1: LitentryIdentity, obj2: LitentryIdentity) {
    return JSON.stringify(obj1) === JSON.stringify(obj2);
}

// campare two array of event_identities idgraph_identities whether equal
export function isArrayEqual(arr1: LitentryIdentity[], arr2: LitentryIdentity[]) {
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
