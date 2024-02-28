export const homePath = process.env.PWD.substring(0, process.env.PWD.indexOf("/tee-worker"));
export const workerPath = `${homePath}/tee-worker`;

export function printLabel(label) {
  const separator = "=".repeat(30);
  console.log(`\n${separator} ${label} ${separator}`);
}
