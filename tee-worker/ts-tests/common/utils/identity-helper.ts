import { KeyringPair } from '@polkadot/keyring/types';
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
import { assert } from 'chai';
import { ethers } from 'ethers';
import { TypeRegistry } from '@polkadot/types';
//<challeng-code> + <litentry-AccountId32> + <Identity>
export function generateVerificationMessage(
    context: IntegrationTestContext,
    challengeCode: Uint8Array,
    signerAddress: Uint8Array,
    identity: LitentryIdentity
): HexString {
    const encode = context.sidechainRegistry.createType('LitentryPrimitivesIdentity', identity).toU8a();
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
    identities: any[],
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
        const encod_identity = context.sidechainRegistry.createType('LitentryPrimitivesIdentity', identity);

        const ciphertext_identity = encryptWithTeeShieldingKey(teeShieldingKey, encod_identity.toU8a()).toString('hex');
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
    type: 'UserShieldingKeySet' | 'IdentityCreated' | 'IdentityVerified' | 'IdentityRemoved' | 'Failed'
): Promise<any[]> {
    let results: IdentityGenericEvent[] = [];

    for (let index = 0; index < events.length; index++) {
        switch (type) {
            case 'UserShieldingKeySet':
                results.push(
                    createIdentityEvent(
                        context.sidechainRegistry,
                        events[index].data.account.toHex(),
                        undefined,
                        decryptWithAES(aesKey, events[index].data.idGraph, 'hex')
                    )
                );
                break;
            case 'IdentityCreated':
                results.push(
                    createIdentityEvent(
                        context.sidechainRegistry,
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
                        context.sidechainRegistry,
                        events[index].data.account.toHex(),
                        decryptWithAES(aesKey, events[index].data.identity, 'hex'),
                        decryptWithAES(aesKey, events[index].data.idGraph, 'hex')
                    )
                );
                break;

            case 'IdentityRemoved':
                results.push(
                    createIdentityEvent(
                        context.sidechainRegistry,
                        events[index].data.account.toHex(),
                        decryptWithAES(aesKey, events[index].data.identity, 'hex')
                    )
                );
                break;
            case 'Failed':
                results.push(events[index].data.detail.toHuman());
                break;
        }
    }
    console.log(`${type} event data:`, results);

    return [...results];
}

export function createIdentityEvent(
    sidechainRegistry: TypeRegistry,
    who: HexString,
    identityString?: HexString,
    idGraphString?: HexString,
    challengeCode?: HexString
): IdentityGenericEvent {
    let identity = identityString
        ? sidechainRegistry.createType('LitentryPrimitivesIdentity', identityString).toJSON()
        : undefined;
    let idGraph = idGraphString
        ? sidechainRegistry
              .createType(
                  'Vec<(LitentryPrimitivesIdentity, PalletIdentityManagementTeeIdentityContext)>',
                  idGraphString
              )
              .toJSON()
        : undefined;
    return <IdentityGenericEvent>{
        who,
        identity,
        idGraph,
        challengeCode,
    };
}

export async function buildValidations(
    context: IntegrationTestContext,
    eventDatas: any[],
    identities: any[],
    network: 'ethereum' | 'substrate' | 'twitter',
    substrateSigners: KeyringPair[] | KeyringPair,
    ethereumSigners?: ethers.Wallet[]
): Promise<LitentryValidationData[]> {
    let signature_ethereum: HexString;
    let signature_substrate: Uint8Array;
    let verifyDatas: LitentryValidationData[] = [];

    for (let index = 0; index < eventDatas.length; index++) {
        const substrateSigner = Array.isArray(substrateSigners) ? substrateSigners[index] : substrateSigners;

        const ethereumSigner = network === 'ethereum' ? ethereumSigners![index] : undefined;

        const data = eventDatas[index];
        const msg = generateVerificationMessage(
            context,
            hexToU8a(data.challengeCode),
            substrateSigner.addressRaw,
            identities[index]
        );
        if (network === 'ethereum') {
            const ethereumValidationData: LitentryValidationData = {
                Web3Validation: {
                    Evm: {
                        message: '' as HexString,
                        signature: {
                            Ethereum: '' as HexString,
                        },
                    },
                },
            };
            console.log('post verification msg to ethereum: ', msg);
            ethereumValidationData!.Web3Validation!.Evm!.message = msg;
            const msgHash = ethers.utils.arrayify(msg);
            signature_ethereum = (await ethereumSigner!.signMessage(msgHash)) as HexString;
            console.log('signature_ethereum', ethereumSigners![index].address, signature_ethereum);

            ethereumValidationData!.Web3Validation!.Evm!.signature!.Ethereum = signature_ethereum;
            assert.isNotEmpty(data.challengeCode, 'ethereum challengeCode empty');
            console.log('ethereumValidationData', ethereumValidationData);

            verifyDatas.push(ethereumValidationData);
        } else if (network === 'substrate') {
            const substrateValidationData: LitentryValidationData = {
                Web3Validation: {
                    Substrate: {
                        message: '' as HexString,
                        signature: {
                            Sr25519: '' as HexString,
                        },
                    },
                },
            };
            console.log('post verification msg to substrate: ', msg);
            substrateValidationData!.Web3Validation!.Substrate!.message = msg;
            signature_substrate = substrateSigner.sign(msg) as Uint8Array;
            substrateValidationData!.Web3Validation!.Substrate!.signature!.Sr25519 = u8aToHex(signature_substrate);
            assert.isNotEmpty(data.challengeCode, 'substrate challengeCode empty');
            verifyDatas.push(substrateValidationData);
        } else if (network === 'twitter') {
            console.log('post verification msg to twitter', msg);
            const twitterValidationData: LitentryValidationData = {
                Web2Validation: {
                    Twitter: {
                        tweet_id: `0x${Buffer.from('100', 'utf8').toString('hex')}`,
                    },
                },
            };
            verifyDatas.push(twitterValidationData);
            assert.isNotEmpty(data.challengeCode, 'twitter challengeCode empty');
        }
    }
    return verifyDatas;
}
