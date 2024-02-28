import { $ } from "zx";
import { workerPath } from "../utils/index.js";

export async function cleanup() {
	try {
		await $`rm -rf ${workerPath}/log/worker-*`;
		await $`pkill -u $USER -9 "litentry"`;
		// TODO: not sure do we need to remove binaries
		// await $`rm -rf ${workerPath}/tmp/worker-*`;
	} catch {}
}
