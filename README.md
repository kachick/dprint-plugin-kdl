# dprint-plugin-kdl

[![CI - Nix Status](https://github.com/kachick/dprint-plugin-kdl/actions/workflows/nix.yml/badge.svg?branch=main)](https://github.com/kachick/dprint-plugin-kdl/actions/workflows/nix.yml?query=branch%3Amain+)

[KDL](https://github.com/kdl-org/kdl) formatter as a [dprint](https://github.com/dprint/dprint) WASM plugin

## Versions for KDL and dprint WASM Schema

| [dprint-plugin-kdl](https://github.com/kachick/dprint-plugin-kdl/releases) | [KDL](https://github.com/kdl-org/kdl/releases)                | [dprint WASM schema](https://github.com/dprint/dprint/blob/main/docs/wasm-plugin-development.md) |
| -------------------------------------------------------------------------- | ------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| 0.4.x                                                                      | [2.0](https://github.com/kdl-org/kdl/releases/tag/2.0.0), 1.0 | [v4](https://github.com/dprint/dprint/pull/858)                                                  |
| 0.3.x                                                                      | 2.0                                                           | v4                                                                                               |
| 0.2.x                                                                      | 1.0                                                           | v4                                                                                               |
| 0.1.x                                                                      | 1.0                                                           | v3                                                                                               |

## Installation

```bash
dprint config add 'kachick/kdl'
```

By default, it formats as KDL 2.0.

```json
{
  "kdl": {
  }
}
```

To format KDL v1 documents, set `"kdlVersion": "v1"`.

```json
{
  "kdl": {
    "kdlVersion": "v1"
  }
}
```

Some famous applications still use KDL v1 for now:

- [Zellij](https://github.com/zellij-org/zellij/issues/3891)
- [niri](https://github.com/niri-wm/niri/issues/888)

## Link

<https://github.com/kdl-org/kdl/issues/393>
