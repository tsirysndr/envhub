module CI.Build (environment, binary, arguments, tests) where

import Prelude
import Dagger (Client)
import Dagger.Container as Container
import Dagger.Modules as Module
import Dagger.Query as Query
import Dagger.Types (Container, Directory, File)
import Data.Maybe (Maybe(..))
import Effect.Aff (Aff)

arguments :: Module.Arguments { source :: Directory }
arguments = ado
  source <- Module.argument "source"
  in { source }

environment :: Client -> Directory -> Container
environment client source = Query.container client
  # Container.from "rust:1.99-bookworm"
  # Container.withEnvVariable "CARGO_HOME" "/usr/local/cargo"
  # Container.withMountedCache "/usr/local/cargo/registry" (Query.cacheVolume "envhub-rust-1.99-registry" client)
  # Container.withMountedCache "/usr/local/cargo/git" (Query.cacheVolume "envhub-rust-1.99-git" client)
  # Container.withMountedCache "/src/target" (Query.cacheVolume "envhub-rust-1.99-target" client)
  # Container.withDirectoryWith "/src" source
      (Container.defaultWithDirectoryOpts
        { exclude = Just [ ".git", ".dagger", ".fluentci", "target", ".devbox", "result", "result-*", "envhub", "dist", ".env", ".env.*" ] })
  # Container.withWorkdir "/src"

binary :: Client -> Directory -> File
binary client source = Container.file "/out/envhub" (environment client source
  # Container.withExec [ "cargo", "build", "--locked", "--release", "-p", "envhub" ]
  # Container.withExec [ "mkdir", "-p", "/out" ]
  # Container.withExec [ "cp", "target/release/envhub", "/out/envhub" ])

tests :: Client -> { source :: Directory } -> Aff String
tests client { source } = do
  void $ Container.sync $ environment client source
    # Container.withExec [ "cargo", "test", "--locked", "--workspace" ]
  pure "Rust workspace tests passed"
