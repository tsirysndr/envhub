packages = [
  "hello"
]

file ".screenrc" {
  source = "dotfiles/.screenrc"
}

file ".bashrc.backup" {
  source = "dotfiles/.bashrc.backup"
}

file ".gradle/gradle.properties" {
  content = "org.gradle.daemon=true"
}

secrets = ["CLOUDFLARE_ROOT_KEY"]