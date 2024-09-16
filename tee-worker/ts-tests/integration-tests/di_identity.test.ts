import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import {
    buildValidations,
    initIntegrationTestContext,
    assertIdGraphMutationResult,
    assertIdGraphHash,
    sleep,
    EthersSigner,
    PolkadotSigner,
    Signer,
} from './common/utils';
import { assertIsInSidechainBlock } from './common/utils/assertion';
import {
    createSignedTrustedCallLinkIdentity,
    createSignedTrustedGetterIdGraph,
    createSignedTrustedCallDeactivateIdentity,
    createSignedTrustedCallActivateIdentity,
    decodeIdGraph,
    getSidechainNonce,
    getTeeShieldingKey,
    sendRsaRequestFromGetter,
    sendRequestFromTrustedCall,
    getIdGraphHash,
} from './common/di-utils'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/common-types';
import { aesKey } from './common/call';
import type { LitentryValidationData, Web3Network, CorePrimitivesIdentity } from 'parachain-api';
import { Vec, Bytes } from '@polkadot/types';
import type { HexString } from '@polkadot/util/types';
import { buildEvmRandomValidation, buildSubstrateRandomValidation } from './common/utils/identity-builder';
     const networkConfigs = [
         {
             networks: ['Polkadot', 'Litentry'],
             buildValidation: buildSubstrateRandomValidation,
         },
         {
             networks: ['Ethereum', 'Bsc'],
             buildValidation: buildEvmRandomValidation,
         },
     ];
        async function testIdentity(
            identityType: string,
            signerType: string,
            networksConfig: any,
            context: IntegrationTestContext,
            aliceSigner: any,
            teeShieldingKey: KeyObject,
            aesKey: string
        ) {
            const currentNonce = (await getSidechainNonce(context, aliceSigner[identityType].identity)).toNumber();

            const idGraphHashResults = [];
            const expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [];

            const linkIdentityRequestParams = await Promise.all(
                networksConfig.map(async (config: any, index: any) => {
                    const nonce = currentNonce + index;
                    const networks = context.api.createType('Vec<Web3Network>', config.networks);
                    const validation = await config.buildValidation(context, aliceSigner[identityType].identity, nonce);
                    expectedIdGraphs.push([[validation.identity, true]]);
                    return {
                        nonce,
                        identity: validation.identity,
                        validation: validation.validation,
                        networks,
                    };
                })
            );

            let counter = 0;
            for (const [index, { nonce, identity, validation, networks }] of linkIdentityRequestParams.entries()) {
                counter++;
                const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
                const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
                    context.api,
                    context.mrEnclave,
                    context.api.createType('Index', nonce),
                    aliceSigner[signerType].wallet,
                    aliceSigner[identityType].identity,
                    identity.toHex(),
                    validation.toHex(),
                    networks.toHex(),
                    context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                    requestIdentifier,
                    {
                        withWrappedBytes: false,
                        withPrefix: counter % 2 === 0, // alternate per entry
                    }
                );

                const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);
                idGraphHashResults.push(
                    await assertIdGraphMutationResult(
                        context,
                        teeShieldingKey,
                        aliceSigner[identityType].identity, // 根据 identityType 选择 identity
                        res,
                        'LinkIdentityResult',
                        expectedIdGraphs[index]
                    )
                );

                await assertIsInSidechainBlock('linkIdentityCall', res);
            }

            console.log(idGraphHashResults.length);
        }
describe('Test Identity', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let bobSolanaIdentity: CorePrimitivesIdentity;
    let aliceSigner: any;

    // Alice links:
    // - alice's solana identity
    // - eve's substrate identity (as alice can't link her own substrate again)
    const linkIdentityRequestParams: {
        nonce: number;
        identity: CorePrimitivesIdentity;
        validation: LitentryValidationData;
        networks: Bytes | Vec<Web3Network>;
    }[] = [];
    this.timeout(6000000);

    before(async () => {
        context = await initIntegrationTestContext(
            process.env.PARACHAIN_ENDPOINT! // @fixme evil assertion; centralize env access
        );
        teeShieldingKey = await getTeeShieldingKey(context);
        // currentNonce = (await getSidechainNonce(context, aliceSolanaIdentity)).toNumber();

        aliceSigner = {
            evm: {
                identity: await context.web3Wallets.evm.Alice.getIdentity(context),
                wallet: context.web3Wallets.evm.Alice,
            },
            substrate: {
                identity: await context.web3Wallets.substrate.Alice.getIdentity(context),
                wallet: context.web3Wallets.substrate.Alice,
            },
        };
    });
    // step('check idGraph from sidechain storage before linking', async function () {
    //     const testArray = ['evm', 'substrate'];
    //     for (const networkType of testArray) {
    //         try {

    //             const identity = aliceSigner[networkType].identity;

    //             const wallet = aliceSigner[networkType].wallet;

    //             const idGraphGetter = await createSignedTrustedGetterIdGraph(context.api, wallet, identity);
    //             console.log(`IdGraphGetter created for ${networkType}`);

    //             const res = await sendRsaRequestFromGetter(context, teeShieldingKey, idGraphGetter);

    //             const idGraph = decodeIdGraph(context.sidechainRegistry, res.value);
    //             console.log(`IdGraph length for ${networkType}:`, idGraph.length);

    //             assert.lengthOf(idGraph, 0, `${networkType}idGrap length should be 0`);

    //         } catch (error) {
    //             console.error(`Error processing ${networkType}:`, error);
    //             throw error;
    //         }
    //     }
    // });

    // step('Testing substrate identity', async function () {
    //     const currentNonce = (await getSidechainNonce(context, aliceSigner.substrate.identity)).toNumber();

    //     const networkConfigs = [
    //         {
    //             networks: ['Polkadot', 'Litentry'],
    //             buildValidation: buildSubstrateRandomValidation,
    //         },
    //         {
    //             networks: ['Ethereum', 'Bsc'],
    //             buildValidation: buildEvmRandomValidation,
    //         },
    //     ];
    //     const idGraphHashResults: HexString[] = [];
    //     const expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [];
    //     const linkIdentityRequestParams = await Promise.all(
    //         networkConfigs.map(async (config, index) => {
    //             const nonce = currentNonce + index;
    //             const networks = context.api.createType('Vec<Web3Network>', config.networks);
    //             const validation = await config.buildValidation(context, aliceSigner.substrate.identity, nonce);
    //             expectedIdGraphs.push([[validation.identity, true]]);
    //             return {
    //                 nonce,
    //                 identity: validation.identity,
    //                 validation: validation.validation,
    //                 networks,
    //             };
    //         })
    //     );

    //     let counter = 0;
    //     for (const [index, { nonce, identity, validation, networks }] of linkIdentityRequestParams.entries()) {
    //         counter++;
    //         const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
    //         const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
    //             context.api,
    //             context.mrEnclave,
    //             context.api.createType('Index', nonce),
    //             context.web3Wallets.substrate.Alice,
    //             aliceSigner.substrate.identity,

    //             identity.toHex(),
    //             validation.toHex(),
    //             networks.toHex(),
    //             context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
    //             requestIdentifier,
    //             {
    //                 withWrappedBytes: false,
    //                 withPrefix: counter % 2 === 0, // alternate per entry
    //             }
    //         );

    //         const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);
    //         idGraphHashResults.push(
    //             await assertIdGraphMutationResult(
    //                 context,
    //                 teeShieldingKey,
    //                 aliceSigner.substrate.identity,
    //                 res,
    //                 'LinkIdentityResult',
    //                 expectedIdGraphs[index]
    //             )
    //         );
    //         await assertIsInSidechainBlock('linkIdentityCall', res);
    //     }

    //     console.log(idGraphHashResults.length);
    // });
    // step('Testing evm identity', async function () {
    //     const currentNonce = (await getSidechainNonce(context, aliceSigner.evm.identity)).toNumber();

    //     const networkConfigs = [
    //         {
    //             networks: ['Polkadot', 'Litentry'],
    //             buildValidation: buildSubstrateRandomValidation,
    //         },
    //         {
    //             networks: ['Ethereum', 'Bsc'],
    //             buildValidation: buildEvmRandomValidation,
    //         },
    //     ];
    //     const idGraphHashResults: HexString[] = [];
    //     const expectedIdGraphs: [CorePrimitivesIdentity, boolean][][] = [];
    //     const linkIdentityRequestParams = await Promise.all(
    //         networkConfigs.map(async (config, index) => {
    //             const nonce = currentNonce + index;
    //             const networks = context.api.createType('Vec<Web3Network>', config.networks);
    //             const validation = await config.buildValidation(context, aliceSigner.evm.identity, nonce);
    //             expectedIdGraphs.push([[validation.identity, true]]);
    //             return {
    //                 nonce,
    //                 identity: validation.identity,
    //                 validation: validation.validation,
    //                 networks,
    //             };
    //         })
    //     );
    //     let counter = 0;
    //     for (const [index, { nonce, identity, validation, networks }] of linkIdentityRequestParams.entries()) {
    //         counter++;
    //         const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
    //         const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
    //             context.api,
    //             context.mrEnclave,
    //             context.api.createType('Index', nonce),
    //             context.web3Wallets.evm.Alice,
    //             aliceSigner.evm.identity,
    //             identity.toHex(),
    //             validation.toHex(),
    //             networks.toHex(),
    //             context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
    //             requestIdentifier,
    //             {
    //                 withWrappedBytes: false,
    //                 withPrefix: counter % 2 === 0, // alternate per entry
    //             }
    //         );

    //         const res = await sendRequestFromTrustedCall(context, teeShieldingKey, linkIdentityCall);
    //         idGraphHashResults.push(
    //             await assertIdGraphMutationResult(
    //                 context,
    //                 teeShieldingKey,
    //                 aliceSigner.evm.identity,
    //                 res,
    //                 'LinkIdentityResult',
    //                 expectedIdGraphs[index]
    //             )
    //         );
    //         await assertIsInSidechainBlock('linkIdentityCall', res);
    //     }
    // });


    step('Testing substrate identity', async function () {
        await testIdentity('substrate', 'substrate', networkConfigs, context, aliceSigner, teeShieldingKey, aesKey);
    });

    step('Testing evm identity', async function () {
        await testIdentity('evm', 'evm', networkConfigs, context, aliceSigner, teeShieldingKey, aesKey);
    });
});
