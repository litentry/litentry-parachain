import { ApiPromise, WsProvider, Keyring } from "@polkadot/api";
import { KeyringPair } from '@polkadot/keyring/types';
import { UInt } from '@polkadot/types/codec';
import { TypeRegistry } from "@polkadot/types/create";
// Import Web3 from 'web3';
import { expect } from "chai";
import { step } from "mocha-steps";
import { OCR_ACCOUNT, describeLitentry } from "./utils"

const privateKey = '0xe82c0c4259710bb0d6cf9f9e8d0ad73419c1278a14d375e5ca691e7618103011';

// Provider is set to localhost for development
const wsProvider = new WsProvider("ws://localhost:9944");

// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: 'sr25519' });

// Configs of test ropsten account
const testEthAddress = "[0x4d88dc5d528a33e4b8be579e9476715f60060582]";

const msgPrefix: string = "Link Litentry: ";

// Create Ethereum Link from ALICE
async function eth_link(api: ApiPromise, alice: KeyringPair) {

  console.log(`\nStep 1: Link Ethereum account`);

  const registry = new TypeRegistry();

  // Encode prefix with concatenated utf8, instead of SCALE codec to match the litentry node implementation
	let encodedPrefix = Buffer.from(msgPrefix, 'utf-8');
  
  let encodedExpiredBlock = new UInt(registry, 10000, 32).toU8a();

  let encodedMsg = new Uint8Array(encodedPrefix.length + alice.addressRaw.length + encodedExpiredBlock.length);
  encodedMsg.set(encodedPrefix);
  encodedMsg.set(alice.addressRaw, encodedPrefix.length);
  encodedMsg.set(encodedExpiredBlock, encodedPrefix.length + alice.addressRaw.length);

  // Web3 is used to sign the message with ethereum prefix ("\x19Ethereum ...")
  const Web3 = require("web3");
  const web3 = new Web3();
   // Convert byte array to hex string
  let hexString = "0x" + Buffer.from(encodedMsg).toString('hex');

  let signedMsg = web3.eth.accounts.sign(hexString, privateKey);
  
  // Convert ethereum address to bytes array
  let ethAddressBytes = web3.utils.hexToBytes(web3.eth.accounts.privateKeyToAccount(privateKey).address);

  console.log(`r is ${signedMsg.r}`);
  console.log(`s is ${signedMsg.s}`);
  console.log(`v is ${signedMsg.v}`);

  const transaction = api.tx.accountLinkerModule.linkEth(alice.address, 0, ethAddressBytes, 10000, signedMsg.r, signedMsg.s, signedMsg.v);

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
async function check_linking_state(api: ApiPromise, alice: KeyringPair) {

	console.log(`\nStep 2: Retrieving linking state of Alice `);

	// Retrieve Alice account with new nonce value
	const { nonce, data: balance } = await api.query.system.account(alice.address);
	console.log(`Alice Substrate Account (nonce: ${nonce}) balance, free: ${balance.free}`);

	const linkedEthAddress = (await api.query.accountLinkerModule.ethereumLink(alice.address));
  console.log(`Linked Ethereum addresses of Alice are: ${linkedEthAddress.toString()}`);

	return linkedEthAddress;
}


// Claim Assets for Alice
async function asset_claim(api: ApiPromise, alice: KeyringPair) {

	console.log(`\nStep 3: Claim assets for Alice`);

	const transaction = await api.tx.offchainWorkerModule.assetClaim();

	const data = new Promise<{ block: string }>(async (resolve, reject) => {
		const unsub = await transaction.signAndSend(alice, (result) => {
			console.log(`Transfer is ${result.status}`);
			if (result.status.isInBlock) {
				console.log(`Transfer included at blockHash ${result.status.asInBlock}`);
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
	return data;
}

// Retrieve assets balances of Alice
async function get_assets(api: ApiPromise, alice: KeyringPair) {

	console.log(`\nStep 4: Retrieving assets of Alice`);

	// Retrieve Alice account with new nonce value
	const { nonce, data: balance } = await api.query.system.account(alice.address);
	console.log(`Alice Substrate Account (nonce: ${nonce}) balance, free: ${balance.free}`);

	const assetsBalances = (await api.query.offchainWorkerModule.accountBalance(alice.address));
  console.log(`Linked Ethereum balances of Alice are: ${assetsBalances.toString()}`);
  
	return assetsBalances;

}

describeLitentry("Test Ethereum Link and Balance Fetch", ``, (context) =>{

  step("Create Ethereum Link", async function () {
    await eth_link(context.api, context.alice);
  })

  step("Retrieving Alice's linked Ethereum accounts", async function () {
    const ethAddr = await check_linking_state(context.api, context.alice);
  
    expect(ethAddr.toString()).to.equal(testEthAddress);
  })

  step("Claim assets for Alice", async function () {
    await asset_claim(context.api, context.alice);
  })

  step("Retrieving assets information of Alice", async function () {
    // Retrieve ocw account balance
    const { nonce: old_n, data: old_balance } = await context.api.query.system.account(OCR_ACCOUNT);

    // Wait for 36s ~ 6 blocks
    await new Promise(r => setTimeout(r, 36000));
    const balances = await get_assets(context.api, context.alice);
    // TODO fetch real time balance and compare it here
    expect(balances.toString()).to.equal(`[null,"0x00000000000000004563918244f40000"]`);

    // Retrieve ocw account balance
    const { nonce: new_n, data: balance } = await context.api.query.system.account(OCR_ACCOUNT);
    console.log(`new is ${balance.free.toString()}  old is ${old_balance.free}`);
    console.log(`difference is ${(Number(balance.free.toString()) - Number(old_balance.free.toString())).toString()}`);
  })

});
