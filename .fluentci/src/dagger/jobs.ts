import { dag, env } from "../../deps.ts";

export enum Job {
  test = "test",
  build = "build",
}

export const exclude = ["target", ".git", ".devbox", ".fluentci"];

export const test = async (
  src = ".",
  options: string[] = []
): Promise<string> => {
  const context = dag.host().directory(src);
  const ctr = dag
    .pipeline(Job.test)
    .container()
    .from("rust:1.76-bullseye")
    .withDirectory("/app", context, { exclude })
    .withWorkdir("/app")
    .withMountedCache("/app/target", dag.cacheVolume("target"))
    .withMountedCache("/root/cargo/registry", dag.cacheVolume("registry"))
    .withExec(["cargo", "install", "--path", "crates/cli", ...options])
    .withExec(["mkdir", "-p", "/demo"])
    .withWorkdir("/demo")
    .withExec(["envhub", "init"])
    .withExec(["ls", "-ltr"])
    .withExec(["cat", "envhub.hcl"])
    .withExec(["envhub", "use"])
    .withExec(["envhub", "status"])
    .withEnvVariable("PATH", "${PATH}:/nix/var/nix/profiles/default/bin", {
      expand: true,
    })
    .withExec(["nix", "--version"])
    .withExec(["ls", "-la", "/root"])
    .withExec(["which", "hello"])
    .withExec(["envhub", "unuse"])
    .withExec(["envhub", "status"]);

  const stdout = await ctr.stdout();
  console.log(stdout);

  await ctr.file("/usr/local/cargo/bin/envhub").export("./envhub");

  return stdout;
};

export const build = (src = "."): Promise<string> => {
  const context = dag.host().directory(src);
  const ctr = dag
    .pipeline(Job.build)
    .container()
    .from("rust:1.76-bullseye")
    .withExec(["apt-get", "update"])
    .withExec([
      "apt-get",
      "install",
      "-y",
      "build-essential",
      "gcc-aarch64-linux-gnu",
    ])
    .withDirectory("/app", context, { exclude })
    .withWorkdir("/app")
    .withMountedCache("/app/target", dag.cacheVolume("target"))
    .withMountedCache("/root/cargo/registry", dag.cacheVolume("registry"))
    .withMountedCache("/assets", dag.cacheVolume("gh-release-assets"))
    .withEnvVariable(
      "CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER",
      env.get("TARGET") === "aarch64-unknown-linux-gnu"
        ? "aarch64-linux-gnu-gcc"
        : ""
    )
    .withEnvVariable("TAG", env.get("TAG") || "latest")
    .withEnvVariable("TARGET", env.get("TARGET") || "x86_64-unknown-linux-gnu")
    .withExec(["sh", "-c", "rustup target add $TARGET"])
    .withExec(["sh", "-c", "cargo build -p envhub --release --target $TARGET"])
    .withExec(["sh", "-c", "cp target/${TARGET}/release/envhub ."])
    .withExec([
      "sh",
      "-c",
      "tar czvf /assets/envhub_${TAG}_${TARGET}.tar.gz envhub",
    ])
    .withExec([
      "sh",
      "-c",
      "shasum -a 256 /assets/envhub_${TAG}_${TARGET}.tar.gz > /assets/envhub_${TAG}_${TARGET}.tar.gz.sha256",
    ]);

  return ctr.stdout();
};

export type JobExec = (src?: string) =>
  | Promise<string>
  | ((
      src?: string,
      options?: {
        ignore: string[];
      }
    ) => Promise<string>);

export const runnableJobs: Record<Job, JobExec> = {
  [Job.test]: test,
  [Job.build]: build,
};

export const jobDescriptions: Record<Job, string> = {
  [Job.test]: "Run tests",
  [Job.build]: "Build the project",
};
