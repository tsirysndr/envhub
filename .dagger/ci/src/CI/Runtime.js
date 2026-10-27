// Session credentials and untruncated engine diagnostics at the process boundary.
export const sessionCredentials = () => ({
  port: Number(process.env.DAGGER_SESSION_PORT),
  token: process.env.DAGGER_SESSION_TOKEN ?? "",
});

export const reportFailure = error => () => {
  process.stderr.write(`Dagger pipeline failed: ${error.message}\n`);
  const seen = new Set();
  for (const extension of [error.extensions, ...(error.errors ?? []).map(item => item.extensions)]) {
    if (extension?._type !== "EXEC_ERROR") continue;
    const key = JSON.stringify(extension);
    if (seen.has(key)) continue;
    seen.add(key);
    process.stderr.write(`Command: ${JSON.stringify(extension.cmd)}\nExit code: ${extension.exitCode}\n`);
    for (const stream of ["stdout", "stderr"]) {
      const output = extension[stream];
      if (typeof output === "string" && output.length) {
        process.stderr.write(`--- ${stream} ---\n`);
        process.stderr.write(output);
        process.stderr.write("\n");
      }
    }
  }
};
