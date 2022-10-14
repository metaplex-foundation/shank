# shank

[Shank](https://github.com/metaplex-foundation/shank) CLI that extracts IDL from your Rust Solana program code annotated with [shank macro
attributes](../shank-macro/README.md). This IDL can then be fed to
[solita](https://github.com/metaplex-foundation/solita) in order to generate low level
TypeScript SDK for that particular Rust program.

![shank-logo](../shank/assets/shank-logo.gif)

## Installation

### Via Cargo

```sh
cargo install shank-cli
```

### Via Yarn/Npm

_Coming soon ... _

## Overview

```
USAGE:
    shank <SUBCOMMAND>

OPTIONS:
    -h, --help    Print help information

SUBCOMMANDS:
    help    Print this message or the help of the given subcommand(s)
    idl
```

## IDL Extraction

```
USAGE:
    shank idl [OPTIONS]

OPTIONS:
    -h, --help                       Print help information
    -o, --out-dir <OUT_DIR>          Output directory for the IDL JSON [default: idl]
    -r, --crate-root <CRATE_ROOT>    Directory of program crate for which to generate the IDL
```

## LICENSE

Apache-2.0
