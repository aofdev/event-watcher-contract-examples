use std::collections::HashMap;
use std::str::FromStr;
use strum::Display;
use strum_macros::EnumString;

use std::time;
use web3::{
    futures::{self, StreamExt},
    types::{BlockNumber, FilterBuilder, Log, H160, H256},
};

const WEBHOOK_URL: &str = "";
const CONTRACT_ADDRESS: &str = "0x0eD7e52944161450477ee417DE9Cd3a859b14fD0";

#[derive(Debug, PartialEq, EnumString, Display)]
enum EventType {
    #[strum(serialize = "0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822")]
    Swap,
    #[strum(serialize = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef")]
    Transfer,
    #[strum(serialize = "0x4c209b5fc8ad50758f13e2e1088ba56a560dff690a1c6fef26394f4c03821c4f")]
    Mint,
    None,
}

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    let web3 = web3::Web3::new(web3::transports::Http::new(
        "https://bsc-dataseed.binance.org/",
    )?);

    let contract_address: H160 = H160::from_str(CONTRACT_ADDRESS).unwrap();
    let signature_event_mint: H256 = H256::from_str(EventType::Mint.to_string().as_str()).unwrap();
    let signature_event_swap: H256 = H256::from_str(EventType::Swap.to_string().as_str()).unwrap();
    let signature_event_transfer: H256 =
        H256::from_str(EventType::Transfer.to_string().as_str()).unwrap();

    let filter = FilterBuilder::default()
        .address(vec![contract_address])
        .topics(
            Some(vec![
                signature_event_mint,
                signature_event_swap,
                signature_event_transfer,
            ]),
            None,
            None,
            None,
        )
        .from_block(BlockNumber::Earliest)
        .to_block(BlockNumber::Latest)
        .build();

    let filter = web3.eth_filter().create_logs_filter(filter).await?;

    let logs_stream = filter.stream(time::Duration::from_millis(100));
    futures::pin_mut!(logs_stream);

    while let Some(result_log) = logs_stream.next().await {
        let log: Log = result_log.unwrap();
        println!("----------------------------------------");
        println!("Block Number: {}", log.block_number.unwrap());
        let link_transaction = format!(
            "https://bscscan.com/tx/{:?}",
            H256::from_slice(&log.transaction_hash.unwrap().0)
        );
        let event_signature = format!("{:?}", H256::from_slice(&log.topics[0].0));
        let from_address = format!("{:?}", H256::from_slice(&log.topics[1].0));
        let to_address = format!(
            "{:?}",
            H256::from_slice(&log.topics.get(2).unwrap_or(&H256::default()).0)
        );

        // Send to Discord
        let mut msg = HashMap::new();
        msg.insert(
            "content",
            format!(
                "Block Number: {} \nTransaction: {} \nEvent: {:?} \nFrom Address: {} \nTo Address: {} \n",
                log.block_number.unwrap(),
                link_transaction,
                EventType::from_str(event_signature.as_str()).unwrap_or(EventType::None),
                from_address,
                to_address
                ),
            );

        msg.insert("username", "EventWatcherBot".to_string());
        let client = reqwest::Client::new();
        let _ = client.post(WEBHOOK_URL).json(&msg).send().await.unwrap();
    }
    Ok(())
}
