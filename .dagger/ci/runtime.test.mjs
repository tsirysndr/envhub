import test from "node:test";
import assert from "node:assert/strict";
import { reportFailure } from "./src/CI/Runtime.js";

test("prints complete exec output once, including the failure after 10 KB", () => {
  const tail = "ACTUAL TEST FAILURE AT THE END";
  const extension = { _type: "EXEC_ERROR", cmd: ["nix", "flake", "check", "-L"], exitCode: 100,
    stdout: "test output", stderr: "x".repeat(25000) + tail };
  let output = "";
  const write = process.stderr.write;
  try {
    process.stderr.write = chunk => { output += chunk; return true; };
    reportFailure(Object.assign(new Error("exit code: 100"), {
      extensions: extension, errors: [{ extensions: extension }],
    }))();
  } finally { process.stderr.write = write; }
  assert.ok(output.includes(JSON.stringify(extension.cmd)));
  assert.ok(output.includes(extension.stdout));
  assert.ok(output.includes(extension.stderr));
  assert.equal(output.split(tail).length - 1, 1);
});
