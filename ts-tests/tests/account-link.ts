import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import { KeyringPair } from '@polkadot/keyring/types';
import { UInt } from '@polkadot/types/codec';
import { TypeRegistry } from '@polkadot/types/create';

// Create Ethereum Link from ALICE
export async function ethLink(api: ApiPromise, alice: KeyringPair, privateKey: string) {
    console.log(`\nStep 1: Link Ethereum account`);

    const registry = new TypeRegistry();

    // Encode prefix with concatenated utf8, instead of SCALE codec to match the litentry node
    // implementation
    const msgPrefix: string = 'Link Litentry: ';
    let encodedPrefix = Buffer.from(msgPrefix, 'utf-8');

    let encodedExpiredBlock = new UInt(registry, 10000, 32).toU8a();

    let encodedMsg = new Uint8Array(encodedPrefix.length + alice.addressRaw.length + encodedExpiredBlock.length);
    encodedMsg.set(encodedPrefix);
    encodedMsg.set(alice.addressRaw, encodedPrefix.length);
    encodedMsg.set(encodedExpiredBlock, encodedPrefix.length + alice.addressRaw.length);

    // Web3 is used to sign the message with ethereum prefix ("\x19Ethereum ...")
    const Web3 = require('web3');
    const web3 = new Web3();
    // Convert byte array to hex string
    let hexString = '0x' + Buffer.from(encodedMsg).toString('hex');

    let signedMsg = web3.eth.accounts.sign(hexString, privateKey);

    // Convert ethereum address to bytes array
    let ethAddressBytes = web3.utils.hexToBytes(web3.eth.accounts.privateKeyToAccount(privateKey).address);

    console.log(`r is ${signedMsg.r}`);
    console.log(`s is ${signedMsg.s}`);
    console.log(`v is ${signedMsg.v}`);

    //let sig = { r: signedMsg.r, s: signedMsg.s, v: signedMsg.v };

    const transaction = api.tx.accountLinkerModule.linkEth(
        alice.address,
        0,
        ethAddressBytes,
        10000,
        signedMsg.signature
    );

    const link = new Promise<{ block: string }>(async (resolve, reject) => {
        const unsub = await transaction.signAndSend(alice, (result) => {
            console.log(`Link creation is ${result.status}`);
            if (result.status.isInBlock) {
                console.log(`Link included at blockHash ${result.status.asInBlock}`);
                console.log(`Waiting for finalization... (can take a minute)`);
            } else if (result.status.isFinalized) {
                console.log(`Transfer finalized at blockHash ${result.status.asFinalized}`);
                unsub();
                resolve({
                    block: result.status.asFinalized.toString(),
                });
            }
        });
    });
    return link;
}

// Retrieve Alice & Link Storage
export async function checkLinkingState(api: ApiPromise, alice: KeyringPair) {
    console.log(`\nStep 2: Retrieving linking state of Alice `);

    // Retrieve Alice account with new nonce value
    const { nonce, data: balance } = await api.query.system.account(alice.address);
    console.log(`Alice Substrate Account (nonce: ${nonce}) balance, free: ${balance.free}`);

    const linkedEthAddress = await api.query.accountLinkerModule.ethereumLink(alice.address);
    console.log(`Linked Ethereum addresses of Alice are: ${linkedEthAddress.toString()}`);

    return linkedEthAddress;
}
