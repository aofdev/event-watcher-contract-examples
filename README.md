# Event watcher contract examples

## Quick Start

```bash
# Run event watcher http
cargo run --bin event-watcher-http

# Run specific block number event watcher
cargo run --bin specific-block-event-watcher

# Run event watcher websocket
cargo run --bin event-watcher-ws

# Run event watcher http and send to discord
cargo run --bin event-watcher-http-discord
```

### Information for examples

> JSON-RPC Endpoint
https://docs.binance.org/smart-chain/developer/rpc.html

 Info | Hash |
--- | --- |
Contract Pancake LPs (CAKE-BNB) | [0x0eD7e52944161450477ee417DE9Cd3a859b14fD0](https://bscscan.com/address/0x0eD7e52944161450477ee417DE9Cd3a859b14fD0) |
Event `Swap` signature(keccak-256) | 0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822 |
Event `Transfer` signature(keccak-256) | 0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef |
Event `Mint` signature(keccak-256) | 0x4c209b5fc8ad50758f13e2e1088ba56a560dff690a1c6fef26394f4c03821c4f |

#### The event log, the topics field has 3 values:

```bash
topics[0] is the keccak-256 of the `Transfer(address,address,uint256)` canonical signature.
topics[1] is the value of the `_from` address.
topics[2] is the value of the `_to` address.
```