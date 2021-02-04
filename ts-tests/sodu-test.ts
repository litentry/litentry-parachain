import Web3 from "web3";
import { JsonRpcResponse } from "web3-core-helpers";
import { spawn, ChildProcess } from "child_process";
import { TypeRegistry } from "@polkadot/types/create";
import { Bytes } from "@polkadot/types";
import 'mocha';
import { ApiPromise, Keyring, WsProvider } from "@polkadot/api";
import { KeyringPair } from '@polkadot/keyring/types';
import fs from 'fs';

export const LITENTRY_BINARY_PATH = `../target/release/rococo-collator`;
export const POLKADOT_BINARY_PATH = `../../parity-polkadot/target/release/polkadot`;
export const APIKEY_SERVER_PATH = `../target/debug/litentry-token-server`;
export const PARA_GENESIS_HEAD_PATH = `../para-1984-genesis`;
export const PARA_WASM_PATH = `../para-1984-wasm`;
export const SPAWNING_TIME = 30000;

async function main() {
  const api = await ApiPromise.create({
    provider: new WsProvider("ws://localhost:9944"),
    types: {
      // mapping the actual specified address format
      Address: "AccountId",
      // mapping the lookup
      LookupSource: "AccountId",
    },
  });

  // Keyring needed to sign using Alice account
  const keyring = new Keyring({ type: "sr25519" });
  // Get keyring of Alice
  const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
  // Get keyring of Sudo
  const sudoKey = await api.query.sudo.key();

  const sudo = keyring.addFromAddress(sudoKey);

  //const sudoPair = keyring.getPair(sudoKey);

  const registry = new TypeRegistry();

  const genesisHeadBytes = fs.readFileSync(PARA_GENESIS_HEAD_PATH, "utf8");

  const validationCodeBytes = fs.readFileSync(PARA_WASM_PATH, "utf8");

  const tx = api.tx.sudo.sudo(
    api.tx.parasSudoWrapper.sudoScheduleParaInitialize(1984, {
      genesisHead: new Bytes(registry, genesisHeadBytes),
      validationCode: new Bytes(registry, validationCodeBytes),
      parachain: true,
    })
  );
  const tx1 = api.tx.sudo.sudo(
    api.tx.parasSudoWrapper.sudoScheduleParaInitialize(1984, {
      genesisHead: new Bytes(registry, genesisHeadBytes),
      validationCode: new Bytes(registry, validationCodeBytes),
      parachain: true,
    })
  );
  console.log(`Parachain registration tx Sent!`);

  //await new Promise(r => setTimeout(r, 6000));
  const parachainRegister = await new Promise<{ block: string }>(
    async (resolve, reject) => {
      const unsub = await tx.signAndSend(alice, (result) => {
        console.log(`Parachain registration is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(
            `Parachain registration included at blockHash ${result.status.asInBlock}`
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
    }
  );
}

main()
  .catch(console.error)
  .then(() => process.exit(0));