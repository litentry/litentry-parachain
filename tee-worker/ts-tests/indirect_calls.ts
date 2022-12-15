import {
    IdentityGenericEvent,
    IntegrationTestContext,
    LitentryIdentity,
    LitentryValidationData,
} from "./type-definitions";
import {encryptWithTeeShieldingKey, listenEncryptedEvents, sendTxUntilInBlock} from "./utils";
import { KeyringPair } from "@polkadot/keyring/types";
import { HexString } from "@polkadot/util/types";
import { generateChallengeCode } from "./web3/setup";
import {ApiPromise} from "@polkadot/api";
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
    const nonce = await context.substrate.rpc.system.accountNextIndex(signer.address);
    await context.substrate.tx.identityManagement
        .createIdentity(context.shard, `0x${ciphertext}`, null)
        .signAndSend(signer, { nonce });
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
): Promise<IdentityGenericEvent | undefined> {
    const encode = context.substrate.createType("LitentryIdentity", identity).toHex();
    const ciphertext = encryptWithTeeShieldingKey(context.teeShieldingKey, encode).toString("hex");

    const tx = context.substrate.tx.identityManagement.removeIdentity(context.shard, `0x${ciphertext}`)
    await sendTxUntilInBlock(context.substrate, tx, signer)

    if (listening) {
        const event = await listenEncryptedEvents(context, aesKey, {
            module: "identityManagement",
            method: "identityRemoved",
            event: "IdentityRemoved",
        });
        const [who, identity, idGraph] = event.eventData;
        return decodeIdentityEvent(context.substrate, who, identity, idGraph)
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
): Promise<IdentityGenericEvent | undefined> {
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

    const tx = context.substrate.tx.identityManagement
        .verifyIdentity(context.shard, `0x${identity_ciphertext}`, `0x${validation_ciphertext}`)
    await sendTxUntilInBlock(context.substrate, tx, signer)

    if (listening) {
        const event = await listenEncryptedEvents(context, aesKey, {
            module: "identityManagement",
            method: "identityVerified",
            event: "IdentityVerified",
        });
        const [who, identity, idGraph] = event.eventData;
        return decodeIdentityEvent(context.substrate, who, identity, idGraph)
    }
    return undefined;
}

function decodeIdentityEvent(api: ApiPromise, who: HexString, identityString: HexString, idGraphString: HexString): IdentityGenericEvent {
    let identity = api.createType("LitentryIdentity", identityString).toJSON();
    let idGraph = api.createType("Vec<(LitentryIdentity, IdentityContext)>", idGraphString).toJSON();
    return <IdentityGenericEvent>{
        who,
        identity,
        idGraph,
    };
}