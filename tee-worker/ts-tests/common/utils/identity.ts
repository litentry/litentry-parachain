import { ApiPromise } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { Codec } from '@polkadot/types/types';
import { HexString } from '@polkadot/util/types';
import { hexToU8a, u8aToHex } from '@polkadot/util';
import { blake2AsHex } from '@polkadot/util-crypto';
import {
    EvmNetwork,
    IdentityGenericEvent,
    IntegrationTestContext,
    LitentryIdentity,
    LitentryValidationData,
    SubstrateNetwork,
    Web2Network,
} from '../type-definitions';
import { decryptWithAES, encryptWithTeeShieldingKey } from './crypto';
import { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';

const base58 = require('micro-base58');

export async function createTrustedCallSigned(
    api: ApiPromise,
    trustedCall: [string, string],
    account: KeyringPair,
    mrenclave: string,
    mrEnclave: string,
    nonce: Codec,
    params: Array<any>
) {
    const [variant, argType] = trustedCall;
    const call = api.createType('TrustedCall', {
        [variant]: api.createType(argType, params),
    });
    const payload = Uint8Array.from([
        ...call.toU8a(),
        ...nonce.toU8a(),
        ...base58.decode(mrenclave),
        ...hexToU8a(mrEnclave),
    ]);
    const signature = api.createType('MultiSignature', {
        Sr25519: u8aToHex(account.sign(payload)),
    });
    return api.createType('TrustedCallSigned', {
        call: call,
        index: nonce,
        signature: signature,
    });
}

//<challeng-code> + <litentry-AccountId32> + <Identity>
export function generateVerificationMessage(
    context: IntegrationTestContext,
    challengeCode: Uint8Array,
    signerAddress: Uint8Array,
    identity: LitentryIdentity
): HexString {
    const encode = context.api.createType('LitentryIdentity', identity).toU8a();
    const msg = Buffer.concat([challengeCode, signerAddress, encode]);
    return blake2AsHex(msg, 256);
}

export async function buildIdentityHelper(
    address: HexString | string,
    network: SubstrateNetwork | EvmNetwork | Web2Network,
    type: 'Evm' | 'Substrate' | 'Web2'
): Promise<LitentryIdentity> {
    const identity: LitentryIdentity = {
        [type]: {
            address,
            network,
        },
    };
    return identity;
}

//If multiple transactions are built from multiple accounts, pass the signers as an array. If multiple transactions are built from a single account, signers cannot be an array.
export async function buildIdentityTxs(
    context: IntegrationTestContext,
    signers: KeyringPair[] | KeyringPair,
    identities: LitentryIdentity[],
    method: 'setUserShieldingKey' | 'createIdentity' | 'verifyIdentity' | 'removeIdentity',
    validations?: LitentryValidationData[]
): Promise<any[]> {
    const txs: any[] = [];
    const api = context.api;
    const mrEnclave = context.mrEnclave;
    const teeShieldingKey = context.teeShieldingKey;
    const len = Array.isArray(signers) ? signers.length : identities.length;
    for (let k = 0; k < len; k++) {
        const signer = Array.isArray(signers) ? signers[k] : signers;
        const identity = identities[k];
        let tx: SubmittableExtrinsic<ApiTypes>;
        let nonce: number;
        const encod_identity = api.createType('LitentryIdentity', identity).toU8a();
        const ciphertext_identity = encryptWithTeeShieldingKey(teeShieldingKey, encod_identity).toString('hex');
        nonce = (await api.rpc.system.accountNextIndex(signer.address)).toNumber();

        switch (method) {
            case 'setUserShieldingKey':
                const ciphertext = encryptWithTeeShieldingKey(
                    context.teeShieldingKey,
                    hexToU8a('0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12')
                ).toString('hex');
                tx = context.api.tx.identityManagement.setUserShieldingKey(context.mrEnclave, `0x${ciphertext}`);
                break;
            case 'createIdentity':
                tx = api.tx.identityManagement.createIdentity(
                    mrEnclave,
                    signer.address,
                    `0x${ciphertext_identity}`,
                    null
                );
                break;
            case 'verifyIdentity':
                const data = validations![k];
                const encode_verifyIdentity_validation = api.createType('LitentryValidationData', data).toU8a();
                const ciphertext_verifyIdentity_validation = encryptWithTeeShieldingKey(
                    teeShieldingKey,
                    encode_verifyIdentity_validation
                ).toString('hex');
                tx = api.tx.identityManagement.verifyIdentity(
                    mrEnclave,
                    `0x${ciphertext_identity}`,
                    `0x${ciphertext_verifyIdentity_validation}`
                );
                break;
            case 'removeIdentity':
                tx = api.tx.identityManagement.removeIdentity(mrEnclave, `0x${ciphertext_identity}`);
                break;
            default:
                throw new Error(`Invalid method: ${method}`);
        }
        txs.push({ tx, nonce });
    }

    return txs;
}

export async function handleIdentityEvents(
    context: IntegrationTestContext,
    aesKey: HexString,
    events: any[],
    type:
        | 'UserShieldingKeySet'
        | 'IdentityCreated'
        | 'IdentityVerified'
        | 'IdentityRemoved'
        | 'Failed'
        | 'CreateIdentityFailed'
): Promise<any[]> {
    let results: IdentityGenericEvent[] = [];

    for (let index = 0; index < events.length; index++) {
        switch (type) {
            case 'UserShieldingKeySet':
                results.push(
                    createIdentityEvent(
                        context.api,
                        events[index].data.account.toHex(),
                        undefined,
                        decryptWithAES(aesKey, events[index].data.idGraph, 'hex')
                    )
                );
                break;
            case 'IdentityCreated':
                results.push(
                    createIdentityEvent(
                        context.api,
                        events[index].data.account.toHex(),
                        decryptWithAES(aesKey, events[index].data.identity, 'hex'),
                        undefined,
                        decryptWithAES(aesKey, events[index].data.code, 'hex')
                    )
                );
                break;
            case 'IdentityVerified':
                results.push(
                    createIdentityEvent(
                        context.api,
                        events[index].data.account.toHex(),
                        decryptWithAES(aesKey, events[index].data.identity, 'hex'),
                        decryptWithAES(aesKey, events[index].data.idGraph, 'hex')
                    )
                );
                break;

            case 'IdentityRemoved':
                results.push(
                    createIdentityEvent(
                        context.api,
                        events[index].data.account.toHex(),
                        decryptWithAES(aesKey, events[index].data.identity, 'hex')
                    )
                );
                break;
            case 'Failed':
            case 'CreateIdentityFailed':
                results.push(events[index].data.detail.toHuman());
                break;
        }
    }
    console.log(`${type} event data:`, results);

    return [...results];
}

export function createIdentityEvent(
    api: ApiPromise,
    who: HexString,
    identityString?: HexString,
    idGraphString?: HexString,
    challengeCode?: HexString
): IdentityGenericEvent {
    let identity = identityString ? api.createType('LitentryIdentity', identityString).toJSON() : undefined;
    let idGraph = idGraphString
        ? api.createType('Vec<(LitentryIdentity, IdentityContext)>', idGraphString).toJSON()
        : undefined;
    return <IdentityGenericEvent>{
        who,
        identity,
        idGraph,
        challengeCode,
    };
}
