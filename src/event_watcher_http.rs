use std::str::FromStr;
use std::time;
use web3::{
    api::Eth,
    confirm,
    futures::{self, StreamExt},
    types::{BlockNumber, Filter, FilterBuilder, Log, H160, H256, U64},
    Transport,
};

#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    let web3 = web3::Web3::new(web3::transports::Http::new(
        "https://bsc-dataseed.binance.org/",
    )?);

    // to be input
    //
    let start_contract_block = BlockNumber::Earliest;
    let confirmations = 15;
    let poll_interval = time::Duration::from_millis(1000);
    //
    //

    let logs_stream = web3
        .eth_filter()
        .create_logs_filter(filter(start_contract_block))
        .await?
        .stream(poll_interval);
    futures::pin_mut!(logs_stream);

    while let Some(result_log) = logs_stream.next().await {
        let log: Log = match result_log {
            Ok(log) => log,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        let log_hash = match log.block_hash {
            Some(log_hash) => log_hash,
            None => {
                println!("how could a log have no hash ???");
                continue;
            }
        };

        let eth = web3.eth();
        confirm::wait_for_confirmations(
            web3.eth(),
            web3.eth_filter(),
            poll_interval,
            confirmations,
            || transaction_receipt_block_number_check(&eth, log_hash),
        )
        .await?;

        println!("----------------------------------------");
        println!("Block Number: {}", log.block_number.unwrap());
        println!("Event Address: {:?}", log.topics[0]);
        println!("From Address: {}", log.topics[1]);
        println!("To Address: {}", log.topics[2]);
        println!("Data: {}", format!("0x{}", hex::encode(log.data.0)));
    }
    Ok(())
}

fn filter(start_at: BlockNumber) -> Filter {
    let contract_address: H160 =
        H160::from_str("0x0eD7e52944161450477ee417DE9Cd3a859b14fD0").unwrap();

    let signature_event_swap: H256 =
        H256::from_str("0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822")
            .unwrap();

    let signature_event_transfer: H256 =
        H256::from_str("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef")
            .unwrap();

    FilterBuilder::default()
        .address(vec![contract_address])
        .topics(
            Some(vec![signature_event_swap, signature_event_transfer]),
            None,
            None,
            None,
        )
        .from_block(start_at)
        .to_block(BlockNumber::Latest)
        .build()
}

async fn transaction_receipt_block_number_check<T: Transport>(
    eth: &Eth<T>,
    hash: H256,
) -> web3::error::Result<Option<U64>> {
    let receipt = eth.transaction_receipt(hash).await?;
    Ok(receipt.and_then(|receipt| receipt.block_number))
}
