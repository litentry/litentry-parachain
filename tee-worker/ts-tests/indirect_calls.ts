import {
    IntegrationTestContext,
    LitentryIdentity,
    LitentryValidationData,
} from "./type-definitions";
import { encryptWithTeeShieldingKey, listenEncryptedEvents } from "./utils";
import { KeyringPair } from "@polkadot/keyring/types";
import { HexString } from "@polkadot/util/types";
import { generateChallengeCode } from "./web3/setup";
export async function setUserShieldingKey(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean
): Promise<HexString | undefined> {
    const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, aesKey).toString("hex");
    await context.substrate.tx.identityManagement
        .setUserShieldingKey(context.shard, `0x${ciphertext}`)
        .signAndSend(signer);
    if (listening) {
        const event = await listenEncryptedEvents(context, aesKey, {
            module: "identityManagement",
            method: "userShieldingKeySet",
            event: "UserShieldingKeySet",
        });
        const [who] = event.eventData;
        return who;
    }
    return undefined;
}

export async function createIdentity(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    identity: LitentryIdentity
): Promise<HexString[] | undefined> {
    const encode = context.substrate.createType("LitentryIdentity", identity).toHex();
    const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString("hex");
    await context.substrate.tx.identityManagement
        .createIdentity(context.shard, `0x${ciphertext}`, null)
        .signAndSend(signer);
    if (listening) {
        const event = await listenEncryptedEvents(context, aesKey, {
            module: "identityManagement",
            method: "challengeCodeGenerated",
            event: "ChallengeCodeGenerated",
        });
        const [who, _identity, challengeCode] = event.eventData;
        return [who, challengeCode];
    }
    return undefined;
}

export async function removeIdentity(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    identity: LitentryIdentity
): Promise<HexString | undefined> {
    const encode = context.substrate.createType("LitentryIdentity", identity).toHex();
    const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString("hex");
    await context.substrate.tx.identityManagement
        .removeIdentity(context.shard, `0x${ciphertext}`)
        .signAndSend(signer);
    if (listening) {
        const event = await listenEncryptedEvents(context, aesKey, {
            module: "identityManagement",
            method: "identityRemoved",
            event: "IdentityRemoved",
        });
        const [who, _identity] = event.eventData;
        return who;
    }
    return undefined;
}

export async function verifyIdentity(
    context: IntegrationTestContext,
    signer: KeyringPair,
    aesKey: HexString,
    listening: boolean,
    identity: LitentryIdentity,
    data: LitentryValidationData
): Promise<HexString | undefined> {
    const identity_encode = context.substrate.createType("LitentryIdentity", identity).toHex();
    const validation_encode = context.substrate.createType("LitentryValidationData", data).toHex();
    const identity_ciphertext = encryptWithTeeShieldingKey(
        context.teeShieldingKey,
        identity_encode
    ).toString("hex");
    const validation_ciphertext = encryptWithTeeShieldingKey(
        context.teeShieldingKey,
        validation_encode
    ).toString("hex");
    await context.substrate.tx.identityManagement
        .verifyIdentity(context.shard, `0x${identity_ciphertext}`, `0x${validation_ciphertext}`)
        .signAndSend(signer);
    if (listening) {
        const event = await listenEncryptedEvents(context, aesKey, {
            module: "identityManagement",
            method: "identityVerified",
            event: "IdentityVerified",
        });
        const [who, _identity] = event.eventData;
        return who;
    }
    return undefined;
}
