packages = [
  "jq"
]

envs {
  EDITOR = "vim"
}

file ".screenrc" {
  source = "dotfiles/.screenrc"
}

package_manager = "devbox"