# dprint-plugin-kdl

[![npm version](https://img.shields.io/npm/v/@dprint/kdl.svg)](https://www.npmjs.com/package/@dprint/kdl) [![CI](https://github.com/dprint/dprint-plugin-kdl/workflows/CI/badge.svg)](https://github.com/dprint/dprint-plugin-kdl/actions?query=workflow%3ACI)

Adapter for [kdl-rs](https://github.com/kdl-org/kdl-rs) for use as a formatting plugin in [dprint](https://github.com/dprint/dprint). It formats [KDL](https://github.com/kdl-org/kdl) files (`.kdl`).

Originally created by [Kenichi Kamiya](https://github.com/kachick).

## Install

[Install](https://dprint.dev/install/) and [setup](https://dprint.dev/setup/) dprint.

Then in your project's directory with a dprint.json file, run:

```shellsession
dprint add kdl
```

## Configuration

By default, it formats as KDL 2.0 and works with no configuration.

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

If your repository has mixed versions of KDL files, consider adding a version marker line to the top of your files:

```kdl
/- kdl-version 1
simplified_ui true
```

```kdl
/- kdl-version 2
simplified_ui #true
```

This version marker is defined as an optional hint in the [KDL specification](https://kdl.dev/spec#compatibility).\
When placed on the first line, this plugin formats the file with that version,\
even if `kdlVersion` in `dprint.json` says otherwise.

## JS Formatting API

- [JS Formatter](https://github.com/dprint/js-formatter) - Browser/Deno and Node
- [npm package](https://www.npmjs.com/package/@dprint/kdl)

## Versioning

This repo automatically upgrades to the latest version of kdl-rs once a day. You can check which version of kdl-rs is being used by looking at the `kdl` entry in the Cargo.toml file in this repo:

https://github.com/dprint/dprint-plugin-kdl/blob/main/Cargo.toml

At the moment, the version of this plugin does not reflect the version of kdl-rs.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) and [NOTICE](NOTICE) files for details.
