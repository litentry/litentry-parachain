import {ChildProcess, spawn} from 'child_process';
import fs from "fs";
import {initWorkerConnection, sleep} from "./utils";
import * as path from "path";
import * as process from "process";
import WebSocketAsPromised = require("websocket-as-promised");

export type WorkerConfig = {
    untrusted_ws_port: number,
    commands: {
        first_launch: string,
        resume: string,
    }
}

function genCommands(node_url: string, node_port: number): { worker0: WorkerConfig, worker1: WorkerConfig } {
    return {
        worker0: {
            untrusted_ws_port: 3000,
            commands: {
                first_launch: "--clean-reset --mu-ra-external-address localhost --mu-ra-port 3443" +
                    " --untrusted-http-port 4545 --ws-external --trusted-external-address wss://localhost" +
                    " --trusted-worker-port 2000 --untrusted-external-address ws://localhost" +
                    " --untrusted-worker-port 3000 --node-url " + node_url +
                    " --node-port " + node_port + " run --skip-ra --dev",

                resume: "--mu-ra-external-address localhost --mu-ra-port 3443" +
                    " --untrusted-http-port 4545 --ws-external --trusted-external-address wss://localhost" +
                    " --trusted-worker-port 2000 --untrusted-external-address ws://localhost" +
                    " --untrusted-worker-port 3000 --node-url " + node_url +
                    " --node-port " + node_port + " run --skip-ra"
            }
        },
        worker1: {
            untrusted_ws_port: 3001,
            commands: {
                first_launch: "--clean-reset --mu-ra-external-address localhost --mu-ra-port 3444" +
                    " --untrusted-http-port 4546 --ws-external --trusted-external-address wss://localhost" +
                    " --trusted-worker-port 2001 --untrusted-external-address ws://localhost" +
                    " --untrusted-worker-port 3001 --node-url " + node_url +
                    " --node-port " + node_port + " run --skip-ra --request-state",

                resume: "--mu-ra-external-address localhost --mu-ra-port 3444" +
                    " --untrusted-http-port 4546 --ws-external --trusted-external-address wss://localhost" +
                    " --trusted-worker-port 2001 --untrusted-external-address ws://localhost" +
                    " --untrusted-worker-port 3001 --node-url " + node_url +
                    " --node-port " + node_port + " run --skip-ra --request-state",
            }
        }
    }
}

async function launchWorker(binary_dir: string, working_dir: string, command: string, init_files: boolean): Promise<{ shard: string, process: ChildProcess }> {
    // const logging = fs.createWriteStream(log, {flags: 'w+'});
    if (init_files) {
        fs.mkdirSync(working_dir, {recursive: true})
        fs.copyFileSync(`${binary_dir}/enclave.signed.so`, `${working_dir}/enclave.signed.so`)
        fs.copyFileSync(`${binary_dir}/integritee-service`, `${working_dir}/integritee-service`)
        fs.closeSync(fs.openSync(`${working_dir}/spid.txt`, 'w'))
        fs.closeSync(fs.openSync(`${working_dir}/key.txt`, 'w'))
    }

    const serviceENV = {
        RUST_LOG: 'warn,sp_io::storage=error,substrate_api_client=warn',
        PATH: "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin:/opt/sgxsdk/bin:/opt/sgxsdk/bin/x64",
        SGX_SDK: "/opt/sgxsdk",
        LD_RUN_PATH: "/usr/lib:/usr/local/lib",
        LD_LIBRARY_PATH: "/usr/lib:/usr/local/lib:/opt/sgxsdk/sdk_libs"
    }

    return new Promise<{ shard: string, process: ChildProcess }>(async (resolve, reject) => {
        const process = spawn(
            `./integritee-service`,
            [command],
            {
                cwd: working_dir,
                shell: "/bin/sh",
                env: serviceENV,
                detached: true,
            }
        );
        process.stdout.setEncoding("utf8")
        let shard = ""
        process.stdout.on("data", (data: string) => {
            if (data.includes("Successfully initialized shard")) {
                const regex = /^Successfully initialized shard (0x[\w\d]{64}).*/g;
                const groups = regex.exec(data)
                if (groups) {
                    shard = groups[1]
                }
            }
            if (data.includes("Untrusted RPC server is spawned on")) {
                resolve({shard, process})
            }
            console.log(data)
        })
        process.stderr.setEncoding("utf8")
        process.stderr.on("data", (data: string) => {
            console.log(data)
        })
        process.on('close', (code) => {
            console.log("close: ", code);
        });
    });
}

async function killWorker(worker: ChildProcess) {
    // https://azimi.me/2014/12/31/kill-child_process-node-js.html
    if (worker.pid) {
        process.kill(-worker.pid, 9)
    }
}

async function latestBlock(connection: WebSocketAsPromised, shard: string): Promise<{ result: undefined | { "number": number, hash: string } }> {
    return await connection.sendRequest({
        "jsonrpc": "2.0",
        "method": "sidechain_latestBlock",
        "params": shard,
        "id": 1
    }, {requestId: 1, timeout: 6000});
}

async function waitWorkerProducingBlock(connection: WebSocketAsPromised, shard: string, atLeast: number) {
    return new Promise<void>(async (resolve, reject) => {
        let block_number = 0
        let start_block_number = 0
        do {
            const resp = await latestBlock(connection, shard);
            if (resp.result) {
                block_number = resp.result.number;
                if (start_block_number == 0) {
                    start_block_number = block_number
                }
                console.log("current block:", block_number)
            }
            await sleep(1)
        } while (block_number >= start_block_number + atLeast)
        resolve()
    })
}


(async () => {
    let binary_dir = path.join(__dirname, "../bin");
    let tmp_dir = path.join(__dirname, "./tmp");
    let commands = genCommands("ws://host.docker.internal", 9944)
    let {
        shard: shard,
        process: worker0
    } = await launchWorker(binary_dir, tmp_dir, commands.worker0.commands.first_launch, true)

    await sleep(15);
    let connection0 = await initWorkerConnection(`ws://localhost:${commands.worker0.untrusted_ws_port}`)
    console.log("shard:", shard);
    await waitWorkerProducingBlock(connection0, shard, 2)
    await killWorker(worker0);
    await sleep(3)
    let {
        shard: _,
        process: worker0_1
    } = await launchWorker(binary_dir, tmp_dir, commands.worker0.commands.resume, false)
    await sleep(30)
    // worker0_1.kill(9)
    process.exit(-1)
})()
