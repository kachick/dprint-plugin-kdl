#!/usr/bin/env bash

set -euxo pipefail

action="$1"
test_name="${2:-}"

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
PLUGIN_PATH="${PLUGIN_PATH:-"${repo_root}/target/wasm32-unknown-unknown/debug/dprint_plugin_kdl.wasm"}"

case "${action}" in
  check)
    cd "${repo_root}/tests/${test_name}"
    for expected in *expected*.kdl; do
      dprint check --plugins="${PLUGIN_PATH}" "$expected"
    done
    for raw in *raw*.kdl; do
      expected="${raw//raw/expected}"
      diff <(cat "$raw" | dprint fmt --stdin "$raw" --plugins="${PLUGIN_PATH}") "$expected"
    done
    ;;
  bump)
    cd "${repo_root}/tests/${test_name}"
    for raw in *raw*.kdl; do
      expected="${raw//raw/expected}"
      cat "$raw" | dprint fmt --stdin "$raw" --plugins="${PLUGIN_PATH}" > "$expected"
    done
    ;;
  error-cases)
    if dprint fmt --stdin test.kdl --plugins="${PLUGIN_PATH}" < "${repo_root}/tests/v1-zellij/raw.kdl" 2>/dev/null; then
      echo "Expected v1 syntax to fail under v2, but it succeeded" >&2
      exit 1
    fi
    if printf "/- kdl-version 3\nnode 1\n" | dprint fmt --stdin test.kdl --plugins="${PLUGIN_PATH}" 2>/dev/null; then
      echo "Expected unsupported version 3 to fail, but it succeeded" >&2
      exit 1
    fi
    ;;
  *)
    echo "Unknown action: ${action}" >&2
    exit 1
    ;;
esac
