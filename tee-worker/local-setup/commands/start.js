import inquirer from "inquirer";
import { isPortAvailable } from "../utils/port.js";
import fs from "fs";
import { $, cd, echo, sleep } from "zx";
import { spawn } from "child_process";
import { homePath, sidechainPath } from "../utils/index.js";

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

async function setAwailablePorts() {
	let offset = 0;
	let portIsAvailable = true;
	do {
		portIsAvailable = await isPortAvailable(Number(process.env.CollatorWSPort) + offset);
		console.log(
			"Port is avaliable",
			portIsAvailable,
			Number(process.env.CollatorWSPort) + offset
		);
		if (!portIsAvailable) offset += 50;
	} while (!portIsAvailable);

	envDefaultServicesWithPorts.forEach(
		(service) => (process.env[service] = Number(process.env[service]) + offset)
	);
}

async function genereateLocalConfig() {
	const today = new Date();
	const parachainDir = `/tmp/parachain_dev_${today.getDate()}_${today.getMonth()}_${today.getFullYear()}`;

	echo("Directory has been assigned to:", parachainDir);

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
	echo("Successfully written ", config_file);
}

async function generateConfigFiles() {
	const envLocalExampleFile = `${sidechainPath}/ts-tests/integration-tests/.env.local.example`;
	const envLocalFile = envLocalExampleFile.slice(0, -".example".length);

	const data = fs.readFileSync(envLocalExampleFile, "utf8");
	const updatedData = data
		.replace(":2000", `:${process.env.TrustedWorkerPort}`)
		.replace(":9944", `:${process.env.CollatorWSPort}`);

	fs.writeFileSync(envLocalFile, updatedData);
	echo("Successfully written ", envLocalFile);
}

export async function runParachainAndWorker() {
	await setAwailablePorts();
	const answers = await questionary();

	await generateConfigFiles();
	if (answers.type !== "remote") {
		await genereateLocalConfig();
	}
	await runParachain(answers);

	const workers = await runWorkers(answers);

	console.log("process.env", JSON.stringify(process.env, null, 2));
	// display status message
}

function buildWorkerOpts(workerNumber) {
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
		"-p",
		Number(process.env.CollatorWSPort),
		"--enable-mock-server",
		"--parentchain-start-block",
		"0",
		workerNumber === 0 && "--enable-metrics",
	].filter(Boolean);
	echo("index", workerNumber, flags);

	const subcommandFlags = ["--skip-ra", "--dev", workerNumber !== 0 && "--request-state"].filter(
		Boolean
	);

	return { flags, subcommandFlags };
}

async function runWorker(index, { flags, subcommandFlags }) {
	const cwd = `${sidechainPath}/tmp/worker-${index}`;
	const logDir = `${sidechainPath}/log`;
	const logFile = `${logDir}/log-${index}.txt`;

	if (!fs.existsSync(cwd)) {
		echo("creating dir");

		fs.mkdirSync(cwd);
	}

	const logStream = fs.createWriteStream(logFile, { flags: "a" });

	await $`cp ${sidechainPath}/bin/litentry-worker ${sidechainPath}/bin/enclave.signed.so ${cwd}`;

	cd(cwd);

	const workerProcess = spawn("./litentry-worker", [...flags, "run", ...subcommandFlags], {
		detached: true,
		stdio: ["ignore", logStream, logStream],
	});

	// TODO: !!! need to figure out when worker run successfully. Probably look at log for specific keywords
	await sleep(index === 0 ? 50000 : 20000);

	// move a process to background
	workerProcess.unref();

	return workerProcess;
}

async function runWorkers(answers) {
	const workers = [];
	for (let index = 0; index < answers.workersCount; index++) {
		const options = buildWorkerOpts(index);
		const result = await runWorker(index, options);
		workers.push(result);
	}
	return workers;
}

async function runParachain(answers) {
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
	}
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
