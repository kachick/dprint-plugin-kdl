# dprint-plugin-kdl

[![CI - Nix Status](https://github.com/kachick/dprint-plugin-kdl/actions/workflows/nix.yml/badge.svg?branch=main)](https://github.com/kachick/dprint-plugin-kdl/actions/workflows/nix.yml?query=branch%3Amain+)

[KDL](https://github.com/kdl-org/kdl) formatter as a [dprint](https://github.com/dprint/dprint) WASM plugin

## Installation

TODO: Make it possible to be installed from GitHub Releases

```bash
nix develop
task build
dprint config add "file:${PWD}/target/wasm32-unknown-unknown/release/dprint_plugin_kdl.wasm"
```

Currently there are no config options, all features are delegated to [upstream crate](https://github.com/kdl-org/kdl-rs).

## Link

<https://github.com/kdl-org/kdl/issues/393>
