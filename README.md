# dprint-plugin-kdl

[![CI - Nix Status](https://github.com/kachick/dprint-plugin-kdl/actions/workflows/nix.yml/badge.svg?branch=main)](https://github.com/kachick/dprint-plugin-kdl/actions/workflows/nix.yml?query=branch%3Amain+)

[KDL](https://github.com/kdl-org/kdl) formatter as a [dprint](https://github.com/dprint/dprint) WASM plugin

## Versions for KDL and dprint WASM Schema

| [dprint-plugin-kdl](https://github.com/kachick/dprint-plugin-kdl/releases) | [KDL](https://github.com/kdl-org/kdl/releases)           | [dprint WASM schema](https://github.com/dprint/dprint/blob/main/docs/wasm-plugin-development.md) |
| -------------------------------------------------------------------------- | -------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| 0.3.x                                                                      | [2.0](https://github.com/kdl-org/kdl/releases/tag/2.0.0) | [v4](https://github.com/dprint/dprint/pull/858)                                                  |
| 0.2.x                                                                      | 1.0                                                      | v4                                                                                               |
| 0.1.x                                                                      | 1.0                                                      | v3                                                                                               |

## Installation

```bash
dprint config add 'kachick/kdl'
```

Currently there are no config options, all formatter features delegating to [upstream crate](https://github.com/kdl-org/kdl-rs).

```json
{
  "kdl": {
  },
  "plugins": [
    "https://plugins.dprint.dev/kachick/kdl-0.3.0.wasm"
  ]
}
```

## Link

<https://github.com/kdl-org/kdl/issues/393>
