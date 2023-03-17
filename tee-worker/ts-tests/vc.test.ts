import { describeLitentry, checkVc, checkFailReason } from './utils';
import { step } from 'mocha-steps';
import { setUserShieldingKey, requestVCs, disableVCs, revokeVCs } from './indirect_calls';
import { Assertion } from './type-definitions';
import { assert } from 'chai';
import { u8aToHex } from '@polkadot/util';
import { HexString } from '@polkadot/util/types';
import { blake2AsHex } from '@polkadot/util-crypto';
import { requestErrorVCs, disableErrorVCs, revokeErrorVCs } from './indirect_error_calls';
import { Event } from '@polkadot/types/interfaces';

const assertion = <Assertion>{
    A1: 'A1',
    A2: ['A2'],
    A3: ['A3', 'A3', 'A3'],
    A4: [10],
    A7: [10],
    A8: ['litentry'],
    A10: [10],
    A11: [10],
};

//It doesn't make much difference test A1 only vs test A1 - A11, one VC type is enough.
//So only use A1 to trigger the wrong event
describeLitentry('VC test', async (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    var indexList: HexString[] = [];
    step('set user shielding key', async function () {
        const who = await setUserShieldingKey(context, context.defaultSigner[0], aesKey, true);
        assert.equal(who, u8aToHex(context.defaultSigner[0].addressRaw), 'check caller error');
    });
    step('Request VC', async () => {
        //request all vc
        const res = (await requestVCs(
            context,
            context.defaultSigner[0],
            aesKey,
            true,
            context.mrEnclave,
            assertion
        )) as {
            account: HexString;
            index: HexString;
            vc: HexString;
        }[];

        for (let k = 0; k < res.length; k++) {
            const vcString = res[k].vc.replace('0x', '');
            const vcObj = JSON.parse(vcString);
            console.log('---------VC json----------', vcObj);

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
        const resp_request_error = (await requestErrorVCs(
            context,
            context.defaultSigner[1],
            aesKey,
            true,
            context.mrEnclave,
            assertion,
            ['A1']
        )) as Event[];

        await checkFailReason(resp_request_error, 'User shielding key is missing', true);
    });
    step('Disable VC', async () => {
        const res = (await disableVCs(context, context.defaultSigner[0], aesKey, true, indexList)) as HexString[];
        for (let k = 0; k < res.length; k++) {
            assert.equal(res[k], indexList[k], 'check index error');
            const registry = (await context.api.query.vcManagement.vcRegistry(indexList[k])) as any;
            assert.equal(registry.toHuman()!['status'], 'Disabled');
        }
    });
    step('Disable error VC(A1)', async () => {
        //Alice has already disabled the A1 VC
        const resp_disable_error = (await disableErrorVCs(
            context,
            context.defaultSigner[0],
            true,

            [indexList[0]]
        )) as HexString[];
        await checkFailReason(resp_disable_error, 'vcManagement.VCAlreadyDisabled', false);
    });

    step('Revoke VC', async () => {
        const res = (await revokeVCs(context, context.defaultSigner[0], aesKey, true, indexList)) as HexString[];
        for (let k = 0; k < res.length; k++) {
            assert.equal(res[k], indexList[k], 'check index error');
            const registry = (await context.api.query.vcManagement.vcRegistry(indexList[k])) as any;
            assert.equal(registry.toHuman(), null);
        }
    });

    step('Revoke Error VC(A1)', async () => {
        //Alice has already revoked the A1 VC
        const resp_revoke_error = (await revokeErrorVCs(context, context.defaultSigner[0], true, [
            indexList[0],
        ])) as string[];
        await checkFailReason(resp_revoke_error, 'vcManagement.VCNotExist', false);
    });
});
