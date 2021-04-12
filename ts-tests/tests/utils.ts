import 'mocha';

import {ApiPromise, Keyring, WsProvider} from '@polkadot/api';
import {KeyringPair} from '@polkadot/keyring/types';
import {Bytes} from '@polkadot/types';
import {TypeRegistry} from '@polkadot/types/create';
import {ChildProcess, spawn, exec} from 'child_process';
import fs from 'fs';
import Web3 from 'web3';
import {JsonRpcResponse} from 'web3-core-helpers';

export const LITENTRY_BINARY_PATH = `../target/release/litentry-collator`;
export const POLKADOT_BINARY_PATH = `../polkadot/target/release/polkadot`;
export const APIKEY_SERVER_PATH = `../token-server/target/release/litentry-token-server`;
export const PARA_GENESIS_HEAD_PATH = `para-1984-genesis`;
export const PARA_WASM_PATH = `para-1984-wasm`;
export const ROCOCO_LOCAL_PATH = `./rococo-local-cfde-real-overseer.json`;
export const RELAY_NODE_SCRIPT = `./scripts/start-alice-and-bob.sh`;
export const SPAWNING_TIME = 30000;

// OCW account
export const OCR_ACCOUNT = '5FEYX9NES9mAJt1Xg4WebmHWywxyeGQK8G3oEBXtyfZrRePX';

// Provider is set to localhost for development
const wsProvider = new WsProvider('ws://localhost:9844');

// Keyring needed to sign using Alice account
const keyring = new Keyring({type: 'sr25519'});

export async function launchAPITokenServer(): Promise<{apikey_server: ChildProcess}>
{
  const apikey_server = spawn(APIKEY_SERVER_PATH, [], {
    env: {
      etherscan: 'RF71W4Z2RDA7XQD6EN19NGB66C2QD9UPHB',
      infura: 'aa0a6af5f94549928307febe80612a2a',
      blockchain: ''
    }
  });

  apikey_server.on('error', (err) => {
    if ((err as any).errno == 'ENOENT')
    {
      console.error(`\x1b[31mMissing litentry-token-server binary (${
          APIKEY_SERVER_PATH}).\nPlease compile the litentry project:\ncargo build\x1b[0m`);
    }
    else
    {
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

  return {apikey_server};
}

export async function launchRelayNodesAndParachainRegister()
{
  const cmd = POLKADOT_BINARY_PATH;
  const aliceArgs = [
    `--chain`,
    ROCOCO_LOCAL_PATH,
    `--tmp`,
    `--port`,
    `30333`,
    `--ws-port`,
    `9944`,
    `--alice`,
  ];
  const bobArgs = [
    `--chain`,
    ROCOCO_LOCAL_PATH,
    `--tmp`,
    `--port`,
    `30334`,
    `--ws-port`,
    `9955`,
    `--bob`,
  ];

  // const aliceLaunch = spawn(cmd, aliceArgs);
  // const bobLaunch = spawn(cmd, bobArgs);

  // aliceLaunch.on("error", (err) => {
  //	if ((err as any).errno == "ENOENT") {
  //		console.error(
  //			`\x1b[31mMissing relay node binary (${POLKADOT_BINARY_PATH}).\nPlease
  // compile the polkadot project!`
  //		);
  //	} else {
  //		console.error(err);
  //	}
  //	process.exit(1);
  // });
  // bobLaunch.on("error", (err) => {
  //	if ((err as any).errno == "ENOENT") {
  //		console.error(
  //			`\x1b[31mMissing relay node binary (${POLKADOT_BINARY_PATH}).\nPlease
  // compile the polkadot project!`
  //		);
  //	} else {
  //		console.error(err);
  //	}
  //	process.exit(1);
  // });

  //  aliceLaunch.stdout.on('data', (data) => {
  //    console.log('Relay Node Output: ' + data.toString());
  //  });
  //
  //  aliceLaunch.stderr.on('data', (data) => {
  //    console.log('Relay Node Output: ' + data.toString());
  //  });

  const api = await ApiPromise.create({
    provider: new WsProvider('ws://localhost:9944'),
    types: {
      // mapping the actual specified address format
      Address: 'MultiAddress',
      // mapping the lookup
      LookupSource: 'MultiAddress',
     // BeefyNextAuthoritySet: {
     //   id: 'u64',
     //   len: 'u32',
     //   root: 'H256'
     // },
     // ValidatorSetId: 'u64',
     // ParaLifecycle: {
     //   _enum: ['Onboarding', 'Parathread', 'Parachain', 'UpgradingToParachain', 'DowngradingToParathread', 'OutgoingParathread', 'OutgoingParachain']
     // },
     // ParachainsInherentData: {
     //   bitfields: 'SignedAvailabilityBitfields',
     //   backedCandidates: 'Vec<BackedCandidate>',
     //   disputes: 'MultiDisputeStatementSet',
     //   parentHeader: 'Header'
     // },
    }
  });
  // Get keyring of Alice
  const alice = keyring.addFromUri('//Alice', {name: 'Alice default'});
  // Get keyring of Sudo
  const sudoKey = await api.query.sudo.key();

  const sudo = keyring.addFromAddress(sudoKey);

  // const sudoPair = keyring.getPair(sudoKey);

  const registry = new TypeRegistry();

  const genesisHeadBytes = fs.readFileSync(PARA_GENESIS_HEAD_PATH, 'utf8');

  const validationCodeBytes = fs.readFileSync(PARA_WASM_PATH, 'utf8');

  const tx = api.tx.sudo.sudo(api.tx.parasSudoWrapper.sudoScheduleParaInitialize(1984, {
    genesisHead: new Bytes(registry, genesisHeadBytes),
    validationCode: new Bytes(registry, validationCodeBytes),
    parachain: true,
  }));
  console.log(`Parachain registration tx Sent!`);

  // await new Promise(r => setTimeout(r, 6000));
  const parachainRegister = new Promise<{block: string}>(async (resolve, reject) => {
    const unsub = await tx.signAndSend(alice, (result) => {
      console.log(`Parachain registration is ${result.status}`);
      if (result.status.isInBlock)
      {
        console.log(`Parachain registration included at blockHash ${result.status.asInBlock}`);
        console.log(`Waiting for finalization... (can take a minute)`);
      }
      else if (result.status.isFinalized)
      {
        console.log(`Transfer finalized at blockHash ${result.status.asFinalized}`);
        unsub();
        resolve({
          block: result.status.asFinalized.toString(),
        });
      }
    });
  });

  return parachainRegister;
}

export async function launchLitentryNode(
    specFilename: string, provider?: string): Promise<{binary: ChildProcess}>
{
  const cmd = LITENTRY_BINARY_PATH;
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
    `--execution`,
    `native`,
    `--`,
    `--execution`,
    `wasm`,
    `--chain`,
    ROCOCO_LOCAL_PATH,
    `--port`,
    `30343`,
    `--ws-port`,
    `9977`,
  ];
  const binary = spawn(cmd, args);

  binary.on('error', (err) => {
    if ((err as any).errno == 'ENOENT')
    {
      console.error(`\x1b[31mMissing litentry-node binary (${
          LITENTRY_BINARY_PATH}).\nPlease compile the litentry project:\ncargo build\x1b[0m`);
    }
    else
    {
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
  //			if (chunk.toString().match("Listening for new connections on
  // 127.0.0.1:9944.")) {
  //
  //				clearTimeout(timer);
  //				console.log(`Litentry Node Starts`);
  //				resolve();
  //			}
  //		};
  //		binary.stderr.on("data", onData);
  //		binary.stdout.on("data", onData);
  //	});

  return {binary};
}

export async function initApiPromise(wsProvider: WsProvider)
{
  console.log(
      `Initiating the API (ignore message "Unable to resolve type B..." and "Unknown types found...")`);

  // Initiate the polkadot API.
  const api = await ApiPromise.create({
    provider: wsProvider,
    types: {
      // mapping the actual specified address format
      Address: 'AccountId',
      // mapping the lookup
      LookupSource: 'AccountId',
      Account: {nonce: 'U256', balance: 'U256'},
      Transaction: {
        nonce: 'U256',
        action: 'String',
        gas_price: 'u64',
        gas_limit: 'u64',
        value: 'U256',
        input: 'Vec<u8>',
        signature: 'Signature'
      },
      Signature: {v: 'u64', r: 'H256', s: 'H256'},
      BlockWeights: 'u64',
      BlockLength: 'u64',
      ParachainInherentData: 'u64',
    }
  });

  console.log(`Initialization done`);
  console.log(`Genesis at block: ${api.genesisHash.toHex()}`);

  // Get keyring of Alice
  const alice = keyring.addFromUri('//Alice', {name: 'Alice default'});

  // Insert ocw session key
  const resInsertKey = api.rpc.author.insertKey(
      'ocw!', 'loop high amazing chat tennis auto denial attend type quit liquid tonight',
      '0x8c35b97c56099cf3b5c631d1f296abbb11289857e74a8f60936290080d56da6d');

  const {nonce, data: balance} = await api.query.system.account(alice.address);
  console.log(`Alice Substrate Account: ${alice.address}`);
  console.log(`Alice Substrate Account (nonce: ${nonce}) balance, free: ${balance.free.toHex()}`);

  return {api, alice};
}

async function sendTokenToOcw(api: ApiPromise, alice: KeyringPair)
{
  // Transfer tokens from Alice to ocw account
  console.log(`Transfer tokens from Alice to ocw account`);
  return new Promise<{block: string}>(async (resolve, reject) => {
    const unsub =
        await api.tx.balances.transfer(OCR_ACCOUNT, 1000000000000000)
            .signAndSend(alice, (result) => {
              console.log(`Current status is ${result.status}`);
              if (result.status.isInBlock)
              {
                console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
              }
              else if (result.status.isFinalized)
              {
                console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);
                unsub();
                resolve({
                  block: result.status.asFinalized.toString(),
                });
              }
            });
  });
}


export function describeLitentry(
    title: string, specFilename: string,
    cb: (context: {api: ApiPromise, alice: KeyringPair}) => void, provider?: string)
{
  describe(title, function() {
    // Set timeout to 6000 seconds (Because of 50-blocks delay of rococo, so called "training wheels")
    this.timeout(6000000);

    let tokenServer: ChildProcess;
    let binary: ChildProcess;
    let relayNodes: ChildProcess;
    let context:
        {api: ApiPromise, alice: KeyringPair} = {api: {} as ApiPromise, alice: {} as KeyringPair};
    // Making sure the Litentry node has started
    before('Starting Litentry Test Node', async function() {
      // this.timeout(SPAWNING_TIME);
      // Run alice and bob relay nodes on a separate process
      relayNodes = spawn(`sh`, [`${RELAY_NODE_SCRIPT}`]);
      // Wait for connection with relay nodes
      await launchRelayNodesAndParachainRegister();
      const initTokenServer = await launchAPITokenServer();
      const initNode = await launchLitentryNode(specFilename, provider);
      tokenServer = initTokenServer.apikey_server;
      binary = initNode.binary;
      const initApi = await initApiPromise(wsProvider);
      context.api = initApi.api;
      context.alice = initApi.alice;
      return sendTokenToOcw(initApi.api, initApi.alice);
    });

    after(async function() {
      console.log(`\x1b[31m Killing RPC\x1b[0m`);
      tokenServer.kill()
      binary.kill();
      relayNodes.kill();
      // FIXME Currently we can only kill background processes with calling killall. 
      //       This needs to be changed later 
      exec(`killall polkadot`);
      context.api.disconnect();
    });

    cb(context);
  });
}
