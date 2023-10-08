# EnvHub

EnvHub is a simple tool to manage dotfiles and packages accross multiple machines.

Written in [Rust](https://www.rust-lang.org/), internally it uses [nix](https://nixos.org)/[homebrew](https://brew.sh)/[pkgx](https://pkgx.sh)/[devbox](https://www.jetpack.io/devbox) to manage packages and [home-manager](https://nix-community.github.io/home-manager/)/[stow](https://www.gnu.org/software/stow/) to manage dotfiles.

Note: This is a work in progress. 🏗️🚧

# Example

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