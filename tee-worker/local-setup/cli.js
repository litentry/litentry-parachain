import { cleanup } from "./commands/cleanup.js";
import { runParachainAndWorker } from "./commands/start.js";
import dotenv from "dotenv";
import { printLabel } from "./utils/index.js";

dotenv.config({ path: ".env.dev" });

async function main() {
  const [_node, _file, command, ...options] = process.argv;
  switch (command) {
    case "run":
      printLabel("Cleanup before start");
      await cleanup();

      await runParachainAndWorker(options);

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
