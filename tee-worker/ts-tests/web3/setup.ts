import { Keyring } from "@polkadot/api";
import type { KeyringPair } from "@polkadot/keyring/types";
const keyring = new Keyring({ type: "sr25519" });
const crypto = require("crypto");
export function getSigner(index: number): KeyringPair {
    let Alice = keyring.addFromUri("//Alice", { name: "Alice" });
    let Bob = keyring.addFromUri("//Bob", { name: "Bob" });
    let Charlie = keyring.addFromUri("//Charlie", { name: "Charlie" });
    let Eve = keyring.addFromUri("//Eve", { name: "Eve" });
    const signers = [Alice, Bob, Charlie, Eve];
    return signers[index];
}

export function generateChallengeCode(): String {
    return crypto.randomBytes(16).toString("hex");
}
