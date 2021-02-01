import Web3 from "web3";
import { JsonRpcResponse } from "web3-core-helpers";
import { spawn, ChildProcess } from "child_process";
import 'mocha';
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { KeyringPair } from '@polkadot/keyring/types';

export const BINARY_PATH = `../target/debug/rococo-collator`;
export const APIKEY_SERVER_PATH = `../target/debug/litentry-token-server`;
export const SPAWNING_TIME = 30000;

// OCW account
export const OCR_ACCOUNT = "5FEYX9NES9mAJt1Xg4WebmHWywxyeGQK8G3oEBXtyfZrRePX";

// Provider is set to localhost for development
const wsProvider = new WsProvider("ws://localhost:9844");

// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: 'sr25519' });

export async function launchAPITokenServer(): Promise<{ apikey_server: ChildProcess }>  {
  
  const apikey_server = spawn(APIKEY_SERVER_PATH, [], {
    env: {
      etherscan: "RF71W4Z2RDA7XQD6EN19NGB66C2QD9UPHB",
      infura: "aa0a6af5f94549928307febe80612a2a",
      blockchain: ""
    }
  });

	apikey_server.on("error", (err) => {
		if ((err as any).errno == "ENOENT") {
			console.error(
				`\x1b[31mMissing litentry-token-server binary (${APIKEY_SERVER_PATH}).\nPlease compile the litentry project:\ncargo build\x1b[0m`
			);
		} else {
			console.error(err);
		}
		process.exit(1);
  });

  apikey_server.stdout.on('data', (data) => {
    console.log('Litentry Token Server Output: ' + data.toString());
  });

  apikey_server.stderr.on('data', (data) => {
    console.log('Litentry Token Server Output: ' + data.toString());
  });

  return { apikey_server };
}

export async function launchLitentryNode(specFilename: string, provider?: string): Promise<{ binary: ChildProcess }> {

	const cmd = BINARY_PATH;
	const args = [
    `--collator`,
    `--tmp`,
    `--parachain-id`,
    `1984`,
    `--port`,
    `40333`,
    `--ws-port`,
    `9844`,
    `--alice`,
    `--`,
    `--execution`,
    `wasm`,
    `--chain`,
    `/Users/han/tmp/rococo-local.json`,
    `--port`,
    `30343`,
    `--ws-port`,
    `9977`,
  ];
	const binary = spawn(cmd, args);

	binary.on("error", (err) => {
		if ((err as any).errno == "ENOENT") {
			console.error(
				`\x1b[31mMissing litentry-node binary (${BINARY_PATH}).\nPlease compile the litentry project:\ncargo build\x1b[0m`
			);
		} else {
			console.error(err);
		}
		process.exit(1);
  });
  
  binary.stdout.on('data', (data) => {
    console.log('Litentry Node Output: ' + data.toString());
  });

  binary.stderr.on('data', (data) => {
    console.log('Litentry Node Output: ' + data.toString());
  });

//	await new Promise((resolve) => {
//		const timer = setTimeout(() => {
//			console.error(`\x1b[31m Failed to start Litentry Node.\x1b[0m`);
//			console.error(`Command: ${cmd} ${args.join(" ")}`);
//			process.exit(1);
//		}, SPAWNING_TIME - 2000);
//
//		const onData = async (chunk) => {
//			if (chunk.toString().match("Listening for new connections on 127.0.0.1:9944.")) {
//
//				clearTimeout(timer);
//				console.log(`Litentry Node Starts`);
//				resolve();
//			}
//		};
//		binary.stderr.on("data", onData);
//		binary.stdout.on("data", onData);
//	});

	return { binary };
}

export async function initApiPromise(wsProvider: WsProvider) {
	console.log(`Initiating the API (ignore message "Unable to resolve type B..." and "Unknown types found...")`);

	// Initiate the polkadot API.
	const api = await ApiPromise.create({
		provider: wsProvider,
		types: {
			// mapping the actual specified address format
			Address: "AccountId",
			// mapping the lookup
			LookupSource: "AccountId",
			Account: {
				nonce: "U256",
				balance: "U256"
			},
			Transaction: {
				nonce: "U256",
				action: "String",
				gas_price: "u64",
				gas_limit: "u64",
				value: "U256",
				input: "Vec<u8>",
				signature: "Signature"
			},
			Signature: {
				v: "u64",
				r: "H256",
				s: "H256"
			},
      BlockWeights: "u64",
      BlockLength: "u64",
		}
  });

	console.log(`Initialization done`);
	console.log(`Genesis at block: ${api.genesisHash.toHex()}`);

  // Get keyring of Alice
	const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });

  // Insert ocw session key
  const resInsertKey = api.rpc.author.insertKey(
    "ocw!",
    "loop high amazing chat tennis auto denial attend type quit liquid tonight",
    "0x8c35b97c56099cf3b5c631d1f296abbb11289857e74a8f60936290080d56da6d"
  );

	const { nonce, data: balance } = await api.query.system.account(alice.address);
	console.log(`Alice Substrate Account: ${alice.address}`);
	console.log(`Alice Substrate Account (nonce: ${nonce}) balance, free: ${balance.free.toHex()}`);

	return { api, alice };
}

async function sendTokenToOcw(api: ApiPromise, alice: KeyringPair) {
  // Transfer tokens from Alice to ocw account
  console.log(`Transfer tokens from Alice to ocw account`);
  return new Promise<{ block: string }>(async (resolve, reject) => {
    const unsub = await api.tx.balances
      .transfer(OCR_ACCOUNT, 1000000000000000)
      .signAndSend(alice, (result) => {
        console.log(`Current status is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(
            `Transaction included at blockHash ${result.status.asInBlock}`
          );
        } else if (result.status.isFinalized) {
          console.log(
            `Transaction finalized at blockHash ${result.status.asFinalized}`
          );
          unsub();
          resolve({
            block: result.status.asFinalized.toString(),
          });
        }
      });
  });
}


export function describeLitentry(title: string, specFilename: string, cb: (context: {api: ApiPromise, alice: KeyringPair}) => void, provider?: string) {
	describe(title, function() {
    // Set timeout to 120 seconds
    this.timeout(120000);

    let tokenServer: ChildProcess;
    let binary: ChildProcess;
    let context: {api: ApiPromise, alice: KeyringPair} = { api:  {} as ApiPromise, alice: {} as KeyringPair};
		// Making sure the Litentry node has started
		before("Starting Litentry Test Node", async function () {
      //this.timeout(SPAWNING_TIME);
      const initTokenServer = await launchAPITokenServer();
      const initNode = await launchLitentryNode(specFilename, provider);
      tokenServer = initTokenServer.apikey_server;
      binary = initNode.binary;
      const initApi = await initApiPromise(wsProvider);
      context.api = initApi.api;
      context.alice = initApi.alice;
      //return sendTokenToOcw(initApi.api, initApi.alice);
		});

		after(async function () {
      //console.log(`\x1b[31m Killing RPC\x1b[0m`);
      tokenServer.kill()
      binary.kill();
      context.api.disconnect();
    });
    
    cb(context);
	});
}