import inquirer from "inquirer";
import { isPortAvailable } from "../utils/port.js";
import fs from "fs";
import { $, cd, sleep } from "zx";
import { spawn } from "child_process";
import { homePath, printLabel, workerPath } from "../utils/index.js";

const envDefaultServicesWithPorts = [
  "AliceWSPort",
  "AliceRPCPort",
  "AlicePort",
  "BobWSPort",
  "BobRPCPort",
  "BobPort",
  "CollatorWSPort",
  "CollatorRPCPort",
  "CollatorPort",
  "TrustedWorkerPort",
  "UntrustedWorkerPort",
  "MuRaPort",
  "UntrustedHttpPort",
];

// checking that parachain port or sidechain port are available, or add offset
async function setAwailablePorts(answers) {
  let offset = 0;
  let parachainPortIsAvailable = true;
  let workerPortIsAvailable = true;
  do {
    parachainPortIsAvailable =
      answers.mode === "remote" ||
      (await isPortAvailable(Number(process.env.CollatorWSPort) + offset));
    workerPortIsAvailable = await isPortAvailable(Number(process.env.TrustedWorkerPort) + offset);

    if (!parachainPortIsAvailable || !workerPortIsAvailable) offset += 50;
  } while (!parachainPortIsAvailable || !workerPortIsAvailable);

  if (offset > 0) console.log("Due to unavailable port shifting ports for", offset);

  envDefaultServicesWithPorts.forEach(
    (service) => (process.env[service] = Number(process.env[service]) + offset)
  );
}

function genereateLocalConfig() {
  const today = new Date();
  const day = String(today.getDate()).padStart(2, "0");
  const month = String(today.getMonth() + 1).padStart(2, "0"); // Months are zero-based
  const year = today.getFullYear();
  const hours = String(today.getHours()).padStart(2, "0");
  const minutes = String(today.getMinutes()).padStart(2, "0");

  const parachainDir = `/tmp/parachain_dev_${day}_${month}_${year}_${hours}${minutes}`;

  process.env.LITENTRY_PARACHAIN_DIR = parachainDir;
  console.log("Directory has been assigned to:", parachainDir);

  const data = {
    eth_endpoint: "http://127.0.0.1:8545",
    eth_address: "[0x4d88dc5d528a33e4b8be579e9476715f60060582]",
    private_key: "0xe82c0c4259710bb0d6cf9f9e8d0ad73419c1278a14d375e5ca691e7618103011",
    ocw_account: "5FEYX9NES9mAJt1Xg4WebmHWywxyeGQK8G3oEBXtyfZrRePX",
    genesis_state_path: `${parachainDir}/genesis-state`,
    genesis_wasm_path: `${parachainDir}/genesis-wasm`,
    parachain_ws: `ws://localhost:${process.env.CollatorWSPort}`,
    relaychain_ws: `ws://localhost:${process.env.AliceWSPort}`,
    bridge_path: "/tmp/parachain_dev/chainbridge",
  };

  const config_file = `${homePath}/ts-tests/config.local.json`;

  fs.writeFileSync(config_file, JSON.stringify(data, null, 4));
  console.log("Config local file generated:", config_file);
}

function generateConfigFiles() {
  const envLocalExampleFile = `${workerPath}/ts-tests/integration-tests/.env.local.example`;
  const envLocalFile = envLocalExampleFile.slice(0, -".example".length);

  const data = fs.readFileSync(envLocalExampleFile, "utf8");
  const updatedData = data
    .replace(":2000", `:${process.env.TrustedWorkerPort}`)
    .replace(":9944", `:${process.env.CollatorWSPort}`);

  fs.writeFileSync(envLocalFile, updatedData);
  console.log("Env config file generated:", envLocalFile);
}

export async function runParachainAndWorker() {
  const answers = await questionary();
  await setAwailablePorts(answers);

  printLabel("Running Parachain");
  await runParachain(answers);

  printLabel("Running worker(s)");
  await runWorkers(answers);
}

function buildWorkerOpts(workerNumber, answers) {
  const offset = workerNumber * 10;
  // run worker
  const flags = [
    "--clean-reset",
    "-T",
    "wss://localhost",
    "-P",
    Number(process.env.TrustedWorkerPort) + offset,
    "-w",
    Number(process.env.UntrustedWorkerPort) + offset,
    "-r",
    Number(process.env.MuRaPort) + offset,
    "-h",
    Number(process.env.UntrustedHttpPort) + offset,
    "--enable-mock-server",
    "--parentchain-start-block",
    "0",
  ];

  const subcommandFlags = ["--skip-ra", "--dev"];

  if (workerNumber === 0) {
    // Only first/main worker enables metrics
    flags.push("--enable-metrics");
  } else {
    // Other than first worker require state requesting
    subcommandFlags.push("--request-state");
  }

  if (answers.mode === "remote") {
    flags.push(
      "-u",
      `${answers.remoteURL.protocol}//${answers.remoteURL.hostname}`,
      "-p",
      answers.remoteURL.port
    );
  } else {
    flags.push("-p", Number(process.env.CollatorWSPort));
  }

  return { flags, subcommandFlags };
}

async function waitForInitialization(logFile, initializationPhrase, timeoutInSeconds = 60) {
  const timeoutMillis = timeoutInSeconds * 1000;
  const startTime = Date.now();
  let found = false;

  while (!found) {
    const elapsedTime = Date.now() - startTime;

    if (elapsedTime >= timeoutMillis) {
      throw new Error(
        `Initialization timeout: Process did not initialize within ${timeoutInSeconds} seconds. Please check ${logFile}`
      );
    }

    const logContent = fs.readFileSync(logFile, "utf8");
    if (logContent.includes(initializationPhrase)) {
      found = true;
    } else {
      await sleep(1000); // Adjust the sleep duration as needed
    }
  }
}

async function runWorker(index, { flags, subcommandFlags }) {
  const cwd = `${workerPath}/tmp/worker-${index}`;
  const logFile = `${workerPath}/log/worker-${index}.txt`;

  if (!fs.existsSync(cwd)) {
    console.log("Creating worker's directory", cwd);

    fs.mkdirSync(cwd);
  }

  const logStream = fs.createWriteStream(logFile, { flags: "a" });

  console.log(`Running worker ${index + 1} and waiting for initialisation`);
  await $`cp ${workerPath}/bin/litentry-worker ${workerPath}/bin/enclave.signed.so ${cwd}`;

  cd(cwd);

  const workerProcess = spawn("./litentry-worker", [...flags, "run", ...subcommandFlags], {
    detached: true,
    stdio: ["ignore", logStream, logStream],
  });

  // Wait for worker initialization phrase by checking the log. If the worker hans more than minute, then probably
  await waitForInitialization(logFile, "Enclave registered at block number", 60);

  console.log("Worker successfully run");
  console.log("./litentry-worker", [...flags, "run", ...subcommandFlags].join(" "));
  console.log("----------------");

  // move a process to background
  workerProcess.unref();

  return workerProcess;
}

async function runWorkers(answers) {
  const workers = [];
  for (let index = 0; index < answers.workersCount; index++) {
    const options = buildWorkerOpts(index, answers);
    const result = await runWorker(index, options);
    workers.push(result);
  }
  return workers;
}

async function runParachain(answers) {
  console.log(`Running parachain in "${answers.mode}" mode`);
  generateConfigFiles();
  if (answers.type !== "remote") genereateLocalConfig();

  try {
    if (answers.mode === "local-docker") {
      await $`${homePath}/tee-worker/scripts/litentry/start_parachain.sh`;
    } else if (answers.mode === "local-binary-standalone") {
      await $`${homePath}/scripts/launch-standalone.sh`;
    } else if (answers.mode === "local-binary") {
      await $`${homePath}/scripts/launch-local-binary.sh rococo`;
    } else if (answers.mode === "remote") {
    }
  } catch (error) {
    console.error("Running parachain fails");
    console.error(error);
    process.exit(1);
  }

  console.log(
    `Exposed ports: \n --port ${process.env.CollatorPort} --ws-port ${process.env.CollatorWSPort}  --rpc-port ${process.env.CollatorRPCPort}`
  );
}

function questionary() {
  return inquirer.prompt([
    {
      type: "list",
      name: "mode",
      message: "Which mode you want to run?",
      choices: ["local-binary-standalone", "local-binary", "local-docker", "remote"],
    },
    {
      type: "input",
      name: "remoteURL",
      message: "Which parachain parachain URL to use?",
      when: (answers) => answers.mode === "remote",
      default: "ws://localhost:9944",
      filter: (input) => new URL(input),
      validate: (input) => {
        try {
          new URL(input);
          return true;
        } catch {
          return `Please, set valid URL! Inserted: ${input}`;
        }
      },
    },
    {
      type: "number",
      name: "workersCount",
      message: "How much workers do you want to run?",
      default: 1,
      validate: (input) => (input >= 1 ? true : "Please set positive number"),
    },
  ]);
}
