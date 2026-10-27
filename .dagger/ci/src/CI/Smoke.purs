module CI.Smoke (check) where

import Prelude
import CI.Build as Build
import Dagger (Client)
import Dagger.Container as Container
import Dagger.Query as Query
import Dagger.Directory as DaggerDirectory
import Dagger.Types (Directory)
import Effect.Aff (Aff)

check :: Client -> { source :: Directory } -> Aff String
check client { source } = do
  let nix = Query.container client # Container.from "nixos/nix:2.28.5"
      ready = Query.container client
        # Container.from "rust:1.99-bookworm"
        # Container.withDirectory "/nix" (Container.directory "/nix" nix)
        # Container.withEnvVariable "NIX_REMOTE" "local"
        # Container.withEnvVariable "NIX_CONFIG" "experimental-features = nix-command flakes\nsandbox = false\nbuild-users-group =\n"
        # Container.withEnvVariable "NIX_PATH" "nixpkgs=channel:nixos-unstable"
        # Container.withEnvVariable "PATH" "/root/.nix-profile/bin:/etc/profiles/per-user/root/bin:/nix/var/nix/profiles/default/bin:/usr/local/bin:/usr/bin:/bin"
        # Container.withEnvVariable "USER" "root"
        # Container.withEnvVariable "CLOUDFLARE_ROOT_KEY" "123"
        # Container.withFile "/usr/local/bin/envhub" (Build.binary client source)
        # Container.withExec [ "git", "config", "--global", "user.email", "ci@example.invalid" ]
        # Container.withExec [ "git", "config", "--global", "user.name", "EnvHub CI" ]
        # Container.withExec [ "nix", "--version" ]
        # Container.withExec [ "mkdir", "-p", "/demo" ]
        # Container.withWorkdir "/demo"
        # Container.withExec [ "envhub", "init" ]
        # Container.withExec [ "envhub", "use" ]
        # Container.withExec [ "envhub", "status" ]
        # Container.withExec [ "hello" ]
        # Container.withExec [ "envhub", "unuse" ]
        # Container.withExec [ "envhub", "status" ]
  void $ Container.sync $ ready
    # Container.withDirectory "/demo-project" (DaggerDirectory.directory "demo" source)
    # Container.withWorkdir "/demo-project"
    # Container.withExec [ "envhub", "init" ]
    # Container.withExec [ "envhub", "use" ]
    # Container.withExec [ "hello" ]
    # Container.withExec [ "envhub", "unuse" ]
  pure "Fresh and repository demo environments activated and deactivated"
