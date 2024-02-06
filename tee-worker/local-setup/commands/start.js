import inquirer from "inquirer";
import { isPortAvailable } from "../utils/port.js";
import fs from "fs";
import { $, cd, sleep } from "zx";

const homePath = process.env.PWD.substring(0, process.env.PWD.indexOf("/tee-worker"));
const sidechainPath = `${homePath}/tee-worker`;
// NOTE: maybe automatically bump ports if busy?

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

		offset += 50;
	} while (!portIsAvailable);

	envDefaultServicesWithPorts.forEach(
		(service) => (process.env[service] = Number(process.env[service]) + offset)
	);
}

export async function runParachainAndWorker() {
	await setAwailablePorts();

	const answers = await questionary();

	await runParachain(answers);
	const workers = await runWorkers(answers);
	process.on("SIGINT", () => {
		workers.map((worker) => {
			console.log("killing worker");
			console.log(worker);

			worker.kill();
		});

		process.exit();
	});

	await sleep(100000);
}

function buildWorkerOpts(workerNumber) {
	// run worker
	const flags = [
		"--clean-reset",
		"-T",
		"wss://localhost",
		"-P",
		Number(process.env.TrustedWorkerPort) + workerNumber * 10,
		"-w",
		Number(process.env.UntrustedWorkerPort) + workerNumber * 10,
		"-r",
		Number(process.env.MuRaPort) + workerNumber * 10,
		"-h",
		Number(process.env.UntrustedHttpPort) + workerNumber * 10,
		"--enable-mock-server",
		"--parentchain-start-block",
		"0",
		workerNumber === 0 && "--enable-metrics",
	].filter(Boolean);
	console.log("index", workerNumber, flags);

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
		console.log("creating dir");

		fs.mkdirSync(cwd);
	}

	const logStream = fs.createWriteStream(logFile, { flags: "a" });

	await $`cp ${sidechainPath}/bin/litentry-worker ${sidechainPath}/bin/enclave.signed.so ${cwd}`;

	cd(cwd);

	const workerProcess = $`./litentry-worker ${flags} run ${subcommandFlags}`;

	workerProcess.stdout.on("data", (data) => {
		logStream.write(data);
	});
	await sleep(5000);
	return workerProcess;
}

async function runWorkers(answers) {
	const workers = [];
	for (let index = 0; index < answers.workersCount; index++) {
		const options = buildWorkerOpts(index);
		const result = await runWorker(index, options);
		workers.push(result);
	}
	console.log("PROCESS ENV", workers);

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
