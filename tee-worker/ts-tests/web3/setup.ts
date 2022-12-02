import { Keyring } from "@polkadot/api";
import type { KeyringPair } from "@polkadot/keyring/types";
const keyring = new Keyring({ type: "sr25519" });

export function getSinger(index: number): KeyringPair {
    let Alice = keyring.addFromUri("//Alice", { name: "Alice" });
    let Bob = keyring.addFromUri("//Bob", { name: "Bob" });
    let Charlie = keyring.addFromUri("//Charlie", { name: "Charlie" });
    let Eve = keyring.addFromUri("//Eve", { name: "Eve" });
    const signers = [Alice, Bob, Charlie, Eve];
    return signers[index];
}
