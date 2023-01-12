import { mnemonicGenerate } from '@polkadot/util-crypto';
import { Keyring } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
var fs = require('fs');
var path = require('path');
const keyring = new Keyring({ type: 'sr25519' });
import { ethers } from 'ethers';
export function generateAccouns(number: Number) {
    const singerList: {
        substrate: KeyringPair;
        ethereum: ethers.Wallet;
    }[] = [];
    for (let index = 0; index < number; index++) {
        const mnemonic = mnemonicGenerate();
        var privateKey = ethers.utils.randomBytes(32);
        var wallet = new ethers.Wallet(privateKey);

        const pair = keyring.addFromUri(mnemonic, { name: 'Address' + (index + 1) }, 'ed25519');
        singerList.push({
            substrate: pair,
            ethereum: wallet,
        });
    }
    const content = JSON.stringify(singerList);
    const file = path.join(__dirname, '../SingerList.json');
    fs.writeFile(file, content, function (err: any) {
        if (err) {
            throw new Error(err);
        }
    });
    return singerList;
}
