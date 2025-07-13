# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Shank is a collection of Rust crates that extract Interface Definition Language (IDL) files from Solana programs using macro annotations. The generated IDL is consumed by tools like [solita](https://github.com/metaplex-foundation/solita) to generate TypeScript SDKs for Solana programs.

## Architecture

This is a Rust workspace containing 6 main crates:

- **shank** - Top-level crate that exports all macros, entry point for users
- **shank-macro** - Provides derive macros (`ShankAccount`, `ShankInstruction`, `ShankType`, etc.)
- **shank-macro-impl** - Core implementation of the derive macros and parsing logic
- **shank-idl** - Processes Rust source files to extract IDL from shank annotations
- **shank-render** - Generates Rust code (like PDA functions) from annotations
- **shank-cli** - Command-line tool that orchestrates IDL extraction

The workflow: Users annotate their Solana program structs/enums with shank macros → shank-cli analyzes the source code → produces JSON IDL → consumed by code generators.

## Common Commands

### Building and Testing
```bash
cargo test                    # Run all tests across workspace
cargo build                   # Build all crates
cargo build --release         # Release build
```

### CLI Usage
```bash
cargo install shank-cli       # Install CLI globally
shank idl                     # Extract IDL to ./idl/ directory
shank idl -o <dir>            # Extract IDL to custom directory
shank idl -r <crate-root>     # Specify program crate root
```

### Release Process
```bash
cargo test && cargo release <major|minor|patch>     # Dry run
cargo release <major|minor|patch> --execute         # Execute release
```

## Key Macro Annotations

- `#[derive(ShankAccount)]` - Marks account structs with optional `#[seeds]` for PDA generation
- `#[derive(ShankInstruction)]` - Marks instruction enums with `#[account]` attributes
- `#[derive(ShankType)]` - Marks custom types for IDL inclusion
- `#[derive(ShankBuilder)]` - Generates instruction builders 
- `#[derive(ShankContext)]` - Generates account context structs

### Field Attributes

- `#[padding]` - Marks field as padding in IDL
- `#[idl_type("TypeName")]` - Overrides field type in IDL
- `#[idl_name("name")]` - Renames field in IDL while keeping Rust field name
- `#[skip]` - Excludes field from IDL entirely

## Testing

Test files are organized in each crate's `tests/` directory with fixture files demonstrating expected behavior. Tests verify both macro expansion and IDL generation accuracy.

## Development Notes

- Uses Rust 2018 edition
- Release configuration in `release.toml` 
- Only releases from `master` branch
- Uses `rustfmt.toml` for consistent formatting
- Heavy use of `syn` and `quote` for macro implementation