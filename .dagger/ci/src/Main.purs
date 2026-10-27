module Main where

import Prelude (Unit)
import CI.Pipeline as Pipeline
import CI.Runtime as Runtime
import Effect (Effect)

main :: Effect Unit
main = Runtime.runModule Pipeline.definitions
