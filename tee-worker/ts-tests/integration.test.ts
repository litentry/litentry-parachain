import './config';
import { describe } from 'mocha';
import { generateAccouns } from './utils/crypto';
import { describeIntegration } from './utils';
import { step } from 'mocha-steps';
import { setUserShieldingKey } from './indirect_calls';
import { assert } from 'chai';
import { hexToU8a, u8aConcat, u8aToHex, u8aToU8a, stringToU8a } from '@polkadot/util';

describeIntegration('test Integration', (context) => {
    const aesKey = '0x22fc82db5b606998ad45099b7978b5b4f9dd4ea6017e57370ac56141caaabd12';
    step('log context', () => {
        console.log(context.signerList.length);
    });

    step('set user shielding key', async function () {
        for (let index = 0; index < context.signerList.length; index++) {
            const who = await setUserShieldingKey(context, context.signerList[index].substrate, aesKey, true);
            assert.equal(who, u8aToHex(context.defaultSigner.addressRaw), 'check caller error');
        }
    });
});
