# dprint-plugin-kdl

[![npm version](https://img.shields.io/npm/v/@kachick/dprint-plugin-kdl.svg)](https://www.npmjs.com/package/@kachick/dprint-plugin-kdl) [![CI - Nix Status](https://github.com/kachick/dprint-plugin-kdl/actions/workflows/nix.yml/badge.svg?branch=main)](https://github.com/kachick/dprint-plugin-kdl/actions/workflows/nix.yml?query=branch%3Amain+)

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

If you only format KDL v1 documents, you can use the "kdlVersion" option:

```json
{
  "kdl": {
    "kdlVersion": "v1"
  }
}
```

### Mixed v1 and v2 Files

Some famous applications still use KDL v1 for now:

- [Zellij](https://github.com/zellij-org/zellij/issues/3891)
- [niri](https://github.com/niri-wm/niri/issues/888)

If your repository has mixed versions of KDL files,
consider adding a version marker line to the top of your files:

```kdl
/- kdl-version 1
simplified_ui true
```

```kdl
/- kdl-version 2
simplified_ui #true
```

This version marker is defined as an optional hint in the [KDL specification](https://kdl.dev/spec#compatibility).\
It is very helpful to avoid version issues during migration.\
When placed on the first line, this plugin formats the file with that version, even if `kdlVersion` in `dprint.json` says otherwise.
