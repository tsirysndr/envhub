packages = [
  "hello"
]

rustup {
  default = "stable"
  toolchains = [
    "nightly"
  ]
  components = [
    "clippy",
    "llvm-tools"
  ]
  targets = [
    "wasm32-wasi",
  ]
}
