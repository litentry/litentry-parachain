import { cleanup } from "./commands/cleanup.js";
import { runParachainAndWorker } from "./commands/start.js";
import dotenv from "dotenv";
dotenv.config({ path: ".env.dev" });

async function main() {
	switch (process.argv[2]) {
		case "run":
			await runParachainAndWorker();

			break;
		case "cleanup":
			await cleanup();
			break;

		default:
			console.warn("Unknown command, check package.json#scripts");

			break;
	}
}

main();
