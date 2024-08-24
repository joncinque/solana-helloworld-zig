# solana-helloworld-zig

A simple hello world program for Solana in Zig.

## Getting started

### Compiler

First, you need a zig compiler built with Solana's LLVM fork. See the README of
[solana-zig-bootstrap](https://github.com/joncinque/solana-zig-bootstrap)
on how to build it, or you can download it from the
[GitHub releases page](https://github.com/joncinque/solana-zig-bootstrap/releases).

There is also a helper script which will install it to the current directory:

```console
./install-solana-zig.sh
```

### Dependencies

This project opts for the zig package manager and the package declared at
[solana-program-sdk-zig](https://github.com/joncinque/solana-program-sdk-zig).

```console
zig fetch --save https://github.com/joncinque/base58-zig/archive/refs/tags/v0.13.3.tar.gz
zig fetch --save https://github.com/joncinque/solana-sdk-zig/archive/refs/tags/v0.13.1.tar.gz
```

### Build

You can build the program by running:

```console
./solana-zig/zig build
```

### Deploy

With the Solana tools, run:

```console
solana program deploy zig-out/lib/helloworld.so
```

## Command-line Interface

The repo has a simple CLI to send instructions to the program:

```console
cd cli
./test.sh
```

The CLI requires a Rust compiler to run, and the test script requires the Solana
CLI to startup a test validator.

## Program Tests

There are also integration tests run against the Agave runtime using the
[`solana-program-test` crate](https://crates.io/solana-program-test).

You can run these tests using the `test.sh` script:

```console
cd program-test/
./test.sh
```

These tests require a Rust compiler along with the solana-zig compiler.
