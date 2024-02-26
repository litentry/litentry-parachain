import { $ } from "zx";
import { sidechainPath } from "../utils/index.js";

export async function cleanup() {
	console.log("Killing proceses");

	await $`pkill -u $USER -9 "litentry"`;
	await $`rm -rf ${sidechainPath}/tmp/w* ${sidechainPath}/log/log-*`;
}
