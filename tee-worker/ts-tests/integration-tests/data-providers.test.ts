import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import {
    buildIdentityFromKeypair,
    decryptWithAes,
    initIntegrationTestContext,
    PolkadotSigner,
    sleep,
} from './common/utils';
import { randomSubstrateWallet } from './common/helpers';
import { assertIsInSidechainBlock } from './common/utils/assertion';
import {
    getSidechainNonce,
    getTeeShieldingKey,
    sendRequestFromTrustedCall,
    createSignedTrustedCallRequestVc,
} from './common/di-utils'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/common-types';
import { CorePrimitivesIdentity, RequestVCResult } from 'parachain-api';
import { aesKey } from './common/call';
import { $ as zx } from 'zx';
import { credentialDefinitionMap } from './common/credential-definitions';
import { subscribeToEventsWithExtHash } from './common/transactions';
import { KeyringPair } from '@polkadot/keyring/types';
import { u8aToHex } from '@polkadot/util';

describe('Test Vc (direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    const substrateIdentities: CorePrimitivesIdentity[] = [];

    const client = process.env.BINARY_DIR + '/litentry-cli';

    const reqExtHash = '0x0000000000000000000000000000000000000000000000000000000000000000';
    const keyringPairs: KeyringPair[] = [];
    let argvId = '';
    this.timeout(6000000);
    before(async () => {
        context = await initIntegrationTestContext(process.env.WORKER_ENDPOINT!, process.env.NODE_ENDPOINT!);
        teeShieldingKey = await getTeeShieldingKey(context);
    });

    // usage example:
    // `pnpm run test-data-providers:local --id=vip3-membership-card-gold` for single test
    // `pnpm run test-data-providers:local` for all tests
    const argv = process.argv.indexOf('--id');
    argvId = process.argv[argv + 1];

    async function linkIdentityViaCli(id: string) {
        const keyringPair = randomSubstrateWallet();
        keyringPairs.push(keyringPair);
        const formatAddress = u8aToHex(keyringPair.publicKey);

        const substrateIdentity = (await buildIdentityFromKeypair(
            new PolkadotSigner(keyringPair),
            context
        )) as CorePrimitivesIdentity;
        substrateIdentities.push(substrateIdentity);
        const eventsPromise = subscribeToEventsWithExtHash(reqExtHash, context);
        try {
            await zx`${client} trusted -d link-identity did:litentry:substrate:${formatAddress}\
                  did:${credentialDefinitionMap[id].mockDid}\
                  ${credentialDefinitionMap[id].mockWeb3Network}`;
        } catch (error: any) {
            console.log(`Exit code: ${error.exitCode}`);
            console.log(`Error: ${error.stderr}`);
            throw error;
        }

        const events = (await eventsPromise).map(({ event }) => event);
        assert.equal(events.length, 1);
    }

    async function requestVc(id: string, index: number) {
        const assertion = {
            [credentialDefinitionMap[id].assertion.id]: credentialDefinitionMap[id].assertion.payload,
        };

        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, substrateIdentities[index])).toNumber();
        const getNextNonce = () => currentNonce++;
        const nonce = getNextNonce();
        console.log(nonce, substrateIdentities[index].toHuman(), u8aToHex(keyringPairs[index].publicKey));

        const requestIdentifier = `0x${randomBytes(32).toString('hex')}`;
        const requestVcCall = await createSignedTrustedCallRequestVc(
            context.api,
            context.mrEnclave,
            context.api.createType('Index', nonce),
            new PolkadotSigner(keyringPairs[index]),
            substrateIdentities[index],
            context.api.createType('Assertion', assertion).toHex(),
            context.api.createType('Option<RequestAesKey>', aesKey).toHex(),
            requestIdentifier
        );
        const res = await sendRequestFromTrustedCall(context, teeShieldingKey, requestVcCall);
        await assertIsInSidechainBlock(`${Object.keys(assertion)[0]} requestVcCall`, res);

        const vcResults = context.api.createType('RequestVCResult', res.value) as unknown as RequestVCResult;
        const decryptVcPayload = decryptWithAes(aesKey, vcResults.vc_payload, 'utf-8').replace('0x', '');
        const vcPayloadJson = JSON.parse(decryptVcPayload);

        assert.equal(vcPayloadJson.credentialSubject.values[0], credentialDefinitionMap[id].expectedCredentialValue);
    }

    // eslint-disable-next-line no-prototype-builtins
    if (argvId && credentialDefinitionMap.hasOwnProperty(argvId)) {
        step(
            `linking identity::${credentialDefinitionMap[argvId].mockDid} via cli and request vc::${credentialDefinitionMap[argvId].id}`,
            async function () {
                await linkIdentityViaCli(argvId);
                await requestVc(argvId, 0);
            }
        );
    } else {
        Object.keys(credentialDefinitionMap).forEach((id, index) => {
            step(
                `linking identity::${credentialDefinitionMap[id].mockDid} via cli and request vc::${credentialDefinitionMap[id].id}`,
                async function () {
                    await linkIdentityViaCli(id);
                    await requestVc(id, index);
                }
            );
        });
    }
});
