import { describeLitentry } from './common/utils';
import { step } from 'mocha-steps';
import { assert } from 'chai';
import { sendTxsWithUtility } from './common/transactions';
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
        assert.equal(events.length, 1);
    });
});
