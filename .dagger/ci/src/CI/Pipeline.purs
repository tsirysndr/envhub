module CI.Pipeline (definitions) where

import CI.Build as Build
import CI.Smoke as Smoke
import CI.Release as Release
import Dagger.Modules as Module

definitions :: Array Module.ObjectDefinition
definitions =
  [ Module.object_ "ProjectCi" {}
      [ Module.function "test" "Run Rust 1.99 workspace tests." Build.arguments Build.tests
      , Module.function "smoke" "Exercise Nix environment activation in disposable containers." Build.arguments Smoke.check
      , Module.function "build" "Build a Linux release archive and checksum." Release.arguments Release.build
      ]
  ]
