# A CLI block explorer for mempool.space websocket

This is an initial approach in building a cli interface for consuming and streaming blocks, transactions and mempool statistics from [mempool.space websocket API](https://mempool.space/docs/api/websocket).

## Executing - WIP
As it's current implementation you need the following environment variables:
``` sh
export DEFAULT_NETWORK=testnet
export BLOCK_EXPLORER=mempool.space
```

And you can execute the code with the following command (it will run with the default commands and exported variables):
``` sh
cargo run
```