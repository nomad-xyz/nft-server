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

## Built-in server

The `axum` feature (on by default) adds a minimal
[axum server](https://docs.rs/nft-server/latest/nft_server/server/index.html)
preconfigured to serve token metadata.

After instantiating your metadata generator, you can serve it over http as
follows:

```rust
use nft_server::prelude::*;

async main() {
    let my_generator = ...;
    let addr = ([0, 0, 0, 0], 8080);
    serve_generator(my_generator, addr).await;
}
```

This server has the following routes:

- `/healthcheck` - returns 200
- `/` - calls `MetadataGenerator::contract_metadata()` and returns the result
- `/:token_id` - as a decimal number. Calls `MetadataGenerator::metadata_for(token_id)` and returns the result

e.g. `localhost:8080/0` will return json metadata for token 0
