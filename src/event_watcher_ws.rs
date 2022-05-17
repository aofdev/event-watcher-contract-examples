use web3::{
    futures::{future, StreamExt},
    types::{BlockNumber, FilterBuilder, Log, H160, H256},
};

use std::str::FromStr;

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    // https://docs.binance.org/smart-chain/developer/rpc.html
    let web3 = web3::Web3::new(
        web3::transports::WebSocket::new("wss://bsc-ws-node.nariox.org:443").await?,
    );

    let contract_address: H160 =
        H160::from_str("0x0eD7e52944161450477ee417DE9Cd3a859b14fD0").unwrap();

    let signature_event_swap: H256 =
        H256::from_str("0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822")
            .unwrap();

    let signature_event_transfer: H256 =
        H256::from_str("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef")
            .unwrap();

    let filter = FilterBuilder::default()
        .address(vec![contract_address])
        .topics(
            Some(vec![signature_event_swap, signature_event_transfer]),
            None,
            None,
            None,
        )
        .from_block(BlockNumber::Earliest)
        .to_block(BlockNumber::Latest)
        .build();

    let sub = web3.eth_subscribe().subscribe_logs(filter).await?;

    sub.for_each(|result_log| {
        let log: Log = result_log.unwrap();
        println!("----------------------------------------");
        println!("Block Number: {}", log.block_number.unwrap());
        println!("Event Address: {:?}", log.topics[0]);
        println!("From Address: {}", log.topics[1]);
        println!("To Address: {}", log.topics[2]);
        println!("Data: {}", format!("0x{}", hex::encode(log.data.0)));
        future::ready(())
    })
    .await;

    Ok(())
}
