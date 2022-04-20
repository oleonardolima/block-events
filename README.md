# A terminal block explorer for mempool.space websocket

A terminal block explorer exposing the features available on mempool.space websocket API [mempool.space websocket API](https://mempool.space/docs/api/websocket).

Currently available features are:
- All data feed from mempool.space for: blocks, mempool-blocks, live-2h-chart, and stats.
- Subscription for address related data: track-address.

## How to use:

To use this CLI you must have rust and cargo installed in your computer, you can check it wih:

``` sh
rustc --version
```

``` sh
cargo --version
```

If you have cargo and rust installed, you can run the following commands:

``` sh
# mainnet connection is default
cargo run -- track-address <your-btc-address>

# to use testnet
cargo run -- track-address <your-btc-address> --endpoint mempool.space/testnet/api
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
