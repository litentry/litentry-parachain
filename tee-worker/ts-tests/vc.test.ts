import {
    describeLitentry,
    checkVc,
    checkErrorDetail,
    checkUserShieldingKeys,
    buildIdentityTxs,
    handleIdentityEvents,
    handleVcEvents,
} from './common/utils';
import { step } from 'mocha-steps';
import { Assertion, IndexingNetwork, TransactionSubmit } from './common/type-definitions';
import { assert } from 'chai';
import { u8aToHex } from '@polkadot/util';
import { HexString } from '@polkadot/util/types';
import { blake2AsHex } from '@polkadot/util-crypto';
import { multiAccountTxSender, sendTxsWithUtility, sendTxUntilInBlockList } from './common/transactions';

const assertion = <Assertion>{
    A1: 'A1',
    A2: ['A2'],
    A3: ['A3', 'A3', 'A3'],
    A4: '10.001',
    A7: '10.002',
    A8: [IndexingNetwork.Litentry],
    A10: '10.003',
    A11: '10.004',
};

//It doesn't make much difference test A1 only vs test A1 - A11, one VC type is enough.
//So only use A1 to trigger the wrong event
describeLitentry('VC test', 0, async (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    var indexList: HexString[] = [];
    var vcKeys: string[] = ['A1', 'A2', 'A3', 'A4', 'A7', 'A8', 'A10', 'A11'];
    step('check user sidechain storage before create', async function () {
        const resp_shieldingKey = await checkUserShieldingKeys(
            context,
            'IdentityManagement',
            'UserShieldingKeys',
            u8aToHex(context.substrateWallet.alice.addressRaw)
        );
        assert.equal(resp_shieldingKey, '0x', 'resp_shieldingKey should be empty before set');
    });
    step('set user shielding key', async function () {
        let [alice_txs] = (await buildIdentityTxs(
            context,
            [context.substrateWallet.alice],
            [],
            'setUserShieldingKey'
        )) as TransactionSubmit[];
        const resp_events = await multiAccountTxSender(
            context,
            [alice_txs],
            [context.substrateWallet.alice],
            'identityManagement',
            ['UserShieldingKeySet']
        );
        const [alice] = await handleIdentityEvents(context, aesKey, resp_events, 'UserShieldingKeySet');
        assert.equal(
            alice.who,
            u8aToHex(context.substrateWallet.alice.addressRaw),
            'alice shielding key should be set'
        );
    });

    step('check user shielding key from sidechain storage after setUserShieldingKey', async function () {
        const resp_shieldingKey = await checkUserShieldingKeys(
            context,
            'IdentityManagement',
            'UserShieldingKeys',
            u8aToHex(context.substrateWallet.alice.addressRaw)
        );
        assert.equal(resp_shieldingKey, aesKey, 'resp_shieldingKey should be equal aesKey after set');
    });
    step('Request VC', async () => {
        //request all vc
        let txs: any = [];
        for (let index = 0; index < vcKeys.length; index++) {
            const key = vcKeys[index];
            const tx = context.api.tx.vcManagement.requestVc(context.mrEnclave, {
                [key]: assertion[key as keyof Assertion],
            });
            txs.push({ tx });
        }

        const resp_events = await sendTxsWithUtility(context, context.substrateWallet.alice, txs, 'vcManagement', [
            'VCIssued',
        ]);
        const res = await handleVcEvents(aesKey, resp_events, 'VCIssued');

        for (let k = 0; k < res.length; k++) {
            const vcString = res[k].vc.replace('0x', '');
            const vcObj = JSON.parse(vcString);
            console.log('---------VC json----------\n', vcObj);

            const vcProof = vcObj.proof;

            const registry = (await context.api.query.vcManagement.vcRegistry(res[k].index)) as any;
            assert.equal(registry.toHuman()!['status'], 'Active', 'check registry error');

            const vcHash = blake2AsHex(Buffer.from(vcString));
            assert.equal(vcHash, registry.toHuman()!['hash_'], 'check vc json hash error');

            //check vc
            const vcValid = await checkVc(vcObj, res[k].index, vcProof, context.api);
            assert.equal(vcValid, true, 'check vc error');
            indexList.push(res[k].index);
        }
    });
    step('Request Error VC(A1)', async () => {
        const tx = context.api.tx.vcManagement.requestVc(context.mrEnclave, assertion.A1);
        const resp_error_events = await sendTxsWithUtility(
            context,
            context.substrateWallet.bob,
            [{ tx }] as any,
            'vcManagement',
            ['RequestVCFailed']
        );
        const error_event_datas = await handleVcEvents(aesKey, resp_error_events, 'Failed');

        await checkErrorDetail(error_event_datas, 'UserShieldingKeyNotFound', false);
    });
    step('Disable VC', async () => {
        let txs: any = [];
        for (let i = 0; i < indexList.length; i++) {
            const tx = context.api.tx.vcManagement.disableVc(indexList[i]);
            txs.push({ tx });
        }
        const resp_events = await sendTxsWithUtility(context, context.substrateWallet.alice, txs, 'vcManagement', [
            'VCDisabled',
        ]);
        const res = await handleVcEvents(aesKey, resp_events, 'VCDisabled');

        for (let k = 0; k < res.length; k++) {
            assert.equal(res[k], indexList[k], 'check index error');
            const registry = (await context.api.query.vcManagement.vcRegistry(indexList[k])) as any;
            assert.equal(registry.toHuman()!['status'], 'Disabled');
        }
    });
    step('Disable error VC(A1)', async () => {
        //Alice has already disabled the A1 VC
        const tx = context.api.tx.vcManagement.disableVc(indexList[0]);
        const nonce = (await context.api.rpc.system.accountNextIndex(context.substrateWallet.alice.address)).toNumber();

        const res = (await sendTxUntilInBlockList(
            context.api,
            [{ tx, nonce }],
            context.substrateWallet.alice
        )) as string[];

        await checkErrorDetail(res, 'vcManagement.VCAlreadyDisabled', false);
    });

    step('Revoke VC', async () => {
        let txs: any = [];
        for (let i = 0; i < indexList.length; i++) {
            const tx = context.api.tx.vcManagement.revokeVc(indexList[i]);
            txs.push({ tx });
        }
        const resp_events = await sendTxsWithUtility(context, context.substrateWallet.alice, txs, 'vcManagement', [
            'VCRevoked',
        ]);

        const res = await handleVcEvents(aesKey, resp_events, 'VCRevoked');

        for (let k = 0; k < indexList.length; k++) {
            assert.equal(res[k], indexList[k], 'check index error');
            const registry = (await context.api.query.vcManagement.vcRegistry(indexList[k])) as any;

            assert.equal(registry.toHuman(), null);
        }
    });

    step('Revoke Error VC(A1)', async () => {
        //Alice has already revoked the A1 VC
        const tx = context.api.tx.vcManagement.revokeVc(indexList[0]);
        const nonce = (await context.api.rpc.system.accountNextIndex(context.substrateWallet.alice.address)).toNumber();
        const res = (await sendTxUntilInBlockList(
            context.api,
            [{ tx, nonce }],
            context.substrateWallet.alice
        )) as string[];

        await checkErrorDetail(res, 'vcManagement.VCNotExist', false);
    });
});
