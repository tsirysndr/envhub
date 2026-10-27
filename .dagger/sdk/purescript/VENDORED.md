# Vendored PureScript Dagger SDK

Copied from `tsirysndr/atradio.fm` commit `d3d59df`, under `.dagger/sdk/purescript`.
The upstream SDK is https://github.com/tsirysndr/dagger-sdk/tree/156eb76/purescript
(Dagger v1.0.0-beta.21). Preserve LICENSE, NOTICE, and schema/LICENSE.

This snapshot includes the Node 24 runtime, HTTP transport deadline fix, and
clean-checkout module bootstrapping. Bootstrap Go bindings are tracked because
beta module execution does not generate them automatically. The module declares
an explicit Spago path dependency so its Git-ignored SDK directory is resolved.

Refresh from a reviewed snapshot and retain these fixes. No sibling SDK checkout
is needed at runtime.
