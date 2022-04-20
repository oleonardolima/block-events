# A terminal block explorer for mempool.space websocket

A terminal block explorer exposing the features available on [mempool.space websocket API](https://mempool.space/docs/api/websocket).

Currently available features are:
- All data feed from mempool.space for: blocks, mempool-blocks, live-2h-chart, and stats.
- Subscription for address related data: track-address.

<br>

## Requirements:

To use this CLI you must have rust and cargo installed in your computer, you can check it with:

``` sh
# check rust version, it should return its version if installed
rustc --version
# check cargo version, it should return its version if installed
cargo --version
```

If you do not have it installed, you can follow this tutorial from [The Rust Programming Language book](https://doc.rust-lang.org/book/ch01-01-installation.html)


<br>

## How to use:

If you have cargo and rust installed, you can run the following commands:

``` sh
# mainnet connection is default
cargo run -- track-address -a <your-btc-address>

# to use testnet
cargo run --  --endpoint mempool.space/testnet/api track-address -a <your-btc-address>
```

``` sh
# all feed [blocks, mempool-blocks, live-2h-chart, and stats] for mainnet:
cargo run -- blocks-data

# or all feed [blocks, mempool-blocks, live-2h-chart, and stats] for testnet:
cargo run -- --endpoint mempool.space/testnet/api blocks-data
```

## Compiling and using:
To compile and use it as a command in terminal with no need of cargo, you can use the following command:

``` sh
cargo install --path .
```
