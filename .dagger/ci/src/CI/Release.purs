module CI.Release (arguments, build) where

import Prelude
import CI.Build as Build
import Dagger (Client)
import Dagger.Container as Container
import Dagger.Modules as Module
import Dagger.Types (Directory)
import Data.Foldable (all)
import Data.Maybe (Maybe(..))
import Data.String (Pattern(..), contains)
import Data.String.CodeUnits (singleton, toCharArray)
import Effect.Aff (Aff, throwError)
import Effect.Exception (error)

type Args = { source :: Directory, target :: String, version :: String }

arguments :: Module.Arguments Args
arguments = ado
  source <- Module.argument "source"
  target <- Module.argument "target"
  version <- Module.argument "version"
  in { source, target, version }

build :: Client -> Args -> Aff Directory
build client { source, target, version } = do
  unless (target == "x86_64-unknown-linux-gnu" || target == "aarch64-unknown-linux-gnu") $
    throwError (error "Unsupported Linux release target")
  unless (version /= "" && all (\c -> contains (Pattern (singleton c))
    "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789._+-") (toCharArray version)) $
    throwError (error "Invalid release version")
  let name = "envhub_" <> version <> "_" <> target <> ".tar.gz"
      packaged = Build.environment client source
        # Container.withExec [ "rustup", "target", "add", target ]
        # Container.withExec [ "cargo", "build", "--locked", "-p", "envhub", "--release", "--target", target ]
        # Container.withExec [ "mkdir", "-p", "/stage", "/out" ]
        # Container.withExec [ "cp", "target/" <> target <> "/release/envhub", "/stage/envhub" ]
        # Container.withExec [ "tar", "-C", "/stage", "-czf", "/out/" <> name, "envhub" ]
        # Container.withWorkdir "/out"
        # Container.withExecWith [ "sha256sum", name ]
            (Container.defaultWithExecOpts { redirectStdout = Just ("/out/" <> name <> ".sha256") })
  pure $ Container.directory "/out" packaged
