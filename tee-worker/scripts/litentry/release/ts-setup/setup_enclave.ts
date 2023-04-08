
const { ApiPromise, WsProvider, Keyring } = require('@polkadot/api');
const { blake2AsHex, cryptoWaitReady, xxhashAsU8a } = require('@polkadot/util-crypto');
const { hexToU8a } = require('@polkadot/util');
const fs = require('fs')
const private_account_pair = require('./private_account.json')

async function main() {
    const keyring = new Keyring({ type: 'sr25519' });

    let mrenclave;
    let enclave_account;
    try {
        const mrenclaveData = await fs.promises.readFile('mrenclave.txt', 'utf8');
        mrenclave = hexToU8a(`0x${mrenclaveData}`);
    } catch (err) {
        console.error(`Error reading mrenclave file: ${err}`);
    }
    try {
        enclave_account = await fs.promises.readFile('enclave_account.txt', 'utf8');
    } catch (err) {
        console.error(`Error reading enclave account file: ${err}`);
    }

    // Construct
    const wsProvider = new WsProvider('ws://localhost:9944');
    const api = await ApiPromise.create({ provider: wsProvider });
    await cryptoWaitReady();
    // let signAccount = keyring.addFromUri('//Alice', { name: 'Alice' });
    let signAccount = keyring.addFromJson(private_account_pair)

    // Transfer 100 LIT to
    let curNonce = (await api.rpc.system.accountNextIndex(signAccount.address)).toNumber();
    const transfer = api.tx.balances.transfer(enclave_account, '100000000000000');
    const transferResult = await transfer.signAndSend(signAccount, { curNonce });
    console.log("Transfer result:", transferResult);

    console.log((await api.query.system.account(signAccount.address)).toHuman());

    // register mrenclave
    let newNonce = curNonce + 1;
    console.log(newNonce);
    setTimeout(() => {
        api.tx.sudo.sudo(
            api.tx.teerex.updateScheduledEnclave(
                0, hexToU8a(`0x${mrenclave}`)
            )
        ).signAndSend(signAccount, { newNonce });
    }, 1000);
}
main()