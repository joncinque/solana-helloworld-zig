#!/usr/bin/env bash

ZIG="$1"
if [[ -z "$ZIG" ]]; then
  ZIG="../../zig-x86_64-linux-gnu-baseline/zig"
fi
set -e
$ZIG build --summary all
SBF_OUT_DIR=../zig-out/lib cargo test
