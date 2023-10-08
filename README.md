# EnvHub

EnvHub is a simple tool to manage dotfiles and packages accross multiple machines.

Written in [Rust](https://www.rust-lang.org/), internally it uses [nix](https://nixos.org)/[homebrew](https://brew.sh)/[pkgx](https://pkgx.sh)/[devbox](https://www.jetpack.io/devbox) to manage packages and [home-manager](https://nix-community.github.io/home-manager/)/[stow](https://www.gnu.org/software/stow/) to manage dotfiles.