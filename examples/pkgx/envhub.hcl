packages = [
  "vim.org/vim",
  "aws",
  "jq"
]

envs {
  EDITOR = "vim"
}

file ".screenrc" {
  source = "dotfiles/.screenrc"
}

package_manager = "pkgx"
