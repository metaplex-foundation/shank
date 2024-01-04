# Shank [![Build+Test](https://github.com/metaplex-foundation/shank/actions/workflows/build+test.yml/badge.svg)](https://github.com/metaplex-foundation/shank/actions/workflows/build+test.yml)

Collection of shank crates used to annotate Rust programs in order to extract IDL via the
included CLI tool. This IDL is used by [solita](https://github.com/metaplex-foundation/solita) in order to generate program SDKs.

![shank-logo](./shank/assets/shank-logo.gif)

## Installation

For _usage_ and _installation_ see the [shank-cli Readme](./shank-cli/README.md).

## Crates

- [shank](./shank) top level crate to be installed and included in your library to add macro
  annotations
- [shank_cli](./shank-cli) the CLI tool that extracts IDL from a specified crate into a file
- [shank-macro](./shank-macro) provides the _derive_ macros shank uses
- [shank-macro-impl](./shank-macro-impl) implements and tests the _derive_ macros
- [shank-idl](./shank-idl) processes files of a crate in order to discover _shank_ macros
  annotations and convert annotated types into an [solita](https://github.com/metaplex-foundation/solita) compatible IDL
- [shank-render](./shank-render) generates Rust `impl` blocks from specific annotations like
  account `seeds` 

## Development

Fork the repo makes some changes and make sure that all is dandy by running `cargo test`. Then
provide a pull request.

If you are a contributor with access to publish to crates.io do the below in order to publish a
new version. NOTE that this only works from the _master_ branch and should be performed _after_
merging a PR into master.

```sh
cargo test && cargo release <major|minor|patch>
```

The above runs all tests and dry-runs the release process. You should verify closely what it is
about to do and then re-run the release command as shown below.

```sh
cargo release <major|minor|patch> --execute
```

## LICENSE

Apache-2.0
