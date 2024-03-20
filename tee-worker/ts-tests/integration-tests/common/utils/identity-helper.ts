import { u8aToHex } from '@polkadot/util';
import { blake2AsHex } from '@polkadot/util-crypto';
import type { IntegrationTestContext } from '../common-types';
import { AesOutput } from 'parachain-api';
import { decryptWithAes, Signer } from './crypto';
import { ethers } from 'ethers';
import type { TypeRegistry } from '@polkadot/types';
import type { PalletIdentityManagementTeeIdentityContext } from 'sidechain-api';
import type { LitentryValidationData, CorePrimitivesIdentity } from 'parachain-api';
import type { HexString } from '@polkadot/util/types';

// blake2_256(<sidechain nonce> + <primary AccountId> + <identity-to-be-linked>)
export function generateVerificationMessage(
    context: IntegrationTestContext,
    signer: CorePrimitivesIdentity,
    identity: CorePrimitivesIdentity,
    sidechainNonce: number
): HexString {
    const encodedIdentity = context.api.createType('CorePrimitivesIdentity', identity).toU8a();
    const encodedWho = context.api.createType('CorePrimitivesIdentity', signer).toU8a();
    const encodedSidechainNonce = context.api.createType('Index', sidechainNonce);
    const msg = Buffer.concat([encodedSidechainNonce.toU8a(), encodedWho, encodedIdentity]);
    return blake2AsHex(msg, 256);
}

export async function buildIdentityHelper(
    address: HexString | string,
    type: CorePrimitivesIdentity['type'],
    context: IntegrationTestContext
): Promise<CorePrimitivesIdentity> {
    const identity = {
        [type]: address,
    };
    return context.api.createType('CorePrimitivesIdentity', identity);
}

export function parseIdGraph(
    sidechainRegistry: TypeRegistry,
    idGraphOutput: AesOutput,
    aesKey: HexString
): [CorePrimitivesIdentity, PalletIdentityManagementTeeIdentityContext][] {
    const decryptedIdGraph = decryptWithAes(aesKey, idGraphOutput, 'hex');
    const idGraph: [CorePrimitivesIdentity, PalletIdentityManagementTeeIdentityContext][] =
        sidechainRegistry.createType(
            'Vec<(CorePrimitivesIdentity, PalletIdentityManagementTeeIdentityContext)>',
            decryptedIdGraph
        );

    return idGraph;
}

export async function buildValidations(
    context: IntegrationTestContext,
    signerIdentitity: CorePrimitivesIdentity,
    linkIdentity: CorePrimitivesIdentity,
    startingSidechainNonce: number,
    network: 'ethereum' | 'substrate' | 'twitter' | 'bitcoin' | 'bitcoinPrettified',
    signer?: Signer
): Promise<LitentryValidationData> {
    let evmSignature: HexString;
    let substrateSignature: Uint8Array;
    let bitcoinSignature: Uint8Array;
    let validation = {} as LitentryValidationData;
    const validationNonce = startingSidechainNonce++;

    const msg = generateVerificationMessage(context, signerIdentitity, linkIdentity, validationNonce);
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
        evmSignature = u8aToHex(await signer!.sign(msgHash));
        console.log('evmSignature', u8aToHex(signer!.getAddressRaw()), evmSignature);

        evmValidationData!.Web3Validation.Evm.signature.Ethereum = evmSignature;
        console.log('evmValidationData', evmValidationData);
        const encodedVerifyIdentityValidation = context.api.createType('LitentryValidationData', evmValidationData);
        validation = encodedVerifyIdentityValidation;
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
        substrateSignature = await signer!.sign(msg);
        substrateValidationData!.Web3Validation.Substrate.signature.Sr25519 = u8aToHex(substrateSignature);
        const encodedVerifyIdentityValidation: LitentryValidationData = context.api.createType(
            'LitentryValidationData',
            substrateValidationData
        );
        validation = encodedVerifyIdentityValidation;
    } else if (network === 'bitcoin') {
        const bitcoinValidationData = {
            Web3Validation: {
                Bitcoin: {
                    message: '' as HexString,
                    signature: {
                        Bitcoin: '' as HexString,
                    },
                },
            },
        };
        console.log('post verification msg to bitcoin: ', msg);
        bitcoinValidationData.Web3Validation.Bitcoin.message = msg;
        // we need to sign the hex string without `0x` prefix, the signature is base64-encoded string
        bitcoinSignature = await signer!.sign(msg.substring(2));
        bitcoinValidationData!.Web3Validation.Bitcoin.signature.Bitcoin = u8aToHex(bitcoinSignature);
        console.log('bitcoin pubkey: ', u8aToHex(signer!.getAddressRaw()));
        console.log('bitcoin sig (base64): ', Buffer.from(bitcoinSignature).toString('base64'));
        console.log('bitcoin sig (hex): ', u8aToHex(bitcoinSignature));
        const encodedVerifyIdentityValidation: LitentryValidationData = context.api.createType(
            'LitentryValidationData',
            bitcoinValidationData
        );
        validation = encodedVerifyIdentityValidation;
    } else if (network === 'bitcoinPrettified') {
        const bitcoinValidationData = {
            Web3Validation: {
                Bitcoin: {
                    message: '' as HexString,
                    signature: {
                        BitcoinPrettified: '' as HexString,
                    },
                },
            },
        };
        console.log('post verification msg to bitcoin: ', msg);
        bitcoinValidationData.Web3Validation.Bitcoin.message = msg;
        bitcoinSignature = await signer!.sign('Litentry authorization token: ' + msg);

        bitcoinValidationData!.Web3Validation.Bitcoin.signature.BitcoinPrettified = u8aToHex(bitcoinSignature);
        console.log('bitcoin pubkey: ', u8aToHex(signer!.getAddressRaw()));
        console.log('bitcoin sig (base64): ', Buffer.from(bitcoinSignature).toString('base64'));

        console.log('bitcoin sig (hex): ', u8aToHex(bitcoinSignature));
        const encodedVerifyIdentityValidation: LitentryValidationData = context.api.createType(
            'LitentryValidationData',
            bitcoinValidationData
        );

        validation = encodedVerifyIdentityValidation;
    } else if (network === 'twitter') {
        console.log('post verification msg to twitter: ', msg);
        const twitterValidationData = {
            Web2Validation: {
                Twitter: {
                    tweet_id: `0x${Buffer.from(validationNonce.toString(), 'utf8').toString('hex')}`,
                },
            },
        };

        const encodedVerifyIdentityValidation = context.api.createType('LitentryValidationData', twitterValidationData);
        validation = encodedVerifyIdentityValidation;
    }

    return validation;
}
