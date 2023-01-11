import { describeLitentry } from './utils';
import { step } from 'mocha-steps';
import { requestVC } from './indirect_calls';

describeLitentry('VC test', async (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';

    step('Request VC', async () => {
        console.log(222333, context.substrate.tx.vcManagement);

        await requestVC(context, context.defaultSigner, aesKey, true, context.shard, 'A1');
        // const res =
    });
});
