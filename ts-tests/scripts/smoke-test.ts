import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { U8aFixed, UInt } from "@polkadot/types/codec";
import { TypeRegistry } from "@polkadot/types/create";
import { Text, U128 } from "@polkadot/types/primitive";
import * as crypto from "@polkadot/util-crypto";
// Import Web3 from 'web3';
import { testValidator } from "@polkadot/util-crypto/base32/is";
import { expect } from "chai";

// Import elliptic crypto
// import { elliptic } from 'elliptic';
var elliptic = require("elliptic");
const ec = new elliptic.ec("secp256k1");

// Import eth lib (wrapper of elliptic lib)
var Account = require("eth-lib/lib/account");
var Hash = require("eth-lib/lib/hash");

const privateKey =
  "0xe82c0c4259710bb0d6cf9f9e8d0ad73419c1278a14d375e5ca691e7618103011";

// Provider is set to localhost for development
const wsProvider = new WsProvider("ws://localhost:9944");

// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: "sr25519" });

// Configs of test ropsten account
const test_eth_address = "[0x4d88dc5d528a33e4b8be579e9476715f60060582]";

const msgPrefix: string = "Link Litentry: ";
// const msgPrefix: string = "\x19Ethereum Signed Message:\n51Link Litentry: ";

const keyringRopsten = new Keyring({ type: "ecdsa" });

// Setup the API and Alice Account
async function init() {
  console.log(
    `Initiating the API (ignore message "Unable to resolve type B..." and "Unknown types found...")`
  );

  // Initiate the polkadot API.
  const api = await ApiPromise.create({
    provider: wsProvider,
    types: {
      // mapping the actual specified address format
      Address: "AccountId",
      // mapping the lookup
      LookupSource: "AccountId",
      BlockWeights: "BlockWeights",
      Account: { nonce: "U256", balance: "U256" },
      Transaction: {
        nonce: "U256",
        action: "String",
        gas_price: "u64",
        gas_limit: "u64",
        value: "U256",
        input: "Vec<u8>",
        signature: "Signature",
      },
      Signature: { v: "u64", r: "H256", s: "H256" },
    },
  });

  console.log(`Initialization done`);
  console.log(`Genesis at block: ${api.genesisHash.toHex()}`);

  const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

  const { nonce, data: balance } = await api.query.system.account(
    alice.address
  );
  console.log(`Alice Substrate Account: ${alice.address}`);
  console.log(
    `Alice Substrate Account (nonce: ${nonce}) balance, free: ${balance.free.toHex()}`
  );

  return { api, alice };
}

// Create Ethereum Link from ALICE
async function eth_link(api: ApiPromise, alice: KeyringPair) {
  console.log(`\nStep 1: Link Ethereum account`);

  const registry = new TypeRegistry();

  // Encode prefix with concatenated utf8, instead of SCALE codec to match the litentry node
  // implementation
  let encodedPrefix = Buffer.from(msgPrefix, "utf-8");

  // let encodedAccId = registry.createType('AccountId', alice.address).toU8a();
  // console.log(encodedAccId);
  // console.log(alice.addressRaw);

  let encodedExpiredBlock = new UInt(registry, 10000, 32).toU8a();

  let encodedMsg = new Uint8Array(
    encodedPrefix.length + alice.addressRaw.length + encodedExpiredBlock.length
  );
  encodedMsg.set(encodedPrefix);
  encodedMsg.set(alice.addressRaw, encodedPrefix.length);
  encodedMsg.set(
    encodedExpiredBlock,
    encodedPrefix.length + alice.addressRaw.length
  );

  // To use manual hash and sign method, a prefix of \x19Ethereum ... is also needed to be prefixed
  // manually
  // let hash = Hash.keccak256s(encodedMsg);

  // console.log('hash is:');
  // console.log(hash);

  // TODO ECDSA keyring from polkadot crypto still not working
  // const ropstenTestAcc =
  // keyringRopsten.addFromUri('0xe82c0c4259710bb0d6cf9f9e8d0ad73419c1278a14d375e5ca691e7618103011');
  // console.log('ropsten pub key: ');
  // console.log(ropstenTestAcc.publicKey);
  // console.log(ropstenTestAcc.address);
  // console.log(crypto.keccakAsU8a(ropstenTestAcc.publicKey));
  // let signedMsg = ropstenTestAcc.sign(new Buffer(hash2.slice(2), "hex"));
  // let r = signedMsg.slice(0, 32);
  // let s = signedMsg.slice(32, 64);
  // let v = signedMsg[64];

  // var signature = Account.sign(hash, privateKey);
  // var vrs = Account.decodeSignature(signature);
  // console.log("signature is :");
  // console.log(signature);
  // console.log(keyPair.sign(new Buffer(hash2.slice(2), "hex"), { canonical: true
  // }).r.toString(16));

  // TODO Web3 could be used to replace eth-lib once ethereum prefix is implemented on
  // account-linker side
  const Web3 = require("web3");
  const web3 = new Web3();
  // Convert byte array to hex string
  let hexString = "0x" + Buffer.from(encodedMsg).toString("hex");

  let signedMsg = web3.eth.accounts.sign(hexString, privateKey);

  // This is not needed as eth-lib already does the same job
  // let keyPair = ec.keyFromPrivate(new Buffer(privateKey.slice(2), "hex"));
  // let privKey = keyPair.getPrivate("hex");
  // let pubKey = keyPair.getPublic();
  // console.log(`Private key: ${privKey}`);
  // console.log("Public key :", pubKey.encode("hex").substr(2));
  // console.log("Public key (compressed):",
  //    pubKey.encodeCompressed("hex"));
  // let signature = ec.sign(hash, privKey, "hex", {canonical: true});

  // Convert ethereum address to bytes array
  let ethAddressBytes = web3.utils.hexToBytes(
    web3.eth.accounts.privateKeyToAccount(privateKey).address
  );

  // const transaction = api.tx.accountLinkerModule.link(alice.address, 0, 10000, vrs[1], vrs[2],
  // vrs[0]);
  const transaction = api.tx.accountLinkerModule.linkEth(
    alice.address,
    0,
    ethAddressBytes,
    10000,
    signedMsg.r,
    signedMsg.s,
    signedMsg.v
  );

  const link = new Promise<{ block: string }>(async (resolve, reject) => {
    const unsub = await transaction.signAndSend(alice, (result) => {
      console.log(`Link creation is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(`Link included at blockHash ${result.status.asInBlock}`);
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        console.log(
          `Transfer finalized at blockHash ${result.status.asFinalized}`
        );
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
  const { nonce, data: balance } = await api.query.system.account(
    alice.address
  );
  console.log(
    `Alice Substrate Account (nonce: ${nonce}) balance, free: ${balance.free}`
  );

  const linkedEthAddress = await api.query.accountLinkerModule.ethereumLink(
    alice.address
  );
  console.log(
    `Linked Ethereum addresses of Alice are: ${linkedEthAddress.toString()}`
  );

  expect(linkedEthAddress.toString()).to.equal(test_eth_address);

  return;
}

// Claim Assets for Alice
async function asset_claim(api: ApiPromise, alice: KeyringPair) {
  console.log(`\nStep 3: Claim assets for Alice`);

  const transaction = await api.tx.offchainWorkerModule.assetClaim();

  const data = new Promise<{ block: string }>(async (resolve, reject) => {
    const unsub = await transaction.signAndSend(alice, (result) => {
      console.log(`Transfer is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(
          `Transfer included at blockHash ${result.status.asInBlock}`
        );
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        console.log(
          `Transfer finalized at blockHash ${result.status.asFinalized}`
        );
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
  const { nonce, data: balance } = await api.query.system.account(
    alice.address
  );
  console.log(
    `Alice Substrate Account (nonce: ${nonce}) balance, free: ${balance.free}`
  );

  const assetsBalances = await api.query.offchainWorkerModule.accountBalance(
    alice.address
  );
  console.log(
    `Linked Ethereum balances of Alice are: ${assetsBalances.toString()}`
  );

  // TODO fetch real time balance and compare it here
  expect(assetsBalances.toString()).to.equal(
    `[0,"0x00000000000000004563918244f40000"]`
  );

  return;
}

async function main() {
  const { api, alice } = await init();

  // step 1: Creating the contract from ALICE
  const link = await eth_link(api, alice);

  // step 2: Retrieving Alice's linked Ethereum accounts
  await check_linking_state(api, alice);

  // step 3: Claim assets for Alice
  await asset_claim(api, alice);

  // step 4: Retrieving assets information of Alice
  await get_assets(api, alice);
}

main()
  .catch(console.error)
  .then(() => process.exit(0));
