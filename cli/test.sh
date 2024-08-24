#!/usr/bin/env bash

ZIG="$1"
ROOT_DIR="$(cd "$(dirname "$0")"/..; pwd)"
if [[ -z "$ZIG" ]]; then
  ZIG="$ROOT_DIR/solana-zig/zig"
fi

set -e
$ZIG build --summary all --verbose
SBF_OUT_DIR="$ROOT_DIR/zig-out/lib" cargo test --manifest-path "$ROOT_DIR/cli/Cargo.toml"
