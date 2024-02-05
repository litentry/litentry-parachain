import inquirer from "inquirer";
import { isPortAvailable } from "../utils/port.js";
import { exec } from "../utils/index.js";
import { $, sleep } from "zx";
// print("Starting litentry parachain in background ...")
// if parachain_type == "local-docker":
//     os.environ['LITENTRY_PARACHAIN_DIR'] = parachain_dir
//     setup_environment(offset, config, parachain_dir)
//     # TODO: use Popen and copy the stdout also to node.log
//     run(["./scripts/litentry/start_parachain.sh"], check=True)
// elif parachain_type == "local-binary-standalone":
//     os.environ['LITENTRY_PARACHAIN_DIR'] = parachain_dir
//     setup_environment(offset, config, parachain_dir)
//     run(["../scripts/launch-standalone.sh"], check=True)
// elif parachain_type == "local-binary":
//     os.environ['LITENTRY_PARACHAIN_DIR'] = parachain_dir
//     setup_environment(offset, config, parachain_dir)
//     run(["../scripts/launch-local-binary.sh", "rococo"], check=True)
// elif parachain_type == "remote":
//     print("Litentry parachain should be started remotely")
// else:
//     sys.exit("Unsupported parachain_type")

// print("Litentry parachain is running")
// print("------------------------------------------------------------")

const homePath = process.env.PWD.substring(0, process.env.PWD.indexOf("/tee-worker"));

// NOTE: maybe automatically bump ports if busy?

export async function runParachainAndWorker() {
	const answers = await questionary();

	// await runParachain(answers);
	const workers = await runWorkers(answers);

	process.on("exit", () => {
		workers.map((worker) => worker.kill());
	});
}

async function buildWorkerOpts(workerNumber, offset = 0) {
	// run worker
	const flags = [
		"--clean-reset",
		"-T",
		"wss://localhost",
		"-P",
		process.env.TrustedWorkerPort + offset + workerNumber * 10,
		"-w",
		process.env.UntrustedWorkerPort + offset + workerNumber * 10,
		"-r",
		process.env.MuRaPort + offset + workerNumber * 10,
		"-h",
		process.env.UntrustedHttpPort + offset + workerNumber * 10,
		"--enable-mock-server",
		"--parentchain-start-block",
		"0",
		workerNumber === 1 && "--enable-metrics",
	].filter(Boolean);

	const subcommandFlags = ["--skip-ra", "--dev", workerNumber !== 1 && "--request-state"].filter(
		Boolean
	);

	return { flags, subcommandFlags };
}

async function runWorker(workerBin, { flags, subcommandFlags }) {
	return $`${workerBin} ${flags} ${subcommandFlags}`;
}

async function runWorkers(answers) {
	const workerBin = homePath + "/tee-worker/bin/litentry-worker";
	const workers = [...Array(answers.workersCount)].map((_, index) => {
		const options = buildWorkerOpts(index + 1);
		runWorker(workerBin, options);
	});
	console.log("🚀 ~ runWorkers ~ runWorkers:", answers);
	await sleep(3000);

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
			console.log("Parachain run remotely on", answers.remoteURL);
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
			choices: ["local-docker", "local-binary-standalone", "local-binary", "remote"],
		},
		{
			type: "input",
			name: "remoteURL",
			message: "Which parachain parachain URL to use?",
			when: (answers) => answers.mode === "remote",
		},
		// {
		// 	type: "number",
		// 	name: "parachainPort",
		// 	message: "Which parachain port to use?",
		// 	default: defaultPorts.wsParachain,
		// 	validate: isPortAvailable,
		// },
		{
			type: "number",
			name: "workersCount",
			message: "How much workers do you want to run?",
			default: 1,
			validate: (input) => (input >= 1 ? true : "Please set positive number"),
		},

		{
			type: "checkbox",
			message: "Select options",
			name: "pm",
			choices: [
				{ name: "npm", value: "npm" },
				{ name: "yarn", value: "yarn" },
				new inquirer.Separator(),
				{ name: "pnpm", value: "pnpm", disabled: true },
				{
					name: "pnpm",
					value: "pnpm",
					disabled: "(pnpm is not available)",
				},
			],
		},
	]);
}
