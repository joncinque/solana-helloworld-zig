# solana-zig-helloworld

A simple hello world program for Solana in Zig

## Getting started

### Compiler

First, you need a zig compiler built with Solana's LLVM fork. See the README of
[zig-bootstrap-solana](https://github.com/joncinque/zig-bootstrap-solana/tree/solana-1.37)
on how to build it.

### Get the submodules

Since zig doesn't really have package management yet, to get the upstream zig
package requirements, you need to fetch git submodules:

```console
$ git submodule update --init --recursive
```

### Build the program

You can build the program by running:

```console
$ path/to/your/zig build
```

### Deploy

With the normal Solana tools, run:

```console
$ solana program deploy zig-out/lib/libhelloworld.so
Program Id: <YOUR_PROGRAM_ADDRESS>
```

### Test

The repo has a simple CLI to send an instruction to the program:

```console
$ cd cli
$ cargo run -- ping <YOUR_PROGRAM_ADDRESS>
```
