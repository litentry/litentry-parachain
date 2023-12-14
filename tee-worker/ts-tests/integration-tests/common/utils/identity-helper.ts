import { u8aToHex } from '@polkadot/util';
import { blake2AsHex } from '@polkadot/util-crypto';
import type { IntegrationTestContext } from '../common-types';
import { AesOutput } from 'parachain-api';
import { decryptWithAes, encryptWithTeeShieldingKey, Signer } from './crypto';
import { ethers } from 'ethers';
import { ECPairInterface } from 'ecpair';
import type { TypeRegistry } from '@polkadot/types';
import type { LitentryPrimitivesIdentity, PalletIdentityManagementTeeIdentityContext } from 'sidechain-api';
import type { LitentryValidationData, Web3Network } from 'parachain-api';
import type { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';
import type { KeyringPair } from '@polkadot/keyring/types';
import type { HexString } from '@polkadot/util/types';

// blake2_256(<sidechain nonce> + <primary AccountId> + <identity-to-be-linked>)
export function generateVerificationMessage(
    context: IntegrationTestContext,
    signer: LitentryPrimitivesIdentity,
    identity: LitentryPrimitivesIdentity,
    sidechainNonce: number
): HexString {
    const encodedIdentity = context.sidechainRegistry.createType('LitentryPrimitivesIdentity', identity).toU8a();
    const encodedWho = context.sidechainRegistry.createType('LitentryPrimitivesIdentity', signer).toU8a();
    const encodedSidechainNonce = context.api.createType('Index', sidechainNonce);
    const msg = Buffer.concat([encodedSidechainNonce.toU8a(), encodedWho, encodedIdentity]);
    return blake2AsHex(msg, 256);
}

export async function buildIdentityHelper(
    address: HexString | string,
    type: LitentryPrimitivesIdentity['type'],
    context: IntegrationTestContext
): Promise<LitentryPrimitivesIdentity> {
    const identity = {
        [type]: address,
    };
    return context.sidechainRegistry.createType(
        'LitentryPrimitivesIdentity',
        identity
    ) as unknown as LitentryPrimitivesIdentity;
}

export async function buildIdentityFromKeypair(
    signer: Signer,
    context: IntegrationTestContext
): Promise<LitentryPrimitivesIdentity> {
    const type: string = (() => {
        switch (signer.type()) {
            case 'ethereum':
                return 'Evm';
            case 'sr25519':
                return 'Substrate';
            case 'ed25519':
                return 'Substrate';
            case 'ecdsa':
                return 'Substrate';
            default:
                return 'Substrate';
        }
    })();

    const address = signer.getAddressRaw();
    const identity = {
        [type]: address,
    };

    return context.sidechainRegistry.createType(
        'LitentryPrimitivesIdentity',
        identity
    ) as unknown as LitentryPrimitivesIdentity;
}

// If multiple transactions are built from multiple accounts, pass the signers as an array.
// If multiple transactions are built from a single account, signers cannot be an array.
//
// TODO: enforce `validations` if method is `linkIdentity`
export async function buildIdentityTxs(
    context: IntegrationTestContext,
    signers: KeyringPair[] | KeyringPair,
    identities: LitentryPrimitivesIdentity[],
    method: 'linkIdentity' | 'deactivateIdentity' | 'activateIdentity',
    validations?: LitentryValidationData[],
    web3networks?: Web3Network[][]
): Promise<any[]> {
    const txs: {
        tx: SubmittableExtrinsic<ApiTypes>;
        nonce: number;
    }[] = [];
    const api = context.api;
    const mrEnclave = context.mrEnclave;
    const teeShieldingKey = context.teeShieldingKey;
    const len = Array.isArray(signers) ? signers.length : identities.length;
    for (let k = 0; k < len; k++) {
        const signer = Array.isArray(signers) ? signers[k] : signers;
        const identity = identities[k];
        let tx: SubmittableExtrinsic<ApiTypes>;
        const ciphertextIdentity =
            identity && encryptWithTeeShieldingKey(teeShieldingKey, identity.toU8a()).toString('hex');
        const nonce = (await api.rpc.system.accountNextIndex(signer.address)).toNumber();

        switch (method) {
            case 'linkIdentity': {
                const validation = api.createType('LitentryValidationData', validations![k]).toU8a();
                const networks = api.createType('Vec<Web3Network>', web3networks![k]).toU8a();
                const ciphertextValidation = encryptWithTeeShieldingKey(teeShieldingKey, validation).toString('hex');
                const ciphertextNetworks = encryptWithTeeShieldingKey(teeShieldingKey, networks).toString('hex');

                tx = api.tx.identityManagement.linkIdentity(
                    mrEnclave,
                    signer.address,
                    `0x${ciphertextIdentity}`,
                    `0x${ciphertextValidation}`,
                    `0x${ciphertextNetworks}`
                );
                break;
            }
            case 'deactivateIdentity': {
                tx = api.tx.identityManagement.deactivateIdentity(mrEnclave, `0x${ciphertextIdentity}`);
                break;
            }
            case 'activateIdentity': {
                tx = api.tx.identityManagement.activateIdentity(mrEnclave, `0x${ciphertextIdentity}`);
                break;
            }
            default:
                throw new Error(`Invalid method: ${method}`);
        }
        txs.push({ tx, nonce });
    }

    return txs;
}

export function parseIdGraph(
    sidechainRegistry: TypeRegistry,
    idGraphOutput: AesOutput,
    aesKey: HexString
): [LitentryPrimitivesIdentity, PalletIdentityManagementTeeIdentityContext][] {
    const decryptedIdGraph = decryptWithAes(aesKey, idGraphOutput, 'hex');
    const idGraph: [LitentryPrimitivesIdentity, PalletIdentityManagementTeeIdentityContext][] =
        sidechainRegistry.createType(
            'Vec<(LitentryPrimitivesIdentity, PalletIdentityManagementTeeIdentityContext)>',
            decryptedIdGraph
        ) as unknown as [LitentryPrimitivesIdentity, PalletIdentityManagementTeeIdentityContext][];

    return idGraph;
}

export function parseIdentity(
    sidechainRegistry: TypeRegistry,
    identityOutput: AesOutput,
    aesKey: HexString
): LitentryPrimitivesIdentity {
    const decryptedIdentity = decryptWithAes(aesKey, identityOutput, 'hex');
    const identity = sidechainRegistry.createType(
        'LitentryPrimitivesIdentity',
        decryptedIdentity
    ) as unknown as LitentryPrimitivesIdentity;
    return identity;
}

export async function buildValidations(
    context: IntegrationTestContext,
    signerIdentities: LitentryPrimitivesIdentity[],
    identities: LitentryPrimitivesIdentity[],
    startingSidechainNonce: number,
    network: 'ethereum' | 'substrate' | 'twitter' | 'bitcoin',
    substrateSigners?: KeyringPair[] | KeyringPair,
    evmSigners?: ethers.Wallet[],
    bitcoinSigners?: ECPairInterface[] | ECPairInterface
): Promise<LitentryValidationData[]> {
    let evmSignature: HexString;
    let substrateSignature: Uint8Array;
    let bitcoinSignature: Uint8Array;
    const validations: LitentryValidationData[] = [];

    for (let index = 0; index < identities.length; index++) {
        const validationNonce = startingSidechainNonce + index;

        const msg = generateVerificationMessage(context, signerIdentities[index], identities[index], validationNonce);
        if (network === 'ethereum') {
            const evmValidationData = {
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
            evmValidationData.Web3Validation.Evm.message = msg;
            const msgHash = ethers.utils.arrayify(msg);
            const evmSigner = evmSigners![index];
            evmSignature = (await evmSigner.signMessage(msgHash)) as HexString;
            console.log('evmSignature', evmSigner.address, evmSignature);

            evmValidationData!.Web3Validation.Evm.signature.Ethereum = evmSignature;
            console.log('evmValidationData', evmValidationData);
            const encodedVerifyIdentityValidation = context.api.createType(
                'LitentryValidationData',
                evmValidationData
            ) as unknown as LitentryValidationData;

            validations.push(encodedVerifyIdentityValidation);
        } else if (network === 'substrate') {
            const substrateValidationData = {
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
            substrateValidationData.Web3Validation.Substrate.message = msg;
            const substrateSigner = Array.isArray(substrateSigners!) ? substrateSigners![index] : substrateSigners!;
            substrateSignature = substrateSigner.sign(msg) as Uint8Array;
            substrateValidationData!.Web3Validation.Substrate.signature.Sr25519 = u8aToHex(substrateSignature);
            const encodedVerifyIdentityValidation: LitentryValidationData = context.api.createType(
                'LitentryValidationData',
                substrateValidationData
            ) as unknown as LitentryValidationData;
            validations.push(encodedVerifyIdentityValidation);
        } else if (network === 'bitcoin') {
            const bitcoinValidationData = {
                Web3Validation: {
                    Bitcoin: {
                        message: '' as HexString,
                        signature: {
                            Ecdsa: '' as HexString,
                        },
                    },
                },
            };
            console.log('post verification msg to bitcoin: ', msg);
            bitcoinValidationData.Web3Validation.Bitcoin.message = msg;
            const bitcoinSigner = Array.isArray(bitcoinSigners!) ? bitcoinSigners![index] : bitcoinSigners!;
            bitcoinSignature = bitcoinSigner.sign(Buffer.from(ethers.utils.arrayify(msg))) as Uint8Array;
            bitcoinValidationData!.Web3Validation.Bitcoin.signature.Ecdsa = u8aToHex(bitcoinSignature);
            const encodedVerifyIdentityValidation: LitentryValidationData = context.api.createType(
                'LitentryValidationData',
                bitcoinValidationData
            ) as unknown as LitentryValidationData;
            validations.push(encodedVerifyIdentityValidation);
        } else if (network === 'twitter') {
            console.log('post verification msg to twitter: ', msg);
            const twitterValidationData = {
                Web2Validation: {
                    Twitter: {
                        tweet_id: `0x${Buffer.from(validationNonce.toString(), 'utf8').toString('hex')}`,
                    },
                },
            };

            const encodedVerifyIdentityValidation = context.api.createType(
                'LitentryValidationData',
                twitterValidationData
            ) as unknown as LitentryValidationData;
            validations.push(encodedVerifyIdentityValidation);
        }
    }
    return validations;
}
