use std::str::FromStr;
use std::time;
use web3::{
    futures::{self, StreamExt},
    types::{BlockNumber, Filter, FilterBuilder, Log, H160, H256, U64},
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
    let poll_interval = time::Duration::from_millis(100);
    //
    //

    let logs_stream = web3
        .eth_filter()
        .create_logs_filter(filter(start_contract_block))
        .await?
        .stream(poll_interval);
    futures::pin_mut!(logs_stream);

    while let Some(result_log) = logs_stream.next().await {
        println!("log received.");
        let log: Log = match result_log {
            Ok(log) => log,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        let log_block_number = match log.block_number {
            Some(log_block_number) => log_block_number,
            None => {
                println!("log has no block number");
                continue;
            }
        };

        println!("event block number: {}", log.block_number.unwrap());
        println!("current block number: {}", web3.eth().block_number().await?);

        let mut current_block_number = web3.eth().block_number().await?;

        if !is_confirmed(log_block_number, current_block_number, confirmations) {
            println!(
                "confirmation needed: {}",
                log_confirmation_needed(log_block_number, current_block_number, confirmations)
                    as usize
            );
            println!("fetching latest block number...");
            let block_stream = web3
                .eth_filter()
                .create_blocks_filter()
                .await?
                .stream(poll_interval)
                .skip(
                    log_confirmation_needed(log_block_number, current_block_number, confirmations)
                        as usize,
                );
            futures::pin_mut!(block_stream);

            while !is_confirmed(log_block_number, current_block_number, confirmations) {
                println!(
                    "log block number {} vs current block number {}; not confirmed",
                    log.block_number.unwrap().low_u64(),
                    web3.eth().block_number().await?
                );
                let _ = block_stream.next().await;
                current_block_number = web3.eth().block_number().await?;
            }
        }

        println!(
            "log block number {} vs current block number {}; confirmed !",
            log.block_number.unwrap().low_u64(),
            web3.eth().block_number().await?
        );
        println!("Log Confirmed.");
        println!("Current Block Number: {}", web3.eth().block_number().await?);

        process_log(log).await;
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

fn is_confirmed(log_block_number: U64, current_block_number: U64, confirmations: u64) -> bool {
    log_confirmation_needed(log_block_number, current_block_number, confirmations) == 0
}

fn log_confirmation_needed(
    log_block_number: U64,
    current_block_number: U64,
    confirmations: u64,
) -> u64 {
    (log_block_number.low_u64() + confirmations)
        .checked_sub(current_block_number.low_u64())
        .unwrap_or(0)
}

async fn process_log(log: Log) {
    println!("----------------------------------------");
    println!("Block Number: {}", log.block_number.unwrap());
    println!("Event Address: {:?}", log.topics[0]);
    println!("From Address: {}", log.topics[1]);
    println!("To Address: {}", log.topics[2]);
    println!("Data: {}", format!("0x{}", hex::encode(log.data.0)));
    println!("----------------------------------------");
    println!("*****");
    println!("*****");
}
