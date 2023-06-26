import { KeyObject } from 'crypto';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { hexToU8a } from '@polkadot/util';

import { buildAddressHelper, initIntegrationTestContext } from './common/utils';
import {
    createSignedTrustedGetterUserShieldingKey,
    getTEEShieldingKey,
    sendRequestFromGetter,
} from './examples/direct-invocation/util'; // @fixme move to a better place
import type { IntegrationTestContext } from './common/type-definitions';

describe('Test Identity (direct invocation)', function () {
    let context: IntegrationTestContext = undefined as any;
    let teeShieldingKey: KeyObject = undefined as any;

    this.timeout(6000000);

    before(async () => {
        context = await initIntegrationTestContext(
            process.env.WORKER_END_POINT!, // @fixme evil assertion; centralize env access
            process.env.SUBSTRATE_END_POINT!, // @fixme evil assertion; centralize env access
            0
        );
        teeShieldingKey = await getTEEShieldingKey(context.tee, context.api);
    });

    it('needs a lot more work to be complete');

    step('check user sidechain storage before create', async function () {
        let alice_address = await buildAddressHelper(context.substrateWallet.alice);
        const shieldingKeyGetter = createSignedTrustedGetterUserShieldingKey(
            context.api,
            context.substrateWallet.alice,
            alice_address
        );

        const shieldingKeyGetResult = await sendRequestFromGetter(
            context.tee,
            context.api,
            context.mrEnclave,
            teeShieldingKey,
            shieldingKeyGetter
        );

        const k = context.api.createType('Option<Bytes>', hexToU8a(shieldingKeyGetResult.value.toHex()));
        assert.isTrue(k.isNone, 'shielding key should be empty before set');

        // @fixme NOT FINISHED YET
        // const twitter_identity = await buildIdentityHelper('mock_user', 'Twitter', 'Web2');
        // const identity_hex = context.api.createType('LitentryIdentity', twitter_identity).toHex();

        // const resp_challengecode = await checkUserChallengeCode(
        //     context,
        //     'IdentityManagement',
        //     'ChallengeCodes',
        //     u8aToHex(context.substrateWallet.alice.addressRaw),
        //     identity_hex
        // );

        // assert.equal(resp_challengecode, '0x', 'challengecode should be empty before create');
    });
});
