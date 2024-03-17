#!/usr/bin/env bash

# Make sure a solana-test-validator is running!
slot=$(solana -ul slot)
if [[ $? -ne 0 ]]; then
  echo "Please start a test validator with solana-test-validator before running this script"
  exit 1
fi

set -e
../../zig-native-linux-gnu-native/zig build --summary all
program_id=$(solana-keygen pubkey ../zig-out/lib/libhelloworld-keypair.json)
solana -ul program deploy ../zig-out/lib/libhelloworld.so --program-id ../zig-out/lib/libhelloworld-keypair.json
cargo run -- -ul ping --dry-run $program_id
