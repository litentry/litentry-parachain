import { spawn } from "child_process";
import fs from "fs";

// wrapper to trigger terminal's commands like grep, netstat, make
export function exec(command, args = []) {
	return new Promise((resolve, reject) => {
		const childProcess = spawn(command, args, {
			env: process.env,
		});

		let result = "";
		let error = "";

		// Capture standard output
		childProcess.stdout.on("data", (data) => {
			result += data.toString();
		});

		// Capture standard error
		childProcess.stderr.on("data", (data) => {
			console.log("data error", data);
			error += data.toString();
		});

		// Handle process exit
		childProcess.on("close", (code) => {
			if (code === 0) {
				resolve(result.trim());
			} else {
				const errorMessage = `Command ${command} ${args.join(
					" "
				)} exited with code ${code}. Error: ${error}`;
				reject(errorMessage);
				// Synchronous error handling for any unhandled exceptions
				try {
					throw new Error(errorMessage);
				} catch (error) {
					console.error("Synchronous error handling:", error);
				}
			}
		});
	});
}

function generateConfigLocalJson(parachainDir) {
	const data = {
		eth_endpoint: "http://127.0.0.1:8545",
		eth_address: "[0x4d88dc5d528a33e4b8be579e9476715f60060582]",
		private_key: "0xe82c0c4259710bb0d6cf9f9e8d0ad73419c1278a14d375e5ca691e7618103011",
		ocw_account: "5FEYX9NES9mAJt1Xg4WebmHWywxyeGQK8G3oEBXtyfZrRePX",
		genesis_state_path: `${parachainDir}/genesis-state`,
		genesis_wasm_path: `${parachainDir}/genesis-wasm`,
		parachain_ws: `ws://localhost:${process.env.CollatorWSPort || "9944"}`,
		relaychain_ws: `ws://localhost:${process.env.AliceWSPort || "9946"}`,
		bridge_path: "/tmp/parachain_dev/chainbridge",
	};

	const configFilePath = "../ts-tests/config.local.json";

	fs.writeFileSync(configFilePath, JSON.stringify(data, null, 4));

	console.log("Successfully written", configFilePath);
}

// Example usage
// generateConfigLocalJson("/path/to/parachain");
