import { randomBytes, KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { buildIdentityFromKeypair, decryptWithAes, initIntegrationTestContext, PolkadotSigner } from './common/utils';
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
import { subscribeToEventsWithExtHash } from './common/transactions';
import { KeyringPair } from '@polkadot/keyring/types';
import { u8aToHex } from '@polkadot/util';
import { CredentialDefinition, credentialsJson } from './common/credential-json';
describe('Test Vc (direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;
    const substrateIdentities: CorePrimitivesIdentity[] = [];

    const clientDir = process.env.LITENTRY_CLI_DIR;
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
    const idIndex = process.argv.indexOf('--id');
    argvId = process.argv[idIndex + 1];
    const {
        protocol: workerProtocal,
        hostname: workerHostname,
        port: workerPort,
    } = new URL(process.env.WORKER_ENDPOINT!);
    const { protocol: nodeProtocal, hostname: nodeHostname, port: nodePort } = new URL(process.env.NODE_ENDPOINT!);

    async function linkIdentityViaCli(id: string) {
        const credentialDefinitions = credentialsJson.find((item) => item.id === id) as CredentialDefinition;
        console.log(`linking identity-${credentialDefinitions.mockDid} via cli`);

        const keyringPair = randomSubstrateWallet();
        keyringPairs.push(keyringPair);
        const formatAddress = u8aToHex(keyringPair.publicKey);

        const substrateIdentity = await buildIdentityFromKeypair(new PolkadotSigner(keyringPair), context);
        substrateIdentities.push(substrateIdentity);
        const eventsPromise = subscribeToEventsWithExtHash(reqExtHash, context);
        try {
            // CLIENT = "$CLIENT_BIN -p $NPORT -P $WORKER1PORT -u $NODEURL -U $WORKER1URL"
            const commandPromise = zx`${clientDir} -p ${nodePort} -P ${workerPort} -u ${
                nodeProtocal + nodeHostname
            } -U ${workerProtocal + workerHostname}\
                  trusted -d link-identity did:litentry:substrate:${formatAddress}\
                  did:${credentialDefinitions.mockDid}\
                  ${credentialDefinitions.mockWeb3Network}`;

            await commandPromise;
        } catch (error: any) {
            console.log(`Exit code: ${error.exitCode}`);
            console.log(`Error: ${error.stderr}`);
            throw error;
        }

        const events = (await eventsPromise).map(({ event }) => event);
        assert.equal(events.length, 1);
    }

    async function requestVc(id: string, index: number) {
        const credentialDefinitions = credentialsJson.find((item) => item.id === id) as CredentialDefinition;
        const assertion = {
            [credentialDefinitions.assertion.id]: credentialDefinitions.assertion.payload,
        };
        console.log('vc description: ', credentialDefinitions.description);

        console.log('assertion: ', assertion);

        let currentNonce = (await getSidechainNonce(context, teeShieldingKey, substrateIdentities[index])).toNumber();
        const getNextNonce = () => currentNonce++;
        const nonce = getNextNonce();

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

        const vcResults = context.api.createType('RequestVCResult', res.value);
        const decryptVcPayload = decryptWithAes(aesKey, vcResults.vc_payload, 'utf-8').replace('0x', '');
        const vcPayloadJson = JSON.parse(decryptVcPayload);
        console.log('vcPayload: ', vcPayloadJson);

        assert.equal(
            vcPayloadJson.credentialSubject.values[0],
            credentialDefinitions.expectedCredentialValue,
            "credential value doesn't match, please check the credential json expectedCredentialValue"
        );
    }

    if (argvId && credentialsJson.find((item) => item.id === argvId)) {
        step(`link identity && request vc with specific credentials for ${argvId}`, async function () {
            await linkIdentityViaCli(argvId);
            await requestVc(argvId, 0);
        });
    } else {
        credentialsJson.forEach(({ id }, index) => {
            step(`link identity && request vc with all credentials for ${id}`, async function () {
                await linkIdentityViaCli(id);
                await requestVc(id, index);
            });
        });
    }
});
