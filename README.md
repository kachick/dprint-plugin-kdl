# dprint-plugin-kdl

[![CI - Nix Status](https://github.com/kachick/dprint-plugin-kdl/actions/workflows/nix.yml/badge.svg?branch=main)](https://github.com/kachick/dprint-plugin-kdl/actions/workflows/nix.yml?query=branch%3Amain+)

[KDL](https://github.com/kdl-org/kdl) formatter as a [dprint](https://github.com/dprint/dprint) WASM plugin

## WASM Schema versions

If you encounter any errors, please try updating this plugin or dprint itself.

| [schema version](https://github.com/dprint/dprint/blob/main/docs/wasm-plugin-development.md) | [dprint-plugin-kdl](https://github.com/kachick/dprint-plugin-kdl/releases) |
| -------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| [v4](https://github.com/dprint/dprint/pull/858)                                              | 0.2.0                                                                      |
| v3                                                                                           | 0.1.0                                                                      |

## Installation

```bash
dprint config add 'https://github.com/kachick/dprint-plugin-kdl/releases/download/0.2.0/plugin.wasm'
```

Currently there are no config options, all formatter features delegating to [upstream crate](https://github.com/kdl-org/kdl-rs).\
However this plugin preserves config key `kdl`.

```json
{
  "kdl": {
  },
  "plugins": [
    "https://github.com/kachick/dprint-plugin-kdl/releases/download/0.2.0/plugin.wasm"
  ]
}
```

## Link

<https://github.com/kdl-org/kdl/issues/393>
