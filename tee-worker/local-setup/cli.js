import { runParachainAndWorker } from "./commands/start.js";
import dotenv from "dotenv";

dotenv.config({ path: ".env.dev" });
async function main() {
	// run parachain
	// run workers
	// display result
	// console.log(process.env);
	await runParachainAndWorker();
}

main();
