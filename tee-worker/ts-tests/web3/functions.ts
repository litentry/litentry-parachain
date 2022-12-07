import { Signer } from "@polkadot/types/types";
import { isString, u8aToHex, u8aToU8a, stringToU8a } from "@polkadot/util";
import type { KeyringPair } from "@polkadot/keyring/types";

export async function Sign(data: string, options?: KeyringPair): Promise<string> {
    let signer = options;
    const signature = signer!.sign(stringToU8a(data));
    return u8aToHex(signature);
}

export function generateTestKeys(): {
    alice: string;
    bob: string;
    charlie: string;
    dave: string;
    eve: string;
} {
    const secp256k1PrivateKeyLength = 32;
    const names = ["alice", "bob", "charlie", "dave", "eve"];
    let keys = new Array<string>();
    for (const name of names) {
        const result = Buffer.alloc(secp256k1PrivateKeyLength);
        result.fill(name, secp256k1PrivateKeyLength - Buffer.from(name, "utf8").length);
        keys.push(result.toString("hex"));
    }

    return { alice: keys[0], bob: keys[1], charlie: keys[2], dave: keys[3], eve: keys[4] };
}
