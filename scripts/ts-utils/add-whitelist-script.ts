
//run:npx ts-node add-whitelist-script.ts

import { initApi } from "./index";

let whitelist = ['5EtChN6SSfL7E13fEqKQn4jZztDtZkSiRqfFTv2BKufL5fzZ'];

//100 token
const transferAmount = '100000000000000';

function transfer(api: any) {
    let txs: any = [];
    for (let index = 0; index < whitelist.length; index++) {
        let tx = api.tx.balances.transfer(whitelist[index], transferAmount);
        txs.push(tx);
    }
    const transfer_hex = api.tx.sudo.sudo(
        api.tx.utility.batchAll(txs)
    ).toHex()
    return transfer_hex
}

async function addIMPWhitelist(api: any) {
    return api.tx.sudo
        .sudo(
            api.tx.impExtrinsicWhitelist.batchAddGroupMembers(whitelist)
        ).toHex()
}

async function addVCMPWhitelist(api: any) {
    return api.tx.sudo.sudo(
        api.tx.vcmpExtrinsicWhitelist.batchAddGroupMembers(whitelist)
    ).toHex()


}

async function main() {

    const { syncApi } = await initApi();

    const transfer_hex = transfer(syncApi);
    console.log('transfer_hex', transfer_hex);

    const addIMPWhitelist_hex = await addIMPWhitelist(syncApi);
    console.log('addIMPWhitelist_hex', addIMPWhitelist_hex);

    const addVCMPWhitelist_hex = await addVCMPWhitelist(syncApi);
    console.log('addVCMPWhitelist_hex', addVCMPWhitelist_hex);

    console.log("done");
    process.exit()

}

main();
