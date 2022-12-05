import { Signer } from "@polkadot/types/types";
import { isString, u8aToHex, u8aToU8a, stringToU8a } from "@polkadot/util";
import type { KeyringPair } from "@polkadot/keyring/types";

export async function Sign(data: string, options?: KeyringPair): Promise<string> {
    let signer = options;
    const signature = signer!.sign(stringToU8a(data));
    return u8aToHex(signature);
}
