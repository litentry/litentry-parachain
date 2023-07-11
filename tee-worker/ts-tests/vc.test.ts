import {
    describeLitentry,
    checkVc,
    checkErrorDetail,
    checkUserShieldingKeys,
    buildIdentityTxs,
    handleIdentityEvents,
    handleVcEvents,
    buildIdentityFromKeypair,
} from './common/utils';
import { step } from 'mocha-steps';
import type { Assertion, IdentityGenericEvent, TransactionSubmit } from './common/type-definitions';
import type { HexString } from '@polkadot/util/types';
import { assert } from 'chai';
import { u8aToHex } from '@polkadot/util';
import { blake2AsHex } from '@polkadot/util-crypto';
import { multiAccountTxSender, sendTxsWithUtility, sendTxUntilInBlockList } from './common/transactions';
import { aesKey } from './common/call';

const allAssertions: Assertion = {
    A1: 'A1',
    A2: ['A2'],
    A3: ['A3', 'A3', 'A3'],
    A4: '10.001',
    A7: '10.002',
    A8: ['Litentry'],
    A10: '10.003',
    A11: '10.004',
};
const assertionA1: Assertion = {
    A1: 'A1',
};
// It doesn't make much difference test A1 only vs test A1 - A11, one VC type is enough.
// So only use A1 to trigger the wrong event
describeLitentry('VC test', 0, async (context) => {
    const indexList: HexString[] = [];
    const vcKeys: string[] = ['A1', 'A2', 'A3', 'A4', 'A7', 'A8', 'A10', 'A11'];
    let aliceSubject = await buildIdentityFromKeypair(context.substrateWallet.alice, context);
    // step('check user sidechain storage before create', async function () {
    //     const shieldingKey = await checkUserShieldingKeys(
    //         context,
    //         'IdentityManagement',
    //         'UserShieldingKeys',
    //         aliceAddress
    //     );
    //     assert.equal(shieldingKey, '0x', 'shieldingKey should be empty before set');
    // });
    step('set user shielding key', async function () {
        const [aliceTxs] = (await buildIdentityTxs(
            context,
            [context.substrateWallet.alice],
            [],
            'setUserShieldingKey'
        )) as TransactionSubmit[];
        const events = await multiAccountTxSender(
            context,
            [aliceTxs],
            [context.substrateWallet.alice],
            'identityManagement',
            ['UserShieldingKeySet']
        );
        const [alice] = (await handleIdentityEvents(
            context,
            aesKey,
            events,
            'UserShieldingKeySet'
        )) as IdentityGenericEvent[];
        assert.equal(
            alice.who,
            u8aToHex(context.substrateWallet.alice.addressRaw),
            'alice shielding key should be set'
        );
    });
    step('check user shielding key from sidechain storage after setUserShieldingKey', async function () {
        const shieldingKey = await checkUserShieldingKeys(
            context,
            'IdentityManagement',
            'UserShieldingKeys',
            aliceSubject
        );
        assert.equal(shieldingKey, aesKey, 'shieldingKey should be equal aesKey after set');
    });
    step('Request VC', async () => {
        // request all vc
        const txs: any = [];
        for (let index = 0; index < vcKeys.length; index++) {
            const key = vcKeys[index];
            const tx = context.api.tx.vcManagement.requestVc(context.mrEnclave, {
                [key]: allAssertions[key as keyof Assertion],
            });
            txs.push({ tx });
        }

        const events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            txs,
            'vcManagement',
            ['VCIssued'],
            30
        );
        const res = await handleVcEvents(aesKey, events, 'VCIssued');

        for (let k = 0; k < res.length; k++) {
            const vcString = res[k].vc.replace('0x', '');
            const vcObj = JSON.parse(vcString);
            console.log('---------VC json----------\n', vcObj);

            const vcProof = vcObj.proof;

            const registry = (await context.api.query.vcManagement.vcRegistry(res[k].index)) as any;
            assert.equal(registry.toHuman()!['status'], 'Active', 'check registry error');

            const vcHash = blake2AsHex(Buffer.from(vcString));
            assert.equal(vcHash, registry.toHuman()!['hash_'], 'check vc json hash error');

            // check vc
            const vcValid = await checkVc(vcObj, res[k].index, vcProof, context.api);
            assert.equal(vcValid, true, 'check vc error');
            indexList.push(res[k].index);
        }
    });
    step('Request Error VC(A1)', async () => {
        const tx = context.api.tx.vcManagement.requestVc(context.mrEnclave, assertionA1);
        const errorEvents = await sendTxsWithUtility(
            context,
            context.substrateWallet.bob,
            [{ tx }] as any,
            'vcManagement',
            ['RequestVCFailed']
        );

        await checkErrorDetail(errorEvents, 'UserShieldingKeyNotFound');
    });
    step('Disable VC', async () => {
        const txs: any = [];
        for (let i = 0; i < indexList.length; i++) {
            const tx = context.api.tx.vcManagement.disableVc(indexList[i]);
            txs.push({ tx });
        }
        const events = await sendTxsWithUtility(context, context.substrateWallet.alice, txs, 'vcManagement', [
            'VCDisabled',
        ]);
        const res = await handleVcEvents(aesKey, events, 'VCDisabled');

        for (let k = 0; k < res.length; k++) {
            assert.equal(res[k], indexList[k], 'check index error');
            const registry = (await context.api.query.vcManagement.vcRegistry(indexList[k])) as any;
            assert.equal(registry.toHuman()!['status'], 'Disabled');
        }
    });
    step('Disable error VC(A1)', async () => {
        // Alice has already disabled the A1 VC
        const tx = context.api.tx.vcManagement.disableVc(indexList[0]);
        const nonce = (await context.api.rpc.system.accountNextIndex(context.substrateWallet.alice.address)).toNumber();

        const [error] = await sendTxUntilInBlockList(context.api, [{ tx, nonce }], context.substrateWallet.alice);

        assert.equal(
            error,
            'vcManagement.VCAlreadyDisabled',
            'check disable vc error: error should be equal to vcManagement.VCAlreadyDisabled'
        );
    });

    step('Revoke VC', async () => {
        const txs: any = [];
        for (let i = 0; i < indexList.length; i++) {
            const tx = context.api.tx.vcManagement.revokeVc(indexList[i]);
            txs.push({ tx });
        }
        const events = await sendTxsWithUtility(context, context.substrateWallet.alice, txs, 'vcManagement', [
            'VCRevoked',
        ]);

        const res = await handleVcEvents(aesKey, events, 'VCRevoked');

        for (let k = 0; k < indexList.length; k++) {
            assert.equal(res[k], indexList[k], 'check index error');
            const registry = (await context.api.query.vcManagement.vcRegistry(indexList[k])) as any;

            assert.equal(registry.toHuman(), null);
        }
    });

    step('Revoke Error VC(A1)', async () => {
        // Alice has already revoked the A1 VC
        const tx = context.api.tx.vcManagement.revokeVc(indexList[0]);
        const nonce = (await context.api.rpc.system.accountNextIndex(context.substrateWallet.alice.address)).toNumber();
        const [error] = await sendTxUntilInBlockList(context.api, [{ tx, nonce }], context.substrateWallet.alice);

        assert.equal(
            error,
            'vcManagement.VCNotExist',
            'check revoke vc error: error should be equal to vcManagement.VCNotExist'
        );
    });
});
