<p align="center">
  <img 
    src="banner.png"
    alt="EnvHub"
    style="width:100%;"
  />
</p>

[![FlakeHub](https://img.shields.io/endpoint?url=https://flakehub.com/f/tsirysndr/envhub/badge)](https://flakehub.com/flake/tsirysndr/envhub)
[![crates](https://img.shields.io/crates/v/envhub.svg)](https://crates.io/crates/envhub)
[![downloads](https://img.shields.io/crates/dr/envhub)](https://crates.io/crates/envhub)
[![discord](https://img.shields.io/discord/1160636024167333979?label=discord&logo=discord&color=5865F2)](https://discord.gg/aTGPE3Myhk)

EnvHub is a simple tool to manage dotfiles and packages accross multiple machines.

Written in [Rust](https://www.rust-lang.org/), internally it uses [nix](https://nixos.org)/[homebrew](https://brew.sh)/[pkgx](https://pkgx.sh)/[devbox](https://www.jetpack.io/devbox) to manage packages and [home-manager](https://nix-community.github.io/home-manager/)/[stow](https://www.gnu.org/software/stow/) to manage dotfiles.

![Made with VHS](https://vhs.charm.sh/vhs-3jvaLIJUoMP67jfuoCMuex.gif)

## 🚚 Installation

Using [Cargo](https://crates.io) :

```bash
cargo install envhub
```

Using [Nix](https://nixos.org) :

```bash
nix profile install --experimental-features "nix-command flakes" github:tsirysndr/envhub
```

Compile from source :

```bash
git clone https://github.com/tsirysndr/envhub.git
cd envhub
nix develop --experimental-features "nix-command flakes"
cargo install --path crates/cli
envhub --help
```

Using [Homebrew](https://brew.sh) :

```bash
brew install tsirysndr/tap/envhub
```

Or download the latest release for your platform [here](https://github.com/tsirysndr/envhub/releases).

## 📚 Example

The following example will install the `hello` package, set the `EDITOR` environment variable to `vim`, and copy the `.screenrc` and `gradle.properties` files from the current directory to the home directory.

```hcl
# Path: envhub.hcl
packages = [
  "hello"
]

envs {
  "EDITOR" = "vim"
}

file ".screenrc" {
  source = ".screenrc"
}

file ".gradle/gradle.properties" {
  content = "org.gradle.daemon=true"
}
```

See [demo](demo) and [examples](examples) for a more complete example.
