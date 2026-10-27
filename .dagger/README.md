# EnvHub CI with PureScript Dagger

This replaces `.fluentci`. GitHub Actions triggers the pipelines; PureScript
modules in `ci/src/CI` define the Linux work. `Main.purs` only starts the runtime.

| Function | Work |
| --- | --- |
| `test` | Run `cargo test --locked --workspace` with Rust 1.99 |
| `smoke` | Build EnvHub with Rust 1.99 and exercise `init`, `use`, `status`, `hello`, and `unuse` in a disposable Nix environment, then repeat activation for the repository demo |
| `build` | Cross-build a Linux release, returning its tarball and SHA-256 checksum as a Dagger Directory |

Dagger v1.0.0-beta.21 uses the vendored PureScript SDK with Node 24. Its runtime
installs PureScript 0.15.16 and Spago 1.0.5. The explicit `dagger-sdk` path in
`ci/spago.yaml` is required: Spago automatic discovery skips that Git-ignored
folder. The runtime supplies core bindings on clean checkouts and schema bindings
when available. Full engine error output is printed without truncation.

Commands for GitHub Actions, run from `.dagger`:

```sh
dagger api call -m ci test --source ..
dagger api call -m ci smoke --source ..
dagger api call -m ci build --source .. \
  --target x86_64-unknown-linux-gnu --version v0.3.0 --output ../dist
```

Supported Linux targets are `x86_64-unknown-linux-gnu` and
`aarch64-unknown-linux-gnu`. Builds use `rust:1.99-bookworm`; ARM releases use
the GNU cross-linker. Cargo downloads and build artifacts use Dagger cache
volumes. Release outputs are copied out of the mounted target cache before
export. No GitHub Actions cache snapshots are used.

The smoke test copies Nix into a disposable Linux container and explicitly
adds Home Manager profiles to PATH. It never activates an environment on the
GitHub host. The value `CLOUDFLARE_ROOT_KEY=123` is a test fixture, not a credential.
Home Manager is still resolved from `home-manager/master`, matching EnvHub's
current implementation. No developer home directory is mounted.

macOS releases remain native, with Rust 1.99.0 and direct GitHub asset uploads.
FlakeHub keeps its existing OIDC workflow. Flakestry is disabled, including its
automatic triggers. No workflow invokes Fluent CI.

The SDK's MIT license and notices are included under `sdk/purescript`.
This migration was reviewed without running local builds, tests, or Dagger
pipelines, as requested; execution is left to GitHub Actions.
