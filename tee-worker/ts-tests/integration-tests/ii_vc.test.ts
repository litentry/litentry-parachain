import { describeLitentry, handleVcEvents } from './common/utils';
import { step } from 'mocha-steps';
import type { HexString } from '@polkadot/util/types';
import { assert } from 'chai';
import { sendTxsWithUtility, sendTxUntilInBlockList } from './common/transactions';
import { ApiTypes, SubmittableExtrinsic } from '@polkadot/api/types';

// TODO: keep the list short, the manual types will be solved in #1878
//       more extensive testing of assertion will be done in #1873
const allAssertions = [
    {
        A1: 'A1',
    },
];

// It doesn't make much difference test A1 only vs test A1 - A11, one VC type is enough.
// So only use A1 to trigger the wrong event
describeLitentry('VC ii test', async (context) => {
    const indexList: HexString[] = [];
    step('Request VC', async () => {
        // request all vc
        const txs: {
            tx: SubmittableExtrinsic<ApiTypes>;
        }[] = [];

        allAssertions.forEach((assertion) => {
            const tx = context.api.tx.vcManagement.requestVc(context.mrEnclave, assertion);
            txs.push({ tx });
        });

        const events = await sendTxsWithUtility(
            context,
            context.substrateWallet.alice,
            txs,
            'vcManagement',
            ['VCIssued'],
            30
        );
        const res = await handleVcEvents(events, 'VCIssued');

        for (let k = 0; k < res.length; k++) {
            const registry = (await context.api.query.vcManagement.vcRegistry(res[k].index)) as any;
            assert.equal(registry.toHuman()!['status'], 'Active', 'check registry error');
            indexList.push(res[k].index);
        }
    });
    step('Disable VC', async () => {
        const txs: {
            tx: SubmittableExtrinsic<ApiTypes>;
        }[] = [];
        for (let i = 0; i < indexList.length; i++) {
            const tx = context.api.tx.vcManagement.disableVc(indexList[i]);
            txs.push({ tx });
        }
        const events = await sendTxsWithUtility(context, context.substrateWallet.alice, txs, 'vcManagement', [
            'VCDisabled',
        ]);
        const res = await handleVcEvents(events, 'VCDisabled');

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
        const txs: {
            tx: SubmittableExtrinsic<ApiTypes>;
        }[] = [];

        for (let i = 0; i < indexList.length; i++) {
            const tx = context.api.tx.vcManagement.revokeVc(indexList[i]);
            txs.push({ tx });
        }
        const events = await sendTxsWithUtility(context, context.substrateWallet.alice, txs, 'vcManagement', [
            'VCRevoked',
        ]);

        const res = await handleVcEvents(events, 'VCRevoked');

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
