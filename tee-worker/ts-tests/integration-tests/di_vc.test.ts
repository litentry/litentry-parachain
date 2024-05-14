import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { buildWeb2Validation, initIntegrationTestContext } from './common/utils';
import { assertIsInSidechainBlock, assertVc } from './common/utils/assertion';
import {
    getSidechainNonce,
    createSignedTrustedCallLinkIdentity,
    getTeeShieldingKey,
    sendRequestFromTrustedCall,
    createSignedTrustedCallRequestVc,
    createSignedTrustedCall,
} from './common/di-utils'; // @fixme move to a better place
import { buildIdentityHelper, buildValidations } from './common/utils';
import type { IntegrationTestContext } from './common/common-types';
import { aesKey } from './common/call';
import type { CorePrimitivesIdentity } from 'parachain-api';
import { mockAssertions } from './common/utils/vc-helper';
import type { LitentryValidationData, Web3Network } from 'parachain-api';
import type { Vec, Bytes } from '@polkadot/types';
import { assert } from 'chai';

describe('Test Vc (direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    let aliceSubstrateIdentity: CorePrimitivesIdentity = undefined as any;

    // Alice links:
    // - a `mock_user` twitter
    // - alice's evm identity
    // - alice's bitcoin identity]
    //
    // We need this linking to not have empty eligible identities for any vc request
    const linkIdentityRequestParams: {
        nonce: number;
        identity: CorePrimitivesIdentity;
        validation: LitentryValidationData;
        networks: Bytes | Vec<Web3Network>;
    }[] = [];
    this.timeout(6000000);

    before(async () => {
        context = await initIntegrationTestContext(
            process.env.WORKER_ENDPOINT!, // @fixme evil assertion; centralize env access
            process.env.NODE_ENDPOINT! // @fixme evil assertion; centralize env access
        );
        teeShieldingKey = await getTeeShieldingKey(context);
        aliceSubstrateIdentity = await context.web3Wallets.substrate.Alice.getIdentity(context);
    });

    step('linking identities (alice)', async function () {
        let currentNonce = (await getSidechainNonce(context, aliceSubstrateIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;

        const twitterNonce = getNextNonce();
        const twitterIdentity = await buildIdentityHelper('mock_user', 'Twitter', context);
        const twitterValidation = await buildWeb2Validation({
            identityType: 'Twitter',
            context,
            signerIdentitity: aliceSubstrateIdentity,
            linkIdentity: twitterIdentity,
            verificationType: 'PublicTweet',
            validationNonce: twitterNonce,
        });
        const twitterNetworks = context.api.createType('Vec<Web3Network>', []);
        linkIdentityRequestParams.push({
            nonce: twitterNonce,
            identity: twitterIdentity,
            validation: twitterValidation,
            networks: twitterNetworks,
        });

        const evmNonce = getNextNonce();

        const evmIdentity = await context.web3Wallets.evm.Alice.getIdentity(context);
        const evmValidation = await buildValidations(
            context,
            aliceSubstrateIdentity,
            evmIdentity,
            evmNonce,
            'ethereum',
            context.web3Wallets.evm.Alice
        );
        const evmNetworks = context.api.createType('Vec<Web3Network>', ['Ethereum', 'Bsc']);
        linkIdentityRequestParams.push({
            nonce: evmNonce,
            identity: evmIdentity,
            validation: evmValidation,
            networks: evmNetworks,
        });

        const bitcoinNonce = getNextNonce();

        const bitcoinIdentity = await context.web3Wallets.bitcoin.Alice.getIdentity(context);
        console.log('bitcoin id: ', bitcoinIdentity.toHuman());
        const bitcoinValidation = await buildValidations(
            context,
            aliceSubstrateIdentity,
            bitcoinIdentity,
            bitcoinNonce,
            'bitcoin',
            context.web3Wallets.bitcoin.Alice
        );
        const bitcoinNetworks = context.api.createType('Vec<Web3Network>', ['BitcoinP2tr']);
        linkIdentityRequestParams.push({
            nonce: bitcoinNonce,
            identity: bitcoinIdentity,
            validation: bitcoinValidation,
            networks: bitcoinNetworks,
        });

        let counter = 0;
        for (const { nonce, identity, validation, networks } of linkIdentityRequestParams) {
            counter++;
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            const linkIdentityCall = await createSignedTrustedCallLinkIdentity(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                context.web3Wallets.substrate.Alice,
                aliceSubstrateIdentity,
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
            await assertIsInSidechainBlock('linkIdentityCall', res);
        }
    });

    mockAssertions.forEach(({ description, assertion }, index) => {
        step(`request vc payload : ${JSON.stringify(assertion)} (alice)`, async function () {
            let currentNonce = (await getSidechainNonce(context, aliceSubstrateIdentity)).toNumber();
            const getNextNonce = () => currentNonce++;
            const nonce = getNextNonce();
            const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
            console.log(`request vc ${Object.keys(assertion)[0]} for Alice ... Assertion description: ${description}`);

            const requestVcCall = await createSignedTrustedCallRequestVc(
                context.api,
                context.mrEnclave,
                context.api.createType('Index', nonce),
                context.web3Wallets.substrate.Alice,
                aliceSubstrateIdentity,
                context.api.createType('Assertion', assertion).toHex(),
                context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
                requestIdentifier,
                {
                    withWrappedBytes: false,
                    withPrefix: index % 2 === 0, // alternate per entry
                }
            );

            const res = await sendRequestFromTrustedCall(context, teeShieldingKey, requestVcCall);
            await assertIsInSidechainBlock(`${Object.keys(assertion)[0]} requestVcCall`, res);

            await assertVc(context, aliceSubstrateIdentity, res.value);
        });
    });

    step('request invalid vc with different primeIdentities', async function () {
        let currentNonce = (await getSidechainNonce(context, aliceSubstrateIdentity)).toNumber();
        const getNextNonce = () => currentNonce++;
        const nonce = getNextNonce();
        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const bobSubstrateIdentity = await context.web3Wallets.substrate.Bob.getIdentity(context);
        const eveSubstrateIdentity = await context.web3Wallets.substrate.Eve.getIdentity(context);
        const call = await createSignedTrustedCall(
            context.api,
            ['request_vc', '(LitentryIdentity, LitentryIdentity, Assertion, Option<RequestAesKey>, H256)'],
            context.web3Wallets.substrate.Alice,
            context.mrEnclave,
            context.api.createType('Index', nonce),
            [
                eveSubstrateIdentity.toHuman(),
                bobSubstrateIdentity.toHuman(),
                context.api.createType('Assertion', { A1: 'A1' }).toHex(),
                aesKey,
                requestIdentifier,
            ]
        );

        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, call);
        console.log('requestInvalidVc call returned', res.toHuman());
        assert.isTrue(res.status.isTrustedOperationStatus && res.status.asTrustedOperationStatus[0].isInvalid);
    });
});
