# NFT Server

Simple rust lib for NFT Metadata, and a basic axum server for delivering it

```
$ cargo build
$ cargo clippy
$ cargo run --bin example
```

## Usage

Implement a `MetadataGenerator` that asynchronously maps a token ID to token metadata, then call `serve` to serve it. See `bin/example.rs` as well as the `crate::generators::disk::LocalJson` generator

Consuming crates need to depend on the following:

- `async_trait`
- `url` (for convenience, the `Url` struct is re-exported)
- `ethers` (for convenience the `U256` struct is re-rexported)
