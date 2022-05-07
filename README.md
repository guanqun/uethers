uethers

A very lightweight synchronous Ethereum client.

Supported RPC endpoints so far:

1. eth_getBalance
2. eth_getStorageAt
3. eth_blockNumber
4. eth_getTransactionCount
5. eth_getCode
6. eth_getBlockByNumber
7. eth_getTransactionByHash


Why?

1. For some small projects, I don't want to pull in lots of async crates used by `ethers-rs`.
2. The main use case for me is to implement EVM backend which just uses some preliminary RPC endpoints.