# solana-helloworld-zig

A simple hello world program for Solana in Zig.

## Getting started

### Compiler

First, you need a zig compiler built with Solana's LLVM fork. See the README of
[solana-zig-bootstrap](https://github.com/joncinque/solana-zig-bootstrap)
on how to build it, or you can download it from the
[GitHub releases page](https://github.com/joncinque/solana-zig-bootstrap/releases).

### Submodules

Since zig's package manager is still in development, this project opts for
directly including the upstream Solana package requirements, hosted at
[solana-sdk-zig](https://github.com/joncinque/solana-sdk-zig).

This repo uses git submodules:

```console
$ git submodule update --init --recursive
```

### Build

You can build the program by running:

```console
$ /path/to/your/zig build
```

### Deploy

With the Solana tools, run:

```console
$ solana program deploy zig-out/lib/libhelloworld.so
Program Id: <YOUR_PROGRAM_ADDRESS>
```

## Command-line Interface

The repo has a simple CLI to send instructions to the program:

```console
$ cd cli
$ ./test.sh
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
